use edn_rs::{map, set, Edn, EdnError, Map, Set, Vector};
use std::str::FromStr;

fn main() -> Result<(), EdnError> {
    let edn_str = "{:a \"2\" :b [true false ] :c #{:A {:a :b } nil } }";
    let edn = Edn::from_str(edn_str)?;

    //TODO: Failing
    // assert_eq!(
    //     edn,
    //     Edn::Map(Map::new(map! {
    //     ":a".to_string() => Edn::Str("2".to_string()),
    //     ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
    //     ":c".to_string() => Edn::Set(Set::new(
    //         set!{
    //             Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
    //             Edn::Key(":A".to_string()),
    //             Edn::Nil}))}))
    // );

    println!("{:?}", edn);

    assert_eq!(edn[":b"][0], Edn::Bool(true));

    Ok(())
}
