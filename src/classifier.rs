use bitflags::bitflags;

use crate::{
    DeviceInfo,
    classifiers::{
        absolute_axes_based::AbsoluteAxesBased, key_based::KeyBased, miscellaneous::Miscellaneous,
        relative_axes_based::RelativeAxesBased,
    },
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]

    /// Device Kind is a bitflag that represents which device kind a device is.
    /// It isn't a simple enum because, e.g. some devices can be both media keys and keyboard
    pub struct DeviceKind: usize {
        const KEYBOARD = 1 << 0;
        const POWER_BUTTON = 1 << 1;
        const SLEEP_BUTTON = 1 << 2;
        const MEDIA_KEYS = 1 << 3;
        const PEN_TABLET = 1 << 4;
        const TOUCH_SCREEN = 1 << 5;
        const TOUCH_PAD = 1 << 6;
        const POINTING_STICK = 1 << 7;
        const MOUSE = 1 << 8;
        const GAMEPAD = 1 << 9;
        const JOYSTICK = 1 << 10;
        const ACCELEROMETER = 1 << 11;
    }
}

const PROMINENCE: [DeviceKind; 12] = [
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

pub fn classify_all_devices<D: DeviceInfo + ?Sized>(devices: Vec<&D>) -> Vec<(&D, DeviceKind)> {
    devices
        .iter()
        .map(|&device| (device, classify_device(device)))
        .collect()
}

pub fn first(kinds: &[DeviceKind]) -> DeviceKind {
    let all = kinds
        .iter()
        .fold(DeviceKind::empty(), |acc, &kind| acc | kind);

    PROMINENCE
        .into_iter()
        .find(|&kind| all.contains(kind))
        .unwrap_or_else(DeviceKind::empty)
}
