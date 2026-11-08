# evdev-classifier

Classify Linux input devices (`/dev/input/event*`) by what they *are*: keyboard, mouse, touchpad, touchscreen, gamepad, and so on.

The kernel's evdev interface only reports a device's raw capabilities (which keys, axes and properties it supports). It never says "this is a touchpad". This crate looks at those capabilities and applies a set of heuristics to name the device type for you.

> **Linux only.** This crate builds on [`evdev`](https://crates.io/crates/evdev), which does not compile on macOS or Windows.

## Features

- Classifies devices into a `DeviceKind` enum (`Keyboard`, `Mouse`, `TouchPad`, `Gamepad`, ...).
- Individual `is_*` predicates (`is_keyboard()`, `is_touchpad()`, ...) if you want to ask a specific question.
- Device discovery that scans `/dev/input`, sorts `event2` before `event10`, and reports devices it couldn't open instead of failing.
- Hardware-free testing: classification works on any type implementing the `DeviceInfo` trait, so you can classify mock devices in unit tests.

## Installation

```toml
[dependencies]
evdev-classifier = "0.1"
```

Requires Rust 1.70 or newer.

## Quick start

```rust
use evdev_classifier::{classify_device, DeviceDiscovery};

fn main() -> std::io::Result<()> {
    // Scan /dev/input. Pass Some(path) to scan another directory or open one device.
    let discovery = DeviceDiscovery::from_path(None)?;

    for device in &discovery.devices {
        let name = device.name().unwrap_or("unnamed");
        println!("{name}: {:?}", classify_device(device));
    }

    // Devices that couldn't be opened (usually permission errors).
    for (path, err) in &discovery.skipped {
        eprintln!("skipped {}: {err}", path.display());
    }
    Ok(())
}
```

Classify a whole batch:

```rust
use evdev_classifier::{classify_all_devices, DeviceDiscovery};

let discovery = DeviceDiscovery::from_path(None)?;
let refs: Vec<_> = discovery.devices.iter().collect();
for (device, kind) in classify_all_devices(refs) {
    println!("{:?}: {kind:?}", device.name());
}
# Ok::<(), std::io::Error>(())
```

Or ask a specific question using the `Classifier` trait:

```rust
use evdev_classifier::Classifier;

# fn demo(device: &evdev::Device) {
if device.is_touchpad() {
    // ...
}
# }
```

## Permissions

Reading `/dev/input/event*` normally requires root or membership in the `input` group:

```sh
sudo usermod -aG input "$USER"   # log out and back in afterwards
```

Devices you can't open end up in `DeviceDiscovery::skipped` rather than aborting discovery.

## Device kinds

| `DeviceKind`    | Detected when the device...                                                              |
| --------------- | ---------------------------------------------------------------------------------------- |
| `Keyboard`      | supports all 26 letter keys plus Enter and Space                                         |
| `PowerButton`   | supports `KEY_POWER` and is not a keyboard                                               |
| `SleepButton`   | supports `KEY_SLEEP` and is not a keyboard                                               |
| `MediaKeys`     | supports volume or transport keys (play/pause, next) and is not a keyboard              |
| `PenTablet`     | reports absolute position and has a pen tool (`BTN_TOOL_PEN` / `BTN_STYLUS`)             |
| `TouchScreen`   | reports absolute position, has no pen tool, and is `DIRECT` (or has `BTN_TOUCH` without finger tools) |
| `TouchPad`      | reports absolute position, has finger tools, no pen tool, and is not `DIRECT`            |
| `PointingStick` | reports relative X/Y and has the `POINTING_STICK` property                               |
| `Mouse`         | has `BTN_LEFT` and either relative X/Y, or absolute X/Y that isn't a pen/touch device    |
| `Gamepad`       | has face buttons (`BTN_SOUTH`/`EAST`/`NORTH`/`WEST`) and directional input               |
| `Joystick`      | has `BTN_TRIGGER` and directional input, right stick or throttle, and is not a gamepad   |
| `Accelerometer` | has the `ACCELEROMETER` property                                                         |
| `Unknown`       | matches none of the above                                                                |

`DeviceKind` is `#[non_exhaustive]`, so new kinds can be added without a breaking change. Include a wildcard arm when you `match` on it.

## How classification works

`classify_device` checks the kinds **in the order listed above** and returns the first match. This matters because real hardware often satisfies several predicates at once. For example, a laptop touchpad with `BTN_LEFT` also looks like an absolute mouse, so `Mouse` explicitly excludes pens, touchpads and touchscreens, and the priority order resolves the rest.

If you call the individual `is_*` predicates directly, remember they can overlap. `classify_device` is the one that picks a single answer.

These are heuristics, not guarantees. If a device is misclassified, please open an issue with the output of `evtest` or `udevadm info` for it.

## Testing without hardware

Anything implementing `DeviceInfo` can be classified, which makes unit tests easy:

```rust
use evdev_classifier::{classify_device, DeviceInfo, DeviceKind};
use evdev::{AbsoluteAxisCode, AttributeSetRef, KeyCode, PropType, RelativeAxisCode};

struct Fake { /* store AttributeSets here */ }

impl DeviceInfo for Fake {
    fn name(&self) -> Option<&str> { Some("fake") }
    fn supported_keys(&self) -> Option<&AttributeSetRef<KeyCode>> { todo!() }
    fn supported_relative_axes(&self) -> Option<&AttributeSetRef<RelativeAxisCode>> { todo!() }
    fn supported_absolute_axes(&self) -> Option<&AttributeSetRef<AbsoluteAxisCode>> { todo!() }
    fn properties(&self) -> &AttributeSetRef<PropType> { todo!() }
}
```

See `src/tests.rs` for a complete builder-style mock used by this crate's own test suite.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

Tests run without any input hardware.