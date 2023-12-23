use edn_rs::Serialize;

struct Foo<'a> {
    value: u64,
    say: &'a str,
}

impl Serialize for Foo<'_> {
    fn serialize(&self) -> String {
        format!("{{:value {}, :say {:?}}}", self.value, self.say)
    }
}

fn serialize() -> String {
    let say = "Hello, World!";
    let foo = Foo {
        value: 42,
        say: say,
    };

    foo.serialize()
}

fn main() {
    println!("{}", serialize());
}

#[test]
fn test_serialize() {
    assert_eq!(serialize(), "{:value 42, :say \"Hello, World!\"}");
}
