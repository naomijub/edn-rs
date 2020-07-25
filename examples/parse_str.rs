use edn_rs::{map, parse_edn, set, Edn, EdnError, Map, Set, Vector};
use std::str::FromStr;

fn main() -> Result<(), EdnError> {
    let edn_str = "{:a \"2\" :b [true false] :c #{:A {:a :b} nil}}";
    let edn = Edn::from_str(edn_str);

    assert_eq!(
        edn,
        Ok(Edn::Map(Map::new(map! {
        ":a".to_string() => Edn::Str("2".to_string()),
        ":b".to_string() => Edn::Vector(Vector::new(vec![Edn::Bool(true), Edn::Bool(false)])),
        ":c".to_string() => Edn::Set(Set::new(
            set!{
                Edn::Map(Map::new(map!{":a".to_string() => Edn::Key(":b".to_string())})),
                Edn::Key(":A".to_string()),
                Edn::Nil}))})))
    );

    println!("{:?}", edn);

    // OR

    let edn_resp = parse_edn(edn_str)?;
    assert_eq!(edn_resp[":b"][0], Edn::Bool(true));
    Ok(())
}
