use edn_rs::{Edn, EdnError};

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

impl From<Edn> for Person {
    fn from(edn: Edn) -> Self {
        Self {
            name: edn[":name"].to_string(),
            age: edn[":age"].to_uint().unwrap(),
        }
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

    Ok(())
}
