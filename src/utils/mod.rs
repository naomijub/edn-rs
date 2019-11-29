use regex::Regex;

use super::edn::{EdnNode, EdnType};

struct EdnTuple(String,EdnType);

pub fn tokenize_edn(edn: String) -> Vec<String> {
    let edn1 = edn.replace("(", " ( ");
    let edn2 = edn1.replace(")", " ) ");
    let edn3 = edn2.replace("]", " ] ");
    let edn4 = edn3.replace("[", " [ ");
    let edn5 = edn4.replace("{", " { ");
    let edn6 = edn5.replace("}", " } ");
    let edn7 = edn6.trim();

    edn7.split(' ').collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| String::from(*s))
        .collect::<Vec<String>>()
}

pub fn ednify(first: String, tokens: &mut Vec<String>) -> EdnNode {
    let tuple = process_token(first);
    if tuple.1 == EdnType::Vector  {
        return EdnNode {
            value: s("["),
            edntype: EdnType::Vector,
            internal: Some(handle_collection(tokens))
        };
    }

    EdnNode {
        value: tuple.0,
        edntype: tuple.1,
        internal: None
    }
}

fn process_token(first: String) -> EdnTuple {
    let keyword_regex = Regex::new(r":+[a-zA-Z0-9_]+[-[a-zA-Z0-9_]+]*").unwrap();

    match &first[..] {
        "[" => EdnTuple(s("["), EdnType::Vector),
        "]" => EdnTuple(s("]"), EdnType::VectorClose),
        _first if _first.is_empty() => EdnTuple(s("nil"), EdnType::Nil),
        _first if keyword_regex.is_match(_first) => EdnTuple(s(_first), EdnType::Key),
        _ => EdnTuple(first, EdnType::Int)
    }
}

fn handle_collection(tokens: &mut Vec<String>) -> Vec<EdnNode> {
    tokens.into_iter()
        .map(|t| t.to_string())
        .map(|t| process_token(t))
        .map(|edn| EdnNode {
            value: edn.0,
            edntype: edn.1,
            internal: None
        })
        .collect::<Vec<EdnNode>>()
}

fn s(s: &str) -> String {
    String::from(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_parenthesis() {
        let actual = tokenize_edn(String::from("(1 2 3)"));
        let expected = vec![String::from("("), String::from("1"), String::from("2"), String::from("3"), String::from(")")];
        assert_eq!(actual, expected);
    }

    #[test]
    fn splits_special_char() {
        let actual = tokenize_edn(String::from("{[(1 2)]}"));
        let expected = vec![String::from("{"), String::from("["), String::from("("), String::from("1"), String::from("2"), String::from(")"), String::from("]"), String::from("}")];
        assert_eq!(actual, expected);
    }

}