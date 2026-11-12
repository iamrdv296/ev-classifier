use crate::{DeviceInfo, classifiers::absolute_axes_based::AbsoluteAxesBased};
use evdev::{AbsoluteAxisCode, KeyCode, PropType};

/// Classifies devices that don't fit the other classifier traits.
///
/// This covers game controllers (gamepads and joysticks) and motion sensors
/// (accelerometers). This trait is blanket-implemented for every type that
/// implements [`DeviceInfo`] (including unsized types such as
/// `dyn DeviceInfo`), so you never need to implement it manually.
///
/// Classifications are not mutually exclusive: a controller with a built-in
/// motion sensor may satisfy both [`is_gamepad`](Self::is_gamepad) and
/// [`is_accelerometer`](Self::is_accelerometer), and some devices may satisfy
/// both gamepad and joystick checks.
pub trait Miscellaneous: DeviceInfo {
    /// Returns `true` if the device is a gamepad.
    ///
    /// A device is considered a gamepad if it supports `BTN_SOUTH`,
    /// provides directional input (see `has_directional_input`) and is not a more
    /// specific absolute device (see [`AbsoluteAxesBased::is_specific_absolute`]).
    fn is_gamepad(&self) -> bool {
        self.has_key(KeyCode::BTN_SOUTH)
            && self.has_directional_input()
            && !self.is_specific_absolute()
    }

    /// Returns `true` if the device is a joystick (e.g. a flight stick).
    ///
    /// A device must first support `BTN_TRIGGER`; without it, this always
    /// returns `false`. It is then considered a joystick if it also has at
    /// least one of:
    ///
    /// - directional input (see `has_directional_input`),
    /// - the `ABS_RX` axis (rotation, such as a twist grip), or
    /// - the `ABS_THROTTLE` axis.
    fn is_joystick(&self) -> bool {
        self.has_key(KeyCode::BTN_TRIGGER)
            && (self.has_directional_input()
                || self.has_absolute(AbsoluteAxisCode::ABS_RX)
                || self.has_absolute(AbsoluteAxisCode::ABS_THROTTLE))
    }

    /// Returns `true` if the device is an accelerometer.
    ///
    /// A device is considered an accelerometer if it has the
    /// `INPUT_PROP_ACCELEROMETER` property.
    fn is_accelerometer(&self) -> bool {
        self.has_property(PropType::ACCELEROMETER)
    }
}

impl<T: DeviceInfo + ?Sized> Miscellaneous for T {}