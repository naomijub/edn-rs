use regex::{Regex, Captures};

pub mod index;

pub fn replace_keywords(json: String) -> String {
    let re = Regex::new(r#""\w*(\s\w*)*":"#).unwrap();

    let edn = re.replace_all(&json[..], |caps: &Captures| {
        let mut rcap = caps[0]
                .replace("\"","")
                .replace(":","")
                .replace("_","-")
                .replace(" ","-");
            rcap.insert(0,':');
            format!("{}", rcap)
        });
    edn.to_string()
}

pub fn replace_inner_keywords(json: String) -> String {
    let re = Regex::new(r#"\w*:"#).unwrap();

    let edn = re.replace_all(&json[..], |caps: &Captures| {
        let mut rcap = caps[0]
                .replace("\"","")
                .replace(":","")
                .replace("_","-");
            rcap.insert(0,':');
            format!("{}", rcap)
        });
    edn.to_string()
}

pub fn replace_char(json: String) -> String {
    let c_re = Regex::new(r#"'.'"#).unwrap();

    let edn = c_re.replace_all(&json[..], |caps: &Captures| {
        let mut rcap = caps[0]
                .replace("\'","");
            rcap.insert(0,'\\');
            format!("{}", rcap)
        });
    edn.to_string()
}

pub trait Attribute {
    fn process(&self) -> String;
}

impl Attribute for f64 {
    fn process(&self) -> String { 
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for f32 {
    fn process(&self) -> String { 
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for isize {
    fn process(&self) -> String {
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for i32 {
    fn process(&self) -> String {
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for i64 {
    fn process(&self) -> String {
        format!("{:?}",self.to_owned())
    }
}


impl Attribute for usize {
    fn process(&self) -> String {
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for u64 {
    fn process(&self) -> String {
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for u32 {
    fn process(&self) -> String {
        format!("{:?}",self.to_owned())
    }
}

impl Attribute for &str {
    fn process(&self) -> String {
        format!("{}",self.to_owned())
    }
}

impl Attribute for bool {
    fn process(&self) -> String {
        format!("{}",self.to_owned())
    }
}