extern crate regex;

pub mod edn;
mod utils;

use edn::{EdnNode, 
        utils::{replace_keywords, replace_char}};

/// `parse_edn` receives a String with the EDN context and transforms it in a EdnNode`
/// 
/// ```rust
/// use edn_rs::parse_edn;
/// 
/// let edn = String::from("[1 2 [:3 \"4\"]]");
/// let value = parse_edn(edn);
/// ```
/// 
/// A response for value is: 
/// 
/// ```
/// use edn_rs::edn::{EdnNode, EdnType};
/// 
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


/// `emit_edn` receives a json string and parses its common key-values to a regular EDN format. 
/// tested examples are: 
/// 1. `"{\"hello world\": \"julia\"}"` becomes `"{:hello-world \"julia\"}"`
/// 2. `"{\"hello\": null}"` becomes `"{:hello nil}"`
/// 3. `{\"hello\": 'c'}` becomes `"{:hello \\c}"`
/// 4. `"{\"multi_string with underscore\": 545643}"` becomes `"{:multi-string-with-underscore 545643}"`
/// 
/// ```
/// use edn_rs::emit_edn;
///
/// fn emits_helloworld_edn() {
///     let json = String::from("{\"hello\": \"world\"}");
///     let edn = String::from("{:hello \"world\"}");
///
///     assert_eq!(edn, emit_edn(json));
/// }
/// ```
pub fn emit_edn(json: String) -> String {
    let edn_aux = replace_keywords(json);
    let edn = replace_char(edn_aux);
    edn.replace("null","nil")
}