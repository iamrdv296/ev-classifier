use bitflags::bitflags;

use crate::{DeviceInfo, classifiers::{absolute_axes_based::AbsoluteAxesBased, key_based::KeyBased, miscellaneous::Miscellaneous, relative_axes_based::RelativeAxesBased}};

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

pub fn classify_device<D: DeviceInfo + ?Sized>(device: &D) -> DeviceKind {
    let mut flags = DeviceKind::empty();
    
    if device.is_keyboard() {
        flags.insert(DeviceKind::KEYBOARD);
    } if device.is_power_button() {
        flags.insert(DeviceKind::POWER_BUTTON);
    } if device.is_sleep_button() {
        flags.insert(DeviceKind::SLEEP_BUTTON);
    } if device.is_media_keys() {
        flags.insert(DeviceKind::MEDIA_KEYS);
    } if device.is_pen_tablet() {
        flags.insert(DeviceKind::PEN_TABLET);
    } if device.is_touchscreen() {
        flags.insert(DeviceKind::TOUCH_SCREEN);
    } if device.is_touchpad() {
        flags.insert(DeviceKind::TOUCH_PAD);
    } if device.is_pointing_stick() {
        flags.insert(DeviceKind::POINTING_STICK);
    } if device.is_mouse() {
        flags.insert(DeviceKind::MOUSE);
    } if device.is_gamepad() {
        flags.insert(DeviceKind::GAMEPAD);
    } if device.is_joystick() {
        flags.insert(DeviceKind::JOYSTICK);
    } if device.is_accelerometer() {
        flags.insert(DeviceKind::ACCELEROMETER);
    } 

    flags
}

pub fn classify_all_devices<D: DeviceInfo + ?Sized>(devices: Vec<&D>) -> Vec<(&D, DeviceKind)> {
    devices
        .iter()
        .map(|&device| (device, classify_device(device)))
        .collect()
}