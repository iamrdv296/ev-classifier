use evdev::{
    AbsoluteAxisCode, AttributeSetRef, Device, KeyCode, PropType, RelativeAxisCode,
};

pub trait DeviceInfo {
    fn name(&self) -> Option<&str>;
    fn supported_keys(&self) -> Option<&AttributeSetRef<KeyCode>>;
    fn supported_relative_axes(&self) -> Option<&AttributeSetRef<RelativeAxisCode>>;
    fn supported_absolute_axes(&self) -> Option<&AttributeSetRef<AbsoluteAxisCode>>;
    fn properties(&self) -> &AttributeSetRef<PropType>;

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys().is_some_and(|sk| sk.contains(key))
    }
    fn has_relative(&self, rel: RelativeAxisCode) -> bool {
        self.supported_relative_axes()
            .is_some_and(|sra| sra.contains(rel))
    }
    fn has_absolute(&self, abs: AbsoluteAxisCode) -> bool {
        self.supported_absolute_axes()
            .is_some_and(|saa| saa.contains(abs))
    }
    fn has_property(&self, prop: PropType) -> bool {
        self.properties().contains(prop)
    }
}

impl DeviceInfo for Device {
    fn name(&self) -> Option<&str> {
        self.name()
    }

    fn supported_keys(&self) -> Option<&AttributeSetRef<KeyCode>> {
        self.supported_keys()
    }

    fn supported_absolute_axes(&self) -> Option<&AttributeSetRef<AbsoluteAxisCode>> {
        self.supported_absolute_axes()
    }

    fn supported_relative_axes(&self) -> Option<&AttributeSetRef<RelativeAxisCode>> {
        self.supported_relative_axes()
    }

    fn properties(&self) -> &AttributeSetRef<PropType> {
        self.properties()
    }
}
