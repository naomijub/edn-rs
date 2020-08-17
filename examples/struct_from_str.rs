use edn_rs::{Deserialize, Edn, EdnError};

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

impl Deserialize for Person {
    fn deserialize(edn: Edn) -> Result<Self, EdnError> {
        Ok(Self {
            name: edn[":name"].to_string(),
            age: edn[":age"].to_uint().ok_or_else(|| {
                EdnError::Deserialize("couldn't convert `:age` into `uint`".to_string())
            })?,
        })
    }
}

fn main() -> Result<(), EdnError> {
    let edn_str = "{:name \"rose\" :age 66}";
    let person: Person = edn_rs::from_str(edn_str)?;

    assert_eq!(
        person,
        Person {
            name: "rose".to_string(),
            age: 66,
        }
    );

    println!("{:?}", person);
    // Person { name: "rose", age: 66 }

    let bad_edn_str = "{:name \"rose\" :age \"not an uint\"}";
    let person: Result<Person, EdnError> = edn_rs::from_str(bad_edn_str);

    assert_eq!(
        person,
        Err(EdnError::Deserialize(
            "couldn't convert `:age` into `uint`".to_string()
        ))
    );

    Ok(())
}
