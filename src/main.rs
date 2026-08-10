use rand::Rng;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::Duration;
#[tokio::main]
async fn main() {
    let visit = Arc::new(Mutex::new(0));

    for i in 1..=10 {
        let visit1 = visit.clone();
        tokio::spawn(async move {
            let num = visit1.lock().await;
            let a = get_vall(num);
            println!("Tries {a}");
            let random = rand::thread_rng().gen_range(1..10);
            tokio::time::sleep(Duration::from_secs(random)).await;
            println!("Tries {a} was made!");
        });
        println!("{i} task was started!");
    }
    tokio::time::sleep(Duration::from_secs(10)).await;
}
fn get_vall(mut val: MutexGuard<i32>) -> i32{
    *val = *val + 1;
    let a = *val;
    a
}
