// async 関数を作ります
async fn hello() -> String {
    "hello, async fn".to_string()
}

#[tokio::main]
async fn main() {
    // async 関数を実行して結果を待ち合わせます
    let greeting: String = hello().await;
    println!("{}", greeting);

    // async ブロックを実行して結果を待ち合わせます
    let world = async {
        println!("hello, async block");
    };
    world.await;
}