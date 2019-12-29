use std::collections::HashMap;
use crate::edn::{Edn, to_edn};

#[macro_export(local_inner_macros)]
macro_rules! edn {
    // Hide distracting implementation details from the generated rustdoc.
    ($($edn:tt)+) => {
        edn_internal!($($edn)+)
    };
}

// Rocket relies on this because they export their own `edn!` with a different
// doc comment than ours, and various Rust bugs prevent them from calling our
// `edn!` from their `edn!` so they call `edn_internal!` directly. Check with
// @SergioBenitez before making breaking changes to this macro.
//
// Changes are fine as long as `edn_internal!` does not call any new helper
// macros and can still be invoked as `edn_internal!($($edn)+)`.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! edn_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: edn_internal!(@vector [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@vector [$($elems:expr)*]) => {
        edn_internal_vec![$($elems)*]
    };

    // Next element is `null`.
    (@vector [$($elems:expr)*] null $($rest:tt)*) => {
        edn_internal!(@vector [$($elems)* edn_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@vector [$($elems:expr)*] true $($rest:tt)*) => {
        edn_internal!(@vector [$($elems)* edn_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@vector [$($elems:expr)*] false $($rest:tt)*) => {
        edn_internal!(@vector [$($elems)* edn_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@vector [$($elems:expr)*] [$($array:tt)*] $($rest:tt)*) => {
        edn_internal!(@vector [$($elems)* edn_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@vector [$($elems:expr)*] {$($map:tt)*} $($rest:tt)*) => {
        edn_internal!(@vector [$($elems)* edn_internal!({$($map)*})] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@vector [$($elems:expr)*] $last:expr) => {
        edn_internal!(@vector [$($elems)* edn_internal!($last)])
    };

    // Comma after the most recent element.
    (@vector [$($elems:expr)*]  $($rest:tt)*) => {
        edn_internal!(@vector [$($elems)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@vector [$($elems:expr)*] $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: edn_internal!(@hashmap $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@hashmap $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@hashmap $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        edn_internal!(@hashmap $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@hashmap $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        edn_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@hashmap $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@hashmap $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@hashmap $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@hashmap $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@hashmap $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@hashmap $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@hashmap $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@hashmap $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        edn_internal!(@hashmap $object [$($key)+] (edn_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@hashmap $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        edn_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@hashmap $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        edn_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@hashmap $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        edn_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@hashmap $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        edn_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@hashmap $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@hashmap $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        edn_internal!(@hashmap $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: edn_internal!($($edn)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        Edn::Nil
    };

    (nil) => {
        Edn::Nil
    };

    (true) => {
        Edn::Bool(true)
    };

    (false) => {
        Edn::Bool(false)
    };

    ($num:tt/$den:tt) => {{
        let q = std::format!("{:?}/{:?}", $num, $den);
        Edn::Rational(q)
    }};

    ([]) => {
        Edn::Vector(Vector::empty())
    };

    (()) => {
        Edn::List(List::empty())
    };

    (#{}) => {
        Edn::Set(Set::empty())
    };

    ({}) => {
        Edn::Map(Map::empty())
     };

    ([ $($tt:tt)+ ]) => {{
        Edn::Vector(Vector::new(edn_internal!(@vector [] $($tt)+)))
    }};

    (( $($tt:tt)+ )) => {{
        let mut v = Vec::new();
        $(
            let value = edn_internal!($tt);
            v.push(value);
        )+
        Edn::List(List::new(v))
    }};

    ({ $($tt:tt)+ }) => {
        Map({
            let mut object = HashMap::new();
            edn_internal!(@hashmap object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:tt) => {{
        let s = std::format!("{:?}", $other);
        $crate::edn::to_edn(s)
    }};
}

// The edn_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! edn_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! edn_unexpected {
    () => {};
}