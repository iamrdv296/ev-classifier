# Changelog

All notable changes to this project are documented here.

## [0.2.0]

### Added
- `DeviceKind` is now a bitflag set, allowing a device to match multiple classifications simultaneously.
- `classify_all_devices` classifies a batch of devices while preserving input order.
- `first` selects the highest-priority classification from a collection of `DeviceKind` values.
- Specialized classifier traits split capability checks into key-based, absolute-axis-based, relative-axis-based, and miscellaneous groups.
- Additional `DeviceInfo` helpers for checking key sets, axes, multi-touch capabilities, directional input, and scroll wheels.
- Hardware-free tests for individual classifications, overlapping classifications, batch classification, and priority selection.
- Discovery tests for event-node filtering and numeric ordering.

### Changed
- Classification methods now return `DeviceKind` bitflags instead of a single enum variant.
- Added `bitflags` as a runtime dependency.
- Updated the public API and documentation to describe overlapping classifications.

### Compatibility
- This release changes the public classification API. Code matching on the former `DeviceKind` enum must be updated to use bitflag operations such as `contains`, `intersects`, and bitwise OR.