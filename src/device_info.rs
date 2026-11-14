use evdev::{AbsoluteAxisCode, AttributeSetRef, Device, KeyCode, PropType, RelativeAxisCode};

/// Absolute axes associated with multi-touch (MT) protocol devices.
///
/// Reporting any one of these axes indicates the device speaks the Linux
/// multi-touch protocol (touchscreens, modern touchpads, etc.). Used by
/// [`DeviceInfo::has_mt_axis`].
const MT_AXES: [AbsoluteAxisCode; 5] = [
    AbsoluteAxisCode::ABS_MT_SLOT,
    AbsoluteAxisCode::ABS_MT_TOUCH_MAJOR,
    AbsoluteAxisCode::ABS_MT_POSITION_X,
    AbsoluteAxisCode::ABS_MT_POSITION_Y,
    AbsoluteAxisCode::ABS_MT_TRACKING_ID,
];

/// The single-touch absolute position axes (`ABS_X` and `ABS_Y`).
///
/// Reported by devices that provide absolute coordinates, such as graphics
/// tablets, touchscreens, and older touchpads. Used by [`DeviceInfo::abs_xy`].
const ABSOLUTE_XY_AXES: [AbsoluteAxisCode; 2] = [AbsoluteAxisCode::ABS_X, AbsoluteAxisCode::ABS_Y];

/// The relative motion axes (`REL_X` and `REL_Y`).
///
/// Reported by devices that provide movement deltas, such as mice and
/// trackballs. Used by [`DeviceInfo::rel_xy`].
const RELATIVE_XY_AXES: [RelativeAxisCode; 2] = [RelativeAxisCode::REL_X, RelativeAxisCode::REL_Y];

/// The multi-touch position axes (`ABS_MT_POSITION_X` and `ABS_MT_POSITION_Y`).
///
/// Reported by multi-touch devices to give the coordinates of each contact.
/// Used by [`DeviceInfo::abs_mt_xy`].
const MT_XY_AXES: [AbsoluteAxisCode; 2] = [
    AbsoluteAxisCode::ABS_MT_POSITION_X,
    AbsoluteAxisCode::ABS_MT_POSITION_Y,
];

/// Relative axes that represent scroll wheels.
///
/// Includes the vertical and horizontal wheels, along with their high
/// resolution variants. Used by [`DeviceInfo::has_wheel`].
const WHEEL_AXES: [RelativeAxisCode; 4] = [
    RelativeAxisCode::REL_WHEEL,
    RelativeAxisCode::REL_HWHEEL,
    RelativeAxisCode::REL_WHEEL_HI_RES,
    RelativeAxisCode::REL_HWHEEL_HI_RES,
];

/// The four directional pad buttons (`BTN_DPAD_UP`, `BTN_DPAD_DOWN`,
/// `BTN_DPAD_LEFT`, and `BTN_DPAD_RIGHT`).
///
/// Some controllers report their D-pad as discrete buttons rather than as a
/// hat axis. A device must support all four to count as having a D-pad. Used
/// by [`DeviceInfo::has_directional_input`].
const DPAD_KEYS: [KeyCode; 4] = [
    KeyCode::BTN_DPAD_UP,
    KeyCode::BTN_DPAD_DOWN,
    KeyCode::BTN_DPAD_LEFT,
    KeyCode::BTN_DPAD_RIGHT,
];

/// Read-only description of an input device's capabilities.
///
/// This abstracts over [`evdev::Device`] so that capability checks (and the
/// classification logic built on top of them, such as
/// [`KeyBased`](crate::KeyBased)) can be run against real devices, test
/// doubles, or trait objects.
///
/// Implementors only need to provide the five required accessors; all the
/// `has_*` and axis-pair helpers are provided as default methods built on top
/// of them.
pub trait DeviceInfo {
    /// Returns the device's human-readable name, if it has one.
    fn name(&self) -> Option<&str>;

    /// Returns the set of key and button codes the device supports, or `None`
    /// if the device reports no key capabilities at all.
    fn supported_keys(&self) -> Option<&AttributeSetRef<KeyCode>>;

    /// Returns the set of relative axes the device supports, or `None` if it
    /// reports no relative axes.
    fn supported_relative_axes(&self) -> Option<&AttributeSetRef<RelativeAxisCode>>;

    /// Returns the set of absolute axes the device supports, or `None` if it
    /// reports no absolute axes.
    fn supported_absolute_axes(&self) -> Option<&AttributeSetRef<AbsoluteAxisCode>>;

    /// Returns the device's input properties (e.g. `INPUT_PROP_POINTER`,
    /// `INPUT_PROP_DIRECT`).
    ///
    /// Unlike the other capability sets this is never `None`; a device with no
    /// properties returns an empty set.
    fn properties(&self) -> &AttributeSetRef<PropType>;

    /// Returns `true` if the device supports the given key or button.
    ///
    /// Returns `false` if the device reports no keys at all.
    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys().is_some_and(|sk| sk.contains(key))
    }

    /// Returns `true` if the device supports **at least one** of the given
    /// keys.
    ///
    /// Returns `false` for an empty slice.
    fn has_any_key(&self, keys: &[KeyCode]) -> bool {
        has_any(keys, |&k| self.has_key(k))
    }

    /// Returns `true` if the device supports **every one** of the given keys.
    ///
    /// Returns `true` for an empty slice (vacuous truth).
    fn has_all_keys(&self, keys: &[KeyCode]) -> bool {
        has_all(keys, |&k| self.has_key(k))
    }

    /// Returns `true` if the device supports the given relative axis.
    ///
    /// Returns `false` if the device reports no relative axes at all.
    fn has_relative(&self, rel: RelativeAxisCode) -> bool {
        self.supported_relative_axes()
            .is_some_and(|sra| sra.contains(rel))
    }

    /// Returns `true` if the device supports the given absolute axis.
    ///
    /// Returns `false` if the device reports no absolute axes at all.
    fn has_absolute(&self, abs: AbsoluteAxisCode) -> bool {
        self.supported_absolute_axes()
            .is_some_and(|saa| saa.contains(abs))
    }

    /// Returns `true` if the device has the given input property.
    fn has_property(&self, prop: PropType) -> bool {
        self.properties().contains(prop)
    }

    /// Returns `true` if the device supports both relative position axes
    /// (`REL_X` and `REL_Y`).
    ///
    /// This is the typical signature of a mouse or trackball.
    fn rel_xy(&self) -> bool {
        has_all(&RELATIVE_XY_AXES, |&r| self.has_relative(r))
    }

    /// Returns `true` if the device supports both absolute position axes
    /// (`ABS_X` and `ABS_Y`).
    ///
    /// Typical of tablets, touchscreens, and single-touch touchpads.
    fn abs_xy(&self) -> bool {
        has_all(&ABSOLUTE_XY_AXES, |&a| self.has_absolute(a))
    }

    /// Returns `true` if the device supports both multi-touch position axes
    /// (`ABS_MT_POSITION_X` and `ABS_MT_POSITION_Y`).
    fn abs_mt_xy(&self) -> bool {
        has_all(&MT_XY_AXES, |&m| self.has_absolute(m))
    }

    /// Returns `true` if the device supports **any** multi-touch axis
    /// (slot, touch major, position X/Y, or tracking ID).
    ///
    /// This is a looser check than [`abs_mt_xy`](Self::abs_mt_xy), which
    /// requires both position axes.
    fn has_mt_axis(&self) -> bool {
        has_any(&MT_AXES, |&a| self.has_absolute(a))
    }

    /// Returns `true` if the device provides some form of directional input.
    ///
    /// This is the case if it supports any of:
    ///
    /// - the absolute position axes (`ABS_X` and `ABS_Y`), such as an analog
    ///   stick,
    /// - the `ABS_HAT0X` axis, which is how many controllers report a D-pad
    ///   as a hat switch, or
    /// - all four D-pad buttons (`BTN_DPAD_UP`, `BTN_DPAD_DOWN`,
    ///   `BTN_DPAD_LEFT`, and `BTN_DPAD_RIGHT`).
    ///
    /// Note that this is a broad capability check and is not specific to
    /// game controllers: any device with `ABS_X` and `ABS_Y` (such as a
    /// touchscreen or tablet) will also return `true`.
    fn has_directional_input(&self) -> bool {
        self.abs_xy()
            || self.has_absolute(AbsoluteAxisCode::ABS_HAT0X)
            || self.has_all_keys(&DPAD_KEYS)
    }

    /// Returns `true` if the device has any scroll wheel axis, vertical or
    /// horizontal, including the high-resolution variants.
    fn has_wheel(&self) -> bool {
        has_any(&WHEEL_AXES, |&w| self.has_relative(w))
    }
}

/// Implements [`DeviceInfo`] for [`evdev::Device`] by delegating to its
/// inherent methods of the same name.
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

/// Returns `true` if `condition` holds for at least one element of `slice`.
///
/// Short-circuits on the first match. Returns `false` for an empty slice.
fn has_any<T>(slice: &[T], condition: impl FnMut(&T) -> bool) -> bool {
    slice.iter().any(condition)
}

/// Returns `true` if `condition` holds for every element of `slice`.
///
/// Short-circuits on the first failure. Returns `true` for an empty slice.
fn has_all<T>(slice: &[T], condition: impl FnMut(&T) -> bool) -> bool {
    slice.iter().all(condition)
}
