use crate::DeviceInfo;
use evdev::{KeyCode, PropType};

/// Tool buttons reported by pen-style input tools.
///
/// `BTN_TOOL_PEN` signals a stylus tip is in proximity, and `BTN_TOOL_RUBBER`
/// signals the eraser end. Having either marks a device as a candidate for a
/// pen tablet. Used by [`AbsoluteAxesBased::is_pen_tablet`].
const PEN_TOOLS: [KeyCode; 2] = [KeyCode::BTN_TOOL_PEN, KeyCode::BTN_TOOL_RUBBER];

/// Tool buttons reported by devices that track fingers.
///
/// Covers a single finger, two fingers (`DOUBLETAP`), and three fingers
/// (`TRIPLETAP`). Touchpads use these to report how many fingers are
/// currently touching the surface. Used by [`AbsoluteAxesBased::is_touchpad`].
const FINGER_TOOLS: [KeyCode; 3] = [
    KeyCode::BTN_TOOL_FINGER,
    KeyCode::BTN_TOOL_DOUBLETAP,
    KeyCode::BTN_TOOL_TRIPLETAP,
];

/// Classifies devices by their absolute axes, input properties, and tool
/// buttons.
///
/// This covers devices that report absolute positions: pen tablets,
/// touchscreens, and touchpads. Implementors only need to implement
/// [`DeviceInfo`]; all methods here are provided as defaults.
///
/// Classifications are not mutually exclusive. For example, a device may
/// satisfy several checks if it exposes the corresponding capabilities.
pub trait AbsoluteAxesBased: DeviceInfo {
    /// Returns `true` if the device reports absolute position coordinates.
    ///
    /// This is the case if it supports either the single-touch axes
    /// (`ABS_X` and `ABS_Y`) or the multi-touch position axes
    /// (`ABS_MT_POSITION_X` and `ABS_MT_POSITION_Y`).
    fn has_abs_position(&self) -> bool {
        self.abs_xy() || self.abs_mt_xy()
    }

    /// Returns `true` if the device is a pen tablet (graphics tablet).
    ///
    /// A device is considered a pen tablet if it reports absolute position
    /// coordinates and supports at least one pen tool button
    /// (`BTN_TOOL_PEN` or `BTN_TOOL_RUBBER`).
    fn is_pen_tablet(&self) -> bool {
        self.has_abs_position() && self.has_any_key(&PEN_TOOLS)
    }

    /// Returns `true` if the device is a touchscreen.
    ///
    /// A device is considered a touchscreen if it has the `INPUT_PROP_DIRECT`
    /// property (input is applied directly on the display) and either:
    ///
    /// - supports the multi-touch position axes, or
    /// - supports the single-touch axes (`ABS_X` and `ABS_Y`) together with
    ///   `BTN_TOUCH`.
    fn is_touchscreen(&self) -> bool {
        self.has_property(PropType::DIRECT)
            && (self.abs_mt_xy() || (self.abs_xy() && self.has_key(KeyCode::BTN_TOUCH)))
    }

    /// Returns `true` if the device is a touchpad.
    ///
    /// A device is considered a touchpad if it reports absolute position
    /// coordinates, has the `INPUT_PROP_POINTER` property (it moves a cursor
    /// rather than acting on the screen directly), and supports at least one
    /// finger tool button (`BTN_TOOL_FINGER`, `BTN_TOOL_DOUBLETAP`, or
    /// `BTN_TOOL_TRIPLETAP`).
    fn is_touchpad(&self) -> bool {
        self.has_abs_position()
            && self.has_property(PropType::POINTER)
            && self.has_any_key(&FINGER_TOOLS)
    }
}

impl<T: DeviceInfo + ?Sized> AbsoluteAxesBased for T {}