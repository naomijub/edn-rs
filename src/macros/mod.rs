#[macro_export(local_inner_macros)]
///
/// Macro to parse `EDN` into Rust Spec
#[cfg_attr(not(feature = "sets"), doc = "```ignore")]
/// ```rust
/// #![recursion_limit="512"] // recommended recursion size
///
/// #[macro_use]
/// extern crate edn_rs;
///
/// use edn_rs::edn::{Edn, List, Set, Map};
///
/// fn main() {
///     let list = edn!((1 1.2 3 false :f nil 3/4));
///     let expected = Edn::List(
///             List::new(
///                 vec![
///                     Edn::Int(1),
///                     Edn::Double(1.2.into()),
///                     Edn::Int(3),
///                     Edn::Bool(false),
///                     Edn::Key(":f".to_string()),
///                     Edn::Nil,
///                     Edn::Rational((3, 4))
///                 ]
///             )
///         );
///
///     assert_eq!(list, expected);
///
///     let set = edn!(#{1 1.2 3 false :f nil 3/4});
///     let expected = Edn::Set(
///     Set::new(
///         set!{
///             Edn::Int(1),
///             Edn::Double(1.2.into()),
///             Edn::Int(3),
///             Edn::Bool(false),
///             Edn::Key(":f".to_string()),
///             Edn::Nil,
///             Edn::Rational((3, 4))
///             }
///         )
///     );
///
///     assert_eq!(set, expected);
///     let map = edn!({1.2 false, :b 3/4});
///     let expected = Edn::Map(
///         Map::new(
///             map!{
///                 String::from("1.2") => Edn::Bool(false),
///                 // Note `:b` becomes `b`
///                 String::from(":b") => Edn::Rational((3, 4))
///             }
///         )
///     );
///
///     assert_eq!(map, expected);
/// }
/// ```
///
/// A more complete example:
#[cfg_attr(not(feature = "sets"), doc = "```ignore")]
/// ```rust
/// #[macro_use]
/// extern crate edn_rs;
///
/// use edn_rs::edn::{Edn, List, Vector, Map};
/// fn main() {
///     let expected = Edn::List(
///     List::new(
///         vec![
///             Edn::Int(1),
///             Edn::Double(1.2.into()),
///             Edn::Int(3),
///             Edn::Map(
///                 Map::new( map![
///                     String::from("false") => Edn::Map(
///                         Map::new( map![
///                             String::from(":f") => Edn::Key(String::from(":b"))
///                         ])),
///                     String::from("nil") => Edn::Vector(
///                         Vector::new( vec![
///                             Edn::Rational((3, 4)),
///                             Edn::Int(1i64)
///                         ]))
///             ]))
///         ]
///     )
///     );
///
///     assert_eq!(edn!(( 1 1.2 3 {false {:f :b} nil [3/4 1]})), expected);
/// }
/// ```
///
///  To navigate through `Edn` data you can just use `get` and `get_mut`:
///
/// ```rust
/// #[macro_use]
/// extern crate edn_rs;
/// use edn_rs::edn::{Edn, Map, Vector};
///
/// fn main() {
///     let edn = edn!([ 1 1.2 3 {false :f nil 3/4}]);
///
///     assert_eq!(edn[1], edn!(1.2));
///     assert_eq!(edn[1], Edn::Double(1.2f64.into()));
///     assert_eq!(edn[3]["false"], edn!(:f));
///     assert_eq!(edn[3]["false"], Edn::Key(":f".to_string()));
/// }
/// ```
///
/// Internal implementation is hidden, please look at source.
macro_rules! edn {
    // Hide distracting implementation details from the generated rustdoc.
    ($($edn:tt)+) => {
        edn_internal!($($edn)+)
    };
}

// Changes are fine as long as `edn_internal!` does not call any new helper
// macros and can still be invoked as `edn_internal!($($edn)+)`.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! edn_internal {
    () => {};
    //////////////////////////////////////////////////////////////////////////
    // The seq implementation.
    //////////////////////////////////////////////////////////////////////////

    (@seq @vec [$($elems:expr,)*]) => {
        std::vec![$($elems,)*]
    };

    (@seq @list [$($elems:expr,)*]) => {
        std::vec![$($elems,)*]
    };

    (@seq @set [$($elems:expr,)*]) => {
        set!{$($elems,)*}
    };

    // this matches an even number of things between square brackets
    (@seq @map [$($key:expr, $val:expr,)*]) => {
        map!{$(std::format!("{}", $key) => $val),*}
    };

    // eat commas with no effect
    (@seq @$kind:ident [$($elems:expr,)*] , $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* ] $($rest)*)
    };

    // keyword follows
    (@seq @$kind:ident [$($elems:expr,)*] :$head:tt $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* edn!(:$head) , ] $($rest)*)
    };

    // keyword follows
    (@seq @$kind:ident [$($elems:expr,)*] $num:tt/$den:tt $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* edn!($num/$den) , ] $($rest)*)
    };

    // anything else
    (@seq @$kind:ident [$($elems:expr,)*] $head:tt $($rest:tt)*) => {
        edn_internal!(@seq @$kind [ $($elems,)* edn!($head) , ] $($rest)*)
    };

    // inner
    (@seq @$kind:ident [$($elems:expr,)* $open:ident $($inner_val:tt)* $close:ident] $($rest:tt)*) => {{
        match (&format!("{:?}",$open), &format!("{:?}",$close)) {
            ("#{","}") => edn_internal!(@seq @$kind [ $($elems,)* edn_internal!(@seq @set [$($inner_val)*]) , ] $($rest)*),
            ("(",")") => edn_internal!(@seq @$kind [ $($elems,)* edn_internal!(@seq @list [$($inner_val)*]) , ] $($rest)*),
            ("[","]") => edn_internal!(@seq @$kind [ $($elems,)* edn_internal!(@seq @vec [$($inner_val)*]) , ] $($rest)*),
        }
    }};

    // // set
    // (@seq @$kind:ident [$($elems:expr,)* #{$($set_val:tt)*}] $($rest:tt)*) => {
    //     edn_internal!(@seq @$kind [ $($elems,)* edn!(#{$($set_val)*}) , ] $($rest)*)
    // };

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
        Edn::Rational(($num, $den))
    }};

    (:$key:tt) => {{
        let k = std::format!(":{}", std::stringify!($key));
        Edn::Key(k)
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

     ( ( $($value:tt)* ) ) => {
        Edn::List(List::new(edn_internal!(@seq @list [] $($value)*)))
    };

    ( [ $($value:tt)* ] ) => {
        Edn::Vector(Vector::new(edn_internal!(@seq @vec [] $($value)*)))
    };

    ( #{ $($value:tt)* } ) => {
        Edn::Set(Set::new(edn_internal!(@seq @set [] $($value)*)))
    };

    ( { $($value:tt)* } ) => {
        Edn::Map(Map::new(edn_internal!(@seq @map [] $($value)*)))
    };

    ($($sy:ident)-+) => {{
        let s: String = std::stringify!($($sy)+).into();
        let symbol = s.replace(" ","-");
        Edn::Symbol(symbol)
    }};

    ($e:expr) => {
        match $crate::edn::utils::Attribute::process(&$e) {
            el if el.parse::<i32>().is_ok() => Edn::Int(el.parse::<i64>().unwrap()),
            el if el.parse::<i64>().is_ok() => Edn::Int(el.parse::<i64>().unwrap()),
            el if el.parse::<u32>().is_ok() => Edn::UInt(el.parse::<u64>().unwrap()),
            el if el.parse::<u64>().is_ok() => Edn::UInt(el.parse::<u64>().unwrap()),
            el if el.parse::<f32>().is_ok() => Edn::Double(el.parse::<f64>().unwrap().into()),
            el if el.parse::<f64>().is_ok() => Edn::Double(el.parse::<f64>().unwrap().into()),
            el if el.parse::<bool>().is_ok() => Edn::Bool(el.parse::<bool>().unwrap()),
            el => Edn::Str(el)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! edn_unexpected {
    () => {};
}

/// Creates a `BTreeMap` from a seq of `$key => $value, `
/// `map!{a => "b", c => "d"}`
#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            extern crate alloc;
            let mut m = alloc::collections::BTreeMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

/// Creates a `BTreeSet` from a seq of `$x, `
/// `set!{1, 2, 3, 4}`
#[macro_export]
macro_rules! set {
    ($($x:expr),+ $(,)?) => (
        {
            extern crate alloc;
            let mut s = alloc::collections::BTreeSet::new();
            $(
                s.insert($x);
            )*
            s
        }
    );
}

/// Creates a `HashMap` from a seq of `$key => $value, `
/// `hmap!{a => "b", c => "d"}`
#[macro_export]
#[cfg(feature = "std")]
macro_rules! hmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

/// Creates a `HashSet` from a seq of `$x, `
/// `set!{1, 2, 3, 4}`
#[macro_export]
#[cfg(feature = "std")]
macro_rules! hset {
    ($($x:expr),+ $(,)?) => (
        {
            let mut s = std::collections::HashSet::new();
            $(
                s.insert($x);
            )*
            s
        }
    );
}
