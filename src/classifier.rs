use evdev::{AbsoluteAxisCode, KeyCode, PropType};

use crate::device_info::DeviceInfo;

const LETTERS: [KeyCode; 26] = [
    KeyCode::KEY_A,
    KeyCode::KEY_B,
    KeyCode::KEY_C,
    KeyCode::KEY_D,
    KeyCode::KEY_E,
    KeyCode::KEY_F,
    KeyCode::KEY_G,
    KeyCode::KEY_H,
    KeyCode::KEY_I,
    KeyCode::KEY_J,
    KeyCode::KEY_K,
    KeyCode::KEY_L,
    KeyCode::KEY_M,
    KeyCode::KEY_N,
    KeyCode::KEY_O,
    KeyCode::KEY_P,
    KeyCode::KEY_Q,
    KeyCode::KEY_R,
    KeyCode::KEY_S,
    KeyCode::KEY_T,
    KeyCode::KEY_U,
    KeyCode::KEY_V,
    KeyCode::KEY_W,
    KeyCode::KEY_X,
    KeyCode::KEY_Y,
    KeyCode::KEY_Z,
];

const PEN_KEYS: [KeyCode; 2] = [KeyCode::BTN_STYLUS, KeyCode::BTN_TOOL_PEN];

const FINGER_TOOLS: [KeyCode; 3] = [
    KeyCode::BTN_TOOL_FINGER,
    KeyCode::BTN_TOOL_DOUBLETAP,
    KeyCode::BTN_TOOL_TRIPLETAP,
];

const GAMEPAD_BUTTONS: [KeyCode; 4] = [
    KeyCode::BTN_SOUTH,
    KeyCode::BTN_EAST,
    KeyCode::BTN_NORTH,
    KeyCode::BTN_WEST,
];

const DPAD_KEYS: [KeyCode; 4] = [
    KeyCode::BTN_DPAD_UP,
    KeyCode::BTN_DPAD_DOWN,
    KeyCode::BTN_DPAD_LEFT,
    KeyCode::BTN_DPAD_RIGHT,
];

pub trait Classifier: DeviceInfo {
    fn has_abs_position(&self) -> bool {
        self.abs_xy() || self.abs_mt_xy()
    }

    fn has_pen_tool(&self) -> bool {
        self.has_any_key(&PEN_KEYS)
    }

    fn has_finger_tool(&self) -> bool {
        self.has_any_key(&FINGER_TOOLS)
    }

    fn has_directional_input(&self) -> bool {
        self.abs_xy()
            || self.has_absolute(AbsoluteAxisCode::ABS_HAT0X)
            || self.has_all_keys(&DPAD_KEYS)
    }

    // ----- keys ------------------------------------------------------------
    fn is_keyboard(&self) -> bool {
        self.has_all_keys(&LETTERS)
            && self.has_key(KeyCode::KEY_ENTER)
            && self.has_key(KeyCode::KEY_SPACE)
    }

    fn is_power_button(&self) -> bool {
        self.has_key(KeyCode::KEY_POWER) && !self.is_keyboard()
    }

    fn is_sleep_button(&self) -> bool {
        self.has_key(KeyCode::KEY_SLEEP) && !self.is_keyboard()
    }

    fn is_media_keys(&self) -> bool {
        self.has_any_key(&[
            KeyCode::KEY_VOLUMEUP,
            KeyCode::KEY_VOLUMEDOWN,
            KeyCode::KEY_MUTE,
        ]) || self.has_any_key(&[
            KeyCode::KEY_PLAYPAUSE,
            KeyCode::KEY_PLAYCD,
            KeyCode::KEY_NEXTSONG,
        ]) && !self.is_keyboard()
    }

    // ----- direct / absolute pointing --------------------------------------
    fn is_pen_tablet(&self) -> bool {
        self.has_abs_position() && self.has_pen_tool()
    }

    fn is_touchscreen(&self) -> bool {
        self.has_abs_position()
            && !self.has_pen_tool()
            && (self.has_property(PropType::DIRECT)
                || (self.has_key(KeyCode::BTN_TOUCH) && !self.has_finger_tool()))
    }

    fn is_touchpad(&self) -> bool {
        self.has_abs_position()
            && self.has_finger_tool()
            && !self.has_pen_tool()
            && !self.has_property(PropType::DIRECT)
    }

    // ----- relative / absolute mice ----------------------------------------
    fn is_pointing_stick(&self) -> bool {
        self.rel_xy() && self.has_property(PropType::POINTING_STICK)
    }

    fn is_mouse(&self) -> bool {
        if self.is_pointing_stick() {
            return false;
        }
        let relative = self.rel_xy() && self.has_key(KeyCode::BTN_LEFT);

        let absolute = self.abs_xy()
            && self.has_key(KeyCode::BTN_LEFT)
            && !self.is_pen_tablet()
            && !self.is_touchpad()
            && !self.is_touchscreen();
        relative || absolute
    }

    fn is_gamepad(&self) -> bool {
        self.has_any_key(&GAMEPAD_BUTTONS) && self.has_directional_input()
    }

    fn is_joystick(&self) -> bool {
        self.has_key(KeyCode::BTN_TRIGGER)
            && (self.has_directional_input()
                || self.has_absolute(AbsoluteAxisCode::ABS_RX)
                || self.has_absolute(AbsoluteAxisCode::ABS_THROTTLE))
            && !self.is_gamepad()
    }

    // ----- sensors ---------------------------------------------------------
    fn is_accelerometer(&self) -> bool {
        self.has_property(PropType::ACCELEROMETER)
    }
}

impl<T: DeviceInfo + ?Sized> Classifier for T {}
