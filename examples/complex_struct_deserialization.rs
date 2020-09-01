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
    list: Vec<Another>,
}

impl Deserialize for Complex {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            list: Deserialize::deserialize(&edn[":list"])?,
        })
    }
}

fn complex_ok() -> Result<(), EdnError> {
    let edn_str = "{ :list [{:name \"rose\" :age 66 :cool true}, {:name \"josh\" :age 33 :cool false}, {:name \"eva\" :age 296 :cool true}] }";
    let complex: Complex = edn_rs::from_str(edn_str)?;
    println!("{:?}", complex);
    // Complex { list: [Another { name: "rose", age: 66, cool: true }, Another { name: "josh", age: 33, cool: false }, Another { name: "eva", age: 296, cool: true }] }

    assert_eq!(
        complex,
        Complex {
            list: vec![
                Another {
                    name: "rose".to_string(),
                    age: 66,
                    cool: true,
                },
                Another {
                    name: "josh".to_string(),
                    age: 33,
                    cool: false,
                },
                Another {
                    name: "eva".to_string(),
                    age: 296,
                    cool: true,
                },
            ],
        }
    );

    Ok(())
}

fn complex_wrong() -> Result<(), EdnError> {
    let bad_edn_str = "{:list [{:name \"rose\" :age \"some text\" :cool true}, {:name \"josh\" :age 33 :cool false}, {:name \"eva\" :age 296 :cool true}]}";
    let complex: Result<Complex, EdnError> = edn_rs::from_str(bad_edn_str);

    assert_eq!(
        complex,
        Err(EdnError::Deserialize(
            "couldn't convert `some text` into `uint`".to_string()
        ))
    );

    Ok(())
}

fn main() -> Result<(), EdnError> {
    complex_ok()?;
    complex_wrong()?;
    Ok(())
}

#[test]
fn test_complex_ok() {
    let _ = complex_ok();
}

#[test]
fn test_complex_wrong() {
    let _ = complex_wrong();
}
