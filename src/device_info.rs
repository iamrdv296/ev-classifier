use evdev::{AbsoluteAxisCode, AttributeSetRef, Device, KeyCode, PropType, RelativeAxisCode};

const MT_AXES: [AbsoluteAxisCode; 5] = [
    AbsoluteAxisCode::ABS_MT_SLOT,
    AbsoluteAxisCode::ABS_MT_TOUCH_MAJOR,
    AbsoluteAxisCode::ABS_MT_POSITION_X,
    AbsoluteAxisCode::ABS_MT_POSITION_Y,
    AbsoluteAxisCode::ABS_MT_TRACKING_ID,
];

const ABSOLUTE_XY_AXES: [AbsoluteAxisCode; 2] = [AbsoluteAxisCode::ABS_X, AbsoluteAxisCode::ABS_Y];

const RELATIVE_XY_AXES: [RelativeAxisCode; 2] = [RelativeAxisCode::REL_X, RelativeAxisCode::REL_Y];

const MT_XY_AXES: [AbsoluteAxisCode; 2] = [
    AbsoluteAxisCode::ABS_MT_POSITION_X,
    AbsoluteAxisCode::ABS_MT_POSITION_Y,
];

const WHEEL_AXES: [RelativeAxisCode; 4] = [
    RelativeAxisCode::REL_WHEEL,
    RelativeAxisCode::REL_HWHEEL,
    RelativeAxisCode::REL_WHEEL_HI_RES,
    RelativeAxisCode::REL_HWHEEL_HI_RES,
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

    fn has_any_key(&self, keys: &[KeyCode]) -> bool {
        has_any(keys, |&k| self.has_key(k))
    }

    fn has_all_keys(&self, keys: &[KeyCode]) -> bool {
        has_all(keys, |&k| self.has_key(k))
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
        has_all(&RELATIVE_XY_AXES, |&r| self.has_relative(r))
    }

    fn abs_xy(&self) -> bool {
        has_all(&ABSOLUTE_XY_AXES, |&a| self.has_absolute(a))
    }

    fn abs_mt_xy(&self) -> bool {
        has_all(&MT_XY_AXES, |&m| self.has_absolute(m))
    }

    fn has_mt_axis(&self) -> bool {
        has_any(&MT_AXES, |&a| self.has_absolute(a))
    }

    fn has_wheel(&self) -> bool {
        has_any(&WHEEL_AXES, |&w| self.has_relative(w))
    }
}

impl DeviceInfo for Device {
    fn name(&self) -> Option<&str> {
        Device::name(self)
    }

    fn supported_keys(&self) -> Option<&AttributeSetRef<KeyCode>> {
        Device::supported_keys(self)
    }

    fn supported_absolute_axes(&self) -> Option<&AttributeSetRef<AbsoluteAxisCode>> {
        Device::supported_absolute_axes(self)
    }

    fn supported_relative_axes(&self) -> Option<&AttributeSetRef<RelativeAxisCode>> {
        Device::supported_relative_axes(self)
    }

    fn properties(&self) -> &AttributeSetRef<PropType> {
        Device::properties(self)
    }
}

fn has_any<T>(slice: &[T], condition: impl FnMut(&T) -> bool) -> bool {
    slice.iter().any(condition)
}

fn has_all<T>(slice: &[T], condition: impl FnMut(&T) -> bool) -> bool {
    slice.iter().all(condition)
}
