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

impl DeviceDiscovery {
    pub fn from_path(path: Option<&str>) -> Result<Self, DevicePathError> {
        let path_bufs = device_path_bufs(path)?;
        Ok(Self::from_path_bufs(path_bufs))
    }

    fn from_path_bufs(path_bufs: Vec<PathBuf>) -> Self {
        let mut skipped = vec![];
        let devices = path_bufs
            .into_iter()
            .filter_map(|path| match Device::open(&path) {
                Ok(device) => Some(device),
                Err(io_err) => {
                    skipped.push((path, io_err));
                    None
                }
            })
            .collect();

        Self { devices, skipped }
    }
}

fn device_path_bufs(path: Option<&str>) -> Result<Vec<PathBuf>, DevicePathError> {
    let path = path.unwrap_or(PATH);

    glob(path)
        .map_err(DevicePathError::Pattern)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(DevicePathError::Glob)
}
