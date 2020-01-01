use std::collections::HashMap;

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
    () => {};
    //////////////////////////////////////////////////////////////////////////
    // The vector implementation.
    //////////////////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
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

    (:$key:tt) => {{
        Edn::Key(std::stringify!($key).into())
    }};

    (#{ }) => {
        Edn::Set(Set::empty())
    };

    ([]) => {
        Edn::Vector(Vector::empty())
    };

    (()) => {
        Edn::List(List::empty())
    };

    ({}) => {
        Edn::Map(Map::empty())
     };

    ([ $($tt:expr)+ ]) => {{
        let mut v = Vec::new();
        $(
            let edn = edn_internal!($tt);
            v.push(edn);
        )+
        Edn::Vector(Vector::new(v))
    }};

    ($e:expr) => {
        match $crate::edn::utils::Attribute::process(&$e) {
            el if el.parse::<i32>().is_ok() => Edn::Int(el.parse::<i128>().unwrap()),
            el if el.parse::<i64>().is_ok() => Edn::Int(el.parse::<i128>().unwrap()),
            el if el.parse::<i128>().is_ok() => Edn::Int(el.parse::<i128>().unwrap()),
            el if el.parse::<u32>().is_ok() => Edn::UInt(el.parse::<u128>().unwrap()),
            el if el.parse::<u64>().is_ok() => Edn::UInt(el.parse::<u128>().unwrap()),
            el if el.parse::<u128>().is_ok() => Edn::UInt(el.parse::<u128>().unwrap()),
            el if el.parse::<f32>().is_ok() => Edn::Double(el.parse::<f64>().unwrap()),
            el if el.parse::<f64>().is_ok() => Edn::Double(el.parse::<f64>().unwrap()),
            el if el.parse::<bool>().is_ok() => Edn::Bool(el.parse::<bool>().unwrap()),
            el => Edn::Str(el)
        }
    };
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