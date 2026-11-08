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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_TEMP_DIR_ID: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let id = NEXT_TEMP_DIR_ID.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "evdev-classifier-discovery-{}-{id}",
                std::process::id()
            ));
            fs::create_dir(&path).expect("create temporary test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn file(&self, name: &str) -> PathBuf {
            let path = self.0.join(name);
            fs::write(&path, []).expect("create test file");
            path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn directory_entries_are_filtered_and_sorted_numerically() {
        let dir = TestDir::new();
        let event10 = dir.file("event10");
        let event2 = dir.file("event2");
        let event0 = dir.file("event0");

        dir.file("mouse0");
        dir.file("event1-extra");
        dir.file("event");
        dir.file("event-1");

        let paths = device_path_bufs(Some(dir.path())).unwrap();

        assert_eq!(paths, vec![event0, event2, event10]);
    }

    #[test]
    fn a_single_file_path_is_returned_as_is() {
        let dir = TestDir::new();
        let path = dir.file("custom-device");

        let paths = device_path_bufs(Some(&path)).unwrap();

        assert_eq!(paths, vec![path]);
    }

    #[test]
    fn unopenable_device_files_are_reported_as_skipped() {
        let dir = TestDir::new();
        let event1 = dir.file("event1");
        let event2 = dir.file("event2");

        let discovery = DeviceDiscovery::from_path(Some(dir.path())).unwrap();

        assert!(discovery.devices.is_empty());
        let skipped_paths: Vec<_> = discovery
            .skipped
            .iter()
            .map(|(path, _)| path.clone())
            .collect();
        assert_eq!(skipped_paths, vec![event1, event2]);
    }

    #[test]
    fn a_missing_directory_returns_an_error() {
        let dir = TestDir::new();
        let missing = dir.path().join("does-not-exist");

        let error = DeviceDiscovery::from_path(Some(&missing)).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
}
