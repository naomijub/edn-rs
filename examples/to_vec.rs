use edn_rs::{Edn, List};

fn main() {
    let v = Edn::List(List::new(vec![
        Edn::Key(":my-key".to_string()),
        Edn::Int(6),
        Edn::Rational("7/4".to_string()),
    ]));
    let expected = vec![":my-key", "6", "7/4"];
    println!("{:?}", v.to_vec().unwrap());
    assert_eq!(expected, v.to_vec().unwrap());
}
