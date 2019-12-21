use regex::Regex;

use super::edn::{EdnNode, EdnType};

struct EdnTuple(String, EdnType);

pub fn tokenize_edn(edn: String) -> Vec<String> {
    let edn1 = edn.replace("(", " ( ");
    let edn2 = edn1.replace(")", " ) ");
    let edn3 = edn2.replace("]", " ] ");
    let edn4 = edn3.replace("[", " [ ");
    let edn5 = edn4.replace("#{", " #{ ");
    let edn6 = edn5.replace("}", " } ");
    let edn7 = edn6.replace("{", "{ ");
    let edn8 = edn7.trim();

    edn8.split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| String::from(*s))
        .collect::<Vec<String>>()
}

pub fn ednify(first: String, tokens: &mut Vec<String>) -> EdnNode {
    let tuple = process_token(first);
    match tuple.1 {
        EdnType::Vector => EdnNode {
            value: tuple.0,
            edntype: EdnType::Vector,
            internal: Some(handle_collection(tokens)),
        },
        EdnType::List => EdnNode {
            value: tuple.0,
            edntype: EdnType::List,
            internal: Some(handle_collection(tokens)),
        },
        EdnType::Set => EdnNode {
            value: tuple.0,
            edntype: EdnType::Set,
            internal: Some(handle_set(tokens)),
        },
        EdnType::Map => {
            if tokens.len() % 2 == 1 {
                return EdnNode {
                    value: tuple.0,
                    edntype: EdnType::Map,
                    internal: Some(handle_collection(tokens)),
                };
            }
            EdnNode {
                value: s("Unbalanced Map"),
                edntype: EdnType::Err,
                internal: None,
            }
        }
        _ => EdnNode {
            value: tuple.0,
            edntype: tuple.1,
            internal: None,
        },
    }
}

fn comma_to_dot(s: String) -> String {
    s.replace(",", ".")
}

fn process_token(first: String) -> EdnTuple {
    let keyword_regex = Regex::new(r":+[a-zA-Z0-9_]+[-[a-zA-Z0-9_]+]*").unwrap();
    let str_regex = Regex::new(r#"".+""#).unwrap();
    let float_regex = Regex::new(r#"\d+,\d+"#).unwrap();
    let rational_regex = Regex::new(r#"\d+/\d+"#).unwrap();


    match &first[..] {
        "[" => EdnTuple(s("["), EdnType::Vector),
        "]" => EdnTuple(s("]"), EdnType::VectorClose),
        "(" => EdnTuple(s("("), EdnType::List),
        ")" => EdnTuple(s(")"), EdnType::ListClose),
        "#{" => EdnTuple(s("#{"), EdnType::Set),
        "{" => EdnTuple(s("{"), EdnType::Map),
        "}" => EdnTuple(s("}"), EdnType::MapSetClose),
        _first if _first.is_empty() => EdnTuple(s("nil"), EdnType::Nil),
        _first if str_regex.is_match(_first) => EdnTuple(s(_first), EdnType::Str),
        _first if keyword_regex.is_match(_first) => EdnTuple(s(_first), EdnType::Key),
        _first if _first.parse::<i64>().is_ok() => EdnTuple(s(_first), EdnType::Int),
        _first if _first.parse::<u64>().is_ok() => EdnTuple(s(_first), EdnType::Int),
        _first if _first.parse::<f64>().is_ok() => EdnTuple(s(_first), EdnType::Double),
        _first if float_regex.is_match(_first) => EdnTuple(comma_to_dot(s(_first)), EdnType::Double),
        _first if rational_regex.is_match(_first) => EdnTuple(comma_to_dot(s(_first)), EdnType::Rational),
        _ => EdnTuple(first, EdnType::Symbol),
    }
}

fn handle_set(tokens: &mut Vec<String>) -> Vec<EdnNode> {
    tokens.sort();
    tokens.dedup();
    handle_collection(tokens)
}

fn handle_collection(tokens: &mut Vec<String>) -> Vec<EdnNode> {
    tokens
        .into_iter()
        .map(|t| t.to_string())
        .map(|t| process_token(t))
        .map(|edn| EdnNode {
            value: edn.0,
            edntype: edn.1,
            internal: None,
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
        let expected = vec![
            String::from("("),
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from(")"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn splits_special_char() {
        let actual = tokenize_edn(String::from("{[(1 2)]}"));
        let expected = vec![
            String::from("{"),
            String::from("["),
            String::from("("),
            String::from("1"),
            String::from("2"),
            String::from(")"),
            String::from("]"),
            String::from("}"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn handle_multiple_tokens() {
        let mut collection = vec![s("1"), s("gasd"), s(":key"), s("2.3"), s("1,3"),
            s("18446744073709551615"), s("3/4")];
        let expected = vec![
            EdnNode {
                value: s("1"),
                edntype: EdnType::Int,
                internal: None,
            },
            EdnNode {
                value: s("gasd"),
                edntype: EdnType::Symbol,
                internal: None,
            },
            EdnNode {
                value: s(":key"),
                edntype: EdnType::Key,
                internal: None,
            },
            EdnNode {
                value: s("2.3"),
                edntype: EdnType::Double,
                internal: None
            },
            EdnNode {
                value: s("1.3"),
                edntype: EdnType::Double,
                internal: None
            },
            EdnNode {
                value: s("18446744073709551615"),
                edntype: EdnType::Int,
                internal: None
            },
            EdnNode {
                value: s("3/4"),
                edntype: EdnType::Rational,
                internal: None
            }
        ];
        assert_eq!(handle_collection(&mut collection), expected);
    }

    #[test]
    fn handle_set_of_tokens() {
        let mut collection = vec![
            s("1"),
            s("gasd"),
            s(":key"),
            s("1"),
            s("1"),
            s(":key"),
            s("\"str\""),
        ];
        let expected = vec![
            EdnNode {
                value: s("\"str\""),
                edntype: EdnType::Str,
                internal: None,
            },
            EdnNode {
                value: s("1"),
                edntype: EdnType::Int,
                internal: None,
            },
            EdnNode {
                value: s(":key"),
                edntype: EdnType::Key,
                internal: None,
            },
            EdnNode {
                value: s("gasd"),
                edntype: EdnType::Symbol,
                internal: None,
            },
        ];
        assert_eq!(handle_set(&mut collection), expected);
    }
}
