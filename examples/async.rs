use edn_rs::{edn, Edn, Vector};
use futures::Future;

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
