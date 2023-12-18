use std::str::FromStr;

fn complex_json() {
    let complex_json = String::from(
        "{\"brain\": null, \"countryOrOrigin\": \"Brazil\", \"peopleList\": [{\"age\": 22, \"firstName\": \"eva\"}, {\"age\": 32.0, \"firstName\": \"Julia\"}], \"queerentener\": true}",
    );
    let edn = "{ :people-list  [ { :first-name \"eva\", :age 22 }, { :first-name \"Julia\", :age 32.0 } ], :country-or-origin \"Brazil\", :queerentener true, :brain nil }";
    let parsed_edn: edn_rs::Edn = edn_rs::Edn::from_str(edn).unwrap();
    let actual_json = parsed_edn.to_json();

    assert_eq!(actual_json, complex_json);
}

fn main() {
    complex_json();
}

#[test]
fn test_complex_json() {
    complex_json();
}
