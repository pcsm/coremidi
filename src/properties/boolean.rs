use core_foundation::{
    string::CFStringRef,
};
use coremidi_sys::*;

/// A MIDI object property whose value is a Boolean
///
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum BooleanProperty {
    /// See [kMIDIPropertyIsEmbeddedEntity](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsEmbeddedEntity)
    IsEmbeddedEntity,
    /// See [kMIDIPropertyIsBroadcast](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsBroadcast)
    IsBroadcast,
    /// See [kMIDIPropertyOffline](https://developer.apple.com/reference/coremidi/kMIDIPropertyOffline)
    Offline,
    /// See [kMIDIPropertyPrivate](https://developer.apple.com/reference/coremidi/kMIDIPropertyPrivate)
    Private,
    /// See [kMIDIPropertySupportsGeneralMIDI](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsGeneralMIDI)
    SupportsGeneralMIDI,
    /// See [kMIDIPropertySupportsMMC](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsMMC)
    SupportsMMC,
    /// See [kMIDIPropertyCanRoute](https://developer.apple.com/reference/coremidi/kMIDIPropertyCanRoute)
    CanRoute,
    /// See [kMIDIPropertyReceivesClock](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesClock)
    ReceivesClock,
    /// See [kMIDIPropertyReceivesMTC](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesMTC)
    ReceivesMTC,
    /// See [kMIDIPropertyReceivesNotes](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesNotes)
    ReceivesNotes,
    /// See [kMIDIPropertyReceivesProgramChanges](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesProgramChanges)
    ReceivesProgramChanges,
    /// See [kMIDIPropertyReceivesBankSelectMSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesBankSelectMSB)
    ReceivesBankSelectMSB,
    /// See [kMIDIPropertyReceivesBankSelectLSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesBankSelectLSB)
    ReceivesBankSelectLSB,
    /// See [kMIDIPropertyTransmitsBankSelectMSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsBankSelectMSB)
    TransmitsBankSelectMSB,
    /// See [kMIDIPropertyTransmitsBankSelectLSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsBankSelectLSB)
    TransmitsBankSelectLSB,
    /// See [kMIDIPropertyTransmitsClock](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsClock)
    TransmitsClock,
    /// See [kMIDIPropertyTransmitsMTC](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsMTC)
    TransmitsMTC,
    /// See [kMIDIPropertyTransmitsNotes](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsNotes)
    TransmitsNotes,
    /// See [kMIDIPropertyTransmitsProgramChanges](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsProgramChanges)
    TransmitsProgramChanges,
    /// See [kMIDIPropertyPanDisruptsStereo](https://developer.apple.com/reference/coremidi/kMIDIPropertyPanDisruptsStereo)
    PanDisruptsStereo,
    /// See [kMIDIPropertyIsSampler](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsSampler)
    IsSampler,
    /// See [kMIDIPropertyIsDrumMachine](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsDrumMachine)
    IsDrumMachine,
    /// See [kMIDIPropertyIsMixer](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsMixer)
    IsMixer,
    /// See [kMIDIPropertyIsEffectUnit](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsEffectUnit)
    IsEffectUnit,
    /// See [kMIDIPropertySupportsShowControl](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsShowControl)
    SupportsShowControl,
}

impl BooleanProperty {
    /// Note: Should only be used internally with predefined CoreMidi constants,
    /// since it compares pointers of the incoming CFStringRef and the constants
    fn try_from_constant_string_ref(key: CFStringRef) -> Result<Self, ()> {
        use self::BooleanProperty::*;
        match_property_keys! {
            key, 
            IsEmbeddedEntity -> kMIDIPropertyIsEmbeddedEntity,
            IsBroadcast -> kMIDIPropertyIsBroadcast,
            Offline -> kMIDIPropertyOffline,
            Private -> kMIDIPropertyPrivate,
            SupportsGeneralMIDI -> kMIDIPropertySupportsGeneralMIDI,
            SupportsMMC -> kMIDIPropertySupportsMMC,
            CanRoute -> kMIDIPropertyCanRoute,
            ReceivesClock -> kMIDIPropertyReceivesClock,
            ReceivesMTC -> kMIDIPropertyReceivesMTC,
            ReceivesNotes -> kMIDIPropertyReceivesNotes,
            ReceivesProgramChanges -> kMIDIPropertyReceivesProgramChanges,
            ReceivesBankSelectMSB -> kMIDIPropertyReceivesBankSelectMSB,
            ReceivesBankSelectLSB -> kMIDIPropertyReceivesBankSelectLSB,
            TransmitsBankSelectMSB -> kMIDIPropertyTransmitsBankSelectMSB,
            TransmitsBankSelectLSB -> kMIDIPropertyTransmitsBankSelectLSB,
            TransmitsClock -> kMIDIPropertyTransmitsClock,
            TransmitsMTC -> kMIDIPropertyTransmitsMTC,
            TransmitsNotes -> kMIDIPropertyTransmitsNotes,
            TransmitsProgramChanges -> kMIDIPropertyTransmitsProgramChanges,
            PanDisruptsStereo -> kMIDIPropertyPanDisruptsStereo,
            IsSampler -> kMIDIPropertyIsSampler,
            IsDrumMachine -> kMIDIPropertyIsDrumMachine,
            IsMixer -> kMIDIPropertyIsMixer,
            IsEffectUnit -> kMIDIPropertyIsEffectUnit,
            SupportsShowControl -> kMIDIPropertySupportsShowControl,
        }
    }
}

impl From<BooleanProperty> for CFStringRef {
    fn from(prop: BooleanProperty) -> Self {
        use self::BooleanProperty::*;
        unsafe {
            match prop {
                IsEmbeddedEntity => kMIDIPropertyIsEmbeddedEntity,
                IsBroadcast => kMIDIPropertyIsBroadcast,
                Offline => kMIDIPropertyOffline,
                Private => kMIDIPropertyPrivate,
                SupportsGeneralMIDI => kMIDIPropertySupportsGeneralMIDI,
                SupportsMMC => kMIDIPropertySupportsMMC,
                CanRoute => kMIDIPropertyCanRoute,
                ReceivesClock => kMIDIPropertyReceivesClock,
                ReceivesMTC => kMIDIPropertyReceivesMTC,
                ReceivesNotes => kMIDIPropertyReceivesNotes,
                ReceivesProgramChanges => kMIDIPropertyReceivesProgramChanges,
                ReceivesBankSelectMSB => kMIDIPropertyReceivesBankSelectMSB,
                ReceivesBankSelectLSB => kMIDIPropertyReceivesBankSelectLSB,
                TransmitsBankSelectMSB => kMIDIPropertyTransmitsBankSelectMSB,
                TransmitsBankSelectLSB => kMIDIPropertyTransmitsBankSelectLSB,
                TransmitsClock => kMIDIPropertyTransmitsClock,
                TransmitsMTC => kMIDIPropertyTransmitsMTC,
                TransmitsNotes => kMIDIPropertyTransmitsNotes,
                TransmitsProgramChanges => kMIDIPropertyTransmitsProgramChanges,
                PanDisruptsStereo => kMIDIPropertyPanDisruptsStereo,
                IsSampler => kMIDIPropertyIsSampler,
                IsDrumMachine => kMIDIPropertyIsDrumMachine,
                IsMixer => kMIDIPropertyIsMixer,
                IsEffectUnit => kMIDIPropertyIsEffectUnit,
                SupportsShowControl => kMIDIPropertySupportsShowControl,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::properties;

    use ::{
        Client,
        VirtualDestination,
    };

    fn setup() -> (Client, VirtualDestination) {
        let client = Client::new("Test Client").unwrap();
        let dest = client.virtual_destination("Test Dest", |_|()).unwrap();
        (client, dest)
    }

    #[test]
    fn test_not_set() {
        let (_client, dest) = setup();
        // Not set by default on Virtual Destinations
        let property = properties::TRANSMITS_PROGRAM_CHANGES;

        let value = dest.get_property_boolean(property);

        assert!(value.is_err())
    }

    #[test]
    fn test_roundtrip() {
        let (_client, dest) = setup();
        let property = properties::PRIVATE;
        
        dest.set_property_boolean(property, true).unwrap();
        let value = dest.get_property_boolean(property).unwrap();

        assert!(value, true)
    }
}
