use bitflags::bitflags;

use crate::{
    DeviceInfo,
    classifiers::{
        absolute_axes_based::AbsoluteAxesBased, key_based::KeyBased, miscellaneous::Miscellaneous,
        relative_axes_based::RelativeAxesBased,
    },
};

bitflags! {
    /// The kinds a device can be classified as, stored as a bitflag set.
    ///
    /// This isn't a simple enum because a single device can be several kinds
    /// at once. For example, a multimedia keyboard is both
    /// [`KEYBOARD`](Self::KEYBOARD) and [`MEDIA_KEYS`](Self::MEDIA_KEYS).
    /// An empty set means the device matched no known kind.
    ///
    /// The set of kinds may grow in future versions, so this type is
    /// `#[non_exhaustive]`. Use [`classify_device`] to produce a value, and
    /// [`first`] to pick the single most prominent kind from a collection.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub struct DeviceKind: usize {
        /// A full keyboard. See [`KeyBased::is_keyboard`].
        const KEYBOARD = 1 << 0;
        /// A power button. See [`KeyBased::is_power_button`].
        const POWER_BUTTON = 1 << 1;
        /// A sleep button. See [`KeyBased::is_sleep_button`].
        const SLEEP_BUTTON = 1 << 2;
        /// Volume or media playback keys. See [`KeyBased::is_media_keys`].
        const MEDIA_KEYS = 1 << 3;
        /// A graphics (pen) tablet. See [`AbsoluteAxesBased::is_pen_tablet`].
        const PEN_TABLET = 1 << 4;
        /// A touchscreen. See [`AbsoluteAxesBased::is_touchscreen`].
        const TOUCH_SCREEN = 1 << 5;
        /// A touchpad. See [`AbsoluteAxesBased::is_touchpad`].
        const TOUCH_PAD = 1 << 6;
        /// A pointing stick (e.g. a TrackPoint).
        /// See [`RelativeAxesBased::is_pointing_stick`].
        const POINTING_STICK = 1 << 7;
        /// A mouse. See [`RelativeAxesBased::is_mouse`].
        const MOUSE = 1 << 8;
        /// A gamepad. See [`Miscellaneous::is_gamepad`].
        const GAMEPAD = 1 << 9;
        /// A joystick, such as a flight stick. See [`Miscellaneous::is_joystick`].
        const JOYSTICK = 1 << 10;
        /// A motion sensor. See [`Miscellaneous::is_accelerometer`].
        const ACCELEROMETER = 1 << 11;
    }
}

/// Kinds ordered from most to least prominent.
///
/// When a device (or group of devices) has several kinds, [`first`] walks this
/// list in order and returns the first kind that is present. Earlier entries
/// win, so pointing and touch devices outrank keyboards, which in turn outrank
/// controllers, media keys, buttons, and sensors.
///
/// The array length must match the number of [`DeviceKind`] flags, so adding a
/// new flag requires adding it here as well.
pub const PROMINENCE: [DeviceKind; 12] = [
    DeviceKind::PEN_TABLET,
    DeviceKind::TOUCH_SCREEN,
    DeviceKind::TOUCH_PAD,
    DeviceKind::MOUSE,
    DeviceKind::KEYBOARD,
    DeviceKind::POINTING_STICK,
    DeviceKind::GAMEPAD,
    DeviceKind::JOYSTICK,
    DeviceKind::MEDIA_KEYS,
    DeviceKind::POWER_BUTTON,
    DeviceKind::SLEEP_BUTTON,
    DeviceKind::ACCELEROMETER,
];

/// Classifies a single device, returning every [`DeviceKind`] it matches.
///
/// Each kind is determined independently by the corresponding classifier
/// method, so the result may contain several flags, or none at all if the
/// device doesn't match any known kind.
///
/// Works with any [`DeviceInfo`] implementor, including trait objects.
///
/// ```ignore
/// let kind = classify_device(&device);
/// if kind.contains(DeviceKind::KEYBOARD | DeviceKind::MEDIA_KEYS) {
///     println!("Multimedia keyboard");
/// }
/// ```
pub fn classify_device<D: DeviceInfo + ?Sized>(device: &D) -> DeviceKind {
    let mut kind = DeviceKind::empty();
    kind.set(DeviceKind::KEYBOARD, device.is_keyboard());
    kind.set(DeviceKind::POWER_BUTTON, device.is_power_button());
    kind.set(DeviceKind::SLEEP_BUTTON, device.is_sleep_button());
    kind.set(DeviceKind::MEDIA_KEYS, device.is_media_keys());
    kind.set(DeviceKind::PEN_TABLET, device.is_pen_tablet());
    kind.set(DeviceKind::TOUCH_SCREEN, device.is_touchscreen());
    kind.set(DeviceKind::TOUCH_PAD, device.is_touchpad());
    kind.set(DeviceKind::POINTING_STICK, device.is_pointing_stick());
    kind.set(DeviceKind::MOUSE, device.is_mouse());
    kind.set(DeviceKind::GAMEPAD, device.is_gamepad());
    kind.set(DeviceKind::JOYSTICK, device.is_joystick());
    kind.set(DeviceKind::ACCELEROMETER, device.is_accelerometer());
    kind
}

/// Classifies several devices at once.
///
/// Returns each input device paired with the result of [`classify_device`]
/// for it, in the same order as the input.
pub fn classify_all_devices<D: DeviceInfo + ?Sized>(devices: Vec<&D>) -> Vec<(&D, DeviceKind)> {
    devices
        .iter()
        .map(|&device| (device, classify_device(device)))
        .collect()
}

/// Returns the single most prominent kind found in `kinds`.
///
/// All the given kinds are merged together, then the result is the first kind
/// in [`PROMINENCE`] order that appears in the merged set. This is useful for
/// choosing one label for something that has several kinds, whether that is
/// one multi-function device or a group of related devices.
///
/// Returns [`DeviceKind::empty`] if `kinds` is empty or contains no known
/// kind.
///
/// ```ignore
/// let kinds = [DeviceKind::KEYBOARD, DeviceKind::MOUSE];
/// assert_eq!(first(&kinds), DeviceKind::MOUSE); // mouse outranks keyboard
/// ```
pub fn first(kinds: &[DeviceKind]) -> DeviceKind {
    let all = kinds
        .iter()
        .fold(DeviceKind::empty(), |acc, &kind| acc | kind);

    PROMINENCE
        .into_iter()
        .find(|&kind| all.contains(kind))
        .unwrap_or_else(DeviceKind::empty)
}
