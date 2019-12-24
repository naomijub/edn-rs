extern crate edn_rs;

#[cfg(test)]
mod tests {
    use crate::edn_rs::parse_edn;
    use crate::edn_rs::edn::{EdnNode, EdnType};

    #[test]
    fn empty_returns_nil() {
        let vec = String::from("");
        let expected = EdnNode {
            value: String::from("nil"),
            edntype: EdnType::Nil,
            internal: None,
        };

        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn int1_returns_edn1() {
        let vec = String::from("1");
        let expected = EdnNode {
            value: String::from("1"),
            edntype: EdnType::Int,
            internal: None,
        };

        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn keyword_returns_edn_key() {
        let vec = String::from(":key");
        let expected = EdnNode {
            value: String::from(":key"),
            edntype: EdnType::Key,
            internal: None,
        };

        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn parse_vector_of_ints() {
        let vec = String::from("[1 2 3]");
        let expected = EdnNode {
            value: String::from("["),
            edntype: EdnType::Vector,
            internal: Some(vec![
                EdnNode {
                    value: String::from("1"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("2"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("3"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("]"),
                    edntype: EdnType::VectorClose,
                    internal: None,
                },
            ]),
        };
        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn parse_map_of_key_ints() {
        let vec = String::from("{:a 1 :b 2}");
        let expected = EdnNode {
            value: String::from("{"),
            edntype: EdnType::Map,
            internal: Some(vec![
                EdnNode {
                    value: String::from(":a"),
                    edntype: EdnType::Key,
                    internal: None,
                },
                EdnNode {
                    value: String::from("1"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from(":b"),
                    edntype: EdnType::Key,
                    internal: None,
                },
                EdnNode {
                    value: String::from("2"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("}"),
                    edntype: EdnType::MapSetClose,
                    internal: None,
                },
            ]),
        };
        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn parse_unbalanced_map() {
        let vec = String::from("{:a 1 :b}");
        let expected = EdnNode {
            value: String::from("Unbalanced Map"),
            edntype: EdnType::Err,
            internal: None,
        };
        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn parse_set_of_values() {
        let vec = String::from("#{ :a 1 2 2 :b  g 1 2 }");
        let expected = EdnNode {
            value: String::from("#{"),
            edntype: EdnType::Set,
            internal: Some(vec![
                EdnNode {
                    value: String::from("1"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("2"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from(":a"),
                    edntype: EdnType::Key,
                    internal: None,
                },
                EdnNode {
                    value: String::from(":b"),
                    edntype: EdnType::Key,
                    internal: None,
                },
                EdnNode {
                    value: String::from("g"),
                    edntype: EdnType::Symbol,
                    internal: None,
                },
                EdnNode {
                    value: String::from("}"),
                    edntype: EdnType::MapSetClose,
                    internal: None,
                },
            ]),
        };
        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn parse_list_of_values() {
        let vec = String::from("(1 :a \"b\" c)");
        let expected = EdnNode {
            value: String::from("("),
            edntype: EdnType::List,
            internal: Some(vec![
                EdnNode {
                    value: String::from("1"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from(":a"),
                    edntype: EdnType::Key,
                    internal: None,
                },
                EdnNode {
                    value: String::from("\"b\""),
                    edntype: EdnType::Str,
                    internal: None,
                },
                EdnNode {
                    value: String::from("c"),
                    edntype: EdnType::Symbol,
                    internal: None,
                },
                EdnNode {
                    value: String::from(")"),
                    edntype: EdnType::ListClose,
                    internal: None,
                },
            ]),
        };
        assert_eq!(parse_edn(vec), expected);
    }
}