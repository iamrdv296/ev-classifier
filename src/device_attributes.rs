use evdev::{AttributeSet, AttributeSetRef, Device, EventType, KeyCode};

const UNKNOWN_NAME: &str = "Unknown Name";

pub trait DeviceAttributesLike {
    fn name(&self) -> &str;
    fn has_event(&self, event: EventType) -> bool;
    fn has_key(&self, key: KeyCode) -> bool;
}

#[derive(Debug, Clone)]
pub struct OwnedDeviceAttributes {
    pub name: String,
    pub supported_events: AttributeSet<EventType>,
    pub supported_keys: Option<AttributeSet<KeyCode>>,
}

pub struct DeviceAttributes<'a> {
    pub name: &'a str,
    pub supported_events: &'a AttributeSetRef<EventType>,
    pub supported_keys: Option<&'a AttributeSetRef<KeyCode>>,
}

// its necessary to use an explicit lifetime as AttributeSetRef doesn't have a
// known size at compile time and it is dependent on the device's lifetime.

impl<'a> DeviceAttributes<'a> {
    pub fn from_device(device: &'a Device) -> Self {
        Self {
            name: device.name().unwrap_or(UNKNOWN_NAME),
            supported_events: device.supported_events(),
            supported_keys: device.supported_keys(),
        }
    }
}

impl<'a> DeviceAttributesLike for DeviceAttributes<'a> {
    fn name(&self) -> &str {
        &self.name
    }

    fn has_event(&self, event: EventType) -> bool {
        self.supported_events.contains(event)
    }

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys.map_or(false, |keys| keys.contains(key))
    }
}

impl<'a> From<DeviceAttributes<'a>> for OwnedDeviceAttributes {
    fn from(attributes: DeviceAttributes<'a>) -> Self {
        Self {
            name: attributes.name.to_owned(),
            supported_events: attributes.supported_events.iter().collect(),
            supported_keys: attributes.supported_keys.map(|keys| keys.iter().collect()),
        }
    }
}

impl DeviceAttributesLike for OwnedDeviceAttributes {
    fn name(&self) -> &str {
        &self.name
    }

    fn has_event(&self, event: EventType) -> bool {
        self.supported_events.contains(event)
    }

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys
            .as_ref()
            .is_some_and(|keys| keys.contains(key))
    }
}
