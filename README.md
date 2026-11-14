# evdev-classifier

Classify Linux input devices (`/dev/input/event*`) by their capabilities: keyboards, mice, touchpads, touchscreens, gamepads, joysticks, and more.

The kernel's evdev interface reports raw capabilities (supported keys, axes, and input properties), not semantic device types. This crate applies heuristics to those capabilities.

> **Linux only.** This crate uses [`evdev`](https://crates.io/crates/evdev), which does not compile on macOS or Windows.

## Features

- Classifies devices into a `DeviceKind` bitflag set.
- Supports overlapping classifications: a multimedia keyboard can be both `KEYBOARD` and `MEDIA_KEYS`.
- Provides individual `is_*` predicates through specialized classifier traits.
- Discovers devices under `/dev/input`, sorts `event2` before `event10`, and records devices it could not open.
- Supports hardware-free classification tests through the `DeviceInfo` trait.

## Installation

```toml
[dependencies]
evdev-classifier = "0.2"
```

Requires Rust 1.85 or newer.

## Quick start

```rust
use evdev_classifier::{classify_device, DeviceDiscovery, DeviceKind};

fn main() -> std::io::Result<()> {
    let discovery = DeviceDiscovery::from_path(None)?;

    for device in &discovery.devices {
        let name = device.name().unwrap_or("unnamed");
        let kinds = classify_device(device);

        if kinds.contains(DeviceKind::KEYBOARD) {
            println!("{name}: keyboard");
        }

        println!("{name}: {kinds:?}");
    }

    for (path, err) in &discovery.skipped {
        eprintln!("skipped {}: {err}", path.display());
    }

    Ok(())
}
```

## Working with classifications

`DeviceKind` is a bitflag set rather than an enum. A device can match several classifications at once.

```rust
use evdev_classifier::{classify_device, DeviceKind};

let kinds = classify_device(&device);

if kinds.contains(DeviceKind::KEYBOARD | DeviceKind::MEDIA_KEYS) {
    println!("Multimedia keyboard");
}

if kinds.intersects(DeviceKind::MOUSE | DeviceKind::TOUCH_PAD) {
    println!("Pointing device");
}
```

Use bitwise OR to combine flags, `contains` to check that all requested flags are present, and `intersects` to check whether at least one requested flag is present. An empty set means that no known classification matched.

### Choosing one classification

If an application needs a single label, `first` selects the highest-priority classification from a collection:

```rust
use evdev_classifier::{first, DeviceKind};

let kinds = [
    DeviceKind::KEYBOARD | DeviceKind::MEDIA_KEYS,
    DeviceKind::MOUSE,
];

let primary = first(&kinds);
assert_eq!(primary, DeviceKind::MOUSE);
```

The priority order is defined by the crate. It is a convenience for choosing one label; it does not change the original classification sets.

## Individual capability checks

The classifier traits expose individual checks. Import the trait whose methods you want to use:

```rust
use evdev_classifier::absolute_axes_based::AbsoluteAxesBased;

if device.is_touchpad() {
    println!("Touchpad detected");
}
```

Available classifier traits:

- `key_based::KeyBased`: keyboards, power buttons, sleep buttons, and media keys.
- `absolute_axes_based::AbsoluteAxesBased`: pen tablets, touchscreens, and touchpads.
- `relative_axes_based::RelativeAxesBased`: mice and pointing sticks.
- `miscellaneous::Miscellaneous`: gamepads, joysticks, and accelerometers.

These traits are blanket-implemented for types that implement `DeviceInfo`; applications do not need to implement the classifier traits themselves.

Individual checks can overlap. `classify_device` collects all matching flags rather than stopping at the first match.

## Device discovery

`DeviceDiscovery::from_path(None)` scans `/dev/input`.

You can also provide a directory or a single device path:

```rust
use evdev_classifier::DeviceDiscovery;
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let discovery = DeviceDiscovery::from_path(Some(Path::new("/dev/input")))?;
    println!("Found {} devices", discovery.devices.len());
    Ok(())
}
```

Directory scans consider entries named `event` followed by a non-negative integer and sort them numerically. Devices that cannot be opened are stored in `DeviceDiscovery::skipped`; an individual open failure does not abort the scan.

Reading `/dev/input/event*` normally requires root or membership in the `input` group:

```sh
sudo usermod -aG input "$USER"
```

Log out and back in after changing group membership.

## Classification heuristics

The classifications are capability-based heuristics, not guarantees. In broad terms:

| Classification | Detected when the device... |
|---|---|
| `KEYBOARD` | Supports all 26 letter keys, Enter, and Space |
| `POWER_BUTTON` | Supports `KEY_POWER` |
| `SLEEP_BUTTON` | Supports `KEY_SLEEP` |
| `MEDIA_KEYS` | Supports volume or media playback keys |
| `PEN_TABLET` | Reports absolute position and a pen tool |
| `TOUCH_SCREEN` | Has the `DIRECT` property, no pen tool, and suitable touch axes |
| `TOUCH_PAD` | Reports absolute position, the `POINTER` property, and finger tools |
| `POINTING_STICK` | Reports relative X/Y and the `POINTING_STICK` property |
| `MOUSE` | Has `BTN_LEFT` and suitable relative axes, or absolute axes plus a wheel |
| `GAMEPAD` | Has `BTN_SOUTH` and directional input |
| `JOYSTICK` | Has `BTN_TRIGGER` and directional, rotation, or throttle input |
| `ACCELEROMETER` | Has the `ACCELEROMETER` property |

Because checks are independent, a device may match more than one row. For example, a device can expose both gamepad and joystick capabilities.

If a device is misclassified, please open an issue with relevant output from `evtest` or `udevadm info`.

## Testing without hardware

Classification works with any type implementing `DeviceInfo`, so tests do not need access to real input devices. The crate's test suite uses mock capability sets to cover individual flags, overlapping flags, and priority selection.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

Tests run without input hardware.