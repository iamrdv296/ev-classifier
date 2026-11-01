use crate::device_attributes::DeviceAttributesLike;

pub trait ClassifierExt {
    fn classify_as<F>(&self, f: F) -> bool
    where
        F: Fn(&Self) -> bool;
}

impl<T: DeviceAttributesLike> ClassifierExt for T {
    fn classify_as<F>(&self, f: F) -> bool
    where
        F: Fn(&Self) -> bool,
    {
        f(self)
    }
}
