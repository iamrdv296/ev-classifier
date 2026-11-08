pub mod classifier;
pub mod device_info;
pub mod discovery;

pub use classifier::Classifier;
pub use device_info::DeviceInfo;
pub use discovery::DeviceDiscovery;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeviceKind {
    Keyboard,
    PowerButton,
    SleepButton,
    MediaKeys,
    PenTablet,
    TouchScreen,
    TouchPad,
    PointingStick,
    Mouse,
    Gamepad,
    Joystick,
    Accelerometer,
    Unknown,
}

pub fn classify_device<D: DeviceInfo + ?Sized>(device: &D) -> DeviceKind {
    if device.is_keyboard() {
        DeviceKind::Keyboard
    } else if device.is_power_button() {
        DeviceKind::PowerButton
    } else if device.is_sleep_button() {
        DeviceKind::SleepButton
    } else if device.is_media_keys() {
        DeviceKind::MediaKeys
    } else if device.is_pen_tablet() {
        DeviceKind::PenTablet
    } else if device.is_touchscreen() {
        DeviceKind::TouchScreen
    } else if device.is_touchpad() {
        DeviceKind::TouchPad
    } else if device.is_pointing_stick() {
        DeviceKind::PointingStick
    } else if device.is_mouse() {
        DeviceKind::Mouse
    } else if device.is_gamepad() {
        DeviceKind::Gamepad
    } else if device.is_joystick() {
        DeviceKind::Joystick
    } else if device.is_accelerometer() {
        DeviceKind::Accelerometer
    } else {
        DeviceKind::Unknown
    }
}

pub fn classify_all_devices<D: DeviceInfo + ?Sized>(devices: Vec<&D>) -> Vec<(&D, DeviceKind)> {
    devices
        .iter()
        .map(|&device| (device, classify_device(device)))
        .collect()
}
