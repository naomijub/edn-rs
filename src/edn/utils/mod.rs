use regex::{Regex, Captures};

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