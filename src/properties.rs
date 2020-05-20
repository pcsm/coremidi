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

pub use self::constants::*;

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
    fn as_string_ref(&self) -> CFStringRef {
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

pub fn get_string_property_inner<N>(object: &Object, name: N) -> Result<String, OSStatus> where
    N: Into<StringPropertyName>,
{
    let name = name.into();
    let mut string_ref = MaybeUninit::uninit();
    let status = unsafe {
        MIDIObjectGetStringProperty(object.0, name.as_string_ref(), string_ref.as_mut_ptr())
    };
    result_from_status(status, || {
        let string_ref = unsafe { string_ref.assume_init() };
        if string_ref.is_null() { return "".to_string().into() };
        let cf_string: CFString = unsafe { TCFType::wrap_under_create_rule(string_ref) };
        cf_string.to_string().into()
    })
}

pub fn set_string_property_inner<N, V>(object: &Object, name: N, value: V) -> Result<(), OSStatus> where
    N: Into<StringPropertyName>,
    V: AsRef<str>,
{
    let name = name.into();
    let string = CFString::new(value.as_ref());
    let string_ref = string.as_concrete_TypeRef();
    let status = unsafe {
        MIDIObjectSetStringProperty(object.0, name.as_string_ref(), string_ref)
    };
    unit_result_from_status(status)
}

/// A MIDI object property whose value is an Integer
///
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

impl IntegerProperty {
    /// Note: Should only be used internally with predefined CoreMidi constants,
    /// since it compares pointers of the incoming CFStringRef and the constants
    fn try_from_constant_string_ref(key: CFStringRef) -> Result<Self, ()> {
        use self::IntegerProperty::*;
        match_property_keys! {
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

pub fn get_integer_property_inner<N>(object: &Object, name: N) -> Result<i32, OSStatus> where
    N: Into<IntegerPropertyName>,
{
    let name = name.into();
    get_integer_property_inner_concrete(object, name.as_string_ref())
}

fn get_integer_property_inner_concrete(object: &Object, name: CFStringRef) -> Result<i32, OSStatus> {
    let mut value = MaybeUninit::uninit();
    let status = unsafe {
        MIDIObjectGetIntegerProperty(object.0, name, value.as_mut_ptr())
    };
    result_from_status(status, || {
        let value = unsafe { value.assume_init() };
        value.into()
    })
}

pub fn set_integer_property_inner<N, V>(object: &Object, name: N, value: V) -> Result<(), OSStatus> where
    N: Into<IntegerPropertyName>,
    V: Into<i32>,
{
    let name = name.into();
    set_integer_property_inner_concrete(object, name.as_string_ref(), value.into())
}

fn set_integer_property_inner_concrete(object: &Object, name: CFStringRef, value: i32) -> Result<(), OSStatus> {
    let status = unsafe {
        MIDIObjectSetIntegerProperty(object.0, name, value)
    };
    unit_result_from_status(status)
}

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

pub fn get_boolean_property_inner<N>(object: &Object, name: N) -> Result<bool, OSStatus> where
    N: Into<BooleanPropertyName>,
{
    let name = name.into();
    get_integer_property_inner_concrete(object, name.as_string_ref()).map(|val| (val == 1))
}

pub fn set_boolean_property_inner<N, V>(object: &Object, name: N, value: V) -> Result<(), OSStatus> where
    N: Into<BooleanPropertyName>,
    V: Into<bool>,
{
    let name = name.into();
    let value = if value.into() { 1 } else { 0 };
    set_integer_property_inner_concrete(object, name.as_string_ref(), value)
}

/// The set of CoreMIDI-defined properties that might be available for MIDI objects.
///
/// Note that [`kMIDIPropertyNameConfiguration`](https://developer.apple.com/reference/coremidi/kMIDIPropertyNameConfiguration)
/// and [`kMIDIPropertyImage`](https://developer.apple.com/reference/coremidi/kMIDIPropertyImage)
/// are not currently supported.
pub mod constants {
    use super::*;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    use ::{
        Client,
        VirtualDestination,
    };

    const NAME_ORIG: &str = "A";

    fn setup() -> (Client, VirtualDestination) {
        let client = Client::new("Test Client").unwrap();
        let dest = client.virtual_destination(NAME_ORIG, |_|()).unwrap();
        (client, dest)
    }

    mod string {
        use super::*;

        const NAME_MODIFIED: &str = "B";

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
            let property = constants::NAME;

            check_get_original(property, &dest);
            check_roundtrip(property, &dest);
        }
    }

    mod integer {
        use super::*;

        const ADVANCED_SCHEDULE_TIME: i32 = 44;

        #[test]
        fn test_not_set() {
            let (_client, dest) = setup();
            // Is not set by default for Virtual Destinations
            let property = constants::ADVANCED_SCHEDULE_TIME_MUSEC;

            let value  = dest.get_property_integer(property);

            assert!(value.is_err())
        }

        #[test]
        fn test_roundtrip() {
            let (_client, dest) = setup();
            let property = constants::ADVANCED_SCHEDULE_TIME_MUSEC;

            dest.set_property_integer(property, ADVANCED_SCHEDULE_TIME).unwrap();
            let num = dest.get_property_integer(property).unwrap();

            assert_eq!(num, ADVANCED_SCHEDULE_TIME);
        }
    }

    mod boolean {
        use super::*;

        #[test]
        fn test_not_set() {
            let (_client, dest) = setup();
            // Not set by default on Virtual Destinations
            let property = constants::TRANSMITS_PROGRAM_CHANGES;

            let value = dest.get_property_boolean(property);

            assert!(value.is_err())
        }

        #[test]
        fn test_roundtrip() {
            let (_client, dest) = setup();
            let property = constants::PRIVATE;
            
            dest.set_property_boolean(property, true).unwrap();
            let value = dest.get_property_boolean(property).unwrap();

            assert!(value, true)
        }
    }
}
