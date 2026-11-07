use evdev::{
    AbsoluteAxisCode, AttributeSet, KeyCode, PropType, RelativeAxisCode,
};
use std::io;

pub trait DeviceInfo {
    fn name(&self) -> Option<&str>;
    fn supported_keys(&self) -> Option<&AttributeSet<KeyCode>>;
    fn supported_relative_axis(&self) -> Option<&AttributeSet<RelativeAxisCode>>;
    fn supported_absolute_axis(&self) -> Option<&AttributeSet<AbsoluteAxisCode>>;
    fn properties(&self) -> &AttributeSet<PropType>;

    fn has_key(&self, key: KeyCode) -> bool {
        self.supported_keys().is_some_and(|sk| sk.contains(key))
    }
    fn has_relative(&self, rel: RelativeAxisCode) -> bool {
        self.supported_relative_axis()
            .is_some_and((|sra| sra.contains(rel)))
    }
    fn has_absolute(&self, abs: AbsoluteAxisCode) -> bool {
        self.supported_absolute_axis()
            .is_some_and(|saa| saa.contains(abs))
    }
}

pub trait DeviceState {
    fn pressed_keys(&self) -> io::Result<AttributeSet<KeyCode>>;
    fn relative_axis_values(&self) -> io::Result<Vec<(RelativeAxisCode, i32)>>;
    fn absolute_axis_values(&self) -> io::Result<Vec<(AbsoluteAxisCode, i32)>>;
}
