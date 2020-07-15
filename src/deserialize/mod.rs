use crate::edn::{Edn, List, Map, Set, Vector};

/// `parse_edn` parses a EDN String into [`Edn`](../edn/enum.Edn.html)
pub fn parse_edn(edn: &str) -> Result<Edn, String> {
    let tokens = tokenize(edn);

    Ok(parse(&tokens[..])?.0)
}

fn tokenize(edn: &str) -> Vec<String> {
    edn.replace("}", " } ")
        .replace("#{", " @ ")
        .replace("{", " { ")
        .replace("[", " [ ")
        .replace("]", " ] ")
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("\n", " ")
        .replace(",", " ")
        .replace("#inst", "")
        .trim()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| String::from(s))
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    let (token, rest) = tokens
        .split_first()
        .ok_or("Could not get token".to_string())?;

    match &token[..] {
        "[" => read_vec(rest),
        "]" => Err("Unexpected Token `]`".to_string()),
        "(" => read_list(rest),
        ")" => Err("Unexpected Token `)`".to_string()),
        "@" => read_set(rest),
        "{" => read_map(rest),
        "}" => Err("Unexpected Token `}`".to_string()),
        _ => Ok((Edn::parse_word(token.to_string()), rest)),
    }
}

fn read_vec<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    let mut res: Vec<Edn> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `]` for Vector".to_string())?;
        if next_token == "]" {
            return Ok((Edn::Vector(Vector::new(res)), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn read_list<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    let mut res: Vec<Edn> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `)` for List".to_string())?;
        if next_token == ")" {
            return Ok((Edn::List(List::new(res)), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn read_set<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    use std::collections::BTreeSet;
    let mut res: BTreeSet<Edn> = BTreeSet::new();
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `}` for Set".to_string())?;
        if next_token == "}" {
            return Ok((Edn::Set(Set::new(res)), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.insert(exp);
        xs = new_xs;
    }
}

fn read_map<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    use std::collections::BTreeMap;
    let mut res = BTreeMap::new();
    let mut xs = tokens;
    loop {
        let (first_token, rest) = xs
            .split_first()
            .ok_or("Could not find closing `}` for Map".to_string())?;
        if first_token == "}" {
            return Ok((Edn::Map(Map::new(res)), rest));
        }

        let (exp1, new_xs1) = parse(&xs)?;
        let (exp2, new_xs2) = parse(&new_xs1)?;

        res.insert(exp1.to_string(), exp2);
        xs = new_xs2;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::{Map, Set};
    use crate::{map, set};

    #[test]
    fn tokenize_edn() {
        let edn = "[1 \"2\" 3.3 :b [true \\c]]";

        assert_eq!(
            tokenize(edn),
            vec![
                "[".to_string(),
                "1".to_string(),
                "\"2\"".to_string(),
                "3.3".to_string(),
                ":b".to_string(),
                "[".to_string(),
                "true".to_string(),
                "\\c".to_string(),
                "]".to_string(),
                "]".to_string()
            ]
        );
    }

    #[test]
    fn parse_simple_vec() {
        let edn = "[1 \"2\" 3.3 :b true \\c]";

        assert_eq!(
            parse_edn(edn),
            Ok(Edn::Vector(Vector::new(vec![
                Edn::Int(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Bool(true),
                Edn::Char('c')
            ])))
        );
    }

    #[test]
    fn parse_list_with_vec() {
        let edn = "(1 \"2\" 3.3 :b [true \\c])";

        assert_eq!(
            parse_edn(edn),
            Ok(Edn::List(List::new(vec![
                Edn::Int(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Char('c')]))
            ])))
        );
    }

    #[test]
    fn parse_list_with_set() {
        let edn = "(1 \"2\" 3.3 :b #{true \\c})";

        assert_eq!(
            parse_edn(edn),
            Ok(Edn::List(List::new(vec![
                Edn::Int(1),
                Edn::Str("2".to_string()),
                Edn::Double(3.3.into()),
                Edn::Key(":b".to_string()),
                Edn::Set(Set::new(set![Edn::Bool(true), Edn::Char('c')]))
            ])))
        );
    }

    #[test]
    fn parse_simple_map() {
        let edn = "{:a \"2\" :b true :c nil}";

        assert_eq!(
            parse_edn(edn),
            Ok(Edn::Map(Map::new(
                map! {":a".to_string() => Edn::Str("2".to_string()),
                ":b".to_string() => Edn::Bool(true), ":c".to_string() => Edn::Nil}
            )))
        );
    }

    #[test]
    fn parse_complex_map() {
        let edn = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";

        assert_eq!(
            parse_edn(edn),
            Ok(Edn::Map(Map::new(map! {
            ":a".to_string() =>Edn::Str("2".to_string()),
            ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
            ":c".to_string() => Edn::Set(Set::new(
                set!{
                    Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
                    Edn::Key(":A".to_string()),
                    Edn::Nil}))})))
        );
    }
}
