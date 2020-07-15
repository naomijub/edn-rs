use crate::edn::{Edn, Vector, Set, Map, List};

enum Type {
    Any,
    Map,
    Set
}

pub fn parse_edn(edn: String) -> Result<Edn, String> {
    let tokens = tokenize(edn);

    Ok(parse(&tokens[..], Type::Any)?.0)
}

fn tokenize(edn: String) -> Vec<String> {
    edn.replace("}", " } ").replace("#{", " @ ").replace("{", " { ")
    .replace("[", " [ ").replace("]", " ] ")
    .replace("(", " ( ").replace(")", " ) ")
    .replace("\n", " ").replace(",", " ")
    .replace("#inst", "").trim().split(" ")
    .map(|s| s.trim()).filter(|s| !s.is_empty())
    .map(|s| String::from(s))
    .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    let (token, rest) = tokens.split_first()
      .ok_or(
        "could not get token".to_string()
      )?;
    println!("token={:?}, rest={:?}", token, rest);
    match &token[..] {
      "[" => read_vec(rest),
      "]" => Err("unexpected `]`".to_string()),
      "(" => read_list(rest),
      ")" => Err("unexpected `)`".to_string()),
      "@" => read_set(rest),
    //   "{" => read_map(rest),
      "}" => Err("unexpected `}`".to_string()),
      _ => Ok((Edn::parse_word(token.to_string()), rest)),
    }
  }

fn read_vec<'a>(tokens: &'a [String]) -> Result<(Edn, &'a [String]), String> {
    let mut res: Vec<Edn> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or("could not find closing `]`".to_string())?;
        if next_token == "]" {
            return Ok((Edn::Vector(Vector::new(res)), rest))
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
            .ok_or("could not find closing `)`".to_string())?;
        if next_token == ")" {
            return Ok((Edn::List(List::new(res)), rest))
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
            .ok_or("could not find closing `}`".to_string())?;
        if next_token == "}" {
            return Ok((Edn::Set(Set::new(res)), rest))
        }
        let (exp, new_xs) = parse(&xs)?;
        res.insert(exp);
        xs = new_xs;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::edn::Set;
    use crate::set;
    use std::collections::BTreeSet;

    #[test]
    fn tokenize_edn() {
        let edn = "[1 \"2\" 3.3 :b [true \\c]]";

        assert_eq!(tokenize(edn.to_string()), vec!["[".to_string(), "1".to_string(), "\"2\"".to_string(), "3.3".to_string(), 
            ":b".to_string(), "[".to_string(), "true".to_string(), "\\c".to_string(), "]".to_string(), "]".to_string()]);
    }

    #[test]
    fn parse_simple_vec() {
        let edn = "[1 \"2\" 3.3 :b true \\c]";

        assert_eq!(parse_edn(edn.to_string()), Ok(
            Edn::Vector(Vector::new(vec![Edn::Int(1), Edn::Str("2".to_string()), Edn::Double(3.3.into()), Edn::Key(":b".to_string()), 
            Edn::Bool(true), Edn::Char('c')]))));
    }

    #[test]
    fn parse_list_with_vec() {
        let edn = "(1 \"2\" 3.3 :b [true \\c])";

        assert_eq!(parse_edn(edn.to_string()), Ok(
            Edn::List(List::new(vec![Edn::Int(1), Edn::Str("2".to_string()), Edn::Double(3.3.into()), Edn::Key(":b".to_string()), 
            Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Char('c')]))]))));
    }

    #[test]
    fn parse_list_with_set() {
        let edn = "(1 \"2\" 3.3 :b #{true \\c})";

        assert_eq!(parse_edn(edn.to_string()), Ok(
            Edn::List(List::new(vec![Edn::Int(1), Edn::Str("2".to_string()), Edn::Double(3.3.into()), Edn::Key(":b".to_string()), 
            Edn::Set(Set::new(set![Edn::Bool(true), Edn::Char('c')]))]))));
    }

    // #[test]
    // fn parse_simple_map() {
    //     let edn = "{:a \"2\" :b true :c nil}";

    //     assert_eq!(parse_edn(edn.to_string()), Ok(
    //         Edn::Vector(Vector::new(vec![Edn::Int(1), Edn::Str("2".to_string()), Edn::Double(3.3.into()), Edn::Key(":b".to_string()), 
    //         Edn::Bool(true), Edn::Char('c')]))));
    // }
}