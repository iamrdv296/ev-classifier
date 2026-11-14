pub mod classifier;
pub mod classifiers;
pub mod device_info;
pub mod discovery;

#[cfg(test)]
mod tests;

pub use classifier::{DeviceKind, classify_all_devices, classify_device};
pub use classifiers::{absolute_axes_based, key_based, miscellaneous, relative_axes_based};
pub use device_info::DeviceInfo;
pub use discovery::DeviceDiscovery;
