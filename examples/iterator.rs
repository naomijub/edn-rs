use edn_rs::{Edn, Vector};

fn iterator() {
    let v = Edn::Vector(Vector::new(vec![Edn::Int(5), Edn::Int(6), Edn::Int(7)]));
    let sum = v
        .iter_some()
        .unwrap()
        .filter(|e| e.to_int().is_some())
        .map(|e| e.to_int().unwrap())
        .sum();

    println!("{:?}", sum);
    assert_eq!(18isize, sum);
}

fn main() {
    iterator();
}

#[test]
fn test_iterator() {
    iterator();
}
