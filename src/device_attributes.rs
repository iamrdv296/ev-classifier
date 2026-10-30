use evdev::{AttributeSetRef, Device, EventType, KeyCode};

const UNKNOWN_NAME: &str = "Unknown Name";

pub struct DeviceAttributes<'a> {
    pub name: String,
    pub supported_events: &'a AttributeSetRef<EventType>,
    pub supported_keys: Option<&'a AttributeSetRef<KeyCode>>,
}

// its necessary to use an explicit lifetime as AttributeSetRef doesn't have a
// known size at compile time and it is dependent on the device's lifetime.

impl<'a> DeviceAttributes<'a> {
    pub fn from_device(device: &'a Device) -> Self {
        Self {
            name: device.name().unwrap_or(UNKNOWN_NAME).to_owned(),
            supported_events: device.supported_events(),
            supported_keys: device.supported_keys(),
        }
    }
}