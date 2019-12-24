use regex::Regex;

use crate::edn::{EdnNode, EdnType};

struct EdnTuple(String, EdnType);

#[derive(Debug, PartialEq)]
struct Range {
    range_type: EdnType,
    init: usize,
    end: usize,
}

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
    let ranges = get_ranges(tokens.clone().to_owned());
    let  mut u: Vec<String> = if ranges.len() != 0 {
        tokens.drain(ranges[0].init..=ranges[0].end).collect()
    } else {
        Vec::new()
    };

    tokens
        .into_iter()
        .map(|t| t.to_string())
        .map(|t| process_token(t))
        .map(|edn| 
            if edn.1 == EdnType::Vector {
                EdnNode {
                    value: edn.0,
                    edntype: edn.1,
                    internal: Some(handle_collection(&mut u)),
                }
            } else if edn.1 == EdnType::Set {
                EdnNode {
                    value: edn.0,
                    edntype: edn.1,
                    internal: Some(handle_set(&mut u)),
                }
            }else {
                EdnNode {
                    value: edn.0,
                    edntype: edn.1,
                    internal: None,
                }
            }
        )
        .collect::<Vec<EdnNode>>()
}

fn s(s: &str) -> String {
    String::from(s)
}

fn get_ranges(tokens: Vec<String>) -> Vec<Range> {
    let mut no_last_tokens = tokens.clone();
    no_last_tokens.pop();

    let enumerable = enumerable_tokens(no_last_tokens);
    
    let group = group_enumerables(enumerable);
    let open_group = group.clone().iter().filter(|i| i.0 == "[" || i.0 == "#{").map(|i| i.to_owned()).collect::<Vec<(String, Vec<(usize, String)>)>>();
    let close_group = group.clone().iter().filter(|i| i.0 == "]" || i.0 == "}").map(|i| i.to_owned()).collect::<Vec<(String, Vec<(usize, String)>)>>();

    open_group.iter()
        .map(|i| 
            match &i.0[..] {
                "[" => {
                    create_ranges(EdnType::Vector, i.1.clone(), close_group.clone())
                },
                "#{" => {
                    create_ranges(EdnType::Set, i.1.clone(), close_group.clone())
                },
                _ => vec![Range{range_type: EdnType::Err, init: 0usize, end: 0usize}]
            }
        )
        .flatten()
        .collect::<Vec<Range>>()
}

fn create_ranges(edn: EdnType, i: Vec<(usize, String)>, close_group: Vec<(String, Vec<(usize, String)>)>) -> Vec<Range> {
    let mut open = i.iter().map(|idx| idx.0).collect::<Vec<usize>>();
    open.sort();
    open.reverse();

    match edn {
        EdnType::Vector => {
            let matching_close = matching_close(edn, close_group);
            open.iter().zip(matching_close.iter())
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
            let matching_close = matching_close(edn, close_group);
            open.iter().zip(matching_close.iter())
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
        _ => vec![Range{range_type: EdnType::Err, init: 0usize, end: 0usize}],
    }
}

fn matching_close(edn: EdnType, close: Vec<(String, Vec<(usize, String)>)>) -> Vec<usize> {
    match edn {
        EdnType::Vector => close.iter()
            .filter(|i| i.0 == "]")
            .map(|i| i.1.iter()
                .map(|idx| idx.0)
                .collect::<Vec<usize>>())
            .flatten()
            .collect::<Vec<usize>>(),
        EdnType::Set => close.iter()
            .filter(|i| i.0 == "}")
            .map(|i| i.1.iter()
                .map(|idx| idx.0)
                .collect::<Vec<usize>>())
            .flatten()
            .collect::<Vec<usize>>(),
        _ => Vec::new()
    }
}

fn enumerable_tokens(tokens: Vec<String>) -> Vec<(usize, String)> {
    tokens.iter()
        .enumerate()
        .filter(|e| e.1 == "[" || e.1 == "]" || e.1 == "#{" || e.1 == "}")
        .map(|e| (e.0, e.1.to_owned()))
        .collect::<Vec<(usize, String)>>()
}

fn group_enumerables(enumerable: Vec<(usize, String)>) -> Vec<(String, Vec<(usize, String)>)>{
    use itertools::Itertools;

    enumerable.clone()
        .into_iter()
        .group_by(|e| e.1.clone())
        .into_iter()
        .map(|(key, assoc_group)| (key, assoc_group.collect()))
        .collect::<Vec<(String, Vec<(usize, String)>)>>()
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
    fn vector_in_vector_range() {
        let vec = vec![s("["), s("1"), s("3"), s("["), s("4"), s("5"), s("]"), s("]"), s("]")];
        let actual = get_ranges(vec);
        let expected = vec![Range { range_type: EdnType::Vector, init: 4, end: 6 }, Range {range_type: EdnType::Vector, init: 1usize, end: 7usize }];
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_in_set_range() {
        let vec = vec![s("#{"), s("1"), s("3"), s("#{"), s("4"), s("5"), s("}"), s("}"), s("]")];
        let actual = get_ranges(vec);
        let expected = vec![Range { range_type: EdnType::Set, init: 4, end: 6 }, Range {range_type: EdnType::Set, init: 1usize, end: 7usize }];
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