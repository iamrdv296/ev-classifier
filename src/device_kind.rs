use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
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
        const ACCELEROMETER = 1 << 1;
    }
}
