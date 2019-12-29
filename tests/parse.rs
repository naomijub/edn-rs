extern crate edn_rs;

#[cfg(test)]
mod tests {
    use crate::edn_rs::{
        edn::Edn,
        parse_edn,
    };

    #[test]
    fn parse_primitive_types() {
        assert_eq!(parse_edn("1".to_string()), Edn::Int(1));
        assert_eq!(parse_edn("12.5".to_string()), Edn::Double(12.5));
        assert_eq!(parse_edn("\\c".to_string()), Edn::Char('c'));
        assert_eq!(parse_edn(":key-word".to_string()), Edn::Key(":key-word".to_string()));
        assert_eq!(parse_edn("\"this is a string\"".to_string()), Edn::Str("\"this is a string\"".to_string()));
        assert_eq!(parse_edn("my-symbol".to_string()), Edn::Symbol("my-symbol".to_string()));
        assert_eq!(parse_edn("3/4".to_string()), Edn::Rational("3/4".to_string()));
        assert_eq!(parse_edn("true".to_string()), Edn::Bool(true));
        assert_eq!(parse_edn("".to_string()), Edn::Nil);
    }

    #[test]
    fn parse_simple_list() {
        let list = String::from("[1 2 3]");
        assert_eq!(parse_edn(list), Edn::Nil);
    }
}