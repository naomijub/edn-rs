use edn_rs::{Edn, Vector};

fn iterator() {
    let v = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
    let sum = v.iter_some().unwrap().filter_map(edn_rs::Edn::to_int).sum();

    println!("{sum:?}");
    assert_eq!(18isize, sum);
}

fn main() {
    iterator();
}

#[test]
fn test_iterator() {
    iterator();
}
