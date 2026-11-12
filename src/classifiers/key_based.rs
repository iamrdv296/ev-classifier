use crate::DeviceInfo;
use evdev::KeyCode;

/// The 26 letter keys of the Latin alphabet (`KEY_A` through `KEY_Z`).
///
/// A device that reports every one of these keys is a strong indicator of a
/// full alphanumeric keyboard, as opposed to a numpad, remote, or other
/// device that merely exposes a handful of keys. Used by
/// [`KeyBased::is_keyboard`].
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

/// Keys that control audio output volume.
///
/// Contains volume up, volume down, and mute. Having any one of these marks a
/// device as a candidate for media keys. Used by [`KeyBased::is_media_keys`].
const VOLUME_KEYS: [KeyCode; 3] = [
    KeyCode::KEY_VOLUMEUP,
    KeyCode::KEY_VOLUMEDOWN,
    KeyCode::KEY_MUTE,
];

/// Keys that control media playback.
///
/// Contains play/pause, play (CD), and next track. Having any one of these
/// marks a device as a candidate for media keys. Used by
/// [`KeyBased::is_media_keys`].
const MEDIA_CONTROL_KEYS: [KeyCode; 3] = [
    KeyCode::KEY_PLAYPAUSE,
    KeyCode::KEY_PLAYCD,
    KeyCode::KEY_NEXTSONG,
];

/// Classifies devices by the set of keys they report.
///
/// This trait is blanket-implemented for every type that implements
/// [`DeviceInfo`] (including unsized types such as `dyn DeviceInfo`), so you
/// never need to implement it manually. Import the trait to use the methods:
///
/// ```ignore
/// use crate::KeyBased;
///
/// if device.is_keyboard() {
///     println!("Found a keyboard");
/// }
/// ```
///
/// Note that classifications are not mutually exclusive: a single device may
/// satisfy several of them at once (for example, a multimedia keyboard is both
/// a keyboard and media keys).
pub trait KeyBased: DeviceInfo {
    /// Returns `true` if the device is a keyboard.
    ///
    /// A device is considered a keyboard if it supports all 26 letter keys
    /// (`KEY_A` through `KEY_Z`), as well as the `KEY_ENTER` and `KEY_SPACE`
    /// keys.
    fn is_keyboard(&self) -> bool {
        self.has_all_keys(&LETTER_KEYS)
            && self.has_key(KeyCode::KEY_ENTER)
            && self.has_key(KeyCode::KEY_SPACE)
    }

    /// Returns `true` if the device is a power button.
    ///
    /// A device is considered a power button if it supports `KEY_POWER`.
    fn is_power_button(&self) -> bool {
        self.has_key(KeyCode::KEY_POWER)
    }

    /// Returns `true` if the device is a sleep button.
    ///
    /// A device is considered a sleep button if it supports `KEY_SLEEP`.
    fn is_sleep_button(&self) -> bool {
        self.has_key(KeyCode::KEY_SLEEP)
    }

    /// Returns `true` if the device provides media keys.
    ///
    /// A device is considered to have media keys if it supports at least one
    /// volume key (`KEY_VOLUMEUP`, `KEY_VOLUMEDOWN`, `KEY_MUTE`) or at least
    /// one media control key (`KEY_PLAYPAUSE`, `KEY_PLAYCD`, `KEY_NEXTSONG`).
    fn is_media_keys(&self) -> bool {
        self.has_any_key(&VOLUME_KEYS) || self.has_any_key(&MEDIA_CONTROL_KEYS)
    }
}

impl<T: DeviceInfo + ?Sized> KeyBased for T {}