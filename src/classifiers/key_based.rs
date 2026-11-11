use crate::DeviceInfo;
use evdev::KeyCode;

const LETTER_KEYS: [KeyCode; 26] = [
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

const VOLUME_KEYS: [KeyCode; 3] = [
    KeyCode::KEY_VOLUMEUP,
    KeyCode::KEY_VOLUMEDOWN,
    KeyCode::KEY_MUTE,
];

const MEDIA_CONTROL_KEYS: [KeyCode; 3] = [
    KeyCode::KEY_PLAYPAUSE,
    KeyCode::KEY_PLAYCD,
    KeyCode::KEY_NEXTSONG,
];

pub trait KeyBased: DeviceInfo {
    fn is_keyboard(&self) -> bool {
        self.has_all_keys(&LETTER_KEYS)
            && self.has_key(KeyCode::KEY_ENTER)
            && self.has_key(KeyCode::KEY_SPACE)
    }

    fn is_power_button(&self) -> bool {
        self.has_key(KeyCode::KEY_POWER)
    }

    fn is_sleep_button(&self) -> bool {
        self.has_key(KeyCode::KEY_SLEEP)
    }

    fn is_media_keys(&self) -> bool {
        self.has_any_key(&VOLUME_KEYS) || self.has_any_key(&MEDIA_CONTROL_KEYS)
    }
}

impl<T: DeviceInfo + ?Sized> KeyBased for T {}

// TODO: DOCUMENTATION