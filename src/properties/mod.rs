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

use std::{
    ffi::c_void,
    fmt,
};

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
        BooleanPropertyKey,
    },
    integer::{
        IntegerProperty,
        IntegerPropertyKey,
    },
    string::{
        StringProperty,
        StringPropertyKey,
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

/// Types that implement this can represent a set of standard CoreMIDI property
/// key constants.
pub trait StandardProperty : Into<CFStringRef> + Into<PropertyName> + Copy + Clone { }

/// The name of a MIDI object property that can be used to access one specific
/// type of data.
///
/// This type only exists to enable easy conversions for the arguments to
/// [`Object`](struct.Object.html)'s getter and setter methods. Can either
/// be one of the CoreMIDI-defined property key constants or a custom property
/// name.
///
/// You should not create this type directly, since it can be created from any
/// `&str`, `String`, or [`StandardProperty`](trait.StandardProperty.html) using `std::convert::From` or
/// `std::convert::Into`.
#[derive(Clone, Debug)]
pub enum TypedPropertyKey<K> where
    K: StandardProperty,
{
    Standard(K),
    Other(CFString),
}

impl<K> TypedPropertyKey<K> where
    K: StandardProperty,
{
    fn custom<S: AsRef<str>>(name: S) -> Self {
        TypedPropertyKey::Other(CFString::new(name.as_ref()))
    }

    /// Return a raw CFStringRef pointing to this property key
    ///
    /// Note: Should never be exposed externally
    pub(crate) fn as_string_ref(&self) -> CFStringRef {
        match self {
            TypedPropertyKey::Standard(constant) => Into::into(*constant),
            TypedPropertyKey::Other(custom) => custom.as_concrete_TypeRef(),
        }
    }
}

impl<K> From<K> for TypedPropertyKey<K> where
    K: StandardProperty,
{
    fn from(prop: K) -> Self {
        TypedPropertyKey::Standard(prop)
    }
}

impl<K> From<String> for TypedPropertyKey<K> where
    K: StandardProperty,
{
    fn from(s: String) -> Self {
        TypedPropertyKey::custom(s)
    }
}

impl<K> From<&str> for TypedPropertyKey<K> where
    K: StandardProperty,
{
    fn from(s: &str) -> Self {
        TypedPropertyKey::custom(s)
    }
}

/// The name of a MIDI object property, as returned from a [`Notification`](enum.Notification.html).
///
/// When matching the value against standard CoreMIDI properties, `==` can be
/// used.
/// 
/// Can be converted to a `String` using `.to_string()` when comparing against
/// custom property names.
///
/// ```
/// use coremidi::{Client, property, PropertyName};
///
/// // Note: Until a Client is created, Property constants won't have the
/// // correct string value, since CoreMIDI initializes them when a client
/// // is created
/// let client = Client::new("Test Client").unwrap();
///
/// // Pretend property_name came from a Notification
/// let p = PropertyName::from("name");
///
/// if p == property::NAME {
///     // Handle name changes
/// } else if p == property::DISPLAY_NAME {
///     // Handle display name changes
/// } else if p == property::OFFLINE {
///     // Handle offline changes
/// } else if p == property::UNIQUE_ID {
///     // Handle unique id changes
/// } else {
///     let p = p.to_string();
///     match p.as_ref() {
///         "com_coremidi_CustomProperty" => {
///             // Handle custom property
///         },
///         _ => { }
///     }
/// }
/// ```
#[derive(Clone, Debug)] 
pub struct PropertyName(CFString);

impl PropertyName {
    /// Converts the given `PropertyName` into a [`ParsedPropertyName`](enum.ParsedPropertyName.html) by parsing it.
    ///
    /// In the worst case, this conversion could require string comparisons to
    /// each of the standard CoreMIDI-defined constants, along with an allocation,
    /// so in cases where you just want to match against the standard property
    /// names or specific custom property names, prefer to use `==` instead.
    ///
    /// ```
    /// use coremidi::{Client, property, PropertyName, ParsedPropertyName};
    ///
    /// // Note: Until a Client is created, Property constants won't have the
    /// // correct string value, since CoreMIDI initializes them when a client
    /// // is created
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// // Pretend property_name came from a Notification
    /// let property_name = PropertyName::from("name");
    ///
    /// // Parse the property name
    /// let parsed = property_name.parse();
    /// 
    /// assert_eq!(parsed, ParsedPropertyName::String(property::NAME));
    pub fn parse(&self) -> ParsedPropertyName {
        let string_ref  = self.0.as_concrete_TypeRef();
        StringProperty::try_from_constant_string_ref(string_ref).map(Into::into)
            .or_else(|| BooleanProperty::try_from_constant_string_ref(string_ref).map(Into::into))
            .or_else(|| IntegerProperty::try_from_constant_string_ref(string_ref).map(Into::into))
            .unwrap_or_else(|| {
                let name: CFString = unsafe { TCFType::wrap_under_get_rule(string_ref) };
                name.to_string().into()
            })
    }

    /// Compares this MIDI object property name to one of the standard CoreMIDI-defined
    /// property names.
    fn matches<P>(&self, property: P) -> bool where
        P: StandardProperty
    {
        let this_key = self.0.as_concrete_TypeRef();
        let other_key = property.into();
        match_property_keys(this_key, other_key)
    }
}

// Note: We could derive this after upgrading core-foundation crate
impl PartialEq for PropertyName {
    fn eq(&self, other: &PropertyName) -> bool {
        let this_key = self.0.as_concrete_TypeRef();
        let other_key = other.0.as_concrete_TypeRef();
        match_property_keys(this_key, other_key)
    }
}

// Note: We could derive this after upgrading core-foundation crate
impl Eq for PropertyName { }

impl PartialEq<StringProperty> for PropertyName {
    fn eq(&self, other: &StringProperty) -> bool {
        self.matches(*other)
    }
}

impl PartialEq<IntegerProperty> for PropertyName {
    fn eq(&self, other: &IntegerProperty) -> bool {
        self.matches(*other)
    }
}

impl PartialEq<BooleanProperty> for PropertyName {
    fn eq(&self, other: &BooleanProperty) -> bool {
        self.matches(*other)
    }
}

impl PartialEq<PropertyName> for StringProperty {
    fn eq(&self, other: &PropertyName) -> bool {
        other.matches(*self)
    }
}

impl PartialEq<PropertyName> for IntegerProperty {
    fn eq(&self, other: &PropertyName) -> bool {
        other.matches(*self)
    }
}

impl PartialEq<PropertyName> for BooleanProperty {
    fn eq(&self, other: &PropertyName) -> bool {
        other.matches(*self)
    }
}

impl<'a> From<&'a str> for PropertyName {
    fn from(string: &str) -> Self {
        PropertyName(CFString::new(string))
    }
}

impl From<CFStringRef> for PropertyName {
    fn from(string_ref: CFStringRef) -> Self {
        PropertyName(unsafe {
            TCFType::wrap_under_get_rule(string_ref)
        })
    }
}

impl From<StringProperty> for PropertyName {
    fn from(prop: StringProperty) -> Self {
        PropertyName::from(CFStringRef::from(prop))
    }
}

impl From<IntegerProperty> for PropertyName {
    fn from(prop: IntegerProperty) -> Self {
        PropertyName::from(CFStringRef::from(prop))
    }
}

impl From<BooleanProperty> for PropertyName {
    fn from(prop: BooleanProperty) -> Self {
        PropertyName::from(CFStringRef::from(prop))
    }
}

impl fmt::Display for PropertyName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

/// A MIDI object property name, along with type information about what kind of
/// data can be accessed via that named property.
/// 
/// Created from a [`PropertyName`](struct.PropertyName.html) using the [`parse`](struct.PropertyName.html#method.parse) method.
///
/// If the property name is one of the known CoreMIDI constants, the appropriate
/// variant will be used based on the type of data that can be accessed through
/// that property. Otherwise, the name will be in `Other(String)`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ParsedPropertyName {
    String(StringProperty),
    Integer(IntegerProperty),
    Boolean(BooleanProperty),
    Other(String),
}

impl From<PropertyName> for ParsedPropertyName {
    fn from(unparsed: PropertyName) -> Self {
        unparsed.parse()
    }
}

impl From<StringProperty> for ParsedPropertyName {
    fn from(prop: StringProperty) -> Self {
        ParsedPropertyName::String(prop)
    }
}

impl From<IntegerProperty> for ParsedPropertyName {
    fn from(prop: IntegerProperty) -> Self {
        ParsedPropertyName::Integer(prop)
    }
}

impl From<BooleanProperty> for ParsedPropertyName {
    fn from(prop: BooleanProperty) -> Self {
        ParsedPropertyName::Boolean(prop)
    }
}

impl From<String> for ParsedPropertyName {
    fn from(string: String) -> Self {
        ParsedPropertyName::Other(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use coremidi_sys::kMIDIPropertyUniqueID;
    use crate::{
        Client,
        property,
    };

    // Note: Until a Client is started, Property constants won't have the
    // correct string value, since CoreMIDI initializes them when a client
    // is created
    fn setup() -> Client {
        Client::new("Test Client").unwrap()
    }

    #[test]
    fn test_property_name_partial_eq() {
        let _client = setup();

        // Known to be the value of kMIDIPropertyOffline
        let prop = PropertyName::from("offline");

        assert_eq!(prop, property::OFFLINE);
    }

    #[test]
    fn test_property_name_partial_eq_false() {
        let _client = setup();

        let prop = PropertyName::from("nOt_A_pRoPeRtY");

        assert_ne!(prop, property::ADVANCED_SCHEDULE_TIME_MUSEC);
    }

    #[test]
    fn test_parse_property_name_from_constant_prop_name() {
        let _client = setup();

        let name = PropertyName::from(unsafe { kMIDIPropertyUniqueID });
        let parsed = name.parse();

        assert_eq!(parsed, ParsedPropertyName::Integer(IntegerProperty::UniqueId))
    }

    #[test]
    fn test_parsed_property_name_from_owned_prop_name() {
        let _client = setup();

        // Known to be the value of kMIDIPropertyOffline
        let name = PropertyName::from("offline");
        let parsed = ParsedPropertyName::from(name);

        assert_eq!(parsed, ParsedPropertyName::Boolean(BooleanProperty::Offline))
    }

    #[test]
    fn test_parsed_property_name_from_unknown_prop_name() {
        let _client = setup();

        const PROPERTY_STR: &str = "nOt_A_rEaL_cOrEmIdI_pRoPerTy";
        let name = PropertyName::from(PROPERTY_STR);
        let parsed = ParsedPropertyName::from(name);

        assert_eq!(parsed, ParsedPropertyName::Other(PROPERTY_STR.to_string()))
    }
}