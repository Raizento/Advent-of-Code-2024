#[tokio::main]
async fn main() {
    tokio::spawn(async {
        test().await
    });
}

async fn test() -> String {
    println!("Moin");
    String::from("Moin")
}
