use std::{fs::{self, File}, io::{self, BufRead, BufReader, Write}};
use std::path::PathBuf;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use rand::seq::SliceRandom;
use rand::thread_rng;
#[tokio::main]
async fn main() {   
    let choice = String::new();
    let _result = hello_menu(choice).await;
}
async fn hello_menu(mut input: String) -> String{
    println!("\n\n\n\n=====================================================");
    println!("[1] Add new deck of cards");
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
    let mut name_file = String::new();
    
    let full_name = loop {
        name_file.clear();
        print!("\n\nMake name to your new deck of card: ");
        std::io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut name_file)
            .expect("Read error");
        let ok_name = name_file.trim();

        let full_name = PathBuf::from("decks_of_card").join(ok_name);

        match tokio::fs::OpenOptions::new().write(true).create_new(true).open(&full_name).await {
            Ok(_) => {
                println!("\n File successfully created!");
                break full_name;
            }
            Err(err) => {
                println!("Error: {err}");
                continue;
            }
        }
    };
    print!("\n\nDo u want to add word in your new deck(yes/no)? ");
    std::io::stdout().flush().unwrap();

    let mut yes_no = String::new();
    io::stdin()
        .read_line(&mut yes_no)
        .expect("Read error");
    let quest = yes_no.trim();
    let lower_string = quest.to_lowercase();

    if lower_string == "yes" || lower_string == "y" ||  lower_string == "" {
        print!("Great!");
        std::io::stdout().flush().unwrap();

        let mut counter = 1;
        loop {
            if counter != 1{
                let mut move_on = String::new();
                print!("Move on (yes/no)? ");
                io::stdout().flush().unwrap();
                
                io::stdin()
                .read_line(&mut move_on)
                .unwrap();
                let move_on = move_on.trim();
                let move_on = move_on.to_lowercase();
                if move_on == "no" || move_on == "n" {
                    println!("Ok! Save the list...");
                    break;
                }
            }

            let mut word = String::new();
            print!("Enter {counter} word or 'stop' if u wanna end: ");
            std::io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut word)
                .expect("Read error... Why I am writing it? It will never happen anyway....");
            let normal_word = word.trim();
            let normal_word = normal_word.to_lowercase();
            
            if let "stop" = normal_word.as_str() {
                break;
            }

            print!("\nEnter translate this ({normal_word}) word: ");
            std::io::stdout().flush().unwrap();

            let mut translate = String::new();
            io::stdin()
                .read_line(&mut translate)
                .expect("Read error... Why I am writing it? It will never happen anyway....");
            let normal_translate = translate.trim();
            let normal_translate = normal_translate.to_lowercase();

            let text_to_append = format!("{counter}. {normal_word}: {normal_translate}\n");

            let mut file = OpenOptions::new()
                .append(true)
                .open(full_name.clone())
                .await
                .unwrap();

            file.write_all(text_to_append.as_bytes()).await.unwrap();

            counter += 1;
        }
    } else {
        println!("Ok! u can change your choice later!");
    }
}
async fn choice_2() {
    println!("That's right! Repeatition is most important in education!");
    println!("Your decks of card: ");

    let paths = fs::read_dir("./decks_of_card").unwrap();

    for path in paths {
        let entry = path.unwrap();
        let name = entry.file_name();

        if let Some(file_name) = name.to_str() {
            println!("{}", file_name);
        }
    }

    print!("Write name the decks of card you wanna repeat: ");
    io::stdout().flush().unwrap();

    let mut name_deck = String::new();
    io::stdin()
        .read_line(&mut name_deck)
        .unwrap();
    


}
async fn choice_3() {

}

async fn repeat_deck(deck_name: &str) {
    let path = format!("./decks_of_card/{}", deck_name); 
    let content = fs::read_to_string(path).unwrap();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        } 

        let mut first_line = line.splitn(2, ". ");

        let word_num = first_line.next().unwrap().trim();
        let words = first_line.next().unwrap().trim();

        let mut words = words.splitn(2, " = ");

        let org_word = words.next().unwrap().trim();
        let translate = words.next().unwrap().trim();

    }
}
async fn generate_random(deck_name: &str) -> Vec<&str>{
    let path = format!("./decks_of_card/{}", deck_name);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let line_count = reader.lines().count(); 

    todo!()
}
