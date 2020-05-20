//! MIDI Object properties that can access `i32` values

use core_foundation::{
    string::CFStringRef,
    base::OSStatus,
};
use coremidi_sys::*;

use std::mem::MaybeUninit;

use {
    Object,
    result_from_status,
    unit_result_from_status,
};

use super::{
    match_property_keys,
    StandardProperty,
    TypedPropertyName,
};

/// CoreMIDI-defined constant property names that can be used to access `i32` values
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum IntegerProperty {
    /// See [kMIDIPropertyDeviceID](https://developer.apple.com/reference/coremidi/kmidipropertydeviceid)
    DeviceId,
    /// See [kMIDIPropertyUniqueID](https://developer.apple.com/reference/coremidi/kmidipropertyuniqueid)
    UniqueId,
    /// See [kMIDIPropertyReceiveChannels](https://developer.apple.com/reference/coremidi/kmidipropertyreceivechannels)
    ReceiveChannels,
    /// See [kMIDIPropertyTransmitChannels](https://developer.apple.com/reference/coremidi/kmidipropertytransmitchannels)
    TransmitChannels,
    /// See [kMIDIPropertyMaxSysExSpeed](https://developer.apple.com/reference/coremidi/kmidipropertymaxsysexspeed)
    MaxSysExSpeed,
    /// See [kMIDIPropertyAdvanceScheduleTimeMuSec](https://developer.apple.com/reference/coremidi/kMIDIPropertyAdvanceScheduleTimeMuSec)
    AdvanceScheduleTimeMuSec,
    /// See [kMIDIPropertySingleRealtimeEntity](https://developer.apple.com/reference/coremidi/kMIDIPropertySingleRealtimeEntity)
    SingleRealtimeEntity,
    /// See [kMIDIPropertyConnectionUniqueID](https://developer.apple.com/reference/coremidi/kMIDIPropertyConnectionUniqueID)
    ConnectionUniqueId,
    /// See [kMIDIPropertyDriverVersion](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverVersion)
    DriverVersion,
    /// See [kMIDIPropertyMaxReceiveChannels](https://developer.apple.com/reference/coremidi/kMIDIPropertyMaxReceiveChannels)
    MaxRecieveChannels,
    /// See [kMIDIPropertyMaxTransmitChannels](https://developer.apple.com/reference/coremidi/kMIDIPropertyMaxTransmitChannels)
    MaxTransmitChannels,
}

/// The name of a MIDI object property that is accessed as a `i32`
pub type IntegerPropertyName = TypedPropertyName<IntegerProperty>;

impl StandardProperty for IntegerProperty { }

impl IntegerProperty {
    /// Note: Should only be used internally with predefined CoreMidi constants,
    /// since it compares pointers of the incoming CFStringRef and the constants
    pub(crate) fn try_from_constant_string_ref(key: CFStringRef) -> Option<Self> {
        use self::IntegerProperty::*;
        convert_property_key_set! {
            key, 
            DeviceId -> kMIDIPropertyDeviceID,
            UniqueId -> kMIDIPropertyUniqueID,
            ReceiveChannels -> kMIDIPropertyReceiveChannels,
            TransmitChannels -> kMIDIPropertyTransmitChannels,
            MaxSysExSpeed -> kMIDIPropertyMaxSysExSpeed,
            AdvanceScheduleTimeMuSec -> kMIDIPropertyAdvanceScheduleTimeMuSec,
            SingleRealtimeEntity -> kMIDIPropertySingleRealtimeEntity,
            ConnectionUniqueId -> kMIDIPropertyConnectionUniqueID,
            DriverVersion -> kMIDIPropertyDriverVersion,
            MaxRecieveChannels -> kMIDIPropertyMaxReceiveChannels,
            MaxTransmitChannels -> kMIDIPropertyMaxTransmitChannels,
        }
    }
}

impl From<IntegerProperty> for CFStringRef {
    fn from(prop: IntegerProperty) -> Self {
        use self::IntegerProperty::*;
        unsafe {
            match prop {
                DeviceId => kMIDIPropertyDeviceID,
                UniqueId => kMIDIPropertyUniqueID,
                ReceiveChannels => kMIDIPropertyReceiveChannels,
                TransmitChannels => kMIDIPropertyTransmitChannels,
                MaxSysExSpeed => kMIDIPropertyMaxSysExSpeed,
                AdvanceScheduleTimeMuSec => kMIDIPropertyAdvanceScheduleTimeMuSec,
                SingleRealtimeEntity => kMIDIPropertySingleRealtimeEntity,
                ConnectionUniqueId => kMIDIPropertyConnectionUniqueID,
                DriverVersion => kMIDIPropertyDriverVersion,
                MaxRecieveChannels => kMIDIPropertyMaxReceiveChannels,
                MaxTransmitChannels => kMIDIPropertyMaxTransmitChannels,
            }
        }
    }
}

pub(crate) fn get_integer_property_inner(object: &Object, name: CFStringRef) -> Result<i32, OSStatus> {
    let mut value = MaybeUninit::uninit();
    let status = unsafe {
        MIDIObjectGetIntegerProperty(object.0, name, value.as_mut_ptr())
    };
    result_from_status(status, || {
        let value = unsafe { value.assume_init() };
        value.into()
    })
}

pub(crate) fn set_integer_property_inner(object: &Object, name: CFStringRef, value: i32) -> Result<(), OSStatus> {
    let status = unsafe {
        MIDIObjectSetIntegerProperty(object.0, name, value)
    };
    unit_result_from_status(status)
}

#[cfg(test)]
mod tests {
    use crate::{
        Client,
        property,
        VirtualDestination,
    };

    const ADVANCED_SCHEDULE_TIME: i32 = 44;

    fn setup() -> (Client, VirtualDestination) {
        let client = Client::new("Test Client").unwrap();
        let dest = client.virtual_destination("Test Dest", |_|()).unwrap();
        (client, dest)
    }

    #[test]
    fn test_not_set() {
        let (_client, dest) = setup();
        // Is not set by default for Virtual Destinations
        let property = property::ADVANCED_SCHEDULE_TIME_MUSEC;

        let value  = dest.get_property_integer(property);

        assert!(value.is_err())
    }

    #[test]
    fn test_roundtrip() {
        let (_client, dest) = setup();
        let property = property::ADVANCED_SCHEDULE_TIME_MUSEC;

        dest.set_property_integer(property, ADVANCED_SCHEDULE_TIME).unwrap();
        let num = dest.get_property_integer(property).unwrap();

        assert_eq!(num, ADVANCED_SCHEDULE_TIME);
    }
}
