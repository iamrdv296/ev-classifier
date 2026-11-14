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
pub fn device_path_bufs(path: Option<&Path>) -> io::Result<Vec<PathBuf>> {
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
