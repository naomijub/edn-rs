#[cfg(feature = "sets")]
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use edn::{Error, List, Vector};
    use edn_rs::{edn, map, set, Edn, Map, Set};

    #[test]
    fn parse_set_with_commas() {
        assert_eq!(
            Edn::from_str("#{true, \\c, 3,four, }").unwrap(),
            Edn::Set(Set::new(set![
                Edn::Symbol("four".to_string()),
                Edn::Bool(true),
                Edn::Char('c'),
                Edn::UInt(3),
            ]))
        );
    }

    #[test]
    fn parse_comment_in_set() {
        assert_eq!(
            Edn::from_str("#{true ; bool true in a set\n \\c 3 }").unwrap(),
            Edn::Set(Set::new(set![
                Edn::Bool(true),
                Edn::Char('c'),
                Edn::UInt(3)
            ]))
        );
    }

    #[test]
    fn parse_true_false_nil_with_comments_in_set() {
        assert_eq!(
            Edn::from_str("#{true;this is true\nfalse;this is false\nnil;this is nil\n}").unwrap(),
            Edn::Set(Set::new(set![Edn::Bool(true), Edn::Bool(false), Edn::Nil,]))
        );
    }

    #[test]
    fn parse_comment_in_set_end() {
        assert_eq!(
            Edn::from_str("#{true \\c 3; int 3 in a set\n}").unwrap(),
            Edn::Set(Set::new(set![
                Edn::Bool(true),
                Edn::Char('c'),
                Edn::UInt(3)
            ]))
        );
    }

    #[test]
    fn parse_complex() {
        assert_eq!(
            Edn::from_str("[:b ( 5 \\c #{true \\c 3 } ) ]").unwrap(),
            Edn::Vector(Vector::new(vec![
                Edn::Key(":b".to_string()),
                Edn::List(List::new(vec![
                    Edn::UInt(5),
                    Edn::Char('c'),
                    Edn::Set(Set::new(set![
                        Edn::Bool(true),
                        Edn::Char('c'),
                        Edn::UInt(3)
                    ]))
                ]))
            ]))
        );
    }

    #[test]
    fn parse_comment_complex() {
        assert_eq!(
            Edn::from_str("[:b ( 5 \\c #{true \\c; char c in a set\n3 } ) ]").unwrap(),
            Edn::Vector(Vector::new(vec![
                Edn::Key(":b".to_string()),
                Edn::List(List::new(vec![
                    Edn::UInt(5),
                    Edn::Char('c'),
                    Edn::Set(Set::new(set![
                        Edn::Bool(true),
                        Edn::Char('c'),
                        Edn::UInt(3)
                    ]))
                ]))
            ]))
        );
    }

    #[test]
    fn edn_element_with_inst() {
        assert_eq!(
            Edn::from_str(
                "#{ :a :b {:c :d :date  #inst \"2020-07-16T21:53:14.628-00:00\" ::c ::d} nil}"
            )
            .unwrap(),
            Edn::Set(Set::new(set! {
                Edn::Key(":a".to_string()),
                Edn::Key(":b".to_string()),
                Edn::Map(Map::new(map! {
                    ":c".to_string() => Edn::Key(":d".to_string()),
                    ":date".to_string() => Edn::Inst("2020-07-16T21:53:14.628-00:00".to_string()),
                    "::c".to_string() => Edn::Key("::d".to_string())
                })),
                Edn::Nil
            }))
        );
    }

    #[test]
    fn parse_discard_space_invalid() {
        assert_eq!(
            Edn::from_str(
                "#_ ,, #{hello, this will be discarded} #_{so will this} #{this is invalid"
            ),
            Err(Error::ParseEdn(
                "None could not be parsed at char count 58".to_string()
            ))
        );
    }

    #[test]
    fn parse_tagged_set() {
        assert_eq!(
            Edn::from_str("#domain/model #{1 2 3}").unwrap(),
            Edn::Tagged(
                String::from("domain/model"),
                Box::new(Edn::Set(Set::new(set![
                    Edn::UInt(1),
                    Edn::UInt(2),
                    Edn::UInt(3)
                ])))
            )
        );
    }
}
