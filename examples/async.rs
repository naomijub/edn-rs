use edn_rs::{edn, Double, Edn, Vector};
use futures::prelude::*;
use futures::Future;
use tokio::prelude::*;

async fn foo() -> impl Future<Output = Edn> + Send {
    edn!([1 1.5 "hello" :key])
}

#[tokio::main]
async fn main() {
    let edn = foo().await.await;

    println!("{}", edn.to_string());
    assert_eq!(edn, edn!([1 1.5 "hello" :key]));

    assert_eq!(edn[1].to_float(), Some(1.5f64));
}
