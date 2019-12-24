extern crate regex;

pub mod edn;
mod utils;

use edn::EdnNode;

/// `parse_edn` receives a String with the EDN context and transforms it in a EdnNode`
/// 
/// ```rust
/// let edn = String::from("[1 2 [:3 \"4\"]]");
/// let value = parse_edn(edn);
/// ```
/// 
/// A response for value is: 
/// ```
/// EdnNode {
///    value: String::from("["),
///    edntype: EdnType::Vector,
///    internal: Some(vec![
///        EdnNode {
///            value: String::from("1"),
///            edntype: EdnType::Int,
///            internal: None,
///        },
///        EdnNode {
///            value: String::from("2"),
///            edntype: EdnType::Int,
///            internal: None,
///        },
///        EdnNode {
///            value: String::from("["),
///            edntype: EdnType::Vector,
///            internal: Some(vec![
///                EdnNode {
///                    value: String::from(":3"),
///                    edntype: EdnType::Key,
///                    internal: None,
///                },
///                EdnNode {
///                    value: String::from("\"4\""),
///                    edntype: EdnType::Str,
///                    internal: None,
///                },
///                EdnNode {
///                    value: String::from("]"),
///                    edntype: EdnType::VectorClose,
///                    internal: None,
///                },
///            ]),
///        },
///        EdnNode {
///            value: String::from("]"),
///            edntype: EdnType::VectorClose,
///            internal: None,
///        },
///    ]),
///  };
/// ```
pub fn parse_edn(edn: String) -> EdnNode {
    let mut end_tokens = utils::tokenize_edn(edn);

    if end_tokens.is_empty() {
        return EdnNode::nil();
    }

    utils::ednify(end_tokens.remove(0), &mut end_tokens)
}