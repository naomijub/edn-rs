extern crate regex;

mod utils;
mod edn;

use edn::{EdnNode, EdnType};

pub fn parse_edn(edn: String) -> EdnNode {
    let mut end_tokens = utils::tokenize_edn(edn);

    if end_tokens.is_empty() {
        return EdnNode::nil();
    }

    utils::ednify(end_tokens.remove(0), &mut end_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_nil() {
        let vec = String::from("");
        let expected = EdnNode {
            value: String::from("nil"),
            edntype: EdnType::Nil,
            internal: None
        };

        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn int1_returns_edn1() {
        let vec = String::from("1");
        let expected = EdnNode {
            value: String::from("1"),
            edntype: EdnType::Int,
            internal: None
        };

        assert_eq!(parse_edn(vec), expected);
    }

    #[test]
    fn keyword_returns_edn_key() {
        let vec = String::from(":key");
        let expected = EdnNode {
            value: String::from(":key"),
            edntype: EdnType::Key,
            internal: None
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
                    internal: None
                },
                EdnNode {
                    value: String::from("2"),
                    edntype: EdnType::Int,
                    internal: None
                },
                EdnNode {
                    value: String::from("3"),
                    edntype: EdnType::Int,
                    internal: None
                },
                EdnNode {
                    value: String::from("]"),
                    edntype: EdnType::VectorClose,
                    internal: None
                }])
        };
        assert_eq!(parse_edn(vec), expected);
    }
}
