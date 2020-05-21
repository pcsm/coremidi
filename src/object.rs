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
    IntegerPropertyKey,
    BooleanPropertyKey,
    StringProperty,
    StringPropertyKey,
    string_property_inner,
    set_string_property_inner,
    int_property_inner,
    set_int_property_inner,
};

/// Represents the type of a MIDI object
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
    /// Get the name of this object.
    ///
    pub fn name(&self) -> Option<String> {
        self.string_property(StringProperty::Name).ok()
    }

    /// Get the unique id of this object.
    ///
    pub fn unique_id(&self) -> Option<u32> {
        self.int_property(IntegerProperty::UniqueId).ok().map(|v: SInt32| v as u32)
    }

    /// Get the display name of this object.
    ///
    pub fn display_name(&self) -> Option<String> {
        self.string_property(StringProperty::DisplayName).ok()
    }

    /// Sets the value of a string-type property for this object.
    ///
    /// Property keys can be [`StringProperty`](enum.StringProperty.html) 
    /// variants, [`coremidi::property`](property/index.html) constants, `&str`s, 
    /// or `Strings`.
    ///
    /// ```
    /// use coremidi::{ Client, property };
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// client.set_string_property(property::NAME, "Your Name Here").unwrap();
    ///
    /// ```
    pub fn set_string_property<K, V>(&self, key: K, value: V) -> Result<(), OSStatus> where
        K: Into<StringPropertyKey>, 
        V: AsRef<str>,
    {
        let key = key.into();
        set_string_property_inner(self, key.as_string_ref(), value)
    }

    /// Gets the value of a string-type property for this object.
    ///
    /// Property keys can be [`StringProperty`](enum.StringProperty.html) 
    /// variants, [`coremidi::property`](property/index.html) constants, `&str`s, 
    /// or `Strings`.
    ///
    /// ```
    /// use coremidi::{ Client, property };
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// let name = client.string_property(property::NAME).unwrap();
    ///
    /// ```
    pub fn string_property<K>(&self, key: K) -> Result<String, OSStatus> where
        K: Into<StringPropertyKey>, 
    {
        let key = key.into();
        string_property_inner(self, key.as_string_ref())
    }

    /// Sets the value of a integer-type property for this object.
    ///
    /// Property keys can be [`IntegerProperty`](enum.IntegerProperty.html) 
    /// variants, [`coremidi::property`](property/index.html) constants, `&str`s, 
    /// or `Strings`.
    ///
    /// ```
    /// use coremidi::{ Client, property };
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// client.set_int_property(property::MAX_TRANSMIT_CHANNELS, 4).unwrap();
    ///
    /// ```
    pub fn set_int_property<K, V>(&self, key: K, value: V) -> Result<(), OSStatus> where
        K: Into<IntegerPropertyKey>, 
        V: Into<i32>,
    {
        let key = key.into();
        set_int_property_inner(self, key.as_string_ref(), value.into())
    }

    /// Gets the value of a integer-type property for this object.
    ///
    /// Property keys can be [`IntegerProperty`](enum.IntegerProperty.html) 
    /// variants, [`coremidi::property`](property/index.html) constants, `&str`s, 
    /// or `Strings`.
    ///
    /// ```
    /// use coremidi::{ Client, property };
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// // Error: No unique id has been set yet
    /// assert!(client.int_property(property::UNIQUE_ID).is_err());
    ///
    /// ```
    pub fn int_property<K>(&self, key: K) -> Result<i32, OSStatus> where 
        K: Into<IntegerPropertyKey>, 
    {
        let key = key.into();
        int_property_inner(self, key.as_string_ref())
    }

    /// Sets the value of a boolean-type property for this object.
    ///
    /// Property keys can be [`BooleanProperty`](enum.BooleanProperty.html) 
    /// variants, [`coremidi::property`](property/index.html) constants, `&str`s, 
    /// or `Strings`.
    ///
    /// CoreMIDI treats booleans as integers (`0`/`1`), but this crate uses `bool`.
    ///
    /// ```
    /// use coremidi::{ Client, property };
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// client.set_bool_property(property::OFFLINE, true).unwrap();
    ///
    /// ```
    pub fn set_bool_property<K, V>(&self, key: K, value: V) -> Result<(), OSStatus> where
        K: Into<BooleanPropertyKey>, 
        V: Into<bool>,
    {
        let key = key.into();
        let value = if value.into() { 1 } else { 0 };
        set_int_property_inner(self, key.as_string_ref(), value)
    }

    /// Gets the value of a boolean-type property for this object.
    ///
    /// Property keys can be [`BooleanProperty`](enum.BooleanProperty.html) 
    /// variants, [`coremidi::property`](property/index.html) constants, `&str`s, 
    /// or `Strings`.
    ///
    /// CoreMIDI treats booleans as integers (`0`/`1`), but this crate uses `bool`.
    ///
    /// ```
    /// use coremidi::{ Client, property };
    /// let client = Client::new("Test Client").unwrap();
    ///
    /// // Error: No offline property has been set yet
    /// assert!(client.bool_property(property::OFFLINE).is_err());
    ///
    /// ```
    pub fn bool_property<K>(&self, key: K) -> Result<bool, OSStatus> where
        K: Into<BooleanPropertyKey>, 
    {
        let key = key.into();
        int_property_inner(self, key.as_string_ref()).map(|val| (val == 1))
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
        let name: String = dest.string_property(name).unwrap();

        assert_eq!(name, NAME_ORIG);
    }

    fn check_roundtrip(name: &str, dest: &VirtualDestination) {
        dest.set_string_property(name, NAME_MODIFIED).unwrap();
        let name: String = dest.string_property(name).unwrap();

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
