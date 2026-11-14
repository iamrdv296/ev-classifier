use evdev::Device;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// The directory scanned for input devices when no path is given.
const DEFAULT_DIR: &str = "/dev/input";

/// The file name prefix that identifies evdev device nodes (`event0`,
/// `event1`, ...) when scanning a directory.
const PREFIX: &str = "event";

/// The result of discovering and opening input devices.
///
/// Discovery is best-effort: a device that can't be opened (for example
/// because of missing permissions) doesn't abort the whole scan. Instead it is
/// recorded in [`skipped`](Self::skipped) along with the reason, so callers can
/// decide whether to report it.
pub struct DeviceDiscovery {
    /// The devices that were opened successfully, in the order their paths
    /// were discovered (numerically ascending by event number when scanning a
    /// directory).
    pub devices: Vec<Device>,

    /// The paths that could not be opened, each paired with the I/O error that
    /// caused it. Uses the same ordering as [`devices`](Self::devices).
    pub skipped: Vec<(PathBuf, io::Error)>,
}

impl DeviceDiscovery {
    /// Discovers and opens input devices at `path`.
    ///
    /// - `None` scans the default directory, `/dev/input`.
    /// - A directory is scanned for `event<N>` entries (see
    ///   [`device_path_bufs`]).
    /// - Any other path is treated as a single device node and opened as is.
    ///
    /// # Errors
    ///
    /// Returns an error if the path can't be inspected or the directory can't
    /// be read, such as [`io::ErrorKind::NotFound`] for a missing path.
    /// Failures to open individual devices are **not** errors; they are
    /// collected in [`skipped`](Self::skipped).
    pub fn from_path(path: Option<&Path>) -> io::Result<Self> {
        Ok(Self::from_path_bufs(device_path_bufs(path)?))
    }

    /// Tries to open every path, partitioning the outcomes into opened
    /// devices and skipped paths while preserving input order.
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

/// Resolves the list of device paths to open for `path`.
///
/// If `path` is `None`, [`DEFAULT_DIR`] is used. If the target is not a
/// directory, it is returned as the only path. If it is a directory, its
/// entries are filtered to names of the form `event<N>`, where `<N>` is a
/// non-negative integer, and sorted numerically by `N` (so `event2` comes
/// before `event10`). Everything else is ignored, including names with a
/// non-numeric suffix (`event1-extra`), a bare `event`, and names that aren't
/// valid UTF-8.
///
/// # Errors
///
/// Returns an error if the target's metadata can't be read, or if the
/// directory or one of its entries can't be read.
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

        let error = match DeviceDiscovery::from_path(Some(&missing)) {
            Ok(_) => panic!("expected a missing path to return an error"),
            Err(error) => error,
        };

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
}
