use edn_rs::{
    edn,
    edn::{Edn, List, Map},
};

fn navigate() {
    let edn = edn!((sym 1.2 3 {false :f nil 3/4}));
    println!("{edn:?}");

    assert_eq!(edn[1], edn!(1.2));
    assert_eq!(edn[1], Edn::Double(1.2f64.into()));
    assert_eq!(edn[3]["false"], edn!(:f));
    assert_eq!(edn[3]["false"], Edn::Key(":f".to_string()));
}

fn main() {
    navigate();
}

#[test]
fn test_navigate() {
    navigate();
}
