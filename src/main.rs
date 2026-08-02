use tokio::join;
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout, Duration};
use rand::Rng;

async fn connect_to_server() -> String {
    sleep(Duration::from_millis(1000)).await;
    "Подключено к серверу".to_string()
}

async fn authenticate() -> String {
    sleep(Duration::from_millis(1000)).await;
    "Авторизация пройдена".to_string()
}

async fn download_file(id: u32) -> Result<String, String> {
    println!("[ID {}] Waiting for a response from server...", id);
    tokio::time::sleep(Duration::from_millis(random)).await;

    let network_status = rand::thread_rng().gen_bool(0.2);
    let download = if network_status {
        println!("Downloading a file...");
        tokio::time::sleep(Duration::from_millis(5000)).await;
        Ok(format!("Successfully!"))
    }else {
        Err(format!("The error of download of file[!!!]"))
    };

    download
}

#[tokio::main]
async fn main() {
    let conect = connect_to_server();
    let authenticate = authenticate();

    let (a, b) =join!(conect, authenticate);

    println!("{a}");
    println!("{b}");
    //==============================================================================
    for i in 1..=5 {

    }
    let handle1 = tokio::spawn(download_file(1));
    handle1.await;

    // Шаг 3: собрать результаты через while let
    // TODO

    // Шаг 4: статистика
    // TODO
}