use evdev::Device;
use glob::{GlobError, PatternError, glob};
use std::path::PathBuf;

const PATH: &str = "/dev/input/event*";

#[derive(Debug)]
pub enum DevicePathError {
    Glob(GlobError),
    Pattern(PatternError),
}

pub struct DeviceDiscovery {
    pub devices: Vec<Device>,
    pub skipped: Vec<(PathBuf, std::io::Error)>,
}

pub fn device_path_bufs(path: Option<&str>) -> Result<Vec<PathBuf>, DevicePathError> {
    let path = path.unwrap_or(PATH);

    glob(path)
        .map_err(DevicePathError::Pattern)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(DevicePathError::Glob)
}

pub fn devices_from_pathbuf(pathbufs: Vec<PathBuf>) -> DeviceDiscovery {
    let mut skipped = vec![];
    let devices = pathbufs
        .into_iter()
        .filter_map(|path| match Device::open(&path) {
            Ok(device) => Some(device),
            Err(io_err) => {
                skipped.push((path, io_err));
                None
            }
        })
        .collect();

    DeviceDiscovery { devices, skipped }
}