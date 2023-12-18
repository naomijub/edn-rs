use edn_rs::json_to_edn;

fn simple_json() {
    let json = String::from(r#"{"hello": "world"}"#);
    let edn = String::from(r#"{:hello "world"}"#);

    println!("{:?}", json_to_edn(json.clone()));
    assert_eq!(edn, json_to_edn(json));
}

fn complex_json() {
    let complex_json = String::from(
        r#"{
            "people": 
            [
                {
                    "name": "eva",
                    "age": 22
                },
                {
                    "name": "Julia",
                    "age": 32.0
                }
            ],
            "country or origin": "Brazil",
            "queerentener": true,
            "brain": null
        }"#,
    );
    let edn = "{ :people  [ { :name \"eva\", :age 22 }, { :name \"Julia\", :age 32.0 } ], :country-or-origin \"Brazil\", :queerentener true, :brain nil }";

    assert_eq!(
        edn,
        json_to_edn(complex_json)
            .replace("  ", "")
            .replace('\n', " ")
    );
}

fn main() {
    simple_json();
    complex_json();
}

#[test]
fn test_simple_json() {
    simple_json();
}

#[test]
fn test_complex_json() {
    complex_json();
}
