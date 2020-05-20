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
    boolean::BooleanProperty,
    integer::IntegerProperty,
    string::{
        StringProperty,
        get_string_property_inner,
        set_string_property_inner,
    },
};

pub(crate) use self::integer::{
    get_integer_property_inner_concrete,
    set_integer_property_inner_concrete,
};

/// A type that can be returned from a Property
pub trait PropertyValue { }
impl PropertyValue for String { }
impl PropertyValue for i32 { }
impl PropertyValue for bool { }

/// A type that can represent a standard CoreMIDI property
pub trait StandardProperty : Into<CFStringRef> + Clone {
    type Value: PropertyValue;
}

impl StandardProperty for StringProperty {
    type Value = String;
}

impl StandardProperty for IntegerProperty {
    type Value = i32;
}

impl StandardProperty for BooleanProperty {
    type Value = bool;
}

/// Can hold the name of any MIDI object property
pub enum PropertyName {
    String(StringProperty),
    Integer(IntegerProperty),
    Boolean(BooleanProperty),
    Custom(String),
}

/// The name of a MIDI object property that is accessed as a `String`
pub type StringPropertyName = TypedPropertyName<StringProperty>;

/// The name of a MIDI object property that is accessed as a `i32`
pub type IntegerPropertyName = TypedPropertyName<IntegerProperty>;

/// The name of a MIDI object property that is accessed as a `bool`
pub type BooleanPropertyName = TypedPropertyName<BooleanProperty>;

/// The name of a MIDI object property, which can either be one of the standard 
/// CoreMIDI constant property names or a custom property name.
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
            TypedPropertyName::Standard(constant) => Into::into(constant.clone()),
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
