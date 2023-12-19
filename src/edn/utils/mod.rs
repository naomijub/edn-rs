use alloc::format;
use alloc::string::{String, ToString};

pub mod index;

pub trait Attribute {
    fn process(&self) -> String;
}

impl Attribute for f64 {
    fn process(&self) -> String {
        format!("{self:?}")
    }
}

impl Attribute for f32 {
    fn process(&self) -> String {
        format!("{self:?}")
    }
}

impl Attribute for i32 {
    fn process(&self) -> String {
        format!("{self:?}")
    }
}

impl Attribute for i64 {
    fn process(&self) -> String {
        format!("{self:?}")
    }
}

impl Attribute for u64 {
    fn process(&self) -> String {
        format!("{self:?}")
    }
}

impl Attribute for u32 {
    fn process(&self) -> String {
        format!("{self:?}")
    }
}

impl Attribute for &str {
    fn process(&self) -> String {
        (*self).to_string()
    }
}

impl Attribute for bool {
    fn process(&self) -> String {
        format!("{self}")
    }
}
