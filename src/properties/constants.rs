use super::{
    BooleanProperty,
    IntegerProperty,
    StringProperty,
};

/// See [kMIDIPropertyName](https://developer.apple.com/reference/coremidi/kmidipropertyname)
pub const NAME: StringProperty = StringProperty::Name;

/// See [kMIDIPropertyManufacturer](https://developer.apple.com/reference/coremidi/kmidipropertymanufacturer)
pub const MANUFACTURER: StringProperty = StringProperty::Manufacturer;

/// See [kMIDIPropertyModel](https://developer.apple.com/reference/coremidi/kmidipropertymodel)
pub const MODEL: StringProperty = StringProperty::Model;

/// See [kMIDIPropertyUniqueID](https://developer.apple.com/reference/coremidi/kmidipropertyuniqueid)
pub const UNIQUE_ID: IntegerProperty = IntegerProperty::UniqueId;

/// See [kMIDIPropertyDeviceID](https://developer.apple.com/reference/coremidi/kmidipropertydeviceid)
pub const DEVICE_ID: IntegerProperty = IntegerProperty::DeviceId;

/// See [kMIDIPropertyReceiveChannels](https://developer.apple.com/reference/coremidi/kmidipropertyreceivechannels)
pub const RECEIVE_CHANNELS: IntegerProperty = IntegerProperty::ReceiveChannels;

/// See [kMIDIPropertyTransmitChannels](https://developer.apple.com/reference/coremidi/kmidipropertytransmitchannels)
pub const TRANSMIT_CHANNELS: IntegerProperty = IntegerProperty::TransmitChannels;

/// See [kMIDIPropertyMaxSysExSpeed](https://developer.apple.com/reference/coremidi/kmidipropertymaxsysexspeed)
pub const MAX_SYSEX_SPEED: IntegerProperty = IntegerProperty::MaxSysExSpeed;

/// See [kMIDIPropertyAdvanceScheduleTimeMuSec](https://developer.apple.com/reference/coremidi/kMIDIPropertyAdvanceScheduleTimeMuSec)
pub const ADVANCED_SCHEDULE_TIME_MUSEC: IntegerProperty = IntegerProperty::AdvanceScheduleTimeMuSec;

/// See [kMIDIPropertyIsEmbeddedEntity](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsEmbeddedEntity)
pub const IS_EMBEDDED_ENTITY: BooleanProperty = BooleanProperty::IsEmbeddedEntity;

/// See [kMIDIPropertyIsBroadcast](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsBroadcast)
pub const IS_BROADCAST: BooleanProperty = BooleanProperty::IsBroadcast;

/// See [kMIDIPropertySingleRealtimeEntity](https://developer.apple.com/reference/coremidi/kMIDIPropertySingleRealtimeEntity)
pub const SINGLE_REALTIME_ENTITY: IntegerProperty = IntegerProperty::SingleRealtimeEntity;

/// See [kMIDIPropertyConnectionUniqueID](https://developer.apple.com/reference/coremidi/kMIDIPropertyConnectionUniqueID)
pub const CONNECTION_UNIQUE_ID: IntegerProperty = IntegerProperty::ConnectionUniqueId;

/// See [kMIDIPropertyOffline](https://developer.apple.com/reference/coremidi/kMIDIPropertyOffline)
pub const OFFLINE: BooleanProperty = BooleanProperty::Offline;

/// See [kMIDIPropertyPrivate](https://developer.apple.com/reference/coremidi/kMIDIPropertyPrivate)
pub const PRIVATE: BooleanProperty = BooleanProperty::Private;

/// See [kMIDIPropertyDriverOwner](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverOwner)
pub const DRIVER_OWNER: StringProperty = StringProperty::DriverOwner;

/// See [kMIDIPropertyDriverVersion](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverVersion)
pub const DRIVER_VERSION: IntegerProperty = IntegerProperty::DriverVersion;

/// See [kMIDIPropertySupportsGeneralMIDI](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsGeneralMIDI)
pub const SUPPORTS_GENERAL_MIDI: BooleanProperty = BooleanProperty::SupportsGeneralMIDI;

/// See [kMIDIPropertySupportsMMC](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsMMC)
pub const SUPPORTS_MMC: BooleanProperty = BooleanProperty::SupportsMMC;

/// See [kMIDIPropertyCanRoute](https://developer.apple.com/reference/coremidi/kMIDIPropertyCanRoute)
pub const CAN_ROUTE: BooleanProperty = BooleanProperty::CanRoute;

/// See [kMIDIPropertyReceivesClock](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesClock)
pub const RECEIVES_CLOCK: BooleanProperty = BooleanProperty::ReceivesClock;

/// See [kMIDIPropertyReceivesMTC](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesMTC)
pub const RECEIVES_MTC: BooleanProperty = BooleanProperty::ReceivesMTC;

/// See [kMIDIPropertyReceivesNotes](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesNotes)
pub const RECEIVES_NOTES: BooleanProperty = BooleanProperty::ReceivesNotes;

/// See [kMIDIPropertyReceivesProgramChanges](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesProgramChanges)
pub const RECEIVES_PROGRAM_CHANGES: BooleanProperty = BooleanProperty::ReceivesProgramChanges;

/// See [kMIDIPropertyReceivesBankSelectMSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesBankSelectMSB)
pub const RECEIVES_BANK_SELECT_MSB: BooleanProperty = BooleanProperty::ReceivesBankSelectMSB;

/// See [kMIDIPropertyReceivesBankSelectLSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesBankSelectLSB)
pub const RECEIVES_BANK_SELECT_LSB: BooleanProperty = BooleanProperty::ReceivesBankSelectLSB;

/// See [kMIDIPropertyTransmitsBankSelectMSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsBankSelectMSB)
pub const TRANSMITS_BANK_SELECT_MSB: BooleanProperty = BooleanProperty::TransmitsBankSelectMSB;

/// See [kMIDIPropertyTransmitsBankSelectLSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsBankSelectLSB)
pub const TRANSMITS_BANK_SELECT_LSB: BooleanProperty = BooleanProperty::TransmitsBankSelectLSB;

/// See [kMIDIPropertyTransmitsClock](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsClock)
pub const TRANSMITS_CLOCK: BooleanProperty = BooleanProperty::TransmitsClock;

/// See [kMIDIPropertyTransmitsMTC](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsMTC)
pub const TRANSMITS_MTC: BooleanProperty = BooleanProperty::TransmitsMTC;

/// See [kMIDIPropertyTransmitsNotes](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsNotes)
pub const TRANSMITS_NOTES: BooleanProperty = BooleanProperty::TransmitsNotes;

/// See [kMIDIPropertyTransmitsProgramChanges](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsProgramChanges)
pub const TRANSMITS_PROGRAM_CHANGES: BooleanProperty = BooleanProperty::TransmitsProgramChanges;

/// See [kMIDIPropertyPanDisruptsStereo](https://developer.apple.com/reference/coremidi/kMIDIPropertyPanDisruptsStereo)
pub const PAN_DISRUPTS_STEREO: BooleanProperty = BooleanProperty::PanDisruptsStereo;

/// See [kMIDIPropertyIsSampler](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsSampler)
pub const IS_SAMPLER: BooleanProperty = BooleanProperty::IsSampler;

/// See [kMIDIPropertyIsDrumMachine](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsDrumMachine)
pub const IS_DRUM_MACHINE: BooleanProperty = BooleanProperty::IsDrumMachine;

/// See [kMIDIPropertyIsMixer](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsMixer)
pub const IS_MIXER: BooleanProperty = BooleanProperty::IsMixer;

/// See [kMIDIPropertyIsEffectUnit](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsEffectUnit)
pub const IS_EFFECT_UNIT: BooleanProperty = BooleanProperty::IsEffectUnit;

/// See [kMIDIPropertyMaxReceiveChannels](https://developer.apple.com/reference/coremidi/kMIDIPropertyMaxReceiveChannels)
pub const MAX_RECEIVE_CHANNELS: IntegerProperty = IntegerProperty::MaxRecieveChannels;

/// See [kMIDIPropertyMaxTransmitChannels](https://developer.apple.com/reference/coremidi/kMIDIPropertyMaxTransmitChannels)
pub const MAX_TRANSMIT_CHANNELS: IntegerProperty = IntegerProperty::MaxTransmitChannels;

/// See [kMIDIPropertyDriverDeviceEditorApp](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverDeviceEditorApp)
pub const DRIVER_DEVICE_EDITOR_APP: StringProperty = StringProperty::DriverDeviceEditorApp;

/// See [kMIDIPropertySupportsShowControl](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsShowControl)
pub const SUPPORTS_SHOW_CONTROL: BooleanProperty = BooleanProperty::SupportsShowControl;

/// See [kMIDIPropertyDisplayName](https://developer.apple.com/reference/coremidi/kMIDIPropertyDisplayName)
pub const DISPLAY_NAME: StringProperty = StringProperty::DisplayName;