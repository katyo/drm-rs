//! # Property
//!
//! A property of a modesetting resource.
//!
//! All modesetting resources have a set of properties that have values that
//! can be modified. These properties are modesetting resources themselves, and
//! may even have their own set of properties.
//!
//! Properties may have mutable values attached to them. These can be changed by
//! either changing the state of a resource (thereby affecting the property),
//! directly changing the property value itself, or by batching property changes
//! together and executing them all atomically.

use control;
use drm_ffi as ffi;

/// A raw property value that does not have a specific property type
pub type RawValue = u64;

/// A handle to a property
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Handle(control::RawResourceHandle);

impl From<Handle> for control::RawResourceHandle {
    fn from(handle: Handle) -> Self {
        handle.0
    }
}

impl From<Handle> for u32 {
    fn from(handle: Handle) -> Self {
        handle.0.into()
    }
}

impl From<control::RawResourceHandle> for Handle {
    fn from(handle: control::RawResourceHandle) -> Self {
        Handle(handle)
    }
}

impl control::ResourceHandle for Handle {
    const FFI_TYPE: u32 = ffi::DRM_MODE_OBJECT_PROPERTY;
}

impl std::fmt::Debug for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("property::Handle").field(&self.0).finish()
    }
}

/// Information about a property
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Info {
    pub(crate) handle: Handle,
    pub(crate) val_type: ValueType,
    pub(crate) mutable: bool,
    pub(crate) atomic: bool,
    pub(crate) info: ffi::drm_mode_get_property,
}

impl Info {
    /// Returns the name of this property.
    pub fn name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(&self.info.name[0] as _) }
    }

    /// Returns the ValueType of this property.
    pub fn value_type(&self) -> ValueType {
        self.val_type
    }

    /// Returns whether this property is mutable.
    pub fn mutable(&self) -> bool {
        self.mutable
    }

    /// Returns whether this property can be atomically updated.
    pub fn atomic(&self) -> bool {
        self.atomic
    }
}

/// A `ValueType` describes the types of value that a property uses.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ValueType {
    /// A catch-all for any unknown types
    Unknown,
    /// A True or False type
    Boolean,
    /// An unsigned integer that has a min and max value
    UnsignedRange(u64, u64),
    /// A signed integer that has a min and max value
    SignedRange(i64, i64),
    /// A set of values that are mutually exclusive
    Enum(EnumValues),
    /// A set of values that can be combined
    Bitmask,
    /// A chunk of binary data that must be acquired
    Blob,
    /// A non-specific DRM object
    Object,
    /// A CRTC object
    CRTC,
    /// A Connector object
    Connector,
    /// An Encoder object
    Encoder,
    /// A Framebuffer object
    Framebuffer,
    /// A Plane object
    Plane,
    /// A Property object
    Property,
}

impl ValueType {
    /// Given a raw value, convert it into a specific `Value`
    pub fn convert_value(&self, value: RawValue) -> Value {
        use std::mem::transmute as tm;

        match self {
            ValueType::Unknown => Value::Unknown(value),
            ValueType::Boolean => Value::Boolean(value != 0),
            ValueType::UnsignedRange(_, _) => Value::UnsignedRange(value),
            ValueType::SignedRange(_, _) => Value::SignedRange(value as i64),
            ValueType::Enum(values) => Value::Enum(values.get_value_from_raw_value(value)),
            ValueType::Bitmask => Value::Bitmask(value),
            ValueType::Blob => Value::Blob(value),
            ValueType::Object => Value::Object(unsafe { tm(value as u32) }),
            ValueType::CRTC => Value::CRTC(unsafe { tm(value as u32) }),
            ValueType::Connector => Value::Connector(unsafe { tm(value as u32) }),
            ValueType::Encoder => Value::Encoder(unsafe { tm(value as u32) }),
            ValueType::Framebuffer => Value::Framebuffer(unsafe { tm(value as u32) }),
            ValueType::Plane => Value::Plane(unsafe { tm(value as u32) }),
            ValueType::Property => Value::Property(unsafe { tm(value as u32) }),
        }
    }
}

/// The value of a property, in a typed format
#[allow(missing_docs)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Value<'a> {
    Unknown(RawValue),
    Boolean(bool),
    UnsignedRange(u64),
    SignedRange(i64),
    Enum(&'a EnumValue),
    Bitmask(u64),
    Blob(u64),
    Object(Option<super::RawResourceHandle>),
    CRTC(Option<super::crtc::Handle>),
    Connector(Option<super::connector::Handle>),
    Encoder(Option<super::encoder::Handle>),
    Framebuffer(Option<super::framebuffer::Handle>),
    Plane(Option<super::plane::Handle>),
    Property(Option<Handle>),
}

impl<'a> From<Value<'a>> for RawValue {
    fn from(value: Value<'a>) -> Self {
        use std::mem::transmute as tm;

        match value {
            Value::Unknown(x) => x,
            Value::Boolean(x) => {
                if x {
                    1
                } else {
                    0
                }
            }
            Value::UnsignedRange(x) => x,
            Value::SignedRange(x) => x as u64,
            Value::Enum(val) => val.value(),
            Value::Bitmask(x) => x,
            Value::Blob(x) => x,
            Value::Object(x) => unsafe { tm::<_, u32>(x).into() },
            Value::CRTC(x) => unsafe { tm::<_, u32>(x).into() },
            Value::Connector(x) => unsafe { tm::<_, u32>(x).into() },
            Value::Encoder(x) => unsafe { tm::<_, u32>(x).into() },
            Value::Framebuffer(x) => unsafe { tm::<_, u32>(x).into() },
            Value::Plane(x) => unsafe { tm::<_, u32>(x).into() },
            Value::Property(x) => unsafe { tm::<_, u32>(x).into() },
        }
    }
}

/// A single value of an `ValueType::Enum`
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct EnumValue(ffi::drm_mode_property_enum);

impl EnumValue {
    /// Returns the `RawValue` of this value
    pub fn value(&self) -> RawValue {
        self.0.value
    }

    /// Returns the name of this value
    pub fn name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(&self.0.name[0] as _) }
    }
}

impl std::fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EnumValue")
            .field("value", &self.value())
            .field("name", &self.name())
            .finish()
    }
}

/// A set of `EnumValue`s for a single property
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct EnumValues {
    pub(crate) values: [u64; 24],
    pub(crate) enums: [EnumValue; 24],
    pub(crate) length: usize,
}

impl EnumValues {
    /// Returns a tuple containing slices to the `RawValue`s and the `EnumValue`s
    pub fn values(&self) -> (&[RawValue], &[EnumValue]) {
        (&self.values[..self.length], &self.enums[..self.length])
    }

    pub fn get_value_from_raw_value(&self, value: RawValue) -> &EnumValue {
        let (_, enums) = self.values();
        &enums[value as usize]
    }
}

impl std::fmt::Debug for EnumValues {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (vals, enums) = self.values();

        f.debug_struct("EnumValues")
            .field("values", &vals)
            .field("enums", &enums)
            .finish()
    }
}
