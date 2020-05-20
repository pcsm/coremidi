#![allow(non_upper_case_globals)]

use core_foundation_sys::base::OSStatus;

use coremidi_sys::{
    SInt32,
    kMIDIObjectType_Other,
    kMIDIObjectType_Device,
    kMIDIObjectType_Entity,
    kMIDIObjectType_Source,
    kMIDIObjectType_Destination,
    kMIDIObjectType_ExternalDevice,
    kMIDIObjectType_ExternalEntity,
    kMIDIObjectType_ExternalSource,
    kMIDIObjectType_ExternalDestination
};

use std::fmt;

use Object;
use properties::{
    IntegerProperty,
    IntegerPropertyName,
    BooleanPropertyName,
    StringProperty,
    StringPropertyName,
    get_string_property_inner,
    set_string_property_inner,
    get_integer_property_inner,
    set_integer_property_inner,
    get_integer_property_inner_concrete,
    set_integer_property_inner_concrete,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ObjectType {
    Other,
    Device,
    Entity,
    Source,
    Destination,
    ExternalDevice,
    ExternalEntity,
    ExternalSource,
    ExternalDestination
}

impl ObjectType {
    pub fn from(value: i32) -> Result<ObjectType, i32> {
        match value {
            kMIDIObjectType_Other => Ok(ObjectType::Other),
            kMIDIObjectType_Device => Ok(ObjectType::Device),
            kMIDIObjectType_Entity => Ok(ObjectType::Entity),
            kMIDIObjectType_Source => Ok(ObjectType::Source),
            kMIDIObjectType_Destination => Ok(ObjectType::Destination),
            kMIDIObjectType_ExternalDevice => Ok(ObjectType::ExternalDevice),
            kMIDIObjectType_ExternalEntity => Ok(ObjectType::ExternalEntity),
            kMIDIObjectType_ExternalSource => Ok(ObjectType::ExternalSource),
            kMIDIObjectType_ExternalDestination => Ok(ObjectType::ExternalDestination),
            unknown => Err(unknown)
        }
    }
}

impl Object {
    /// Get the name for the object.
    ///
    pub fn name(&self) -> Option<String> {
        self.get_property_string(StringProperty::Name).ok()
    }

    /// Get the unique id for the object.
    ///
    pub fn unique_id(&self) -> Option<u32> {
        self.get_property_integer(IntegerProperty::UniqueId).ok().map(|v: SInt32| v as u32)
    }

    /// Get the display name for the object.
    ///
    pub fn display_name(&self) -> Option<String> {
        self.get_property_string(StringProperty::DisplayName).ok()
    }

    /// Sets an object's string-type property.
    ///
    pub fn set_property_string<N, V>(&self, name: N, value: V) -> Result<(), OSStatus> where
        N: Into<StringPropertyName>, 
        V: AsRef<str>,
    {
        set_string_property_inner(self, name, value)
    }

    /// Gets an object's string-type property.
    ///
    pub fn get_property_string<N>(&self, name: N) -> Result<String, OSStatus> where
        N: Into<StringPropertyName>, 
    {
        get_string_property_inner(self, name)
    }

    /// Sets an object's integer-type property.
    ///
    pub fn set_property_integer<N, V>(&self, name: N, value: V) -> Result<(), OSStatus> where
        N: Into<IntegerPropertyName>, 
        V: Into<i32>,
    {
        set_integer_property_inner(self, name, value)
    }

    /// Gets an object's integer-type property.
    ///
    pub fn get_property_integer<N>(&self, name: N) -> Result<i32, OSStatus> where 
        N: Into<IntegerPropertyName>, 
    {
        get_integer_property_inner(self, name)
    }

    /// Sets an object's boolean-type property.
    ///
    /// CoreMIDI treats booleans as integers (0/1) but this API uses native bool types
    ///
    pub fn set_property_boolean<N, V>(&self, name: N, value: V) -> Result<(), OSStatus> where
        N: Into<BooleanPropertyName>, 
        V: Into<bool>,
    {
        let name = name.into();
        let value = if value.into() { 1 } else { 0 };
        set_integer_property_inner_concrete(self, name.as_string_ref(), value)
    }

    /// Gets an object's boolean-type property.
    ///
    /// CoreMIDI treats booleans as integers (0/1) but this API uses native bool types
    ///
    pub fn get_property_boolean<N>(&self, name: N) -> Result<bool, OSStatus> where
        N: Into<BooleanPropertyName>, 
    {
        let name = name.into();
        get_integer_property_inner_concrete(self, name.as_string_ref()).map(|val| (val == 1))
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object({:x})", self.0 as usize)
    }
}

#[cfg(test)]
mod tests {
    use coremidi_sys::{
        kMIDIObjectType_Other,
        kMIDIObjectType_Device,
        kMIDIObjectType_Entity,
        kMIDIObjectType_Source,
        kMIDIObjectType_Destination,
        kMIDIObjectType_ExternalDevice,
        kMIDIObjectType_ExternalEntity,
        kMIDIObjectType_ExternalSource,
        kMIDIObjectType_ExternalDestination
    };

    use ::{
        Client,
        VirtualDestination,
        object::ObjectType,
    };

    const NAME_ORIG: &str = "A";
    const NAME_MODIFIED: &str = "B";

    fn setup() -> (Client, VirtualDestination) {
        let client = Client::new("Test Client").unwrap();
        let dest = client.virtual_destination(NAME_ORIG, |_|()).unwrap();
        (client, dest)
    }

    // Test getting the original value of the property
    fn check_get_original(name: &str, dest: &VirtualDestination) {
        let name: String = dest.get_property_string(name).unwrap();

        assert_eq!(name, NAME_ORIG);
    }

    fn check_roundtrip(name: &str, dest: &VirtualDestination) {
        dest.set_property_string(name, NAME_MODIFIED).unwrap();
        let name: String = dest.get_property_string(name).unwrap();

        assert_eq!(name, NAME_MODIFIED);
    }

    #[test]
    fn objecttype_from() {
        assert_eq!(ObjectType::from(kMIDIObjectType_Other), Ok(ObjectType::Other));
        assert_eq!(ObjectType::from(kMIDIObjectType_Device), Ok(ObjectType::Device));
        assert_eq!(ObjectType::from(kMIDIObjectType_Entity), Ok(ObjectType::Entity));
        assert_eq!(ObjectType::from(kMIDIObjectType_Source), Ok(ObjectType::Source));
        assert_eq!(ObjectType::from(kMIDIObjectType_Destination), Ok(ObjectType::Destination));
        assert_eq!(ObjectType::from(kMIDIObjectType_ExternalDevice), Ok(ObjectType::ExternalDevice));
        assert_eq!(ObjectType::from(kMIDIObjectType_ExternalEntity), Ok(ObjectType::ExternalEntity));
        assert_eq!(ObjectType::from(kMIDIObjectType_ExternalSource), Ok(ObjectType::ExternalSource));
        assert_eq!(ObjectType::from(kMIDIObjectType_ExternalDestination), Ok(ObjectType::ExternalDestination));
    }

    #[test]
    fn objecttype_from_error() {
        assert_eq!(ObjectType::from(0xffff as i32), Err(0xffff));
    }

    #[test]
    fn test_set_property_string() {
        let (_client, dest) = setup();
        // "name" is the value of the CoreMidi constant kMIDIPropertyName
        let name = "name";

        check_get_original(name, &dest);
        check_roundtrip(name, &dest);
    }
}
