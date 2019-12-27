use regex::Regex;
use std::collections::HashMap;

use crate::edn::{EdnNode, EdnType};

struct EdnTuple(String, EdnType);

/// `Range` is a value defining one item of the vec in `internal` `EdnNode`.
/// It contains the `EdnType` and the range for all the internal values in reverse order of starting.
/// Exemple
/// ```
/// #[test]
/// fn vector_in_vector_range() {
///     let vec = vec![s("["), s("1"), s("3"), s("["), s("4"), s("5"), s("]"), s("]"), s("]")];
///     let actual = get_ranges(vec);
///     let expected = vec![Range { range_type: EdnType::Vector, init: 4, end: 6 }, Range {range_type: EdnType::Vector, init: 1usize, end: 7usize }];
///     assert_eq!(expected, actual);
/// }
/// ``` 
/// 
/// We have the first `s("[")` and its values are `s("1"), s("3"), s("["),`, which will start a new range containing `s("4"), s("5"), s("]")`. 
/// This is why there is an extra `s("]")`
#[derive(Debug, PartialEq)]
struct Range {
    range_type: EdnType,
    init: usize,
    end: usize,
}

pub fn tokenize_edn(edn: String) -> Vec<String> {
    let edn0 = edn.replace("'(", "(");
    let edn1 = edn0.replace("(", " ( ");
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
    let mut ranges = get_ranges(tokens.clone().to_owned());

    match tuple.1 {
        EdnType::Vector => EdnNode {
            value: tuple.0,
            edntype: EdnType::Vector,
            internal: Some(handle_collection(tokens, &mut ranges, false)),
        },
        EdnType::List => EdnNode {
            value: tuple.0,
            edntype: EdnType::List,
            internal: Some(handle_collection(tokens, &mut ranges, false)),
        },
        EdnType::Set => EdnNode {
            value: tuple.0,
            edntype: EdnType::Set,
            internal: Some(handle_collection(tokens, &mut ranges, false)),
        },
        EdnType::Map => {
            EdnNode {
                value: tuple.0,
                edntype: EdnType::Map,
                internal: Some(handle_collection(tokens, &mut ranges, false)),
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
    let char_regex = Regex::new(r#"\\."#).unwrap();

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
        _first if char_regex.is_match(_first) => EdnTuple(s(_first), EdnType::Char),
        _first if _first.parse::<i64>().is_ok() => EdnTuple(s(_first), EdnType::Int),
        _first if _first.parse::<u64>().is_ok() => EdnTuple(s(_first), EdnType::Int),
        _first if _first.parse::<f64>().is_ok() => EdnTuple(s(_first), EdnType::Double),
        _first if float_regex.is_match(_first) => EdnTuple(comma_to_dot(s(_first)), EdnType::Double),
        _first if rational_regex.is_match(_first) => EdnTuple(comma_to_dot(s(_first)), EdnType::Rational),
        _ => EdnTuple(first, EdnType::Symbol),
    }
}


fn handle_collection(tokens: &mut Vec<String>, ranges: &mut Vec<Range>, inner: bool) -> Vec<EdnNode> {
    ranges.sort_by(|a, b| a.init.partial_cmp(&b.init).unwrap());

    let init_len = tokens.clone().len();    
    let mut counter = 0usize;
    let mut u = if ranges.len() != 0 && !inner {
        ranges.iter()
            .map(|i| (i.init, i.end))
            .map(|r| {
                let fix_idx = init_len - tokens.len();
                let a = tokens.drain(r.0.clone() - fix_idx..=r.1.clone() - fix_idx).collect::<Vec<String>>();
                a
            })
            .fold(Vec::new(),|mut acc, i| {acc.push(i); acc})
    } else {
        Vec::new()
    };

    tokens
        .into_iter()
        .map(|t| t.to_string())
        .map(|t| process_token(t))
        .map(|edn| 
            if edn.1 == EdnType::Vector || edn.1 == EdnType::Set || edn.1 == EdnType::List || edn.1 == EdnType::Map {
                let node = EdnNode {
                    value: edn.0,
                    edntype: edn.1,
                    internal: Some(handle_collection(&mut u[counter], ranges, true)),
                };
                counter += 1;
                node
            } else {
                EdnNode {
                    value: edn.0,
                    edntype: edn.1,
                    internal: None,
                }
            }
        )
        .collect::<Vec<EdnNode>>()
    // vec![EdnNode::nil()]
}

fn s(s: &str) -> String {
    String::from(s)
}

fn get_ranges(tokens: Vec<String>) -> Vec<Range> {
    let mut ranges = Vec::new();
    let mut no_last_tokens = tokens.clone();
    no_last_tokens.pop();

    let enumerable = enumerable_tokens(no_last_tokens);
    let group = group_enumerables(enumerable);

    if let Some(k) = group.get("[") {
        let mut open = k.to_owned();
        open.sort();
        let close = group.get("]").unwrap().to_owned();

        ranges.append(&mut create_ranges(EdnType::Vector, open, close));
    }

    if let Some(k) = group.get("(") {
        let mut open = k.to_owned();
        open.sort();
        let close = group.get(")").unwrap().to_owned();

        ranges.append(&mut create_ranges(EdnType::List, open, close));
    }

    if let Some(k) = group.get("#{") {
        let mut open = k.to_owned();
        open.sort();
        let close = group.get("}").unwrap().to_owned();

        ranges.append(&mut create_ranges(EdnType::Set, open, close));
    }

    if let Some(k) = group.get("{") {
        let mut open = k.to_owned();
        open.sort();
        let close = group.get("}").unwrap().to_owned();

        ranges.append(&mut create_ranges(EdnType::Map, open, close));
    }

    ranges
}

fn create_ranges(edn: EdnType, open: Vec<usize>, close: Vec<usize>) -> Vec<Range> {
    match edn {
        EdnType::Vector => {
            open.iter().zip(close.iter())
                .collect::<Vec<(&usize, &usize)>>()
                .iter()
                .map(|idx| 
                    Range {
                        range_type: EdnType::Vector,
                        init: idx.0.to_owned() + 1,
                        end: idx.1.to_owned(),
                    }
                ).collect::<Vec<Range>>()
        },
        EdnType::Set => {
            open.iter().zip(close.iter())
                .collect::<Vec<(&usize, &usize)>>()
                .iter()
                .map(|idx| 
                    Range {
                        range_type: EdnType::Set,
                        init: idx.0.to_owned() + 1,
                        end: idx.1.to_owned(),
                    }
                ).collect::<Vec<Range>>()
        },
        EdnType::Map => {
            open.iter().zip(close.iter())
                .collect::<Vec<(&usize, &usize)>>()
                .iter()
                .map(|idx| 
                    Range {
                        range_type: EdnType::Map,
                        init: idx.0.to_owned() + 1,
                        end: idx.1.to_owned(),
                    }
                ).collect::<Vec<Range>>()
        },
        EdnType::List => {
            open.iter().zip(close.iter())
                .collect::<Vec<(&usize, &usize)>>()
                .iter()
                .map(|idx| 
                    Range {
                        range_type: EdnType::List,
                        init: idx.0.to_owned() + 1,
                        end: idx.1.to_owned(),
                    }
                ).collect::<Vec<Range>>()
        },
        _ => vec![Range{range_type: EdnType::Err, init: 0usize, end: 0usize}],
    }
}

fn enumerable_tokens(tokens: Vec<String>) -> Vec<(usize, String)> {
    tokens.iter()
        .enumerate()
        .filter(|e| e.1 == "[" || e.1 == "]" || e.1 == "{" || e.1 == "#{" || e.1 == "}" || e.1 == "(" || e.1 == ")")
        .map(|e| (e.0, e.1.to_owned()))
        .collect::<Vec<(usize, String)>>()
}

fn group_enumerables(enumerable: Vec<(usize, String)>) -> HashMap<String, Vec<usize>>{
    let map: HashMap<String,Vec<usize>> = HashMap::new();

    let groups = enumerable.clone()
        .into_iter()
        .fold(map,|mut acc, i|
        {
            let mut v = Vec::new();
            if let Some(k) = acc.get_mut(&i.1) {
                k.push(i.0);
                acc
            } else {
                acc.insert(i.1, {v.push(i.0); v});
                acc
            }
        }
    );
    groups
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
            s("18446744073709551615"), s("3/4"), s("\\c")];
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
            },
            EdnNode {
                value: s("\\c"),
                edntype: EdnType::Char,
                internal: None
            }
        ];
        let mut ranges: Vec<Range> = Vec::new();
        assert_eq!(handle_collection(&mut collection, &mut ranges, false), expected);
    }
}

#[cfg(test)]
mod ranges_tests {
    use super::{get_ranges, Range, s};
    use crate::edn::{EdnType};

    #[test]
    fn one_vector_range() {
        let vec = vec![s("["), s("1"), s("3"), s("]"), s("]")];
        let actual = get_ranges(vec);
        let expected = vec![Range {range_type: EdnType::Vector, init: 1usize, end: 3usize }];
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_in_vector_range() {
        let vec = vec![s("["), s("1"), s("3"), s("#{"), s("4"), s("5"), s("}"), s("]"), s("]")];
        let actual = get_ranges(vec);
        let expected = vec![Range {range_type: EdnType::Vector, init: 1usize, end: 7usize }, Range { range_type: EdnType::Set, init: 4, end: 6 }];
        assert_eq!(expected, actual);
    }
}