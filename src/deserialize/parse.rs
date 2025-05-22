use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::primitive::str;
use std::println;
use edn_parser::{edn_parse, Cst, Node, NodeRef, Rule, Token};

use crate::edn::Set;
use crate::edn::{Edn, Error, List, Map, Vector};

pub fn parse(edn: &str) -> Result<Edn, Error> {
    let parsed = edn_parse(edn)?.cst;
    let Node::Rule(Rule::Edn, _) = parsed.get(NodeRef::ROOT) else {
        return Err(Error::InvalidEdn);
    };
    // println!("CST: {parsed:#?}");
    let Some(first_node_ref) = parsed.children(NodeRef::ROOT).find(|node_ref| {
        !matches!(
            parsed.get(*node_ref),
            Node::Token(Token::Whitespace | Token::Comment | Token::Newline, _)
        )
    }) else {
        return Err(Error::InvalidEdn);
    };

    parse_rule(&parsed, first_node_ref, edn)
}

fn parse_rule(cst: &Cst<'_>, node_ref: NodeRef, source: &str) -> Result<Edn, Error> {
    match cst.get(node_ref) {
        Node::Rule(Rule::Literal, ..) => {
            parse_token(cst, cst.children(node_ref).next().unwrap(), source)
        }
        Node::Rule(Rule::Uuid | Rule::Inst, ..) => {
            parse_token(cst, cst.children(node_ref).nth(2).unwrap(), source)
        }
        Node::Rule(Rule::Composed, ..) => {
            parse_rule(cst, cst.children(node_ref).next().unwrap(), source)
        }
        Node::Rule(Rule::Keyword, ..) => {
            parse_keyword(cst, cst.children(node_ref).nth(1).unwrap(), source)
        }
        Node::Rule(Rule::Symbol, ..) => {
            parse_symbol(cst, cst.children(node_ref).next().unwrap(), source)
        }
        Node::Rule(Rule::List, ..) => {
            let mut list = Vec::new();
            for sub_node_ref in cst.children(node_ref) {
                let edn = parse_rule(cst, sub_node_ref, source)?;
                if edn != Edn::Empty {
                    list.push(edn);
                }
            }

            Ok(Edn::List(List::new(list)))
        }
        Node::Rule(Rule::Vector, ..) => {
            let mut vector = Vec::new();
            for sub_node_ref in cst.children(node_ref) {
                let edn = parse_rule(cst, sub_node_ref, source)?;
                if edn != Edn::Empty {
                    vector.push(edn);
                }
            }

            Ok(Edn::Vector(Vector::new(vector)))
        }
        Node::Rule(Rule::Set, ..) => {
            let mut set = BTreeSet::new();
            for sub_node_ref in cst.children(node_ref) {
                let edn = parse_rule(cst, sub_node_ref, source)?;
                if edn != Edn::Empty {
                    set.insert(edn);
                }
            }

            Ok(Edn::Set(Set::new(set)))
        }
        Node::Rule(Rule::Member, ..) => {
            let mut vector = Vec::new();
            for sub_node_ref in cst.children(node_ref) {
                let edn = parse_rule(cst, sub_node_ref, source)?;
                if edn != Edn::Empty {
                    vector.push(edn);
                }
            }

            Ok(Edn::Vector(Vector::new(vector)))
        }
        Node::Rule(Rule::Map, ..) => {
            let mut map = BTreeMap::new();
            for sub_node_ref in cst.children(node_ref) {
                let member = parse_rule(cst, sub_node_ref, source)?;
                if let Edn::Vector(Vector(edn)) = member {
                    if edn.len() != 2 {
                        return Err(Error::IncompleteEdnMap);
                    }
                    let key = edn[0].to_string();
                    let value = edn[1].clone();
                    map.insert(key.to_string(), value);
                }
            }

            Ok(Edn::Map(Map::new(map)))
        }
        // Skip collections tokens
        #[allow(clippy::match_same_arms)]
        Node::Token(
            Token::LSetBrace
            | Token::LParen
            | Token::LBrace
            | Token::LBrak
            | Token::RParen
            | Token::RBrace
            | Token::RBrak,
            ..,
        ) => Ok(Edn::Empty),
        // Skip whitespace tokens
        Node::Token(Token::Whitespace | Token::Comma | Token::Newline | Token::Comment | Token::Discard, ..) => {
            Ok(Edn::Empty)
        }
        x => unimplemented!("{x:?}"),
    }
}

fn parse_token(cst: &Cst<'_>, node_ref: NodeRef, source: &str) -> Result<Edn, Error> {
    match cst.get(node_ref) {
        Node::Rule(..) => Err(Error::Infallable()),
        Node::Token(Token::Null, ..) => Ok(Edn::Nil),
        Node::Token(Token::False, ..) => Ok(Edn::Bool(false)),
        Node::Token(Token::True, ..) => Ok(Edn::Bool(true)),
        Node::Token(Token::Chars, ..) => {
            let span = cst.span(node_ref);
            let text = match &source[span.start+1..span.end] {
                "newline" => "\n",
                "return" => "\r",
                "space" => " ",
                "tab" => "\t",
                c => c,
            };

            Ok(Edn::Char(text.parse()?))
        }
        Node::Token(Token::String, ..) => {
            let span = cst.span(node_ref);
            let text = source[span.start + 1..span.end - 1].to_string();
            Ok(Edn::Str(text))
        }
        Node::Token(Token::Number, ..) => {
            let span = cst.span(node_ref);
            let text = source[span].to_string();
            if text.contains('.') {
                Ok(Edn::Double(text.parse::<f64>()?.into()))
            } else if text.starts_with('-') {
                Ok(Edn::Int(text.parse::<i64>()?))
            } else {
                Ok(Edn::UInt(text.parse::<u64>()?))
            }
        }
        Node::Token(Token::Timestamp, ..) => {
            let span = cst.span(node_ref);
            let text = source[span.start + 1..span.end - 1].to_string();
            Ok(Edn::Tagged("inst".to_string(), Box::new(Edn::Str(text))))
        }
        Node::Token(Token::Id, ..) => {
            let span = cst.span(node_ref);
            let text = source[span.start + 1..span.end - 1].to_string();
            Ok(Edn::Tagged("uuid".to_string(), Box::new(Edn::Str(text))))
        }
        Node::Token(Token::Rational, ..) => {
            let span = cst.span(node_ref);
            let text = source[span.start..span.end].to_string();
            Ok(Edn::Rational(text))
        }
        Node::Token(token, ..) => unimplemented!("{token:?}"),
    }
}

fn parse_keyword(cst: &Cst<'_>, node_ref: NodeRef, source: &str) -> Result<Edn, Error> {
    match cst.get(node_ref) {
        Node::Rule(..) => Err(Error::Infallable()),
        Node::Token(Token::Name, ..) => {
            let span = cst.span(node_ref);
            let text = &source[span];
            let keyword = ":".to_string() + text;
            Ok(Edn::Key(keyword))
        }
        Node::Token(token, ..) => Err(Error::ParseEdn(format!(
            "Unexpected token `{token:?}` is not a keyword"
        ))),
    }
}

fn parse_symbol(cst: &Cst<'_>, node_ref: NodeRef, source: &str) -> Result<Edn, Error> {
    match cst.get(node_ref) {
        Node::Rule(..) => Err(Error::Infallable()),
        Node::Token(Token::Name, ..) => {
            let span = cst.span(node_ref);
            let text = source[span].to_string();

            Ok(Edn::Symbol(text))
        }
        Node::Token(token, ..) => Err(Error::ParseEdn(format!(
            "Unexpected token `{token:?}` is not a symbol"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn parse_nil() {
        assert_eq!(parse("nil").unwrap(), Edn::Nil);
    }

    #[test]
    fn parse_discard() {
        assert_eq!(
            parse("[nil #_nil nil]").unwrap(), 
            Edn::Vector(Vector::new(vec![Edn::Nil, Edn::Nil]))
        );
    }

    #[test]
    fn parse_booleans() {
        assert_eq!(parse("true").unwrap(), Edn::Bool(true));
        assert_eq!(parse("false").unwrap(), Edn::Bool(false));
    }

    #[test]
    fn parse_string() {
        assert_eq!(parse("\"string\"").unwrap(), Edn::Str("string".to_string()));
    }

     #[test]
    fn parse_rational() {
        assert_eq!(parse("42/3").unwrap(), Edn::Rational("42/3".to_string()));
    }

    #[test]
    fn parse_char() {
        assert_eq!(parse("\\c").unwrap(), Edn::Char('c'));
        assert_eq!(parse("\\newline").unwrap(), Edn::Char('\n'));
        assert_eq!(parse("\\tab").unwrap(), Edn::Char('\t'));
        assert_eq!(parse("\\return").unwrap(), Edn::Char('\r'));
        assert_eq!(parse("\\space").unwrap(), Edn::Char(' '));
    }

    #[test]
    fn parse_tagged() {
        assert_eq!(
            parse("#uuid \"4877284c-1661-4efe-be83-57d9366700a8\"").unwrap(),
            Edn::Tagged(
                "uuid".to_string(),
                Box::new(Edn::Str("4877284c-1661-4efe-be83-57d9366700a8".to_string()))
            )
        );
        assert_eq!(
            parse("#inst \"1985-04-12T23:20:50.52Z\"").unwrap(),
            Edn::Tagged(
                "inst".to_string(),
                Box::new(Edn::Str("1985-04-12T23:20:50.52Z".to_string()))
            )
        );
    }

    #[test]
    fn parse_double() {
        assert_eq!(parse("42.3").unwrap(), Edn::Double(42.3f64.into()));
        assert_eq!(parse("-42.3").unwrap(), Edn::Double((-42.3f64).into()));
        assert_eq!(parse("423").unwrap(), Edn::UInt(423));
        assert_eq!(parse("-423").unwrap(), Edn::Int(-423));
    }

    #[test]
    fn parse_keyword() {
        assert_eq!(parse(":keyword").unwrap(), Edn::Key(":keyword".to_string()));
        assert_eq!(parse(":k").unwrap(), Edn::Key(":k".to_string()));
        assert_eq!(
            parse(":key-word_with_underscore").unwrap(),
            Edn::Key(":key-word_with_underscore".to_string())
        );
        assert_eq!(parse(":.").unwrap(), Edn::Key(":.".to_string()));
        assert_eq!(parse(":ns/k").unwrap(), Edn::Key(":ns/k".to_string()));
    }

    #[test]
    fn parse_symbols() {
        assert_eq!(parse("symbol").unwrap(), Edn::Symbol("symbol".to_string()));
        assert_eq!(
            parse("this_is-a_symbol").unwrap(),
            Edn::Symbol("this_is-a_symbol".to_string())
        );
    }

    #[test]
    fn parse_comments_are_skipped() {
        assert_eq!(
            parse(
                "
        ; this is a number 123
        :keyword
        ; this is a comment"
            )
            .unwrap(),
            Edn::Key(":keyword".to_string())
        );
    }

    #[test]
    fn parse_list() {
        assert_eq!(
            parse("(nil :value \n true, 1)").unwrap(),
            Edn::List(List::new(vec![
                Edn::Nil,
                Edn::Key(":value".to_string()),
                Edn::Bool(true),
                Edn::UInt(1),
            ]))
        );
    }

    #[test]
    fn parse_vector() {
        assert_eq!(
            parse("[nil :value \n (true, 1)]").unwrap(),
            Edn::Vector(Vector::new(vec![
                Edn::Nil,
                Edn::Key(":value".to_string()),
                Edn::List(List::new(vec![Edn::Bool(true), Edn::UInt(1),])),
            ]))
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            parse("#{nil :value \n [true, 1]}").unwrap(),
            Edn::Set(Set::new(
                vec![
                    Edn::Nil,
                    Edn::Key(":value".to_string()),
                    Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::UInt(1),])),
                ]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn parse_map() {
        assert_eq!(
            parse("{nil :value :key 1}").unwrap(),
            Edn::Map(Map::new(
                [
                    ("nil".to_string(), Edn::Key(":value".to_string())),
                    (":key".to_string(), Edn::UInt(1)),
                ]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn parse_tagged_collections() {
        assert_eq!(
            parse("#ns/edn [1, 2, 3]").unwrap(),
            Edn::Tagged(
                "ns/edn".to_string(),
                Box::new(Edn::Vector(Vector::new(vec![Edn::UInt(1), Edn::UInt(2), Edn::UInt(3)])))
            )
        );
    }
}
