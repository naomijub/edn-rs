#![recursion_limit="512"]
extern crate regex;

pub mod edn;
pub mod macros;

use edn::{utils::{replace_keywords, replace_char}};


// pub fn parse_edn(edn: String) -> Edn {
//     from_edn(edn)
// }

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