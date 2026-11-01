use evdev::{AttributeSet, AttributeSetRef, Device, EventType, KeyCode};

pub trait DeviceAttributesLike {
    fn name(&self) -> Option<&str>;
    fn has_event(&self, event: EventType) -> bool;
    fn has_key(&self, key: KeyCode) -> bool;
}

#[derive(Debug, Clone)]
pub struct OwnedDeviceAttributes {
    pub name: Option<String>,
    pub supported_events: AttributeSet<EventType>,
    pub supported_keys: Option<AttributeSet<KeyCode>>,
}

#[derive(Copy, Clone)]
pub struct DeviceAttributes<'a> {
    pub name: Option<&'a str>,
    pub supported_events: &'a AttributeSetRef<EventType>,
    pub supported_keys: Option<&'a AttributeSetRef<KeyCode>>,
}

// its necessary to use an explicit lifetime as AttributeSetRef doesn't have a
// known size at compile time and it is dependent on the device's lifetime.

impl<'a> From<&'a Device> for DeviceAttributes<'a> {
    fn from(device: &'a Device) -> Self {
        Self {
            name: device.name(),
            supported_events: device.supported_events(),
            supported_keys: device.supported_keys(),
        }
    }
}

impl<'a> DeviceAttributesLike for DeviceAttributes<'a> {
    fn name(&self) -> Option<&str> {
        self.name
    }

    fn has_event(&self, event: EventType) -> bool {
        self.supported_events.contains(event)
    }

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys.is_some_and(|keys| keys.contains(key))
    }
}

impl<'a> From<DeviceAttributes<'a>> for OwnedDeviceAttributes {
    fn from(attributes: DeviceAttributes<'a>) -> Self {
        Self {
            name: attributes.name.map(|name| name.to_owned()),
            supported_events: attributes.supported_events.iter().collect(),
            supported_keys: attributes.supported_keys.map(|keys| keys.iter().collect()),
        }
    }
}

impl OwnedDeviceAttributes {
    pub fn new(
        name: Option<String>,
        supported_events: AttributeSet<EventType>,
        supported_keys: Option<AttributeSet<KeyCode>>,
    ) -> Self {
        Self {
            name,
            supported_events,
            supported_keys,
        }
    }
}

impl DeviceAttributesLike for OwnedDeviceAttributes {
    fn name(&self) -> Option<&str> {
        match &self.name {
            Some(name) => Some(name),
            None => None
        }
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
