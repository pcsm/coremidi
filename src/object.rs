#![allow(non_upper_case_globals)]

use core_foundation::{
    base::TCFType,
    string::CFString,
};
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
    Property,
    PropertyGetter,
    PropertySetter,
    Properties,
    TypedProperty,
    IntegerProperty,
    BooleanProperty,
    StringProperty,
    get_string_property_inner,
    set_string_property_inner,
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
        Properties::name().value_from(self).ok()
    }

    /// Get the unique id for the object.
    ///
    pub fn unique_id(&self) -> Option<u32> {
        Properties::unique_id().value_from(self).ok().map(|v: SInt32| v as u32)
    }

    /// Get the display name for the object.
    ///
    pub fn display_name(&self) -> Option<String> {
        Properties::display_name().value_from(self).ok()
    }

    /// Sets an object's string-type property.
    ///
    pub fn set_property_string<N, V>(&self, name: N, value: V) -> Result<(), OSStatus> where
        N: Into<TypedProperty<StringProperty>>, 
        V: AsRef<str>,
    {
        let prop = name.into();
        set_string_property_inner(self, prop.as_string_ref(), value)
    }

    /// Gets an object's string-type property.
    ///
    pub fn get_property_string<N>(&self, name: N) -> Result<String, OSStatus> where
        N: AsRef<str>
    {
        let name = CFString::new(name.as_ref());
        get_string_property_inner(self, name.as_concrete_TypeRef())
    }

    /// Sets an object's integer-type property.
    ///
    pub fn set_property_integer(&self, name: &str, value: i32) -> Result<(), OSStatus> {
        IntegerProperty::new(name).set_value(self, value)
    }

    /// Gets an object's integer-type property.
    ///
    pub fn get_property_integer(&self, name: &str) -> Result<i32, OSStatus> {
        IntegerProperty::new(name).value_from(self)
    }

    /// Sets an object's boolean-type property.
    ///
    /// CoreMIDI treats booleans as integers (0/1) but this API uses native bool types
    ///
    pub fn set_property_boolean(&self, name: &str, value: bool) -> Result<(), OSStatus> {
        BooleanProperty::new(name).set_value(self, value)
    }

    /// Gets an object's boolean-type property.
    ///
    /// CoreMIDI treats booleans as integers (0/1) but this API uses native bool types
    ///
    pub fn get_property_boolean(&self, name: &str) -> Result<bool, OSStatus> {
        BooleanProperty::new(name).value_from(self)
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
