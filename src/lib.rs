use std::path::PathBuf;

use evdev::{AttributeSetRef, Device, EventType, KeyCode};
use glob::{GlobError, glob};

const UNKNOWN_NAME: &str = "Unknown Name";

pub struct DeviceAttributes<'a> {
    pub name: String,
    pub supported_events: &'a AttributeSetRef<EventType>,
    pub supported_keys: Option<&'a AttributeSetRef<KeyCode>>,
}

// its necessary to use an explicit lifetime as AttributeSetRef doesn't have a
// known size at compile time and it is dependent on the device's lifetime.

impl<'a> DeviceAttributes<'a> {
    pub fn from_device(device: &'a Device) -> Self {
        Self {
            name: device.name().unwrap_or(UNKNOWN_NAME).to_owned(),
            supported_events: device.supported_events(),
            supported_keys: device.supported_keys(),
        }
    }
}

const PATH: &str = "/dev/input/event*";
const GLOB_ERROR_MESSAGE: &str = "Failed to parse input device glob pattern.";

fn device_paths(path: Option<&str>) -> Result<Vec<PathBuf>, GlobError> {
    let path = path.unwrap_or(PATH);

    glob(path).expect(GLOB_ERROR_MESSAGE).collect()
}

fn devices_from_pathbuf(pathbuf: Vec<PathBuf>) -> Vec<Device> {
    pathbuf
        .into_iter()
        .filter_map(|path| match Device::open(&path) {
            Ok(device) => Some(device),
            Err(io_err) => {
                eprintln!("Warning: Skipping device at {:?}: {}", &path, io_err);
                None
            }
        })
        .collect()
}

pub fn devices_from_path(path: Option<&str>) -> Vec<Device> {
    if let Ok(pathbuf) = device_paths(path) {
        devices_from_pathbuf(pathbuf)
    } else {
        vec![]   
    }
}