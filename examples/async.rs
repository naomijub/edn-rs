use std::str::{self, FromStr};

use edn_rs::{edn, Edn, Vector};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut file = File::open("examples/test_edn.txt").await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    let edn = Edn::from_str(str::from_utf8(&contents).unwrap());
    println!("{edn:?}");

    let edn = edn!([1 1.5 "hello" :key]);
    println!("{edn:?}");
    Ok(())
}
