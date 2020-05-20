use core_foundation::{
    string::{
        CFString, 
        CFStringRef,
    },
    base::TCFType,
};

macro_rules! match_property_keys {
    ($match_var:ident, $($prop_name:ident -> $key_name: ident,)*) => {
        Ok(match $match_var {
            $(x if x == unsafe { $key_name } => $prop_name,)*
            _ => return {
                Err(())
            }
        })
    }
}

pub mod boolean;
pub mod constants;
pub mod integer;
pub mod string;

pub use self::{
    constants::*,
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
        get_integer_property_inner,
        set_integer_property_inner,
    },
    string::{
        get_string_property_inner,
        set_string_property_inner,
    },
};

/// Can hold the name of any MIDI object property
pub enum PropertyName {
    String(StringProperty),
    Integer(IntegerProperty),
    Boolean(BooleanProperty),
    Custom(String),
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
/// any `&str`, `String`, or `StandardProperty` using `std::convert::From` or
/// `std::convert::Into`.
#[derive(Clone, Debug)]
pub enum TypedPropertyName<K> where
    K: StandardProperty,
{
    Standard(K),
    Custom(CFString),
}

impl<K> TypedPropertyName<K> where
    K: StandardProperty,
{
    fn custom<S: AsRef<str>>(name: S) -> Self {
        TypedPropertyName::Custom(CFString::new(name.as_ref()))
    }

    /// Return a raw CFStringRef pointing to this property key
    ///
    /// Note: Should never be exposed externally
    pub(crate) fn as_string_ref(&self) -> CFStringRef {
        match self {
            TypedPropertyName::Standard(constant) => Into::into(*constant),
            TypedPropertyName::Custom(custom) => custom.as_concrete_TypeRef(),
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
