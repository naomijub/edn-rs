use edn_rs::json_to_edn;

fn main() {
    let json = String::from(r#"{"hello": "world"}"#);
    let edn = String::from(r#"{:hello "world"}"#);

    println!("{:?}", json_to_edn(json.clone()));
    assert_eq!(edn, json_to_edn(json));

    let complex_json = String::from(r#"{
            "people": 
            [
                {
                    "name": "otavio",
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
        }"#);

    println!("{:?}", json_to_edn(complex_json.clone()).replace("  ", "").replace("\n", " "));
    // "{ :people  [ { :name \"otavio\", :age 22 }, { :name \"Julia\", :age 32.0 } ], :country-or-origin \"Brazil\", :queerentener true, :brain nil }"
}