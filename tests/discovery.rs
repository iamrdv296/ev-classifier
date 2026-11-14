use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use evdev_classifier::DeviceDiscovery;
use evdev_classifier::discovery::device_path_bufs;

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
