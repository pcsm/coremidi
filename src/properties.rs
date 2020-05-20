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

pub trait PropertyGetter<T> {
    fn value_from(&self, object: &Object) -> Result<T, OSStatus>;
}

pub trait PropertySetter<T> {
    fn set_value(&self, object: &Object, value: T) -> Result<(), OSStatus>;
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

impl PropertyGetter<String> for StringProperty {
    fn value_from(&self, object: &Object) -> Result<String, OSStatus> {
        get_string_property_inner(object, *self)
    }
}

impl<T> PropertySetter<T> for StringProperty where T: AsRef<str> {
    fn set_value(&self, object: &Object, value: T) -> Result<(), OSStatus> {
        set_string_property_inner(object, *self, value)
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

impl PropertyGetter<i32> for IntegerProperty where {
    fn value_from(&self, object: &Object) -> Result<i32, OSStatus> {
        get_integer_property_inner(object, *self)
    }
}

impl <T> PropertySetter<T> for IntegerProperty where T: Into<i32> {
    fn set_value(&self, object: &Object, value: T) -> Result<(), OSStatus> {
        set_integer_property_inner(object, *self, value)
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

impl PropertyGetter<bool> for BooleanProperty {
    fn value_from(&self, object: &Object) -> Result<bool, OSStatus> {
        get_boolean_property_inner(object, *self)
    }
}

impl<T> PropertySetter<T> for BooleanProperty where T: Into<bool> {
    fn set_value(&self, object: &Object, value: T) -> Result<(), OSStatus> {
        set_boolean_property_inner(object, *self, value)
    }
}

/// The set of properties that might be available for MIDI objects.
///
pub struct Properties;

impl Properties {
    /// See [kMIDIPropertyName](https://developer.apple.com/reference/coremidi/kmidipropertyname)
    pub fn name() -> StringProperty { StringProperty::Name }
    
    /// See [kMIDIPropertyManufacturer](https://developer.apple.com/reference/coremidi/kmidipropertymanufacturer)
    pub fn manufacturer() -> StringProperty { StringProperty::Manufacturer }
    
    /// See [kMIDIPropertyModel](https://developer.apple.com/reference/coremidi/kmidipropertymodel)
    pub fn model() -> StringProperty { StringProperty::Model }
    
    /// See [kMIDIPropertyUniqueID](https://developer.apple.com/reference/coremidi/kmidipropertyuniqueid)
    pub fn unique_id() -> IntegerProperty { IntegerProperty::UniqueId }
    
    /// See [kMIDIPropertyDeviceID](https://developer.apple.com/reference/coremidi/kmidipropertydeviceid)
    pub fn device_id() -> IntegerProperty { IntegerProperty::DeviceId }
    
    /// See [kMIDIPropertyReceiveChannels](https://developer.apple.com/reference/coremidi/kmidipropertyreceivechannels)
    pub fn receive_channels() -> IntegerProperty { IntegerProperty::ReceiveChannels }
    
    /// See [kMIDIPropertyTransmitChannels](https://developer.apple.com/reference/coremidi/kmidipropertytransmitchannels)
    pub fn transmit_channels() -> IntegerProperty { IntegerProperty::TransmitChannels }
    
    /// See [kMIDIPropertyMaxSysExSpeed](https://developer.apple.com/reference/coremidi/kmidipropertymaxsysexspeed)
    pub fn max_sysex_speed() -> IntegerProperty { IntegerProperty::MaxSysExSpeed }
    
    /// See [kMIDIPropertyAdvanceScheduleTimeMuSec](https://developer.apple.com/reference/coremidi/kMIDIPropertyAdvanceScheduleTimeMuSec)
    pub fn advance_schedule_time_musec() -> IntegerProperty { IntegerProperty::AdvanceScheduleTimeMuSec }
    
    /// See [kMIDIPropertyIsEmbeddedEntity](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsEmbeddedEntity)
    pub fn is_embedded_entity() -> BooleanProperty { BooleanProperty::IsEmbeddedEntity }
    
    /// See [kMIDIPropertyIsBroadcast](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsBroadcast)
    pub fn is_broadcast() -> BooleanProperty { BooleanProperty::IsBroadcast }
    
    /// See [kMIDIPropertySingleRealtimeEntity](https://developer.apple.com/reference/coremidi/kMIDIPropertySingleRealtimeEntity)
    pub fn single_realtime_entity() -> IntegerProperty { IntegerProperty::SingleRealtimeEntity }
    
    /// See [kMIDIPropertyConnectionUniqueID](https://developer.apple.com/reference/coremidi/kMIDIPropertyConnectionUniqueID)
    pub fn connection_unique_id() -> IntegerProperty { IntegerProperty::ConnectionUniqueId }
    
    /// See [kMIDIPropertyOffline](https://developer.apple.com/reference/coremidi/kMIDIPropertyOffline)
    pub fn offline() -> BooleanProperty { BooleanProperty::Offline }
    
    /// See [kMIDIPropertyPrivate](https://developer.apple.com/reference/coremidi/kMIDIPropertyPrivate)
    pub fn private() -> BooleanProperty { BooleanProperty::Private }
    
    /// See [kMIDIPropertyDriverOwner](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverOwner)
    pub fn driver_owner() -> StringProperty { StringProperty::DriverOwner }
    
    // /// See [kMIDIPropertyNameConfiguration](https://developer.apple.com/reference/coremidi/kMIDIPropertyNameConfiguration)
    // pub fn name_configuration() -> Property { unsafe { Property(kMIDIPropertyNameConfiguration) } }
    
    // /// See [kMIDIPropertyImage](https://developer.apple.com/reference/coremidi/kMIDIPropertyImage)
    // pub fn image() -> Property { unsafe { Property(kMIDIPropertyImage) } }
    
    /// See [kMIDIPropertyDriverVersion](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverVersion)
    pub fn driver_version() -> IntegerProperty { IntegerProperty::DriverVersion }
    
    /// See [kMIDIPropertySupportsGeneralMIDI](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsGeneralMIDI)
    pub fn supports_general_midi() -> BooleanProperty { BooleanProperty::SupportsGeneralMIDI }
    
    /// See [kMIDIPropertySupportsMMC](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsMMC)
    pub fn supports_mmc() -> BooleanProperty { BooleanProperty::SupportsMMC }
    
    /// See [kMIDIPropertyCanRoute](https://developer.apple.com/reference/coremidi/kMIDIPropertyCanRoute)
    pub fn can_route() -> BooleanProperty { BooleanProperty::CanRoute }
    
    /// See [kMIDIPropertyReceivesClock](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesClock)
    pub fn receives_clock() -> BooleanProperty { BooleanProperty::ReceivesClock }
    
    /// See [kMIDIPropertyReceivesMTC](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesMTC)
    pub fn receives_mtc() -> BooleanProperty { BooleanProperty::ReceivesMTC }
    
    /// See [kMIDIPropertyReceivesNotes](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesNotes)
    pub fn receives_notes() -> BooleanProperty { BooleanProperty::ReceivesNotes }
    
    /// See [kMIDIPropertyReceivesProgramChanges](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesProgramChanges)
    pub fn receives_program_changes() -> BooleanProperty { BooleanProperty::ReceivesProgramChanges }
    
    /// See [kMIDIPropertyReceivesBankSelectMSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesBankSelectMSB)
    pub fn receives_bank_select_msb() -> BooleanProperty { BooleanProperty::ReceivesBankSelectMSB }

    /// See [kMIDIPropertyReceivesBankSelectLSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyReceivesBankSelectLSB)
    pub fn receives_bank_select_lsb() -> BooleanProperty { BooleanProperty::ReceivesBankSelectLSB }

    /// See [kMIDIPropertyTransmitsBankSelectMSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsBankSelectMSB)
    pub fn transmits_bank_select_msb() -> BooleanProperty { BooleanProperty::TransmitsBankSelectMSB }

    /// See [kMIDIPropertyTransmitsBankSelectLSB](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsBankSelectLSB)
    pub fn transmits_bank_select_lsb() -> BooleanProperty { BooleanProperty::TransmitsBankSelectLSB }

    /// See [kMIDIPropertyTransmitsClock](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsClock)
    pub fn transmits_clock() -> BooleanProperty { BooleanProperty::TransmitsClock }

    /// See [kMIDIPropertyTransmitsMTC](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsMTC)
    pub fn transmits_mtc() -> BooleanProperty { BooleanProperty::TransmitsMTC }

    /// See [kMIDIPropertyTransmitsNotes](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsNotes)
    pub fn transmits_notes() -> BooleanProperty { BooleanProperty::TransmitsNotes }

    /// See [kMIDIPropertyTransmitsProgramChanges](https://developer.apple.com/reference/coremidi/kMIDIPropertyTransmitsProgramChanges)
    pub fn transmits_program_changes() -> BooleanProperty { BooleanProperty::TransmitsProgramChanges }

    /// See [kMIDIPropertyPanDisruptsStereo](https://developer.apple.com/reference/coremidi/kMIDIPropertyPanDisruptsStereo)
    pub fn pan_disrupts_stereo() -> BooleanProperty { BooleanProperty::PanDisruptsStereo }

    /// See [kMIDIPropertyIsSampler](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsSampler)
    pub fn is_sampler() -> BooleanProperty { BooleanProperty::IsSampler }

    /// See [kMIDIPropertyIsDrumMachine](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsDrumMachine)
    pub fn is_drum_machine() -> BooleanProperty { BooleanProperty::IsDrumMachine }

    /// See [kMIDIPropertyIsMixer](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsMixer)
    pub fn is_mixer() -> BooleanProperty { BooleanProperty::IsMixer }

    /// See [kMIDIPropertyIsEffectUnit](https://developer.apple.com/reference/coremidi/kMIDIPropertyIsEffectUnit)
    pub fn is_effect_unit() -> BooleanProperty { BooleanProperty::IsEffectUnit }

    /// See [kMIDIPropertyMaxReceiveChannels](https://developer.apple.com/reference/coremidi/kMIDIPropertyMaxReceiveChannels)
    pub fn max_receive_channels() -> IntegerProperty { IntegerProperty::MaxRecieveChannels }

    /// See [kMIDIPropertyMaxTransmitChannels](https://developer.apple.com/reference/coremidi/kMIDIPropertyMaxTransmitChannels)
    pub fn max_transmit_channels() -> IntegerProperty { IntegerProperty::MaxTransmitChannels }

    /// See [kMIDIPropertyDriverDeviceEditorApp](https://developer.apple.com/reference/coremidi/kMIDIPropertyDriverDeviceEditorApp)
    pub fn driver_device_editor_app() -> StringProperty { StringProperty::DriverDeviceEditorApp }

    /// See [kMIDIPropertySupportsShowControl](https://developer.apple.com/reference/coremidi/kMIDIPropertySupportsShowControl)
    pub fn supports_show_control() -> BooleanProperty { BooleanProperty::SupportsShowControl }

    /// See [kMIDIPropertyDisplayName](https://developer.apple.com/reference/coremidi/kMIDIPropertyDisplayName)
    pub fn display_name() -> StringProperty { StringProperty::DisplayName }
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
        fn check_get_original(property: &StringProperty, dest: &VirtualDestination) {
            let name: String = property.value_from(dest).unwrap();

            assert_eq!(name, NAME_ORIG);
        }

        // Test setting then getting the "name" property
        fn check_roundtrip(property: &StringProperty, dest: &VirtualDestination) {
            property.set_value(dest, NAME_MODIFIED).unwrap();
            let name: String = property.value_from(dest).unwrap();

            assert_eq!(name, NAME_MODIFIED);
        }

        #[test]
        fn test_from_constant() {
            let (_client, dest) = setup();
            let property = Properties::name();

            check_get_original(&property, &dest);
            check_roundtrip(&property, &dest);
        }
    }

    mod integer {
        use super::*;

        const ADVANCED_SCHEDULE_TIME: i32 = 44;

        #[test]
        fn test_not_set() {
            let (_client, dest) = setup();
            // Is not set by default for Virtual Destinations
            let property = Properties::advance_schedule_time_musec();

            let value: Result<i32, _> = property.value_from(&dest);

            assert!(value.is_err())
        }

        #[test]
        fn test_roundtrip() {
            let (_client, dest) = setup();
            let property = Properties::advance_schedule_time_musec();

            property.set_value(&dest, ADVANCED_SCHEDULE_TIME).unwrap();
            let num: i32 = property.value_from(&dest).unwrap();

            assert_eq!(num, ADVANCED_SCHEDULE_TIME);
        }
    }

    mod boolean {
        use super::*;

        #[test]
        fn test_not_set() {
            let (_client, dest) = setup();
            // Not set by default on Virtual Destinations
            let property = Properties::transmits_program_changes();

            let value: Result<bool, _> = property.value_from(&dest);

            assert!(value.is_err())
        }

        #[test]
        fn test_roundtrip() {
            let (_client, dest) = setup();
            let property = Properties::private();
            
            property.set_value(&dest, true).unwrap();
            let value: bool = property.value_from(&dest).unwrap();

            assert!(value, true)
        }
    }
}
