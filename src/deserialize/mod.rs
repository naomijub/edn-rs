use crate::edn::{Edn, Vector, Map};
use std::collections::HashMap;

pub fn parse_edn(edn: String) -> Vec<Edn>{
    let edn_token = edn.replace("{", " { ").replace("}", " } ")
    .replace("[", " [ ").replace("]", " ] ")
    .replace("(", " ( ").replace(")", " ) ")
    .replace("\n", " ").replace(",", " ")
    .replace("#inst", "").trim().split(" ")
    .map(|s| s.trim()).filter(|s| !s.is_empty())
    .map(|s| Edn::parse_word(String::from(s)))
    .collect::<Vec<Edn>>();

    let mut node = EdnNode {value: Edn::Empty, child: Vec::new()};
    for e in edn_token.clone().into_iter() {
        match (&node.value, e) {
            (&Edn::Empty, Edn::Vector(_)) => node.value = Edn::Vector(Vector::empty()),
            (&Edn::Empty, Edn::Map(_)) => node.value = Edn::Map(Map::empty()),
            (&Edn::Vector(_), Edn::Vector(_))
            (_n, Edn::Empty) => (),
            (_n, t) => node.child.push(t),
        }
    }

    println!("{:?}", node);

    edn_token
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_vec() {
        let edn = "[1 \"2\" 3.3 :b [true \\c]]";

        assert_eq!(parse_edn(edn.to_string()), vec![Edn::Empty]);
    }
}

#[derive(Debug,Clone)]
struct EdnNode {
    value: Edn,
    child: Vec<EdnNode>

}

fn parse String -> Edn.
String -> Vec<Edb>.
Vec<Edn> -> Tree

BTreeMap<Edn, Edn>