use edn_rs::{
    edn,
    edn::{Edn, List},
};

fn tokenize() {
    let edn = edn!((sym 1.2 3 false :f nil 3/4));
    let expected = Edn::List(List::new(vec![
        Edn::Symbol("sym".to_string()),
        Edn::Double(1.2.into()),
        Edn::Int(3),
        Edn::Bool(false),
        Edn::Key(":f".to_string()),
        Edn::Nil,
        Edn::Rational((3, 4)),
    ]));

    println!("{edn:?}");
    assert_eq!(edn, expected);
}

fn main() {
    tokenize();
}

#[test]
fn test_tokenize() {
    tokenize();
}
