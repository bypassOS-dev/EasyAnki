use rand::Rng;
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    let visits = Arc::new(0);

    for i in 1..=10 {
        tokio::spawn(async move {

        });
        println!("{i} task was started!");
    }
}
