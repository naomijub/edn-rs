use alloc::collections::{BTreeMap, BTreeSet, LinkedList};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Trait that allows you to implement Serialization for each type of your choice.
/// Example:
/// ```rust
/// use edn_rs::serialize::Serialize;
///
/// #[derive(Debug)]
/// struct YourType;
///
/// impl Serialize for YourType {
///     fn serialize(&self) -> String {
///         format!("{:?}", self)
///     }
/// }
/// ```
///
/// Implemented for all generic types.
pub trait Serialize {
    fn serialize(&self) -> String;
}

macro_rules! ser_primitives {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for $name
            {
                fn serialize(&self) -> String {
                    format!("{:?}", self)
                }
            }
        )+
    };
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(Serialize::serialize)
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('[');
        s.push_str(&aux_vec.join(", "));
        s.push(']');
        s
    }
}

#[cfg(feature = "std")]
impl<T, H: std::hash::BuildHasher> Serialize for std::collections::HashSet<T, H>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(Serialize::serialize)
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('#');
        s.push('{');
        s.push_str(&aux_vec.join(", "));
        s.push('}');
        s
    }
}

impl<T> Serialize for BTreeSet<T>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(Serialize::serialize)
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('#');
        s.push('{');
        s.push_str(&aux_vec.join(", "));
        s.push('}');
        s
    }
}

impl<T> Serialize for LinkedList<T>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(Serialize::serialize)
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('(');
        s.push_str(&aux_vec.join(", "));
        s.push(')');
        s
    }
}

#[cfg(feature = "std")]
impl<T, H: std::hash::BuildHasher> Serialize for std::collections::HashMap<String, T, H>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(|(k, v)| format!(":{} {}", k.replace([' ', '_'], "-"), v.serialize()))
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('{');
        s.push_str(&aux_vec.join(", "));
        s.push('}');
        s
    }
}

#[cfg(feature = "std")]
impl<T, H: ::std::hash::BuildHasher> Serialize for std::collections::HashMap<&str, T, H>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(|(k, v)| format!(":{} {}", k.replace([' ', '_'], "-"), v.serialize()))
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('{');
        s.push_str(&aux_vec.join(", "));
        s.push('}');
        s
    }
}

impl<T> Serialize for BTreeMap<String, T>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(|(k, v)| format!(":{} {}", k.replace([' ', '_'], "-"), v.serialize()))
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('{');
        s.push_str(&aux_vec.join(", "));
        s.push('}');
        s
    }
}

impl<T> Serialize for BTreeMap<&str, T>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        let aux_vec = self
            .iter()
            .map(|(k, v)| format!(":{} {}", k.replace([' ', '_'], "-"), v.serialize()))
            .collect::<Vec<String>>();
        let mut s = String::new();
        s.push('{');
        s.push_str(&aux_vec.join(", "));
        s.push('}');
        s
    }
}

// Primitive Types
ser_primitives![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool];

impl Serialize for () {
    fn serialize(&self) -> String {
        "nil".to_string()
    }
}

impl Serialize for String {
    fn serialize(&self) -> String {
        format!("{self:?}")
    }
}

impl Serialize for &str {
    fn serialize(&self) -> String {
        format!("{self:?}")
    }
}

impl Serialize for char {
    fn serialize(&self) -> String {
        format!("\\{self}")
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(&self) -> String {
        self.as_ref().map_or_else(
            || String::from("nil"),
            crate::serialize::Serialize::serialize,
        )
    }
}

// Complex types
impl<A: Serialize> Serialize for (A,) {
    fn serialize(&self) -> String {
        format!("({})", self.0.serialize())
    }
}

impl<A: Serialize, B: Serialize> Serialize for (A, B) {
    fn serialize(&self) -> String {
        format!("({}, {})", self.0.serialize(), self.1.serialize())
    }
}

impl<A: Serialize, B: Serialize, C: Serialize> Serialize for (A, B, C) {
    fn serialize(&self) -> String {
        format!(
            "({}, {}, {})",
            self.0.serialize(),
            self.1.serialize(),
            self.2.serialize()
        )
    }
}

impl<A: Serialize, B: Serialize, C: Serialize, D: Serialize> Serialize for (A, B, C, D) {
    fn serialize(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.0.serialize(),
            self.1.serialize(),
            self.2.serialize(),
            self.3.serialize()
        )
    }
}

impl<A: Serialize, B: Serialize, C: Serialize, D: Serialize, E: Serialize> Serialize
    for (A, B, C, D, E)
{
    fn serialize(&self) -> String {
        format!(
            "({}, {}, {}, {}, {})",
            self.0.serialize(),
            self.1.serialize(),
            self.2.serialize(),
            self.3.serialize(),
            self.4.serialize()
        )
    }
}

impl<A: Serialize, B: Serialize, C: Serialize, D: Serialize, E: Serialize, F: Serialize> Serialize
    for (A, B, C, D, E, F)
{
    fn serialize(&self) -> String {
        format!(
            "({}, {}, {}, {}, {}, {})",
            self.0.serialize(),
            self.1.serialize(),
            self.2.serialize(),
            self.3.serialize(),
            self.4.serialize(),
            self.5.serialize()
        )
    }
}

#[cfg(test)]
mod test {
    use alloc::collections::BTreeSet;
    use alloc::vec;

    use super::*;

    #[test]
    fn unit() {
        assert_eq!(().serialize(), "nil");
    }

    #[test]
    fn primitive_types() {
        let i = -34i32;
        assert_eq!(i.serialize(), String::from("-34"));
        assert_eq!('c'.serialize(), String::from("\\c"));
        assert_eq!(8i8.serialize(), String::from("8"));
        assert_eq!(8i16.serialize(), String::from("8"));
        assert_eq!(8i32.serialize(), String::from("8"));
        assert_eq!(8i64.serialize(), String::from("8"));
        assert_eq!(8i64.serialize(), String::from("8"));
        assert_eq!(8isize.serialize(), String::from("8"));
        assert_eq!(128u8.serialize(), String::from("128"));
        assert_eq!(128u16.serialize(), String::from("128"));
        assert_eq!(128u32.serialize(), String::from("128"));
        assert_eq!(128u64.serialize(), String::from("128"));
        assert_eq!(128u64.serialize(), String::from("128"));
        assert_eq!(128usize.serialize(), String::from("128"));
        assert_eq!(true.serialize(), String::from("true"));
    }

    #[test]
    fn tuples() {
        let t2 = (12i32, 3.5f32);
        let t3 = (12i32, 3.5f32, "oi");
        let t4 = (12i32, 3.5f32, "oi", 'd');

        assert_eq!(t2.serialize(), "(12, 3.5)");
        assert_eq!(t3.serialize(), "(12, 3.5, \"oi\")");
        assert_eq!(t4.serialize(), "(12, 3.5, \"oi\", \\d)");
    }

    #[test]
    fn vectors() {
        let v_i8 = vec![3i8, 12i8, 24i8, 72i8];
        let v_u16 = vec![3u16, 12u16, 24u16, 72u16];
        let v_f32 = vec![3.0f32, 12.1f32, 24.2f32, 72.3f32];
        let v_i64 = vec![3i64, 12i64, 24i64, 72i64];
        let v_u64 = vec![3u64, 12u64, 24u64, 72u64];
        let v_bool = vec![true, false];

        assert_eq!(v_i8.serialize(), "[3, 12, 24, 72]");
        assert_eq!(v_u16.serialize(), "[3, 12, 24, 72]");
        assert_eq!(v_f32.serialize(), "[3.0, 12.1, 24.2, 72.3]");
        assert_eq!(v_i64.serialize(), "[3, 12, 24, 72]");
        assert_eq!(v_u64.serialize(), "[3, 12, 24, 72]");
        assert_eq!(v_bool.serialize(), "[true, false]");
    }

    #[test]
    fn literals_vec() {
        let v_str = vec!["aba", "cate", "azul"];
        let v_string = vec!["aba".to_string(), "cate".to_string(), "azul".to_string()];

        assert_eq!(v_str.serialize(), "[\"aba\", \"cate\", \"azul\"]");
        assert_eq!(v_string.serialize(), "[\"aba\", \"cate\", \"azul\"]");
    }

    #[test]
    fn hashsets() {
        use alloc::collections::BTreeSet;

        let set_i8 = (vec![3i8, 12i8, 24i8, 72i8]
            .into_iter()
            .collect::<BTreeSet<i8>>())
        .serialize();
        let set_u16 = (vec![3u16, 12u16, 24u16, 72u16]
            .into_iter()
            .collect::<BTreeSet<u16>>())
        .serialize();
        let set_i64 = (vec![3i64, 12i64, 24i64, 72i64]
            .into_iter()
            .collect::<BTreeSet<i64>>())
        .serialize();
        let set_bool = (vec![true, false].into_iter().collect::<BTreeSet<bool>>()).serialize();
        let set_str = (vec!["aba", "cate", "azul"]
            .into_iter()
            .collect::<BTreeSet<&str>>())
        .serialize();
        let set_string = (vec!["aba".to_string(), "cate".to_string(), "azul".to_string()]
            .into_iter()
            .collect::<BTreeSet<String>>())
        .serialize();

        assert!(
            set_i8.contains("#{")
                && set_i8.contains(',')
                && set_i8.contains('3')
                && set_i8.contains('}')
        );
        assert!(
            set_u16.contains("#{")
                && set_u16.contains(',')
                && set_u16.contains('3')
                && set_u16.contains('}')
        );
        assert!(
            set_i64.contains("#{")
                && set_i64.contains(',')
                && set_i64.contains('3')
                && set_i64.contains('}')
        );
        assert!(
            set_bool.contains("#{")
                && set_bool.contains(',')
                && set_bool.contains("true")
                && set_bool.contains("false")
                && set_bool.contains('}')
        );
        assert!(
            set_str.contains("#{")
                && set_str.contains(',')
                && set_str.contains("\"aba\"")
                && set_str.contains("\"cate\"")
                && set_str.contains('}')
        );
        assert!(
            set_string.contains("#{")
                && set_string.contains(',')
                && set_string.contains("\"aba\"")
                && set_string.contains("\"cate\"")
                && set_string.contains('}')
        );
    }

    #[test]
    fn btreesets() {
        let set_i8 = (vec![3i8, 12i8, 24i8, 72i8]
            .into_iter()
            .collect::<BTreeSet<i8>>())
        .serialize();
        let set_u16 = (vec![3u16, 12u16, 24u16, 72u16]
            .into_iter()
            .collect::<BTreeSet<u16>>())
        .serialize();
        let set_i64 = (vec![3i64, 12i64, 24i64, 72i64]
            .into_iter()
            .collect::<BTreeSet<i64>>())
        .serialize();
        let set_bool = (vec![true, false].into_iter().collect::<BTreeSet<bool>>()).serialize();
        let set_str = (vec!["aba", "cate", "azul"]
            .into_iter()
            .collect::<BTreeSet<&str>>())
        .serialize();
        let set_string = (vec!["aba".to_string(), "cate".to_string(), "azul".to_string()]
            .into_iter()
            .collect::<BTreeSet<String>>())
        .serialize();

        assert!(
            set_i8.contains("#{")
                && set_i8.contains(',')
                && set_i8.contains('3')
                && set_i8.contains('}')
        );
        assert!(
            set_u16.contains("#{")
                && set_u16.contains(',')
                && set_u16.contains('3')
                && set_u16.contains('}')
        );
        assert!(
            set_i64.contains("#{")
                && set_i64.contains(',')
                && set_i64.contains('3')
                && set_i64.contains('}')
        );
        assert!(
            set_bool.contains("#{")
                && set_bool.contains(',')
                && set_bool.contains("true")
                && set_bool.contains("false")
                && set_bool.contains('}')
        );
        assert!(
            set_str.contains("#{")
                && set_str.contains(',')
                && set_str.contains("\"aba\"")
                && set_str.contains("\"cate\"")
                && set_str.contains('}')
        );
        assert!(
            set_string.contains("#{")
                && set_string.contains(',')
                && set_string.contains("\"aba\"")
                && set_string.contains("\"cate\"")
                && set_string.contains('}')
        );
    }

    #[test]
    fn lists() {
        use alloc::collections::LinkedList;

        let list_i8 = (vec![3i8, 12i8, 24i8, 72i8]
            .into_iter()
            .collect::<LinkedList<i8>>())
        .serialize();
        let list_u16 = (vec![3u16, 12u16, 24u16, 72u16]
            .into_iter()
            .collect::<LinkedList<u16>>())
        .serialize();
        let list_i64 = (vec![3i64, 12i64, 24i64, 72i64]
            .into_iter()
            .collect::<LinkedList<i64>>())
        .serialize();
        let list_f64 = (vec![3.1f64, 12.2f64, 24.3f64, 72.4f64]
            .into_iter()
            .collect::<LinkedList<f64>>())
        .serialize();
        let list_bool = (vec![true, false].into_iter().collect::<LinkedList<bool>>()).serialize();
        let list_str = (vec!["aba", "cate", "azul"]
            .into_iter()
            .collect::<LinkedList<&str>>())
        .serialize();
        let list_string = (vec!["aba".to_string(), "cate".to_string(), "azul".to_string()]
            .into_iter()
            .collect::<LinkedList<String>>())
        .serialize();

        assert_eq!(list_i8, "(3, 12, 24, 72)");
        assert_eq!(list_u16, "(3, 12, 24, 72)");
        assert_eq!(list_i64, "(3, 12, 24, 72)");
        assert_eq!(list_f64, "(3.1, 12.2, 24.3, 72.4)");
        assert_eq!(list_bool, "(true, false)");
        assert_eq!(list_str, "(\"aba\", \"cate\", \"azul\")");
        assert_eq!(list_string, "(\"aba\", \"cate\", \"azul\")");
    }

    #[test]
    fn hashmap() {
        let m_i64 = map! {"hello world" => 5i64, "bye_bye" => 125i64}.serialize();
        let m_bool =
            map! {"hello world".to_string() => true, "bye_bye".to_string() => false}.serialize();
        let m_str = map!{"hello world".to_string() => "this is str 1", "bye_bye".to_string() => "this is str 2"}.serialize();

        assert!(
            m_i64.contains(":hello-world 5")
                && m_i64.contains(":bye-bye 125")
                && m_i64.contains('{')
                && m_i64.contains('}')
        );
        assert!(
            m_bool.contains(":hello-world true")
                && m_bool.contains(":bye-bye false")
                && m_bool.contains('{')
                && m_bool.contains('}')
        );
        assert!(
            m_str.contains(":hello-world \"this is str 1\"")
                && m_str.contains(":bye-bye \"this is str 2\"")
                && m_str.contains('{')
                && m_str.contains('}')
        );
    }

    #[test]
    fn multi_sized_tuples() {
        assert_eq!((1,).serialize(), "(1)");
        assert_eq!((1, "cool").serialize(), "(1, \"cool\")");
        assert_eq!((1, "cool", false).serialize(), "(1, \"cool\", false)");
        assert_eq!(
            (1, "cool", false, 'z').serialize(),
            "(1, \"cool\", false, \\z)"
        );
        assert_eq!(
            (1, "cool", false, 'z', None::<String>).serialize(),
            "(1, \"cool\", false, \\z, nil)"
        );
    }
}
