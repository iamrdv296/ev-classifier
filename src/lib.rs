pub mod classifiers;
pub mod device_info;
pub mod classifier;
pub mod discovery;

#[cfg(test)]
mod tests;

pub use classifier::DeviceKind;
pub use device_info::DeviceInfo;
pub use discovery::DeviceDiscovery;
pub use classifiers::{relative_axes_based, absolute_axes_based, miscellaneous, key_based};