extern crate regex;

pub mod edn;
mod utils;

use edn::EdnNode;

pub fn parse_edn(edn: String) -> EdnNode {
    let mut end_tokens = utils::tokenize_edn(edn);

    if end_tokens.is_empty() {
        return EdnNode::nil();
    }

    utils::ednify(end_tokens.remove(0), &mut end_tokens)
}