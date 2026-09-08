use std::io::{self, Write};

#[tokio::main]
async fn main() {   
    let choice = String::new();
    let result = hello_menu(choice).await;
    println!("{result}")
}
async fn hello_menu(mut input: String) -> String{
    println!("\n\n\n\n=====================================================");
    println!("[1] Add new deck os cards");
    println!("[2] Learn/repeat exiting decks card");
    println!("[3] Edit/remove something from all my decks cart");
    println!("=====================================================");
    print!("Your choice: ");
    std::io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("[!!!]Read error");
    println!();

    match input.trim() {
        "1" => {
            choice_1().await;
        }
        "2" => {
            choice_2().await;
        }
        "3" => {
            choice_3().await;
        }
        _ => {
            println!("Hello world!")
        }
    }
    input
}
async fn choice_1 () {
    
}
async fn choice_2() {

}
async fn choice_3() {

}
