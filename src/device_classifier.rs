use evdev::{KeyCode, PropType};

use crate::device_info::DeviceInfo;

pub trait Classifier: DeviceInfo {
    fn is_keyboard(&self) -> bool {
        self.has_key(KeyCode::KEY_A)
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
        self.has_key(KeyCode::KEY_VOLUMEUP)
            && self.has_key(KeyCode::KEY_PLAYPAUSE)
            && !self.is_keyboard()
    }

    fn is_mouse(&self) -> bool {
        self.rel_xy() && self.has_key(KeyCode::BTN_LEFT)
    }

    fn is_pointing_stick(&self) -> bool {
        self.rel_xy() && self.has_property(PropType::POINTING_STICK)
    }

    fn is_pen_tablet(&self) -> bool {
        self.abs_xy() && self.has_key(KeyCode::BTN_STYLUS)
    }

    fn is_touchpad(&self) -> bool {
        self.abs_xy()
            && self.has_key(KeyCode::BTN_TOOL_FINGER)
            && !self.has_property(PropType::DIRECT)
            && !self.is_pen_tablet()
    }

    fn is_gamepad(&self) -> bool {
        self.abs_xy() && self.has_key(KeyCode::BTN_SOUTH)
    }

    fn is_joystick(&self) -> bool {
        self.abs_xy() && self.has_key(KeyCode::BTN_TRIGGER) && !self.is_gamepad()
    }

    fn is_accelerometer(&self) -> bool {
        self.has_property(PropType::ACCELEROMETER)
    }
}

impl<T: DeviceInfo + ?Sized> Classifier for T {}
