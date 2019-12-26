extern crate edn_rs;

#[cfg(test)]
mod tests {
    use crate::edn_rs::emit_edn;

    #[test]
    fn emits_helloworld_edn() {
        let json = String::from("{\"hello\": \"world\"}");
        let edn = String::from("{:hello \"world\"}");

        assert_eq!(edn, emit_edn(json));
    }

    #[test]
    fn replaces_whitespace_in_keys_by_dash() {
        let json = String::from("{\"hello world\": \"julia\"}");
        let edn = String::from("{:hello-world \"julia\"}");

        assert_eq!(edn, emit_edn(json));
    }

    #[test]
    fn emits_nil_edn() {
        let json = String::from("{\"hello\": null}");
        let edn = String::from("{:hello nil}");

        assert_eq!(edn, emit_edn(json));
    }

    #[test]
    fn emits_char_edn() {
        let json = String::from("{\"hello\": 'c'}");
        let edn = String::from("{:hello \\c}");

        assert_eq!(edn, emit_edn(json));
    }
}
