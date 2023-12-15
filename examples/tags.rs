use std::str::FromStr;

use edn_rs::Edn;

fn uuid_printer(uuid: &Edn) {
    println!("Received UUID: {}", uuid);
}

fn inst_printer(inst: &Edn) {
    println!("Received Inst: {}", inst);
}

fn print_tagged_or_panic(tagged_data: &Edn) {
    if let Edn::Tagged(t, d) = tagged_data {
        match t.as_str() {
            "uuid" => uuid_printer(d),
            "inst" => inst_printer(d),
            _ => println!("\"{t}\" tag with {d}"),
        }
    } else {
        panic!();
    };
}

fn tagged_data() {
    let edn = "{:date   #inst \"2020-07-16T21:53:14.628-00:00\"
                :uuid   #uuid \"af6d8699-f442-4dfd-8b26-37d80543186b\"
                :foobar #arbitrary 0x2A}";
    let parsed_edn: edn_rs::Edn = edn_rs::Edn::from_str(edn).unwrap();

    print_tagged_or_panic(parsed_edn.get(":date").unwrap());
    print_tagged_or_panic(parsed_edn.get(":uuid").unwrap());
    print_tagged_or_panic(parsed_edn.get(":foobar").unwrap());
}

fn main() {
    tagged_data();
}

#[test]
fn test_tagged_data() {
    tagged_data();
}
