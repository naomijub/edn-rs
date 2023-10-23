#[cfg(feature = "json")]
use regex::{Captures, Regex};

pub mod index;

#[cfg(feature = "json")]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn replace_keywords(json: String) -> String {
    let re = Regex::new(r#""\w*(\s\w*)*":"#).unwrap();

    let edn = re.replace_all(&json[..], |caps: &Captures| {
        let mut rcap = caps[0].replace(['\"', ':'], "").replace(['_', ' '], "-");
        rcap.insert(0, ':');
        rcap.to_string()
    });
    edn.to_string()
}

#[cfg(feature = "json")]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn replace_char(json: String) -> String {
    let c_re = Regex::new(r#"'.'"#).unwrap();

    let edn = c_re.replace_all(&json[..], |caps: &Captures| {
        let mut rcap = caps[0].replace('\'', "");
        rcap.insert(0, '\\');
        rcap.to_string()
    });
    edn.to_string()
}

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
