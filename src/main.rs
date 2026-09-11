use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;
use std::{
    fs::{self},
    io::{self, Write},
};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

mod choise1;
mod choise2;
use choise1::choice_1;
use choise2::choice_2;

#[tokio::main]
async fn main() {
    loop {
        println!("\n\n\n\n=====================================================");
        println!("[1] Add new deck of cards");
        println!("[2] Learn/repeat exiting decks card");
        println!("[3] Edit/remove something from all my decks cart");
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

async fn choice_3() {
    println!("You are lazybones! just change file bro!");
}
