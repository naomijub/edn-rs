extern crate edn_rs;

#[cfg(test)]
mod tests {
    use crate::edn_rs::parse_edn;
    use crate::edn_rs::edn::{EdnNode, EdnType};

   #[test]
    fn parse_vector_in_vector() {
        let vec = String::from("[1 2 [:3 \"4\"]]");
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
                    value: String::from("["),
                    edntype: EdnType::Vector,
                    internal: Some(vec![
                        EdnNode {
                            value: String::from(":3"),
                            edntype: EdnType::Key,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("\"4\""),
                            edntype: EdnType::Str,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("]"),
                            edntype: EdnType::VectorClose,
                            internal: None,
                        },
                    ]),
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
    fn parse_set_in_vector() {
        let vec = String::from("[1 2 #{:3 \"4\"}]");
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
                    value: String::from("#{"),
                    edntype: EdnType::Set,
                    internal: Some(vec![
                        EdnNode {
                            value: String::from(":3"),
                            edntype: EdnType::Key,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("\"4\""),
                            edntype: EdnType::Str,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("}"),
                            edntype: EdnType::MapSetClose,
                            internal: None,
                        },
                    ]),
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
    fn parse_set_in_set() {
        let set = String::from("#{1 2 #{:3 \"4\"}}");
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
                    value: String::from("#{"),
                    edntype: EdnType::Set,
                    internal: Some(vec![
                        EdnNode {
                            value: String::from(":3"),
                            edntype: EdnType::Key,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("\"4\""),
                            edntype: EdnType::Str,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("}"),
                            edntype: EdnType::MapSetClose,
                            internal: None,
                        },
                    ]),
                },
                EdnNode {
                    value: String::from("}"),
                    edntype: EdnType::MapSetClose,
                    internal: None,
                },
            ]),
        };
        assert_eq!(parse_edn(set), expected);
    }

    #[test]
    fn parse_vec_in_set() {
        let set = String::from("#{1 2 [:3 \"4\"]}");
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
                    value: String::from("["),
                    edntype: EdnType::Vector,
                    internal: Some(vec![
                        EdnNode {
                            value: String::from(":3"),
                            edntype: EdnType::Key,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("\"4\""),
                            edntype: EdnType::Str,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("]"),
                            edntype: EdnType::VectorClose,
                            internal: None,
                        },
                    ]),
                },
                EdnNode {
                    value: String::from("}"),
                    edntype: EdnType::MapSetClose,
                    internal: None,
                },
            ]),
        };
        assert_eq!(parse_edn(set), expected);
    }

    #[test]
    fn parse_list_in_list() {
        let vec = String::from("(1 2 (:3 \"4\"))");
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
                    value: String::from("2"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("("),
                    edntype: EdnType::List,
                    internal: Some(vec![
                        EdnNode {
                            value: String::from(":3"),
                            edntype: EdnType::Key,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("\"4\""),
                            edntype: EdnType::Str,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from(")"),
                            edntype: EdnType::ListClose,
                            internal: None,
                        },
                    ]),
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

    #[test]
    fn parse_set_in_list() {
        let vec = String::from("'(1 2 #{:3 \"4\"})");
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
                    value: String::from("2"),
                    edntype: EdnType::Int,
                    internal: None,
                },
                EdnNode {
                    value: String::from("#{"),
                    edntype: EdnType::Set,
                    internal: Some(vec![
                        EdnNode {
                            value: String::from(":3"),
                            edntype: EdnType::Key,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("\"4\""),
                            edntype: EdnType::Str,
                            internal: None,
                        },
                        EdnNode {
                            value: String::from("}"),
                            edntype: EdnType::MapSetClose,
                            internal: None,
                        },
                    ]),
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
