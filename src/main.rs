use tokio::join;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use rand::Rng;

async fn connect_to_server() -> String {
    sleep(Duration::from_millis(1000)).await;
    "Comnected to server".to_string()
}

async fn authenticate() -> String {
    sleep(Duration::from_millis(1000)).await;
    "authenticate is pass".to_string()
}

async fn download_file(id: i32) -> Result<String, String> {
    let random = rand::thread_rng().gen_range(1..=5000);

    println!("[ID {}] Waiting for a response from server...", id);
    tokio::time::sleep(Duration::from_millis(random)).await;

    let network_status = rand::thread_rng().gen_bool(0.8);
    let download = if network_status {
        println!("[ID {}]Downloading a file...", id);
        tokio::time::sleep(Duration::from_millis(4000)).await;
        Ok(format!("[ID {}]Successfully!", id))
    }else {
        Err(format!("[ID {}]The error of download of file[!!!]", id))
    };

    download
}

#[tokio::main]
async fn main() {
    let (tx,mut rx) = mpsc::channel(32);

    let conect = connect_to_server();
    let authenticate = authenticate();

    let (a, b) =join!(conect, authenticate);

    println!("{a}");
    println!("{b}");
    //==============================================================================
    for i in 1..=5 {
        let tx1 = tx.clone();
        tokio::spawn(async move {
            let download = tokio::time::timeout(Duration::from_millis(2000),download_file(i)).await;
            tx1.send((download, i)).await.unwrap();
        });
    }
    drop(tx);
    while let Some((response, id)) = rx.recv().await{
        let response = match response {
            Ok(Ok(msg)) => msg,
            Ok(Err(err)) => err,
            Err(_) => format!("Error of timeout"),
        };
        println!("[ID {}]RESULT[!!!]", id);
        println!("[ID {}]Conclusion: {}",id, response);
        println!("");
    }
}
