use edn_rs::{Deserialize, Edn, EdnError};

#[derive(Debug, PartialEq)]
struct Another {
    name: String,
    age: usize,
    cool: bool,
}

impl Deserialize for Another {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            name: Deserialize::deserialize(&edn[":name"])?,
            age: Deserialize::deserialize(&edn[":age"])?,
            cool: Deserialize::deserialize(&edn[":cool"])?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Complex {
    id: usize,
    maybe: Option<Another>,
}

impl Deserialize for Complex {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            id: Deserialize::deserialize(&edn[":id"])?,
            maybe: Deserialize::deserialize(&edn[":maybe"])?,
        })
    }
}

fn maybe_is_some() -> Result<(), EdnError> {
    let edn_str = "{ :id 22 :maybe {:name \"rose\" :age 66 :cool true} }";
    let complex: Complex = edn_rs::from_str(edn_str)?;
    println!("{:?}", complex);
    // Complex { id: 22, maybe: Another { name: "rose", age: 66, cool: true } }

    assert_eq!(
        complex,
        Complex {
            id: 22,
            maybe: Some(Another {
                name: "rose".to_string(),
                age: 66,
                cool: true,
            }),
        }
    );
    Ok(())
}

fn maybe_is_none() -> Result<(), EdnError> {
    let edn_str = "{ :id 1 }";
    let complex: Complex = edn_rs::from_str(edn_str)?;
    println!("{:?}", complex);
    // Complex { id: 1, maybe: None }

    assert_eq!(complex, Complex { id: 1, maybe: None });

    Ok(())
}

fn main() -> Result<(), EdnError> {
    maybe_is_some()?;
    maybe_is_none()?;

    Ok(())
}

#[test]
fn test_maybe_some() {
    let _ = maybe_is_some();
}

#[test]
fn test_maybe_none() {
    let _ = maybe_is_none();
}
