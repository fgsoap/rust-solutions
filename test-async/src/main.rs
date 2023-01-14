use futures::executor::block_on;

fn main() {
    block_on(greeter())
}

async fn greeter() {
    println!("Hello, girl!");
    hello().await;
}

async fn hello() {
    println!("Hello, boy!")
}
