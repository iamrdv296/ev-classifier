use evdev::Device;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const DEFAULT_DIR: &str = "/dev/input";
const PREFIX: &str = "event";

pub struct DeviceDiscovery {
    pub devices: Vec<Device>,
    pub skipped: Vec<(PathBuf, io::Error)>,
}

impl DeviceDiscovery {
    pub fn from_path(path: Option<&Path>) -> io::Result<Self> {
        Ok(Self::from_path_bufs(device_path_bufs(path)?))
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

fn device_path_bufs(path: Option<&Path>) -> io::Result<Vec<PathBuf>> {
    let target = path.unwrap_or_else(|| Path::new(DEFAULT_DIR));

    if !fs::metadata(target)?.is_dir() {
        return Ok(vec![target.to_path_buf()]);
    }

    let mut found: Vec<(u32, PathBuf)> = Vec::new();
    for entry in fs::read_dir(target)? {
        let path = entry?.path();
        let number = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_prefix(PREFIX))
            .and_then(|n| n.parse::<u32>().ok());
        if let Some(n) = number {
            found.push((n, path));
        }
    }

    found.sort_by_key(|(n, _)| *n);
    Ok(found.into_iter().map(|(_, p)| p).collect())
}
