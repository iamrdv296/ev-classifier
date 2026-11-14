use crate::DeviceInfo;
use evdev::{KeyCode, PropType};

/// Classifies devices by their relative axes, buttons, and input properties.
///
/// This covers pointing devices that report movement as deltas: mice and
/// pointing sticks. Implementors only need to implement [`DeviceInfo`] (and
/// [`AbsoluteAxesBased`], which is used to rule out absolute-axis devices
/// that are not mice); all methods here are provided as defaults.
///
/// Classifications are not mutually exclusive in principle, though
/// [`is_mouse`](Self::is_mouse) explicitly excludes pointing sticks on the
/// relative path.
pub trait RelativeAxesBased: DeviceInfo {
    /// Returns `true` if the device is a pointing stick (e.g. a TrackPoint).
    ///
    /// A device is considered a pointing stick if it supports both relative
    /// position axes (`REL_X` and `REL_Y`) and has the
    /// `INPUT_PROP_POINTING_STICK` property.
    fn is_pointing_stick(&self) -> bool {
        self.rel_xy() && self.has_property(PropType::POINTING_STICK)
    }

    /// Returns `true` if the device is a mouse.
    ///
    /// A device must first support `BTN_LEFT`; without it, this always
    /// returns `false`. It is then considered a mouse if either:
    ///
    /// - **Relative:** it supports `REL_X` and `REL_Y` and does *not* have the
    ///   `INPUT_PROP_POINTING_STICK` property (so pointing sticks are not
    ///   misclassified as mice), or
    /// - **Absolute:** it supports `ABS_X` and `ABS_Y`, has a wheel.
    ///   This covers absolute
    ///   pointers such as those exposed by virtual machines.
    fn is_mouse(&self) -> bool {
        if !self.has_key(KeyCode::BTN_LEFT) {
            return false;
        }

        let relative = self.rel_xy() && !self.has_property(PropType::POINTING_STICK);
        let absolute = self.abs_xy() && self.has_wheel();

        relative || absolute
    }
}

impl<T: DeviceInfo + ?Sized> RelativeAxesBased for T {}
