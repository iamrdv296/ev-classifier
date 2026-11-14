use evdev::{
    AbsoluteAxisCode as Abs, AttributeSet, KeyCode as Key, PropType, RelativeAxisCode as Rel,
};

use evdev_classifier::{DeviceInfo, DeviceKind, classify_all_devices, classify_device, first};

// ----- mock device -----------------------------------------------------------

struct Mock {
    keys: Option<AttributeSet<Key>>,
    rel: Option<AttributeSet<Rel>>,
    abs: Option<AttributeSet<Abs>>,
    props: AttributeSet<PropType>,
}

impl Mock {
    fn new() -> Self {
        Self {
            keys: None,
            rel: None,
            abs: None,
            props: AttributeSet::new(),
        }
    }

    fn keys(mut self, keys: &[Key]) -> Self {
        let set = self.keys.get_or_insert_with(AttributeSet::new);
        for &key in keys {
            set.insert(key);
        }
        self
    }

    fn rel(mut self, axes: &[Rel]) -> Self {
        let set = self.rel.get_or_insert_with(AttributeSet::new);
        for &axis in axes {
            set.insert(axis);
        }
        self
    }

    fn abs(mut self, axes: &[Abs]) -> Self {
        let set = self.abs.get_or_insert_with(AttributeSet::new);
        for &axis in axes {
            set.insert(axis);
        }
        self
    }

    fn props(mut self, properties: &[PropType]) -> Self {
        for &property in properties {
            self.props.insert(property);
        }
        self
    }
}

impl DeviceInfo for Mock {
    fn name(&self) -> Option<&str> {
        Some("mock")
    }

    fn supported_keys(&self) -> Option<&evdev::AttributeSetRef<Key>> {
        self.keys.as_deref()
    }

    fn supported_relative_axes(&self) -> Option<&evdev::AttributeSetRef<Rel>> {
        self.rel.as_deref()
    }

    fn supported_absolute_axes(&self) -> Option<&evdev::AttributeSetRef<Abs>> {
        self.abs.as_deref()
    }

    fn properties(&self) -> &evdev::AttributeSetRef<PropType> {
        &self.props
    }
}

fn letters() -> Vec<Key> {
    vec![
        Key::KEY_A,
        Key::KEY_B,
        Key::KEY_C,
        Key::KEY_D,
        Key::KEY_E,
        Key::KEY_F,
        Key::KEY_G,
        Key::KEY_H,
        Key::KEY_I,
        Key::KEY_J,
        Key::KEY_K,
        Key::KEY_L,
        Key::KEY_M,
        Key::KEY_N,
        Key::KEY_O,
        Key::KEY_P,
        Key::KEY_Q,
        Key::KEY_R,
        Key::KEY_S,
        Key::KEY_T,
        Key::KEY_U,
        Key::KEY_V,
        Key::KEY_W,
        Key::KEY_X,
        Key::KEY_Y,
        Key::KEY_Z,
    ]
}

fn keyboard() -> Mock {
    Mock::new()
        .keys(&letters())
        .keys(&[Key::KEY_ENTER, Key::KEY_SPACE])
}

// ----- key classifications ---------------------------------------------------

#[test]
fn keyboard_sets_keyboard_flag() {
    assert_eq!(classify_device(&keyboard()), DeviceKind::KEYBOARD);
}

#[test]
fn keyboard_missing_a_letter_is_not_keyboard() {
    let mut keys = letters();
    keys.pop();
    let device = Mock::new()
        .keys(&keys)
        .keys(&[Key::KEY_ENTER, Key::KEY_SPACE]);

    assert!(!classify_device(&device).contains(DeviceKind::KEYBOARD));
}

#[test]
fn multimedia_keyboard_keeps_all_matching_flags() {
    let device = keyboard().keys(&[
        Key::KEY_POWER,
        Key::KEY_SLEEP,
        Key::KEY_VOLUMEUP,
        Key::KEY_PLAYPAUSE,
    ]);

    assert_eq!(
        classify_device(&device),
        DeviceKind::KEYBOARD
            | DeviceKind::POWER_BUTTON
            | DeviceKind::SLEEP_BUTTON
            | DeviceKind::MEDIA_KEYS
    );
}

#[test]
fn power_button_sets_power_button_flag() {
    assert_eq!(
        classify_device(&Mock::new().keys(&[Key::KEY_POWER])),
        DeviceKind::POWER_BUTTON
    );
}

#[test]
fn sleep_button_sets_sleep_button_flag() {
    assert_eq!(
        classify_device(&Mock::new().keys(&[Key::KEY_SLEEP])),
        DeviceKind::SLEEP_BUTTON
    );
}

#[test]
fn media_keys_with_volume_and_transport() {
    let device = Mock::new().keys(&[Key::KEY_VOLUMEUP, Key::KEY_VOLUMEDOWN, Key::KEY_PLAYPAUSE]);
    assert_eq!(classify_device(&device), DeviceKind::MEDIA_KEYS);
}

#[test]
fn media_keys_volume_only() {
    let device = Mock::new().keys(&[Key::KEY_VOLUMEUP, Key::KEY_VOLUMEDOWN]);
    assert_eq!(classify_device(&device), DeviceKind::MEDIA_KEYS);
}

// ----- pen / touch -----------------------------------------------------------

#[test]
fn external_pen_tablet() {
    let device = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_PRESSURE])
        .keys(&[Key::BTN_TOOL_PEN, Key::BTN_STYLUS, Key::BTN_TOUCH]);

    assert_eq!(classify_device(&device), DeviceKind::PEN_TABLET);
}

#[test]
fn pen_on_direct_screen_is_pen_tablet_not_touchscreen() {
    let device = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_PRESSURE])
        .keys(&[Key::BTN_TOOL_PEN, Key::BTN_STYLUS, Key::BTN_TOUCH])
        .props(&[PropType::DIRECT]);

    assert_eq!(classify_device(&device), DeviceKind::PEN_TABLET);
}

#[test]
fn multitouch_touchscreen() {
    let device = Mock::new()
        .abs(&[
            Abs::ABS_MT_POSITION_X,
            Abs::ABS_MT_POSITION_Y,
            Abs::ABS_MT_SLOT,
        ])
        .keys(&[Key::BTN_TOUCH])
        .props(&[PropType::DIRECT]);

    assert_eq!(classify_device(&device), DeviceKind::TOUCH_SCREEN);
}

#[test]
fn single_touch_touchscreen() {
    let device = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y])
        .keys(&[Key::BTN_TOUCH])
        .props(&[PropType::DIRECT]);

    assert_eq!(classify_device(&device), DeviceKind::TOUCH_SCREEN);
}

#[test]
fn touchpad_sets_touchpad_flag() {
    let device = Mock::new()
        .abs(&[
            Abs::ABS_X,
            Abs::ABS_Y,
            Abs::ABS_MT_POSITION_X,
            Abs::ABS_MT_POSITION_Y,
        ])
        .keys(&[
            Key::BTN_LEFT,
            Key::BTN_TOUCH,
            Key::BTN_TOOL_FINGER,
            Key::BTN_TOOL_DOUBLETAP,
        ])
        .props(&[PropType::POINTER]);

    assert_eq!(classify_device(&device), DeviceKind::TOUCH_PAD);
}

// ----- mice / pointing sticks ------------------------------------------------

#[test]
fn relative_mouse() {
    let device = Mock::new()
        .rel(&[Rel::REL_X, Rel::REL_Y, Rel::REL_WHEEL])
        .keys(&[Key::BTN_LEFT, Key::BTN_RIGHT]);

    assert_eq!(classify_device(&device), DeviceKind::MOUSE);
}

#[test]
fn absolute_mouse_requires_a_wheel() {
    let without_wheel = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y])
        .keys(&[Key::BTN_LEFT]);
    assert!(!classify_device(&without_wheel).contains(DeviceKind::MOUSE));

    let with_wheel = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y])
        .rel(&[Rel::REL_WHEEL])
        .keys(&[Key::BTN_LEFT]);
    assert_eq!(classify_device(&with_wheel), DeviceKind::MOUSE);
}

#[test]
fn pointing_stick_is_not_a_mouse() {
    let device = Mock::new()
        .rel(&[Rel::REL_X, Rel::REL_Y])
        .keys(&[Key::BTN_LEFT])
        .props(&[PropType::POINTING_STICK]);

    assert_eq!(classify_device(&device), DeviceKind::POINTING_STICK);
}

// ----- game controllers ------------------------------------------------------

#[test]
fn gamepad_with_stick() {
    let device = Mock::new()
        .keys(&[Key::BTN_SOUTH, Key::BTN_EAST, Key::BTN_NORTH, Key::BTN_WEST])
        .abs(&[Abs::ABS_X, Abs::ABS_Y]);

    assert_eq!(classify_device(&device), DeviceKind::GAMEPAD);
}

#[test]
fn gamepad_with_dpad_only() {
    let device = Mock::new().keys(&[
        Key::BTN_SOUTH,
        Key::BTN_DPAD_UP,
        Key::BTN_DPAD_DOWN,
        Key::BTN_DPAD_LEFT,
        Key::BTN_DPAD_RIGHT,
    ]);

    assert_eq!(classify_device(&device), DeviceKind::GAMEPAD);
}

#[test]
fn joystick() {
    let device =
        Mock::new()
            .keys(&[Key::BTN_TRIGGER])
            .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_THROTTLE]);

    assert_eq!(classify_device(&device), DeviceKind::JOYSTICK);
}

#[test]
fn gamepad_and_joystick_flags_can_overlap() {
    let device = Mock::new().keys(&[
        Key::BTN_SOUTH,
        Key::BTN_TRIGGER,
        Key::BTN_DPAD_UP,
        Key::BTN_DPAD_DOWN,
        Key::BTN_DPAD_LEFT,
        Key::BTN_DPAD_RIGHT,
    ]);

    assert_eq!(
        classify_device(&device),
        DeviceKind::GAMEPAD | DeviceKind::JOYSTICK
    );
}

#[test]
fn gamepad_buttons_without_directional_input_is_not_gamepad() {
    let device = Mock::new().keys(&[Key::BTN_SOUTH, Key::BTN_EAST]);
    assert!(!classify_device(&device).contains(DeviceKind::GAMEPAD));
}

// ----- sensors / edge cases --------------------------------------------------

#[test]
fn accelerometer() {
    let device = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_Z])
        .props(&[PropType::ACCELEROMETER]);

    assert_eq!(classify_device(&device), DeviceKind::ACCELEROMETER);
}

#[test]
fn empty_device_matches_nothing() {
    assert!(classify_device(&Mock::new()).is_empty());
}

// ----- batch classification -------------------------------------------------

#[test]
fn classify_all_devices_preserves_order_and_flags() {
    let keyboard = keyboard();
    let mouse = Mock::new()
        .rel(&[Rel::REL_X, Rel::REL_Y])
        .keys(&[Key::BTN_LEFT]);
    let unknown = Mock::new();

    let results = classify_all_devices(vec![&keyboard, &mouse, &unknown]);

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].1, DeviceKind::KEYBOARD);
    assert_eq!(results[1].1, DeviceKind::MOUSE);
    assert!(results[2].1.is_empty());
    assert!(std::ptr::eq(results[0].0, &keyboard));
    assert!(std::ptr::eq(results[1].0, &mouse));
    assert!(std::ptr::eq(results[2].0, &unknown));
}

#[test]
fn classify_all_devices_handles_empty_input() {
    let results: Vec<(&Mock, DeviceKind)> = classify_all_devices(Vec::new());
    assert!(results.is_empty());
}

// ----- primary classification -----------------------------------------------

#[test]
fn first_prefers_mouse_over_keyboard() {
    let kinds = [DeviceKind::KEYBOARD, DeviceKind::MOUSE];
    assert_eq!(first(&kinds), DeviceKind::MOUSE);
}

#[test]
fn first_merges_flags_before_selecting_priority() {
    let kinds = [
        DeviceKind::KEYBOARD | DeviceKind::MEDIA_KEYS,
        DeviceKind::POWER_BUTTON,
        DeviceKind::TOUCH_PAD,
    ];

    assert_eq!(first(&kinds), DeviceKind::TOUCH_PAD);
}

#[test]
fn first_returns_empty_for_empty_input() {
    assert!(first(&[]).is_empty());
}

#[test]
fn first_returns_empty_when_no_known_flags_are_present() {
    assert!(first(&[DeviceKind::empty(), DeviceKind::empty()]).is_empty());
}
