use evdev::{AttributeSet, Device, EventType, KeyCode};

pub trait DeviceAttributesLike {
    fn name(&self) -> Option<&str>;
    fn has_event(&self, event: EventType) -> bool;
    fn has_key(&self, key: KeyCode) -> bool;
}

#[derive(Debug, Clone)]
pub struct DeviceAttributes {
    pub name: Option<String>,
    pub supported_events: AttributeSet<EventType>,
    pub supported_keys: AttributeSet<KeyCode>,
}

impl From<&Device> for DeviceAttributes {
    fn from(device: &Device) -> Self {
        Self {
            name: device.name().map(str::to_string),
            supported_events: device.supported_events().iter().collect(),
            supported_keys: device
                .supported_keys()
                .map(|keys| keys.iter().collect())
                .unwrap_or_default(),
        }
    }
}

impl DeviceAttributesLike for DeviceAttributes {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn has_event(&self, event: EventType) -> bool {
        self.supported_events.contains(event)
    }

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys.contains(key)
    }
}

impl DeviceAttributesLike for Device {
    fn name(&self) -> Option<&str> {
        self.name()
    }

    fn has_event(&self, event: EventType) -> bool {
        self.supported_events().contains(event)
    }

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys().is_some_and(|sk| sk.contains(key))
    }
}

// TODO: Make this more DRY maybe this kinda sucks rn
