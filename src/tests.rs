use evdev::{
    AbsoluteAxisCode as Abs, AttributeSet, KeyCode as Key, PropType, RelativeAxisCode as Rel,
};

use crate::{classifier::Classifier, device_info::DeviceInfo};

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

    fn keys(mut self, ks: &[Key]) -> Self {
        let set = self.keys.get_or_insert_with(AttributeSet::new);
        for &k in ks {
            set.insert(k);
        }
        self
    }

    fn rel(mut self, rs: &[Rel]) -> Self {
        let set = self.rel.get_or_insert_with(AttributeSet::new);
        for &r in rs {
            set.insert(r);
        }
        self
    }

    fn abs(mut self, a: &[Abs]) -> Self {
        let set = self.abs.get_or_insert_with(AttributeSet::new);
        for &x in a {
            set.insert(x);
        }
        self
    }

    fn props(mut self, ps: &[PropType]) -> Self {
        for &p in ps {
            self.props.insert(p);
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

// ----- helpers ---------------------------------------------------------------
fn matches(d: &Mock) -> Vec<&'static str> {
    [
        ("keyboard", d.is_keyboard()),
        ("power_button", d.is_power_button()),
        ("sleep_button", d.is_sleep_button()),
        ("media_keys", d.is_media_keys()),
        ("pen_tablet", d.is_pen_tablet()),
        ("touchscreen", d.is_touchscreen()),
        ("touchpad", d.is_touchpad()),
        ("pointing_stick", d.is_pointing_stick()),
        ("mouse", d.is_mouse()),
        ("gamepad", d.is_gamepad()),
        ("joystick", d.is_joystick()),
        ("accelerometer", d.is_accelerometer()),
    ]
    .into_iter()
    .filter_map(|(name, hit)| hit.then_some(name))
    .collect()
}

fn assert_only(d: &Mock, expected: &str) {
    assert_eq!(matches(d), vec![expected]);
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

// ----- keys ------------------------------------------------------------------
#[test]
fn keyboard_is_keyboard() {
    assert_only(&keyboard(), "keyboard");
}

#[test]
fn keyboard_missing_a_letter_is_not_keyboard() {
    let mut ks = letters();
    ks.pop();
    let d = Mock::new()
        .keys(&ks)
        .keys(&[Key::KEY_ENTER, Key::KEY_SPACE]);
    assert!(!d.is_keyboard());
}

#[test]
fn keyboard_with_power_sleep_and_media_keys_stays_keyboard() {
    let d = keyboard().keys(&[
        Key::KEY_POWER,
        Key::KEY_SLEEP,
        Key::KEY_VOLUMEUP,
        Key::KEY_PLAYPAUSE,
    ]);
    assert_only(&d, "keyboard");
}

#[test]
fn power_button() {
    assert_only(&Mock::new().keys(&[Key::KEY_POWER]), "power_button");
}

#[test]
fn sleep_button() {
    assert_only(&Mock::new().keys(&[Key::KEY_SLEEP]), "sleep_button");
}

#[test]
fn media_keys_with_volume_and_transport() {
    let d = Mock::new().keys(&[Key::KEY_VOLUMEUP, Key::KEY_VOLUMEDOWN, Key::KEY_PLAYPAUSE]);
    assert_only(&d, "media_keys");
}

#[test]
fn media_keys_volume_only() {
    let d = Mock::new().keys(&[Key::KEY_VOLUMEUP, Key::KEY_VOLUMEDOWN]);
    assert_only(&d, "media_keys");
}

// ----- pen / touch -----------------------------------------------------------

#[test]
fn external_pen_tablet() {
    let d = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_PRESSURE])
        .keys(&[Key::BTN_TOOL_PEN, Key::BTN_STYLUS, Key::BTN_TOUCH]);
    assert_only(&d, "pen_tablet");
}

/// Regression: pen digitizers on built-in screens carry INPUT_PROP_DIRECT
/// and previously matched neither pen_tablet nor touchscreen.
#[test]
fn pen_on_direct_screen_is_pen_tablet() {
    let d = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_PRESSURE])
        .keys(&[Key::BTN_TOOL_PEN, Key::BTN_STYLUS, Key::BTN_TOUCH])
        .props(&[PropType::DIRECT]);
    assert_only(&d, "pen_tablet");
}

#[test]
fn multitouch_touchscreen() {
    let d = Mock::new()
        .abs(&[
            Abs::ABS_MT_POSITION_X,
            Abs::ABS_MT_POSITION_Y,
            Abs::ABS_MT_SLOT,
        ])
        .keys(&[Key::BTN_TOUCH])
        .props(&[PropType::DIRECT]);
    assert_only(&d, "touchscreen");
}

#[test]
fn single_touch_touchscreen() {
    let d = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y])
        .keys(&[Key::BTN_TOUCH])
        .props(&[PropType::DIRECT]);
    assert_only(&d, "touchscreen");
}

#[test]
fn touchpad() {
    let d = Mock::new()
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
    assert_only(&d, "touchpad");
}

// ----- mice ------------------------------------------------------------------

#[test]
fn relative_mouse() {
    let d = Mock::new()
        .rel(&[Rel::REL_X, Rel::REL_Y, Rel::REL_WHEEL])
        .keys(&[Key::BTN_LEFT, Key::BTN_RIGHT]);
    assert_only(&d, "mouse");
}

#[test]
fn absolute_mouse_such_as_vm_tablet() {
    let d = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y])
        .keys(&[Key::BTN_LEFT]);
    assert_only(&d, "mouse");
}

#[test]
fn pointing_stick_is_not_a_mouse() {
    let d = Mock::new()
        .rel(&[Rel::REL_X, Rel::REL_Y])
        .keys(&[Key::BTN_LEFT])
        .props(&[PropType::POINTING_STICK]);
    assert_only(&d, "pointing_stick");
}

// ----- game controllers ------------------------------------------------------

#[test]
fn gamepad_with_stick() {
    let d = Mock::new()
        .keys(&[Key::BTN_SOUTH, Key::BTN_EAST, Key::BTN_NORTH, Key::BTN_WEST])
        .abs(&[Abs::ABS_X, Abs::ABS_Y]);
    assert_only(&d, "gamepad");
}

#[test]
fn gamepad_with_dpad_only() {
    let d = Mock::new().keys(&[
        Key::BTN_SOUTH,
        Key::BTN_DPAD_UP,
        Key::BTN_DPAD_DOWN,
        Key::BTN_DPAD_LEFT,
        Key::BTN_DPAD_RIGHT,
    ]);
    assert_only(&d, "gamepad");
}

#[test]
fn joystick() {
    let d = Mock::new()
        .keys(&[Key::BTN_TRIGGER])
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_THROTTLE]);
    assert_only(&d, "joystick");
}

#[test]
fn gamepad_buttons_without_directional_input_is_not_gamepad() {
    let d = Mock::new().keys(&[Key::BTN_SOUTH, Key::BTN_EAST]);
    assert!(matches(&d).is_empty());
}

// ----- sensors / edge cases --------------------------------------------------

#[test]
fn accelerometer() {
    let d = Mock::new()
        .abs(&[Abs::ABS_X, Abs::ABS_Y, Abs::ABS_Z])
        .props(&[PropType::ACCELEROMETER]);
    assert_only(&d, "accelerometer");
}

#[test]
fn empty_device_matches_nothing() {
    assert!(matches(&Mock::new()).is_empty());
}
