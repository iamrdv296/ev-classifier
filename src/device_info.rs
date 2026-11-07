use evdev::{AbsoluteAxisCode, AttributeSetRef, Device, KeyCode, PropType, RelativeAxisCode};

const MT_AXES: [AbsoluteAxisCode; 5] = [
    AbsoluteAxisCode::ABS_MT_SLOT,
    AbsoluteAxisCode::ABS_MT_TOUCH_MAJOR,
    AbsoluteAxisCode::ABS_MT_POSITION_X,
    AbsoluteAxisCode::ABS_MT_POSITION_Y,
    AbsoluteAxisCode::ABS_MT_TRACKING_ID,
];

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

    fn rel_xy(&self) -> bool {
        self.has_all_relative(&[RelativeAxisCode::REL_X, RelativeAxisCode::REL_Y])
    }

    fn abs_xy(&self) -> bool {
        self.has_all_absolute(&[AbsoluteAxisCode::ABS_X, AbsoluteAxisCode::ABS_Y])
    }

    fn has_any_key(&self, keys: &[KeyCode]) -> bool {
        keys.iter().any(|&k| self.has_key(k))
    }

    fn has_all_keys(&self, keys: &[KeyCode]) -> bool {
        keys.iter().all(|&k| self.has_key(k))
    }

    fn has_any_relative(&self, axes: &[RelativeAxisCode]) -> bool {
        axes.iter().any(|&a| self.has_relative(a))
    }

    fn has_all_relative(&self, axes: &[RelativeAxisCode]) -> bool {
        axes.iter().all(|&a| self.has_relative(a))
    }

    fn has_any_absolute(&self, axes: &[AbsoluteAxisCode]) -> bool {
        axes.iter().any(|&a| self.has_absolute(a))
    }

    fn has_all_absolute(&self, axes: &[AbsoluteAxisCode]) -> bool {
        axes.iter().all(|&a| self.has_absolute(a))
    }

    fn has_any_property(&self, props: &[PropType]) -> bool {
        props.iter().any(|&p| self.has_property(p))
    }

    fn abs_mt_xy(&self) -> bool {
        self.has_all_absolute(&[
            AbsoluteAxisCode::ABS_MT_POSITION_X,
            AbsoluteAxisCode::ABS_MT_POSITION_Y,
        ])
    }

    fn has_mt_axis(&self) -> bool {
        self.has_any_absolute(&MT_AXES)
    }

    fn has_wheel(&self) -> bool {
        self.has_any_relative(&[
            RelativeAxisCode::REL_WHEEL,
            RelativeAxisCode::REL_HWHEEL,
            RelativeAxisCode::REL_WHEEL_HI_RES,
            RelativeAxisCode::REL_HWHEEL_HI_RES,
        ])
    }
}

impl DeviceInfo for Device {
    fn name(&self) -> Option<&str> {
        Device::name(self)
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
