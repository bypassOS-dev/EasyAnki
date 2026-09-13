use std::{
    io::{self, Write},
};

mod choise1;
mod choise2;
mod choise3;
use choise1::choice_1;
use choise2::choice_2;
use choise3::choice_3;

#[tokio::main]
async fn main() {
    loop {
        println!("\n\n\n\n=====================================================");
        println!("[1] Add new deck of cards");
        println!("[2] Learn/repeat exiting decks card");
        println!("[3] Info");
        println!("=====================================================");
        print!("Your choice: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("[!!!]Read error");
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
    }
}
