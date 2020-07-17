/// Trait that allows you to implement Serialization for each type of your choice.
/// Exemple:
/// ```rust
/// use edn_rs::serialize::Serialize;
///
/// #[derive(Debug)]
/// struct YourType;
///
/// impl Serialize for  YourType {
///     fn serialize(self) -> String {
///         format!("{:?}", self)
///     }
/// }
/// ```
///
/// Implemented types:
/// `String, &str, char` and `Options`
/// `ser_primitives![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool];` and `Options`
/// `ser_vec![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];`
/// `ser_vec![Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<isize>, Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<usize>, Vec<f32>, Vec<f64>, Vec<bool>, Vec<String>, Vec<&str>];`
/// `ser_hashset![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];`
/// `ser_set![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];`
/// `ser_list![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];`
/// `ser_hashmap![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];`
/// `ser_hashmap_str![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];`
/// `ser_hashmap![Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<isize>, Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<usize>, Vec<f32>, Vec<f64>, Vec<bool>, Vec<String>, Vec<&str>];`
/// `ser_hashmap_str![Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<isize>, Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<usize>, Vec<f32>, Vec<f64>, Vec<bool>, Vec<String>, Vec<&str>];`
/// `ser_vec![HashMap<String, i8>, HashMap<String, i16>, HashMap<String, i32>, HashMap<String, i64>, HashMap<String, isize>, HashMap<String, u8>, HashMap<String, u16>, HashMap<String, u32>, HashMap<String, u64>, HashMap<String, usize>, HashMap<String, f32>, HashMap<String, f64>, HashMap<String, bool>, HashMap<String, String>, HashMap<String, &str>];`
/// `ser_vec![HashMap<&str, i8>, HashMap<&str, i16>, HashMap<&str, i32>, HashMap<&str, i64>, HashMap<&str, isize>, HashMap<&str, u8>, HashMap<&str, u16>, HashMap<&str, u32>, HashMap<&str, u64>, HashMap<&str, usize>, HashMap<&str, f32>, HashMap<&str, f64>, HashMap<&str, bool>, HashMap<&str, String>, HashMap<&str, &str>];`
/// `ser_vec![HashMap<String, Vec<i8>>, HashMap<String, Vec<i16>>, HashMap<String, Vec<i32>>, HashMap<String, Vec<i64>>, HashMap<String, Vec<isize>>, HashMap<String, Vec<u8>>, HashMap<String, Vec<u16>>, HashMap<String, Vec<u32>>, HashMap<String, Vec<u64>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<f32>>, HashMap<String, Vec<f64>>, HashMap<String, Vec<bool>>, HashMap<String, Vec<String>>, HashMap<String, Vec<&str>>];`
/// `ser_vec![HashMap<&str, Vec<i8>>, HashMap<&str, Vec<i16>>, HashMap<&str, Vec<i32>>, HashMap<&str, Vec<i64>>, HashMap<&str, Vec<isize>>, HashMap<&str, Vec<u8>>, HashMap<&str, Vec<u16>>, HashMap<&str, Vec<u32>>, HashMap<&str, Vec<u64>>, HashMap<&str, Vec<usize>>, HashMap<&str, Vec<f32>>, HashMap<&str, Vec<f64>>, HashMap<&str, Vec<bool>>, HashMap<&str, Vec<String>>, HashMap<&str, Vec<&str>>];`
/// `ser_hashmap![Vec<HashMap<String, i8>>, Vec<HashMap<String, i16>>, Vec<HashMap<String, i32>>, Vec<HashMap<String, i64>>, Vec<HashMap<String, isize>>, Vec<HashMap<String, u8>>, Vec<HashMap<String, u16>>, Vec<HashMap<String, u32>>, Vec<HashMap<String, u64>>, Vec<HashMap<String, usize>>, Vec<HashMap<String, f32>>, Vec<HashMap<String, f64>>, Vec<HashMap<String, bool>>, Vec<HashMap<String, String>>, Vec<HashMap<String, &str>>];`
/// `ser_hashmap_str![Vec<HashMap<&str, i8>>, Vec<HashMap<&str, i16>>, Vec<HashMap<&str, i32>>, Vec<HashMap<&str, i64>>, Vec<HashMap<&str, isize>>, Vec<HashMap<&str, u8>>, Vec<HashMap<&str, u16>>, Vec<HashMap<&str, u32>>, Vec<HashMap<&str, u64>>, Vec<HashMap<&str, usize>>, Vec<HashMap<&str, f32>>, Vec<HashMap<&str, f64>>, Vec<HashMap<&str, bool>>, Vec<HashMap<&str, String>>, Vec<HashMap<&str, &str>>];`
/// `ser_hashmap![HashMap<String, Vec<i8>>, HashMap<String, Vec<i16>>, HashMap<String, Vec<i32>>, HashMap<String, Vec<i64>>, HashMap<String, Vec<isize>>, HashMap<String, Vec<u8>>, HashMap<String, Vec<u16>>, HashMap<String, Vec<u32>>, HashMap<String, Vec<u64>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<f32>>, HashMap<String, Vec<f64>>, HashMap<String, Vec<bool>>, HashMap<String, Vec<String>>, HashMap<String, Vec<&str>>];`
/// `ser_hashmap_str![HashMap<String, Vec<i8>>, HashMap<String, Vec<i16>>, HashMap<String, Vec<i32>>, HashMap<String, Vec<i64>>, HashMap<String, Vec<isize>>, HashMap<String, Vec<u8>>, HashMap<String, Vec<u16>>, HashMap<String, Vec<u32>>, HashMap<String, Vec<u64>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<f32>>, HashMap<String, Vec<f64>>, HashMap<String, Vec<bool>>, HashMap<String, Vec<String>>, HashMap<String, Vec<&str>>];`
/// `ser_hashmap![HashSet<i8>, HashSet<i16>, HashSet<i32>, HashSet<i64>, HashSet<isize>, HashSet<u8>, HashSet<u16>, HashSet<u32>, HashSet<u64>, HashSet<usize>, HashSet<f32>, HashSet<f64>, HashSet<bool>, HashSet<String>, HashSet<&str>];`
/// `ser_hashmap_str![HashSet<i8>, HashSet<i16>, HashSet<i32>, HashSet<i64>, HashSet<isize>, HashSet<u8>, HashSet<u16>, HashSet<u32>, HashSet<u64>, HashSet<usize>, HashSet<f32>, HashSet<f64>, HashSet<bool>, HashSet<String>, HashSet<&str>];`
/// `ser_hashmap![BTreeSet<i8>, BTreeSet<i16>, BTreeSet<i32>, BTreeSet<i64>, BTreeSet<isize>, BTreeSet<u8>, BTreeSet<u16>, BTreeSet<u32>, BTreeSet<u64>, BTreeSet<usize>, BTreeSet<f32>, BTreeSet<f64>, BTreeSet<bool>, BTreeSet<String>, BTreeSet<&str>];`
/// `ser_hashmap_str![BTreeSet<i8>, BTreeSet<i16>, BTreeSet<i32>, BTreeSet<i64>, BTreeSet<isize>, BTreeSet<u8>, BTreeSet<u16>, BTreeSet<u32>, BTreeSet<u64>, BTreeSet<usize>, BTreeSet<f32>, BTreeSet<f64>, BTreeSet<bool>, BTreeSet<String>, BTreeSet<&str>];`
///
pub trait Serialize {
    fn serialize(self) -> String;
}

#[doc(hidden)]
pub fn field_names(id: Vec<String>) -> std::collections::HashMap<String, String> {
    let mut hashmap = std::collections::HashMap::new();
    for i in id.into_iter() {
        let mut value = format!("{}", i)
            .replace("___", "/")
            .replace("__", ".")
            .replace("_", "-");
        value.insert(0, ':');
        hashmap.insert(format!("{}", i), value);
    }
    hashmap
}

macro_rules! ser_primitives {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for $name
            {
                fn serialize(self) -> String {
                    format!("{:?}", self)
                }
            }
        )+
    };
}

macro_rules! ser_option_primitives {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for $name
            {
                fn serialize(self) -> String {
                    if let Some(v) = self {
                        format!("{:?}", v)
                    } else {
                        String::from("nil")
                    }
                }
            }
        )+
    };
}

macro_rules! ser_vec {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for Vec<$name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter().map(|i| i.to_owned().serialize()).collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("[");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("]");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_hashset {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::HashSet<$name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter().map(|i| i.to_owned().serialize()).collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("#{");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("}");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_set {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::BTreeSet<$name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter().map(|i| i.to_owned().serialize()).collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("#{");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("}");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_list {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::LinkedList<$name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter().map(|i| i.to_owned().serialize()).collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("(");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str(")");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_hashmap {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::HashMap<String, $name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter()
                        .map(|(k, v)|
                            format!(":{} {}",
                            k.to_string().replace(" ", "-").replace("_", "-"),
                            v.to_owned().serialize())
                        )
                        .collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("{");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("}");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_hashmap_str {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::HashMap<&str, $name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter()
                        .map(|(k, v)|
                            format!(":{} {}",
                            k.to_string().replace(" ", "-").replace("_", "-"),
                            v.to_owned().serialize())
                        )
                        .collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("{");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("}");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_btreemap {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::BTreeMap<String, $name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter()
                        .map(|(k, v)|
                            format!(":{} {}",
                            k.to_string().replace(" ", "-").replace("_", "-"),
                            v.to_owned().serialize())
                        )
                        .collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("{");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("}");
                    s
                }
            }
        )+
    };
}

macro_rules! ser_btreemap_str {
    ( $( $name:ty ),+ ) => {
        $(
            impl Serialize for std::collections::BTreeMap<&str, $name>
            {
                fn serialize(self) -> String {
                    let aux_vec = self.iter()
                        .map(|(k, v)|
                            format!(":{} {}",
                            k.to_string().replace(" ", "-").replace("_", "-"),
                            v.to_owned().serialize())
                        )
                        .collect::<Vec<String>>();
                    let mut s = String::new();
                    s.push_str("{");
                    s.push_str(&aux_vec.join(", "));
                    s.push_str("}");
                    s
                }
            }
        )+
    };
}

// Primitive Types
ser_primitives![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool];
ser_option_primitives![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>
];

impl Serialize for String {
    fn serialize(self) -> String {
        format!("{:?}", self)
    }
}

impl Serialize for &str {
    fn serialize(self) -> String {
        format!("{:?}", self)
    }
}

impl Serialize for char {
    fn serialize(self) -> String {
        format!("\\{}", self)
    }
}

impl Serialize for Option<String> {
    fn serialize(self) -> String {
        if let Some(v) = self {
            format!("{:?}", v)
        } else {
            String::from("nil")
        }
    }
}

impl Serialize for Option<&str> {
    fn serialize(self) -> String {
        if let Some(v) = self {
            format!("{:?}", v)
        } else {
            String::from("nil")
        }
    }
}

impl Serialize for Option<char> {
    fn serialize(self) -> String {
        if let Some(v) = self {
            format!("{:?}", v)
        } else {
            String::from("nil")
        }
    }
}

// Complex types
impl<A: Serialize, B: Serialize> Serialize for (A, B) {
    fn serialize(self) -> String {
        format!("({}, {})", self.0.serialize(), self.1.serialize())
    }
}

impl<A: Serialize, B: Serialize, C: Serialize> Serialize for (A, B, C) {
    fn serialize(self) -> String {
        format!(
            "({}, {}, {})",
            self.0.serialize(),
            self.1.serialize(),
            self.2.serialize()
        )
    }
}

impl<A: Serialize, B: Serialize, C: Serialize, D: Serialize> Serialize for (A, B, C, D) {
    fn serialize(self) -> String {
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
    fn serialize(self) -> String {
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
    fn serialize(self) -> String {
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

ser_vec![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_vec![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_vec![
    Vec<i8>,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<isize>,
    Vec<u8>,
    Vec<u16>,
    Vec<u32>,
    Vec<u64>,
    Vec<usize>,
    Vec<f32>,
    Vec<f64>,
    Vec<bool>,
    Vec<String>,
    Vec<&str>
];
ser_vec![
    Vec<Option<i8>>,
    Vec<Option<i16>>,
    Vec<Option<i32>>,
    Vec<Option<i64>>,
    Vec<Option<isize>>,
    Vec<Option<u8>>,
    Vec<Option<u16>>,
    Vec<Option<u32>>,
    Vec<Option<u64>>,
    Vec<Option<usize>>,
    Vec<Option<f32>>,
    Vec<Option<f64>>,
    Vec<Option<bool>>,
    Vec<Option<String>>,
    Vec<Option<&str>>
];
ser_hashset![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_hashset![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_set![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_set![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_list![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_list![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_hashmap![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_hashmap![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_hashmap_str![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_hashmap_str![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_hashmap![
    Vec<i8>,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<isize>,
    Vec<u8>,
    Vec<u16>,
    Vec<u32>,
    Vec<u64>,
    Vec<usize>,
    Vec<f32>,
    Vec<f64>,
    Vec<bool>,
    Vec<String>,
    Vec<&str>
];
ser_hashmap![
    Vec<Option<i8>>,
    Vec<Option<i16>>,
    Vec<Option<i32>>,
    Vec<Option<i64>>,
    Vec<Option<isize>>,
    Vec<Option<u8>>,
    Vec<Option<u16>>,
    Vec<Option<u32>>,
    Vec<Option<u64>>,
    Vec<Option<usize>>,
    Vec<Option<f32>>,
    Vec<Option<f64>>,
    Vec<Option<bool>>,
    Vec<Option<String>>,
    Vec<Option<&str>>
];
ser_hashmap_str![
    Vec<i8>,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<isize>,
    Vec<u8>,
    Vec<u16>,
    Vec<u32>,
    Vec<u64>,
    Vec<usize>,
    Vec<f32>,
    Vec<f64>,
    Vec<bool>,
    Vec<String>,
    Vec<&str>
];
ser_hashmap_str![
    Vec<Option<i8>>,
    Vec<Option<i16>>,
    Vec<Option<i32>>,
    Vec<Option<i64>>,
    Vec<Option<isize>>,
    Vec<Option<u8>>,
    Vec<Option<u16>>,
    Vec<Option<u32>>,
    Vec<Option<u64>>,
    Vec<Option<usize>>,
    Vec<Option<f32>>,
    Vec<Option<f64>>,
    Vec<Option<bool>>,
    Vec<Option<String>>,
    Vec<Option<&str>>
];
ser_btreemap![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_btreemap![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_btreemap_str![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str];
ser_btreemap_str![
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<isize>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<usize>,
    Option<f32>,
    Option<f64>,
    Option<bool>,
    Option<String>,
    Option<&str>
];
ser_btreemap![
    Vec<i8>,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<isize>,
    Vec<u8>,
    Vec<u16>,
    Vec<u32>,
    Vec<u64>,
    Vec<usize>,
    Vec<f32>,
    Vec<f64>,
    Vec<bool>,
    Vec<String>,
    Vec<&str>
];
ser_btreemap![
    Vec<Option<i8>>,
    Vec<Option<i16>>,
    Vec<Option<i32>>,
    Vec<Option<i64>>,
    Vec<Option<isize>>,
    Vec<Option<u8>>,
    Vec<Option<u16>>,
    Vec<Option<u32>>,
    Vec<Option<u64>>,
    Vec<Option<usize>>,
    Vec<Option<f32>>,
    Vec<Option<f64>>,
    Vec<Option<bool>>,
    Vec<Option<String>>,
    Vec<Option<&str>>
];
ser_btreemap_str![
    Vec<i8>,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<isize>,
    Vec<u8>,
    Vec<u16>,
    Vec<u32>,
    Vec<u64>,
    Vec<usize>,
    Vec<f32>,
    Vec<f64>,
    Vec<bool>,
    Vec<String>,
    Vec<&str>
];
ser_btreemap_str![
    Vec<Option<i8>>,
    Vec<Option<i16>>,
    Vec<Option<i32>>,
    Vec<Option<i64>>,
    Vec<Option<isize>>,
    Vec<Option<u8>>,
    Vec<Option<u16>>,
    Vec<Option<u32>>,
    Vec<Option<u64>>,
    Vec<Option<usize>>,
    Vec<Option<f32>>,
    Vec<Option<f64>>,
    Vec<Option<bool>>,
    Vec<Option<String>>,
    Vec<Option<&str>>
];

use std::collections::HashMap;
ser_vec![HashMap<String, i8>, HashMap<String, i16>, HashMap<String, i32>, HashMap<String, i64>, HashMap<String, isize>, HashMap<String, u8>, HashMap<String, u16>, HashMap<String, u32>, HashMap<String, u64>, HashMap<String, usize>, HashMap<String, f32>, HashMap<String, f64>, HashMap<String, bool>, HashMap<String, String>, HashMap<String, &str>];
ser_vec![HashMap<&str, i8>, HashMap<&str, i16>, HashMap<&str, i32>, HashMap<&str, i64>, HashMap<&str, isize>, HashMap<&str, u8>, HashMap<&str, u16>, HashMap<&str, u32>, HashMap<&str, u64>, HashMap<&str, usize>, HashMap<&str, f32>, HashMap<&str, f64>, HashMap<&str, bool>, HashMap<&str, String>, HashMap<&str, &str>];
ser_vec![HashMap<String, Vec<i8>>, HashMap<String, Vec<i16>>, HashMap<String, Vec<i32>>, HashMap<String, Vec<i64>>, HashMap<String, Vec<isize>>, HashMap<String, Vec<u8>>, HashMap<String, Vec<u16>>, HashMap<String, Vec<u32>>, HashMap<String, Vec<u64>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<f32>>, HashMap<String, Vec<f64>>, HashMap<String, Vec<bool>>, HashMap<String, Vec<String>>, HashMap<String, Vec<&str>>];

ser_hashmap![
    Vec<HashMap<String, i8>>,
    Vec<HashMap<String, i16>>,
    Vec<HashMap<String, i32>>,
    Vec<HashMap<String, i64>>,
    Vec<HashMap<String, isize>>,
    Vec<HashMap<String, u8>>,
    Vec<HashMap<String, u16>>,
    Vec<HashMap<String, u32>>,
    Vec<HashMap<String, u64>>,
    Vec<HashMap<String, usize>>,
    Vec<HashMap<String, f32>>,
    Vec<HashMap<String, f64>>,
    Vec<HashMap<String, bool>>,
    Vec<HashMap<String, String>>,
    Vec<HashMap<String, &str>>
];
ser_hashmap_str![
    Vec<HashMap<&str, i8>>,
    Vec<HashMap<&str, i16>>,
    Vec<HashMap<&str, i32>>,
    Vec<HashMap<&str, i64>>,
    Vec<HashMap<&str, isize>>,
    Vec<HashMap<&str, u8>>,
    Vec<HashMap<&str, u16>>,
    Vec<HashMap<&str, u32>>,
    Vec<HashMap<&str, u64>>,
    Vec<HashMap<&str, usize>>,
    Vec<HashMap<&str, f32>>,
    Vec<HashMap<&str, f64>>,
    Vec<HashMap<&str, bool>>,
    Vec<HashMap<&str, String>>,
    Vec<HashMap<&str, &str>>
];
ser_hashmap![HashMap<String, Vec<i8>>, HashMap<String, Vec<i16>>, HashMap<String, Vec<i32>>, HashMap<String, Vec<i64>>, HashMap<String, Vec<isize>>, HashMap<String, Vec<u8>>, HashMap<String, Vec<u16>>, HashMap<String, Vec<u32>>, HashMap<String, Vec<u64>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<f32>>, HashMap<String, Vec<f64>>, HashMap<String, Vec<bool>>, HashMap<String, Vec<String>>, HashMap<String, Vec<&str>>];
ser_hashmap_str![HashMap<String, Vec<i8>>, HashMap<String, Vec<i16>>, HashMap<String, Vec<i32>>, HashMap<String, Vec<i64>>, HashMap<String, Vec<isize>>, HashMap<String, Vec<u8>>, HashMap<String, Vec<u16>>, HashMap<String, Vec<u32>>, HashMap<String, Vec<u64>>, HashMap<String, Vec<usize>>, HashMap<String, Vec<f32>>, HashMap<String, Vec<f64>>, HashMap<String, Vec<bool>>, HashMap<String, Vec<String>>, HashMap<String, Vec<&str>>];

use std::collections::HashSet;
ser_hashmap![
    HashSet<i8>,
    HashSet<i16>,
    HashSet<i32>,
    HashSet<i64>,
    HashSet<isize>,
    HashSet<u8>,
    HashSet<u16>,
    HashSet<u32>,
    HashSet<u64>,
    HashSet<usize>,
    HashSet<f32>,
    HashSet<f64>,
    HashSet<bool>,
    HashSet<String>,
    HashSet<&str>
];

ser_hashmap![
    HashSet<Option<i8>>,
    HashSet<Option<i16>>,
    HashSet<Option<i32>>,
    HashSet<Option<i64>>,
    HashSet<Option<isize>>,
    HashSet<Option<u8>>,
    HashSet<Option<u16>>,
    HashSet<Option<u32>>,
    HashSet<Option<u64>>,
    HashSet<Option<usize>>,
    HashSet<Option<f32>>,
    HashSet<Option<f64>>,
    HashSet<Option<bool>>,
    HashSet<Option<String>>,
    HashSet<Option<&str>>
];
ser_hashmap_str![
    HashSet<i8>,
    HashSet<i16>,
    HashSet<i32>,
    HashSet<i64>,
    HashSet<isize>,
    HashSet<u8>,
    HashSet<u16>,
    HashSet<u32>,
    HashSet<u64>,
    HashSet<usize>,
    HashSet<f32>,
    HashSet<f64>,
    HashSet<bool>,
    HashSet<String>,
    HashSet<&str>
];

ser_hashmap_str![
    HashSet<Option<i8>>,
    HashSet<Option<i16>>,
    HashSet<Option<i32>>,
    HashSet<Option<i64>>,
    HashSet<Option<isize>>,
    HashSet<Option<u8>>,
    HashSet<Option<u16>>,
    HashSet<Option<u32>>,
    HashSet<Option<u64>>,
    HashSet<Option<usize>>,
    HashSet<Option<f32>>,
    HashSet<Option<f64>>,
    HashSet<Option<bool>>,
    HashSet<Option<String>>,
    HashSet<Option<&str>>
];

use std::collections::BTreeSet;
ser_hashmap![
    BTreeSet<i8>,
    BTreeSet<i16>,
    BTreeSet<i32>,
    BTreeSet<i64>,
    BTreeSet<isize>,
    BTreeSet<u8>,
    BTreeSet<u16>,
    BTreeSet<u32>,
    BTreeSet<u64>,
    BTreeSet<usize>,
    BTreeSet<f32>,
    BTreeSet<f64>,
    BTreeSet<bool>,
    BTreeSet<String>,
    BTreeSet<&str>
];
ser_hashmap![
    BTreeSet<Option<i8>>,
    BTreeSet<Option<i16>>,
    BTreeSet<Option<i32>>,
    BTreeSet<Option<i64>>,
    BTreeSet<Option<isize>>,
    BTreeSet<Option<u8>>,
    BTreeSet<Option<u16>>,
    BTreeSet<Option<u32>>,
    BTreeSet<Option<u64>>,
    BTreeSet<Option<usize>>,
    BTreeSet<Option<f32>>,
    BTreeSet<Option<f64>>,
    BTreeSet<Option<bool>>,
    BTreeSet<Option<String>>,
    BTreeSet<Option<&str>>
];
ser_hashmap_str![
    BTreeSet<i8>,
    BTreeSet<i16>,
    BTreeSet<i32>,
    BTreeSet<i64>,
    BTreeSet<isize>,
    BTreeSet<u8>,
    BTreeSet<u16>,
    BTreeSet<u32>,
    BTreeSet<u64>,
    BTreeSet<usize>,
    BTreeSet<f32>,
    BTreeSet<f64>,
    BTreeSet<bool>,
    BTreeSet<String>,
    BTreeSet<&str>
];
ser_hashmap_str![
    BTreeSet<Option<i8>>,
    BTreeSet<Option<i16>>,
    BTreeSet<Option<i32>>,
    BTreeSet<Option<i64>>,
    BTreeSet<Option<isize>>,
    BTreeSet<Option<u8>>,
    BTreeSet<Option<u16>>,
    BTreeSet<Option<u32>>,
    BTreeSet<Option<u64>>,
    BTreeSet<Option<usize>>,
    BTreeSet<Option<f32>>,
    BTreeSet<Option<f64>>,
    BTreeSet<Option<bool>>,
    BTreeSet<Option<String>>,
    BTreeSet<Option<&str>>
];

/// `ser_struct!` creates a struct with the serialization trait already implemented:
///
/// `
/// ser_struct! {
///     #[derive(Debug)]
///     struct Foo {
///         foo: i32,
///         bar: String,
///         boz: char
///     }
/// }
/// `
///
/// then you can use the struct normally:
/// ```rust
/// #[macro_use]
/// extern crate edn_rs;
/// use edn_rs::serialize::Serialize;
///
/// fn main() {
///     ser_struct! {
///         #[derive(Debug)]
///         struct Foo {
///             foo: i32,
///             bar: String,
///             boz: char
///         }
///     }
///     let foo  = Foo { foo: 1, bar: String::from("blahb"), boz: 'c'};
///
///     assert_eq!(foo.serialize(), "{ :foo 1, :bar \"blahb\", :boz \\c, }");
/// }
/// ```
///
/// There is also the possibility to create a public struct. This is done by adding the `pub` keyword
/// before the structs naming, `pub struct Foo {`. Note that all inner fields will be public as well.
/// ```
/// #![recursion_limit="512"]
/// #[macro_use] extern crate edn_rs;
///
/// #[test]
/// fn pub_struct() {
///     let edn = helper::Edn {
///         val: 6i32,
///         tuples: (3i32, true, 'd')
///     };
///
///     assert_eq!(edn.val, 6i32);
///     assert_eq!(edn.tuples, (3i32, true, 'd'));
/// }
///
/// mod helper {
///     use std::collections::{HashMap, HashSet};
///     use crate::edn_rs::serialize::Serialize;
///
///     ser_struct!{
///         #[derive(Debug, Clone)]
///         pub struct Edn {
///             val: i32,
///             tuples: (i32, bool, char),
///         }
///     }
/// }
/// ```
///
/// Note than when you `serialize` `_` will become `-`, `__` will become `.` and `___` will become `/`
/// **PLEASE USE `#[derive(Debug)]` for now**
#[macro_export]
macro_rules! ser_struct {
    (@gen () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
        $(#[$attr])* struct $name { $($id: $ty),* }

        impl Serialize for $name {
            fn serialize(self) -> String {
                let mut s = String::new();
                let mut v = Vec::new();
                $(
                    v.push(format!("{}", stringify!($id)));
                )*
                let hm_field_names = $crate::serialize::field_names(v);
                s.push_str("{ ");
                $(
                    s.push_str(&hm_field_names.get(&format!("{}", stringify!($id))).expect("fails to parse field name"));
                    s.push_str(" ");
                    s.push_str(&self.$id.serialize());
                    s.push_str(", ");
                )*
                s.push_str("}");
                s
            }
        }
    };

    (@gen () -> {$(#[$attr:meta])* pub struct $name:ident $(($id:ident: $ty:ty))*}) => {
        $(#[$attr])* pub struct $name { $(pub $id: $ty),* }

        impl Serialize for $name {
            fn serialize(self) -> String {
                let mut s = String::new();
                let mut v = Vec::new();
                $(
                    v.push(format!("{}", stringify!($id)));
                )*
                let hm_field_names = $crate::serialize::field_names(v);
                s.push_str("{ ");
                $(
                    s.push_str(&hm_field_names.get(&format!("{}", stringify!($id))).expect("fails to parse field name"));
                    s.push_str(" ");
                    s.push_str(&self.$id.serialize());
                    s.push_str(", ");
                )*
                s.push_str("}");
                s
            }
        }
    };

    // last field
    (@gen ($id:tt: $ty:ty) -> {$($output:tt)*}) => {
        ser_struct!(@gen () -> {$($output)* ($id: $ty)});
    };

    // next field
    (@gen ($id:tt: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        ser_struct!(@gen ($($next)*) -> {$($output)* ($id: $ty)});
    };

    // entry point
    ($(#[$attr:meta])* struct $name:ident { $($input:tt)*} ) => {
        ser_struct!(@gen ($($input)*) -> {$(#[$attr])* struct $name});
    };

     // pub struct
     ($(#[$attr:meta])* pub struct $name:ident { $($input:tt)*} ) => {
        ser_struct!(@gen ($($input)*) -> {$(#[$attr])* pub struct $name});
    };
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
    assert_eq!(8isize.serialize(), String::from("8"));
    assert_eq!(128u8.serialize(), String::from("128"));
    assert_eq!(128u16.serialize(), String::from("128"));
    assert_eq!(128u32.serialize(), String::from("128"));
    assert_eq!(128u64.serialize(), String::from("128"));
    assert_eq!(128usize.serialize(), String::from("128"));
    assert_eq!(true.serialize(), String::from("true"));
}

#[test]
fn basic_struct_ser() {
    ser_struct! {
        #[derive(Debug)]
        struct Foo {
            foo: i32,
            bar: String,
            boz: char
        }
    }
    let actual = Foo {
        foo: 1,
        bar: String::from("blahb"),
        boz: 'c',
    };

    assert_eq!(actual.serialize(), "{ :foo 1, :bar \"blahb\", :boz \\c, }");
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
    let v_usize = vec![3usize, 12usize, 24usize, 72usize];
    let v_bool = vec![true, false];

    assert_eq!(v_i8.serialize(), "[3, 12, 24, 72]");
    assert_eq!(v_u16.serialize(), "[3, 12, 24, 72]");
    assert_eq!(v_f32.serialize(), "[3.0, 12.1, 24.2, 72.3]");
    assert_eq!(v_i64.serialize(), "[3, 12, 24, 72]");
    assert_eq!(v_usize.serialize(), "[3, 12, 24, 72]");
    assert_eq!(v_bool.serialize(), "[true, false]")
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
    use std::collections::HashSet;

    let set_i8 = (vec![3i8, 12i8, 24i8, 72i8]
        .into_iter()
        .collect::<HashSet<i8>>())
    .serialize();
    let set_u16 = (vec![3u16, 12u16, 24u16, 72u16]
        .into_iter()
        .collect::<HashSet<u16>>())
    .serialize();
    let set_i64 = (vec![3i64, 12i64, 24i64, 72i64]
        .into_iter()
        .collect::<HashSet<i64>>())
    .serialize();
    let set_bool = (vec![true, false].into_iter().collect::<HashSet<bool>>()).serialize();
    let set_str = (vec!["aba", "cate", "azul"]
        .into_iter()
        .collect::<HashSet<&str>>())
    .serialize();
    let set_string = (vec!["aba".to_string(), "cate".to_string(), "azul".to_string()]
        .into_iter()
        .collect::<HashSet<String>>())
    .serialize();

    assert!(
        set_i8.contains("#{")
            && set_i8.contains(",")
            && set_i8.contains("3")
            && set_i8.contains("}")
    );
    assert!(
        set_u16.contains("#{")
            && set_u16.contains(",")
            && set_u16.contains("3")
            && set_u16.contains("}")
    );
    assert!(
        set_i64.contains("#{")
            && set_i64.contains(",")
            && set_i64.contains("3")
            && set_i64.contains("}")
    );
    assert!(
        set_bool.contains("#{")
            && set_bool.contains(",")
            && set_bool.contains("true")
            && set_bool.contains("false")
            && set_bool.contains("}")
    );
    assert!(
        set_str.contains("#{")
            && set_str.contains(",")
            && set_str.contains("\"aba\"")
            && set_str.contains("\"cate\"")
            && set_str.contains("}")
    );
    assert!(
        set_string.contains("#{")
            && set_string.contains(",")
            && set_string.contains("\"aba\"")
            && set_string.contains("\"cate\"")
            && set_string.contains("}")
    );
}

#[test]
fn btreesets() {
    use std::collections::BTreeSet;

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
            && set_i8.contains(",")
            && set_i8.contains("3")
            && set_i8.contains("}")
    );
    assert!(
        set_u16.contains("#{")
            && set_u16.contains(",")
            && set_u16.contains("3")
            && set_u16.contains("}")
    );
    assert!(
        set_i64.contains("#{")
            && set_i64.contains(",")
            && set_i64.contains("3")
            && set_i64.contains("}")
    );
    assert!(
        set_bool.contains("#{")
            && set_bool.contains(",")
            && set_bool.contains("true")
            && set_bool.contains("false")
            && set_bool.contains("}")
    );
    assert!(
        set_str.contains("#{")
            && set_str.contains(",")
            && set_str.contains("\"aba\"")
            && set_str.contains("\"cate\"")
            && set_str.contains("}")
    );
    assert!(
        set_string.contains("#{")
            && set_string.contains(",")
            && set_string.contains("\"aba\"")
            && set_string.contains("\"cate\"")
            && set_string.contains("}")
    );
}

#[test]
fn lists() {
    use std::collections::LinkedList;

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
            && m_i64.contains("{")
            && m_i64.contains("}")
    );
    assert!(
        m_bool.contains(":hello-world true")
            && m_bool.contains(":bye-bye false")
            && m_bool.contains("{")
            && m_bool.contains("}")
    );
    assert!(
        m_str.contains(":hello-world \"this is str 1\"")
            && m_str.contains(":bye-bye \"this is str 2\"")
            && m_str.contains("{")
            && m_str.contains("}")
    );
}

#[test]
fn out_pub_struct_ser() {
    ser_struct! {
        #[derive(Debug)]
        pub struct Foo {
            foo: i32,
            bar: String,
            boz: char
        }
    }
    let actual = Foo {
        foo: 1,
        bar: String::from("blahb"),
        boz: 'c',
    };

    assert_eq!(actual.serialize(), "{ :foo 1, :bar \"blahb\", :boz \\c, }");
}

#[test]
fn struct_fields_special_chars() {
    ser_struct! {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct Foo {
            foo__bar___boz: i32,
        }
    }

    let foobar = Foo { foo__bar___boz: 3 };
    assert_eq!(foobar.serialize(), "{ :foo.bar/boz 3, }")
}
