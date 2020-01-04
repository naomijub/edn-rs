
#![recursion_limit="512"]
#[macro_use]
pub mod macros;

#[macro_use] extern crate serde;
#[cfg(feature = "preserve_order")]
extern crate regex;

pub mod edn;
pub mod serialize;

use edn::{utils::{replace_keywords, replace_char}};

/// `json_to_edn` receives a json string and parses its common key-values to a regular EDN format. 
/// tested examples are: 
/// 1. `"{\"hello world\": \"julia\"}"` becomes `"{:hello-world \"julia\"}"`
/// 2. `"{\"hello\": null}"` becomes `"{:hello nil}"`
/// 3. `{\"hello\": 'c'}` becomes `"{:hello \\c}"`
/// 4. `"{\"multi_string with underscore\": 545643}"` becomes `"{:multi-string-with-underscore 545643}"`
/// 
/// ```
/// use edn_rs::json_to_edn;
///
/// fn emits_helloworld_edn() {
///     let json = String::from("{\"hello\": \"world\"}");
///     let edn = String::from("{:hello \"world\"}");
///
///     assert_eq!(edn, json_to_edn(json));
/// }
/// 
/// fn emits_vec_of_map_edn() {
///     let json = String::from("[{\"hello\": \"world\"}, {\"hello\": \"julia\"}, {\"hello\": \"serde\"}");
///     let edn = String::from("[{:hello \"world\"} {:hello \"julia\"} {:hello \"serde\"}]");
///
///     assert_eq!(edn, json_to_edn(json));
/// }
/// ```
pub fn json_to_edn(json: String) -> String {
    let edn_aux = replace_keywords(json);
    let edn = replace_char(edn_aux);
    edn.replace("null","nil")
}