use alloc::boxed::Box;
use alloc::collections::BTreeMap;
#[cfg(feature = "sets")]
use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::iter;
use core::primitive::str;

#[cfg(feature = "sets")]
use crate::edn::Set;
use crate::edn::{Edn, Error, List, Map, Vector};

const DELIMITERS: [char; 8] = [',', ']', '}', ')', ';', '(', '[', '{'];

pub fn parse(edn: &str) -> Result<Edn, Error> {
    let owned = String::from(edn);
    let mut tokens = owned.chars().enumerate();
    (parse_internal(tokens.next(), &mut tokens)?).map_or_else(|| Ok(Edn::Empty), Ok)
}

fn parse_consuming(
    c: Option<(usize, char)>,
    chars: &mut iter::Enumerate<core::str::Chars<'_>>,
) -> Result<Edn, Error> {
    (parse_internal(c, chars)?).map_or_else(|| Ok(Edn::Empty), Ok)
}

fn parse_internal(
    c: Option<(usize, char)>,
    chars: &mut iter::Enumerate<core::str::Chars<'_>>,
) -> Result<Option<Edn>, Error> {
    Ok(match c {
        Some((_, '[')) => Some(read_vec(chars)?),
        Some((_, '(')) => Some(read_list(chars)?),
        Some((_, '#')) => tagged_or_set_or_discard(chars)?,
        Some((_, '{')) => Some(read_map(chars)?),
        Some((_, ';')) => {
            // Consumes the content
            chars.find(|c| c.1 == '\n');
            read_if_not_container_end(chars)?
        }
        Some((_, s)) if s.is_whitespace() || s == ',' => read_if_not_container_end(chars)?,
        None => None,
        edn => Some(edn_element(edn, chars)?),
    })
}

fn edn_element(
    c: Option<(usize, char)>,
    chars: &mut iter::Enumerate<core::str::Chars<'_>>,
) -> Result<Edn, Error> {
    match c {
        Some((_, '\"')) => read_str(chars),
        Some((_, ':')) => Ok(read_key(chars)),
        Some((_, n)) if n.is_numeric() => Ok(read_number(n, chars)?),
        Some((_, n))
            if (n == '-' || n == '+')
                && chars
                    .clone()
                    .peekable()
                    .peek()
                    .is_some_and(|n| n.1.is_numeric()) =>
        {
            Ok(read_number(n, chars)?)
        }
        Some((_, '\\')) => Ok(read_char(chars)?),
        Some((_, b)) if b == 't' || b == 'f' || b == 'n' => Ok(read_bool_or_nil(b, chars)?),
        Some((_, a)) => Ok(read_symbol(a, chars)?),
        None => Err(Error::ParseEdn("Edn could not be parsed".to_string())),
    }
}

fn tagged_or_set_or_discard(
    chars: &mut iter::Enumerate<core::str::Chars<'_>>,
) -> Result<Option<Edn>, Error> {
    match chars.clone().next() {
        Some((_, '{')) => read_set(chars).map(Some),
        Some((_, '_')) => read_discard(chars),
        _ => read_tagged(chars).map(Some),
    }
}

fn read_key(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Edn {
    let key_chars = chars
        .clone()
        .take_while(|c| !c.1.is_whitespace() && !DELIMITERS.contains(&c.1));
    let c_len = key_chars.clone().count();

    let mut key = String::from(":");
    let key_chars = chars.take(c_len).map(|c| c.1).collect::<String>();
    key.push_str(&key_chars);
    Edn::Key(key)
}

fn read_str(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let result = chars.try_fold(
        (false, String::new()),
        |(last_was_escape, mut s), (_, c)| {
            if last_was_escape {
                // Supported escape characters, per https://github.com/edn-format/edn#strings
                match c {
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    'n' => s.push('\n'),
                    '\\' => s.push('\\'),
                    '\"' => s.push('\"'),
                    _ => {
                        return Err(Err(Error::ParseEdn(format!(
                            "Invalid escape sequence \\{c}"
                        ))))
                    }
                };

                Ok((false, s))
            } else if c == '\"' {
                // Unescaped quote means we're done
                Err(Ok(s))
            } else if c == '\\' {
                Ok((true, s))
            } else {
                s.push(c);
                Ok((false, s))
            }
        },
    );

    match result {
        // An Ok means we actually finished parsing *without* seeing the end of the string, so that's
        // an error.
        Ok(_) => Err(Error::ParseEdn("Unterminated string".to_string())),
        Err(Err(e)) => Err(e),
        Err(Ok(string)) => Ok(Edn::Str(string)),
    }
}

fn read_symbol(a: char, chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let c_len = chars
        .clone()
        .enumerate()
        .take_while(|&(i, c)| i <= 200 && !c.1.is_whitespace() && !DELIMITERS.contains(&c.1))
        .count();
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;

    if a.is_whitespace() {
        return Err(Error::ParseEdn(format!(
            "\"{a}\" could not be parsed at char count {i}"
        )));
    }

    let mut symbol = String::from(a);
    let symbol_chars = chars.take(c_len).map(|c| c.1).collect::<String>();
    symbol.push_str(&symbol_chars);
    Ok(Edn::Symbol(symbol))
}

fn read_tagged(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let tag = chars
        .take_while(|c| !c.1.is_whitespace() && c.1 != ',')
        .map(|c| c.1)
        .collect::<String>();

    Ok(Edn::Tagged(
        tag,
        Box::new(parse_consuming(chars.next(), chars)?),
    ))
}

fn read_discard(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Option<Edn>, Error> {
    let _discard_underscore = chars.next();
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    match parse_consuming(chars.next(), chars) {
        Err(e) => Err(e),
        Ok(Edn::Empty) => Err(Error::ParseEdn(format!(
            "Discard sequence must have a following element at char count {i}"
        ))),
        _ => read_if_not_container_end(chars),
    }
}

fn read_number(n: char, chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    let c_len = chars
        .clone()
        .take_while(|(_, c)| !c.is_whitespace() && !DELIMITERS.contains(c))
        .count();
    let (number, radix) = {
        let mut number = String::new();
        // The EDN spec allows for a redundant '+' symbol, we just ignore it.
        if n != '+' {
            number.push(n);
        }
        for (_, c) in chars.take(c_len) {
            number.push(c);
        }
        if number.to_lowercase().starts_with("0x") {
            number.remove(0);
            number.remove(0);
            (number, 16)
        } else if number.to_lowercase().starts_with("-0x") {
            number.remove(1);
            number.remove(1);
            (number, 16)
        } else if let Some(index) = number.to_lowercase().find('r') {
            let negative = number.starts_with('-');
            let radix = {
                if negative {
                    (number[1..index]).parse::<u32>()
                } else {
                    (number[0..index]).parse::<u32>()
                }
            };
            match radix {
                Ok(r) => {
                    // from_str_radix panics if radix is not in the range from 2 to 36
                    if !(2..=36).contains(&r) {
                        return Err(Error::ParseEdn(format!("Radix of {r} is out of bounds")));
                    }

                    if negative {
                        for _ in 0..(index) {
                            number.remove(1);
                        }
                    } else {
                        for _ in 0..=index {
                            number.remove(0);
                        }
                    }
                    (number, r)
                }
                Err(e) => {
                    return Err(Error::ParseEdn(format!(
                        "{e} while trying to parse radix from {number}"
                    )));
                }
            }
        } else {
            (number, 10)
        }
    };

    match number {
        n if (n.contains('E') || n.contains('e')) && n.parse::<f64>().is_ok() => {
            Ok(Edn::Double(n.parse::<f64>()?.into()))
        }
        n if u64::from_str_radix(&n, radix).is_ok() => {
            Ok(Edn::UInt(u64::from_str_radix(&n, radix)?))
        }
        n if i64::from_str_radix(&n, radix).is_ok() => {
            Ok(Edn::Int(i64::from_str_radix(&n, radix)?))
        }
        n if n.parse::<f64>().is_ok() => Ok(Edn::Double(n.parse::<f64>()?.into())),
        n if n.contains('/') && n.split('/').all(|d| d.parse::<f64>().is_ok()) => {
            Ok(Edn::Rational(n))
        }
        n if n.to_uppercase().chars().filter(|c| c == &'E').count() > 1 => {
            let mut n = n.chars();
            read_symbol(n.next().unwrap_or(' '), &mut n.enumerate())
        }
        _ => Err(Error::ParseEdn(format!(
            "{number} could not be parsed at char count {i} with radix {radix}"
        ))),
    }
}

fn read_char(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let element = chars
        .clone()
        .enumerate()
        .take_while(|&(i, c)| i <= 200 && !c.1.is_whitespace())
        .map(|(_, c)| c.1)
        .collect::<String>();

    let mut consume_chars = |n| {
        // We need to map/collect to consume out of the Enumerate
        let _ = chars.take(n).map(|c| c.1).collect::<String>();
    };

    match element {
        _ if element.starts_with("newline") => {
            consume_chars(7);
            Ok(Edn::Char('\n'))
        }
        _ if element.starts_with("return") => {
            consume_chars(6);
            Ok(Edn::Char('\r'))
        }
        _ if element.starts_with("tab") => {
            consume_chars(3);
            Ok(Edn::Char('\t'))
        }
        _ if element.starts_with("space") => {
            consume_chars(5);
            Ok(Edn::Char(' '))
        }
        c if !c.is_empty() => {
            consume_chars(1);
            Ok(Edn::Char(c.chars().next().unwrap()))
        }
        _ => Err(Error::ParseEdn(format!(
            "{element:?} could not be parsed as a symbol"
        ))),
    }
}

fn read_bool_or_nil(
    c: char,
    chars: &mut iter::Enumerate<core::str::Chars<'_>>,
) -> Result<Edn, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    match c {
        't' if {
            let val = chars
                .clone()
                .take_while(|(_, c)| !c.is_whitespace() && !DELIMITERS.contains(c))
                .map(|c| c.1)
                .collect::<String>();
            val.eq("rue")
        } =>
        {
            let mut string = String::new();
            let t = chars.take(3).map(|c| c.1).collect::<String>();
            string.push(c);
            string.push_str(&t);
            Ok(Edn::Bool(string.parse::<bool>()?))
        }
        'f' if {
            let val = chars
                .clone()
                .take_while(|(_, c)| !c.is_whitespace() && !DELIMITERS.contains(c))
                .map(|c| c.1)
                .collect::<String>();
            val.eq("alse")
        } =>
        {
            let mut string = String::new();
            let f = chars.take(4).map(|c| c.1).collect::<String>();
            string.push(c);
            string.push_str(&f);
            Ok(Edn::Bool(string.parse::<bool>()?))
        }
        'n' if {
            let val = chars
                .clone()
                .take_while(|(_, c)| !c.is_whitespace() && !DELIMITERS.contains(c))
                .map(|c| c.1)
                .collect::<String>();
            val.eq("il")
        } =>
        {
            let mut string = String::new();
            let n = chars.take(2).map(|c| c.1).collect::<String>();
            string.push(c);
            string.push_str(&n);
            match &string[..] {
                "nil" => Ok(Edn::Nil),
                _ => Err(Error::ParseEdn(format!(
                    "{string} could not be parsed at char count {i}"
                ))),
            }
        }
        _ => read_symbol(c, chars),
    }
}

fn read_vec(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    let mut res: Vec<Edn> = vec![];
    loop {
        match chars.next() {
            Some((_, ']')) => return Ok(Edn::Vector(Vector::new(res))),
            Some(c) => {
                if let Some(e) = parse_internal(Some(c), chars)? {
                    res.push(e);
                }
            }
            err => {
                return Err(Error::ParseEdn(format!(
                    "{err:?} could not be parsed at char count {i}"
                )))
            }
        }
    }
}

fn read_list(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    let mut res: Vec<Edn> = vec![];
    loop {
        match chars.next() {
            Some((_, ')')) => return Ok(Edn::List(List::new(res))),
            Some(c) => {
                if let Some(e) = parse_internal(Some(c), chars)? {
                    res.push(e);
                }
            }
            err => {
                return Err(Error::ParseEdn(format!(
                    "{err:?} could not be parsed at char count {i}"
                )))
            }
        }
    }
}

#[cfg(feature = "sets")]
fn read_set(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let _discard_brackets = chars.next();
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    let mut res: BTreeSet<Edn> = BTreeSet::new();
    loop {
        match chars.next() {
            Some((_, '}')) => return Ok(Edn::Set(Set::new(res))),
            Some(c) => {
                if let Some(e) = parse_internal(Some(c), chars)? {
                    res.insert(e);
                }
            }
            err => {
                return Err(Error::ParseEdn(format!(
                    "{err:?} could not be parsed at char count {i}"
                )))
            }
        }
    }
}

#[cfg(not(feature = "sets"))]
fn read_set(_chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    Err(Error::ParseEdn(
        "Could not parse set due to feature not being enabled".to_string(),
    ))
}

fn read_map(chars: &mut iter::Enumerate<core::str::Chars<'_>>) -> Result<Edn, Error> {
    let i = chars
        .clone()
        .next()
        .ok_or_else(|| Error::ParseEdn("Could not identify symbol index".to_string()))?
        .0;
    let mut res: BTreeMap<String, Edn> = BTreeMap::new();
    let mut key: Option<Edn> = None;
    let mut val: Option<Edn> = None;
    loop {
        match chars.next() {
            Some((_, '}')) => return Ok(Edn::Map(Map::new(res))),
            Some(c) => {
                if key.is_some() {
                    val = Some(parse_consuming(Some(c), chars)?);
                } else {
                    key = parse_internal(Some(c), chars)?;
                }
            }
            err => {
                return Err(Error::ParseEdn(format!(
                    "{err:?} could not be parsed at char count {i}"
                )))
            }
        }

        if key.is_some() && val.is_some() {
            res.insert(key.unwrap().to_string(), val.unwrap());
            key = None;
            val = None;
        }
    }
}

fn read_if_not_container_end(
    chars: &mut iter::Enumerate<core::str::Chars<'_>>,
) -> Result<Option<Edn>, Error> {
    Ok(match chars.clone().next() {
        Some(c) if c.1 == ']' || c.1 == ')' || c.1 == '}' => None,
        Some(_) => parse_internal(chars.next(), chars)?,
        None => None,
    })
}

#[cfg(test)]
mod test {}
