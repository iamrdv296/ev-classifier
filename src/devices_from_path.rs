use std::path::PathBuf;
use glob::{GlobError, PatternError, glob};

const PATH: &str = "/dev/input/event*";
const GLOB_ERROR_MESSAGE: &str = "Failed to parse input device glob pattern.";

fn device_path_bufs(path: Option<&str>) -> Result<Vec<PathBuf>, GlobError> {
    let path = path.unwrap_or(PATH);

    glob(path)
        .map_err(PatternError)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(DevicePathsError::Glob)
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
    if let Ok(pathbuf) = device_path_bufs(path) {
        devices_from_pathbuf(pathbuf)
    } else {
        vec![]
    }
}