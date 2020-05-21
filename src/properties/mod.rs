use core_foundation::{
    base::{
        CFEqual,
        TCFType,
    },
    string::{
        CFString, 
        CFStringRef,
    },
};

use std::ffi::c_void;

pub(crate) fn match_property_keys(key1: CFStringRef, key2: CFStringRef) -> bool {
    if key1.is_null() || key2.is_null() { return false; }
    
    let key1 = key1 as *const c_void;
    let key2 = key2 as *const c_void;
    return unsafe { CFEqual(key1, key2) } == 1;
}

macro_rules! convert_property_key_set {
    ($match_var:ident, $($prop_name:ident -> $key_name:expr,)*) => {
        Some(match $match_var {
            $(x if match_property_keys(x, unsafe { $key_name }) => $prop_name,)*
            _ => return None,
        })
    }
}

pub mod boolean;
pub mod constants;
pub mod integer;
pub mod string;

pub use self::{
    boolean::{
        BooleanProperty,
        BooleanPropertyName,
    },
    integer::{
        IntegerProperty,
        IntegerPropertyName,
    },
    string::{
        StringProperty,
        StringPropertyName,
    },
};

pub(crate) use self::{
    integer::{
        int_property_inner,
        set_int_property_inner,
    },
    string::{
        string_property_inner,
        set_string_property_inner,
    },
};

/// The name of a MIDI object property, as returned from a [`Notification`](enum.Notification.html)
///
/// If the property name is one of the known CoreMIDI constants, the appropriate
/// variant will be used based on the type of data that can be accessed through
/// that property. Otherwise, the name will be in `Other(String)`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum PropertyName {
    String(StringProperty),
    Integer(IntegerProperty),
    Boolean(BooleanProperty),
    Other(String),
}

impl From<CFStringRef> for PropertyName {
    fn from(string_ref: CFStringRef) -> Self {
        StringProperty::try_from_constant_string_ref(string_ref).map(Into::into)
            .or_else(|| BooleanProperty::try_from_constant_string_ref(string_ref).map(Into::into))
            .or_else(|| IntegerProperty::try_from_constant_string_ref(string_ref).map(Into::into))
            .unwrap_or_else(|| {
                let name: CFString = unsafe { TCFType::wrap_under_get_rule(string_ref) };
                name.to_string().into()
            })
    }
}

impl From<StringProperty> for PropertyName {
    fn from(prop: StringProperty) -> Self {
        PropertyName::String(prop)
    }
}

impl From<IntegerProperty> for PropertyName {
    fn from(prop: IntegerProperty) -> Self {
        PropertyName::Integer(prop)
    }
}

impl From<BooleanProperty> for PropertyName {
    fn from(prop: BooleanProperty) -> Self {
        PropertyName::Boolean(prop)
    }
}

impl From<String> for PropertyName {
    fn from(string: String) -> Self {
        PropertyName::Other(string)
    }
}

/// Types that implement this can represent a set of standard CoreMIDI property
/// name constants
pub trait StandardProperty : Into<CFStringRef> + Copy + Clone { }

/// The name of a MIDI object property, with info about the type of data this
/// property can be used to access.
///
/// Can either be one of the CoreMIDI-defined property name constants or a
/// custom property name.
///
/// You should typically not create this directly, since it can be created from
/// any `&str`, `String`, or [`StandardProperty`](trait.StandardProperty.html) using `std::convert::From` or
/// `std::convert::Into`.
#[derive(Clone, Debug)]
pub enum TypedPropertyName<K> where
    K: StandardProperty,
{
    Standard(K),
    Other(CFString),
}

impl<K> TypedPropertyName<K> where
    K: StandardProperty,
{
    fn custom<S: AsRef<str>>(name: S) -> Self {
        TypedPropertyName::Other(CFString::new(name.as_ref()))
    }

    /// Return a raw CFStringRef pointing to this property key
    ///
    /// Note: Should never be exposed externally
    pub(crate) fn as_string_ref(&self) -> CFStringRef {
        match self {
            TypedPropertyName::Standard(constant) => Into::into(*constant),
            TypedPropertyName::Other(custom) => custom.as_concrete_TypeRef(),
        }
    }
}

impl<K> From<K> for TypedPropertyName<K> where
    K: StandardProperty,
{
    fn from(prop: K) -> Self {
        TypedPropertyName::Standard(prop)
    }
}

impl<K> From<String> for TypedPropertyName<K> where
    K: StandardProperty,
{
    fn from(s: String) -> Self {
        TypedPropertyName::custom(s)
    }
}

impl<K> From<&str> for TypedPropertyName<K> where
    K: StandardProperty,
{
    fn from(s: &str) -> Self {
        TypedPropertyName::custom(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use coremidi_sys::kMIDIPropertyUniqueID;
    use crate::Client;

    // Note: Until a Client is started, Property constants won't have the
    // correct string value, since CoreMIDI initializes them when a client
    // is created
    fn setup() -> Client {
        Client::new("Test Client").unwrap()
    }

    #[test]
    fn test_property_from_constant() {
        let _client = setup();

        let property = PropertyName::from(unsafe { kMIDIPropertyUniqueID });

        assert_eq!(property, PropertyName::Integer(IntegerProperty::UniqueId))
    }

    #[test]
    fn test_property_from_known_string() {
        let _client = setup();

        // Known to be the value of kMIDIPropertyOffline
        let name = CFString::new("offline");
        let property = PropertyName::from(name.as_concrete_TypeRef());

        assert_eq!(property, PropertyName::Boolean(BooleanProperty::Offline))
    }

    #[test]
    fn test_property_from_unknown_string() {
        let _client = setup();

        const PROPERTY_STR: &str = "nOt_A_rEaL_cOrEmIdI_pRoPerTy";
        let name = CFString::new(PROPERTY_STR);
        let property = PropertyName::from(name.as_concrete_TypeRef());

        assert_eq!(property, PropertyName::Other(PROPERTY_STR.to_string()))
    }
}