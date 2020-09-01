use edn_rs::{map, set, Edn, EdnError, Map, Set, Vector};
use std::str::FromStr;

fn edn_from_str() -> Result<Edn, EdnError> {
    let edn_str = "{:a \"2\" :b [true false] :c #{:A nil {:a :b}}}";
    Edn::from_str(edn_str)
}

fn main() -> Result<(), EdnError> {
    let edn = edn_from_str()?;

    println!("{:?}", edn);

    Ok(())
}

#[test]
fn test_edn_from_str() {
    let edn = edn_from_str().unwrap();
    assert_eq!(
        edn,
        Edn::Map(Map::new(map! {
        ":a".to_string() => Edn::Str("2".to_string()),
        ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
        ":c".to_string() => Edn::Set(Set::new(
            set!{
                Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
                Edn::Key(":A".to_string()),
                Edn::Nil}))}))
    );
    assert_eq!(edn[":b"][0], Edn::Bool(true));
}
