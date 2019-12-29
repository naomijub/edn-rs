use std::collections::{HashMap};

pub mod utils;

/// `EdnType` is an Enum with possible values for an EDN type
#[derive(Debug, PartialEq, Clone)]
pub enum Edn {
    Vector(Vec<Edn>),
    Set(Vec<Edn>),
    Map(HashMap<String,Edn>),
    List(Vec<Edn>),
    Int(i64),
    Key(String),
    Symbol(String),
    Str(String),
    Double(f64),
    Rational(String),
    Char(char),
    Bool(bool),
    Nil,
}

#[macro_export]
macro_rules! parse_edn {
    // Hide distracting implementation details from the generated rustdoc.
    ($($edn:tt)+) => {
        edn_internal!($($edn)+)
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! edn_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of a vector [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: edn_internal!(@vector [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@vector [$($elems:expr,)*]) => {
        edn_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@vector [$($elems:expr),*]) => {
        edn_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@vector [$($elems:expr,)*] null $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@vector [$($elems:expr,)*] true $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@vector [$($elems:expr,)*] false $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!(false)] $($rest)*)
    };

    // Next element is a vec.
    (@vector [$($elems:expr,)*] [$($vec:tt)*] $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!([$($vec)*])] $($rest)*)
    };

    // Next element is a set.
    (@vector [$($elems:expr,)*] #{$($set:tt)*} $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!(#{$($set)*})] $($rest)*)
    };

    // Next element is a list.
    (@vector [$($elems:expr,)*] ($($list:tt)*) $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!(($($list)*))] $($rest)*)
    };

    // Next element is a map.
    (@vector [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@vector [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)* edn_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@vector [$($elems:expr,)*] $last:expr) => {
        edn_internal!(@vector [$($elems,)* edn_internal!($last)])
    };

    // Comma after the most recent element.
    (@vector [$($elems:expr),*] , $($rest:tt)*) => {
        edn_internal!(@vector [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@vector [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of a set [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: edn_internal!(@set [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@set #{$($elems:expr,)*}) => {
        edn_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@set #{$($elems:expr),*}) => {
        edn_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@set #{$($elems:expr,)*} null $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!(null)} $($rest)*)
    };

    // Next element is `true`.
    (@set #{$($elems:expr,)*} true $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!(true)} $($rest)*)
    };

    // Next element is `false`.
    (@set #{$($elems:expr,)*} false $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!(false)} $($rest)*)
    };

    // Next element is a set.
    (@set #{$($elems:expr,)*} #{$($set:tt)*} $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!(#{$($set)*})} $($rest)*)
    };

     // Next element is a vec.
     (@set #{$($elems:expr,)*} [$($vec:tt)*] $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!([$($vec)*])} $($rest)*)
    };

    // Next element is a list.
    (@set #{$($elems:expr,)*} ($($list:tt)*) $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!(($($list)*))} $($rest)*)
    };

    // Next element is a map.
    (@set #{$($elems:expr,)*} {$($map:tt)*} $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!({$($map)*})} $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@set #{$($elems:expr,)*} $next:expr, $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)* edn_internal!($next),} $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@set #{$($elems:expr,)*} $last:expr) => {
        edn_internal!(@set #{$($elems,)* edn_internal!($last)})
    };

    // Comma after the most recent element.
    (@set #{$($elems:expr),*} , $($rest:tt)*) => {
        edn_internal!(@set #{$($elems,)*} $($rest)*)
    };

    // Unexpected token after most recent element.
    (@set #{$($elems:expr),*} $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of a list [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: edn_internal!(@list [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@list ($($elems:expr,)*)) => {
        edn_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@list ($($elems:expr),*)) => {
        edn_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@list ($($elems:expr,)*) null $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!(null)) $($rest)*)
    };

    // Next element is `true`.
    (@list ($($elems:expr,)*) true $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!(true)) $($rest)*)
    };

    // Next element is `false`.
    (@list ($($elems:expr,)*) false $($rest:tt)*) => {
        edn_internal!(@list [$($elems,)* edn_internal!(false)] $($rest)*)
    };

    // Next element is a vec.
    (@list ($($elems:expr,)*) [$($vec:tt)*] $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!([$($vec)*])) $($rest)*)
    };

    // Next element is a set.
    (@list ($($elems:expr,)*) #{$($set:tt)*} $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!(#{$($set)*})) $($rest)*)
    };

    // Next element is a list.
    (@list ($($elems:expr,)*) ($($list:tt)*) $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!(($($list)*))) $($rest)*)
    };

    // Next element is a map.
    (@list ($($elems:expr,)*) {$($map:tt)*} $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!({$($map)*})) $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@list ($($elems:expr,)*) $next:expr, $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)* edn_internal!($next),) $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@list ($($elems:expr,)*) $last:expr) => {
        edn_internal!(@list ($($elems,)* edn_internal!($last)))
    };

    // Comma after the most recent element.
    (@list ($($elems:expr),*) , $($rest:tt)*) => {
        edn_internal!(@list ($($elems,)*) $($rest)*)
    };

    // Unexpected token after most recent element.
    (@list ($($elems:expr),*) $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an hashmap {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: edn_internal!(@hashmap $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@hashmap $hashmap:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@hashmap $hashmap:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $hashmap.insert(($($key)+).to_string(), $value);
        edn_internal!(@hashmap $hashmap () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@hashmap $hashmap:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@hashmap $hashmap:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $hashmap.insert(($($key)+).to_string(), $value);
    };

    // Next value is `null`.
    (@hashmap $hashmap:ident ($($key:tt)+) (null $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@hashmap $hashmap:ident ($($key:tt)+) (true $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@hashmap $hashmap:ident ($($key:tt)+) (false $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!(false)) $($rest)*);
    };

    // Next value is a vec.
    (@hashmap $hashmap:ident ($($key:tt)+) ([$($vec:tt)*] $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!([$($vec)*])) $($rest)*);
    };

    // Next value is a set.
    (@hashmap $hashmap:ident ($($key:tt)+) (#{$($set:tt)*} $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!(#{$($set)*})) $($rest)*);
    };

    // Next value is a list.
    (@hashmap $hashmap:ident ($($key:tt)+) (($($list:tt)*) $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!(($($list)*))) $($rest)*);
    };

    // Next value is a map.
    (@hashmap $hashmap:ident ($($key:tt)+) ({$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@hashmap $hashmap:ident ($($key:tt)+) ($value:expr , $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@hashmap $hashmap:ident ($($key:tt)+) ($value:expr) $copy:tt) => {
        edn_internal!(@hashmap $hashmap [$($key)+] (edn_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@hashmap $hashmap:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        edn_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@hashmap $hashmap:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        edn_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@hashmap $hashmap:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        edn_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@hashmap $hashmap:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        edn_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@hashmap $hashmap:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@hashmap $hashmap:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $hashmap ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: edn_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        Edn::Nil
    };

    (true) => {
        Edn::Bool(true)
    };

    (false) => {
        Edn::Bool(false)
    };

    (#{}) => {
        Edn::Set(Vec::new())
    };

    (()) => {
        Edn::List(Vec::new())
    };

    ([]) => {
        Edn::Vector(Vec::new())
    };

    ({}) => {
        Edn::hashmap($crate::Map::new())
    };

    ([ $($tt:tt)+ ]) => {
        Edn::Vector(edn_internal!(@vector [] $($tt)+))
    };

    (( $($tt:tt)+ )) => {
        Edn::List(edn_internal!(@list () $($tt)+))
    };

    (#{ $($tt:tt)+ }) => {
        Edn::Set(edn_internal!(@vector #{} $($tt)+))
    };

    ({ $($tt:tt)+ }) => {
        Edn::Map({
            let mut hashmap = HashMap::new();
            edn_internal!(@hashmap hashmap () ($($tt)+) ($($tt)+));
            hashmap
        })
    };

    // Any Serialize type: numbers, strings, symbols, chars, keywords, rationals etc.
    // Must be below every other rule.
    ($other:expr) => {{
        use regex::Regex;
        let keyword_regex = Regex::new(r":+[a-zA-Z0-9_]+[-[a-zA-Z0-9_]+]*").unwrap();
        let str_regex = Regex::new(r#"".+""#).unwrap();
        let float_regex = Regex::new(r#"\d+,\d+"#).unwrap();
        let rational_regex = Regex::new(r#"\d+/\d+"#).unwrap();
        let char_regex = Regex::new(r#"\\."#).unwrap();
        let list_regex = Regex::new(r#"(.*)"#).unwrap();

        match &$other {
            element if element.is_empty() => Edn::Nil,
            element if element.parse::<bool>().is_ok() => Edn::Bool(element.parse::<bool>().unwrap()),
            element if str_regex.is_match(element) => Edn::Str(element.to_string()),
            element if keyword_regex.is_match(element) => Edn::Key(element.to_string()),
            element if char_regex.is_match(element) => Edn::Char(element.chars().last().unwrap()),
            element if element.parse::<i64>().is_ok() => Edn::Int(element.parse::<i64>().unwrap()),
            element if element.parse::<f64>().is_ok() => Edn::Double(element.parse::<f64>().unwrap()),
            element if float_regex.is_match(element) => Edn::Double(comma_to_dot(element.to_string()).parse::<f64>().unwrap()),
            element if rational_regex.is_match(element) => Edn::Rational(element.to_string()),
            element if list_regex.is_match(element) => edn_internal!(($other)),
            _ => Edn::Symbol($other.to_string())
        }
    }};
}

pub fn comma_to_dot(s: String) -> String {
    s.replace(",", ".")
}

// The edn_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! edn_internal_vec {
    ($($content:tt)*) => {{
        let v = Vec::new();
        $(v.push($content);)*
        v
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! edn_unexpected {
    () => {};
}