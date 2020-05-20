use core_foundation::{
    string::{
        CFString, 
        CFStringRef,
    },
    base::{
        OSStatus,
        TCFType,
    }
};
use coremidi_sys::*;

use std::mem::MaybeUninit;

use {
    Object,
    result_from_status,
    unit_result_from_status,
};

/// A valid MIDI object property whose value is a String
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum StringProperty {
    /// See [kMIDIPropertyName](https://developer.apple.com/reference/coremidi/kmidipropertyname)
    Name,
    /// See [kMIDIPropertyManufacturer](https://developer.apple.com/reference/coremidi/kmidipropertymanufacturer)
    Manufacturer,
    /// See [kMIDIPropertyModel](https://developer.apple.com/reference/coremidi/kmidipropertymodel)
    Model,
    /// See [kMIDIPropertyDriverOwner](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverOwner)
    DriverOwner,
    /// See [kMIDIPropertyDriverDeviceEditorApp](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverDeviceEditorApp)
    DriverDeviceEditorApp,
    /// See [kMIDIPropertyDisplayName](https://developer.apple.com/reference/coremidi/kMIDIPropertyDisplayName)
    DisplayName,
}

impl StringProperty {
    /// Note: Should only be used internally with predefined CoreMidi constants,
    /// since it compares pointers of the incoming CFStringRef and the constants
    fn try_from_constant_string_ref(key: CFStringRef) -> Result<Self, ()> {
        use self::StringProperty::*;
        match_property_keys! {
            key, 
            Name -> kMIDIPropertyName,
            Manufacturer -> kMIDIPropertyManufacturer,
            Model -> kMIDIPropertyModel,
            DriverOwner -> kMIDIPropertyDriverOwner,
            DriverDeviceEditorApp -> kMIDIPropertyDriverDeviceEditorApp,
            DisplayName -> kMIDIPropertyDisplayName,
        }
    }
}

impl From<StringProperty> for CFStringRef {
    fn from(prop: StringProperty) -> Self {
        use self::StringProperty::*;
        unsafe {
            match prop {
                Name => kMIDIPropertyName,
                Manufacturer => kMIDIPropertyManufacturer,
                Model => kMIDIPropertyModel,
                DriverOwner => kMIDIPropertyDriverOwner,
                DriverDeviceEditorApp => kMIDIPropertyDriverDeviceEditorApp,
                DisplayName => kMIDIPropertyDisplayName,
            }
        }
    }
}

pub(crate) fn get_string_property_inner(object: &Object, name: CFStringRef) -> Result<String, OSStatus> {
    let mut string_ref = MaybeUninit::uninit();
    let status = unsafe {
        MIDIObjectGetStringProperty(object.0, name, string_ref.as_mut_ptr())
    };
    result_from_status(status, || {
        let string_ref = unsafe { string_ref.assume_init() };
        if string_ref.is_null() { return "".to_string().into() };
        let cf_string: CFString = unsafe { TCFType::wrap_under_create_rule(string_ref) };
        cf_string.to_string().into()
    })
}

pub(crate) fn set_string_property_inner<V>(object: &Object, name: CFStringRef, value: V) -> Result<(), OSStatus> where
    V: AsRef<str>,
{
    let string = CFString::new(value.as_ref());
    let string_ref = string.as_concrete_TypeRef();
    let status = unsafe {
        MIDIObjectSetStringProperty(object.0, name, string_ref)
    };
    unit_result_from_status(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Client,
        properties,
        VirtualDestination,
    };

    const NAME_ORIG: &str = "A";
    const NAME_MODIFIED: &str = "B";

    fn setup() -> (Client, VirtualDestination) {
        let client = Client::new("Test Client").unwrap();
        let dest = client.virtual_destination(NAME_ORIG, |_|()).unwrap();
        (client, dest)
    }

    // Test getting the original value of the "name" property
    fn check_get_original(property: StringProperty, dest: &VirtualDestination) {
        let name = dest.get_property_string(property).unwrap();

        assert_eq!(name, NAME_ORIG);
    }

    // Test setting then getting the "name" property
    fn check_roundtrip(property: StringProperty, dest: &VirtualDestination) {
        dest.set_property_string(property, NAME_MODIFIED).unwrap();
        let name = dest.get_property_string(property).unwrap();

        assert_eq!(name, NAME_MODIFIED);
    }

    #[test]
    fn test_from_constant() {
        let (_client, dest) = setup();
        let property = properties::NAME;

        check_get_original(property, &dest);
        check_roundtrip(property, &dest);
    }
}
