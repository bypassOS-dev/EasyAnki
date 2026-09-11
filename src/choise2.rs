use std::{fs::{self}, io::{self, Write}};
use rand::seq::SliceRandom;
use rand::thread_rng;
use colored::Colorize;

struct Data {
    value: String,
}

pub async fn choice_2() {
    println!("{}", "That's right! Repeatition is most important in education!".green().bold());
    println!("{}", "Your decks of card: \n".green().bold());

    let paths = fs::read_dir("./decks_of_card").unwrap();

    for path in paths {
        let entry = path.unwrap();
        let name = entry.file_name();

        if let Some(file_name) = name.to_str() {
            println!("{}", file_name);
        }
    }

    print!("{}", "\nWrite name the decks of card you wanna repeat: ".bold());
    io::stdout().flush().unwrap();

    let mut name_deck = String::new();
    io::stdin()
        .read_line(&mut name_deck)
        .unwrap();

    let name_deck = name_deck.trim();
    repeat_deck(&name_deck).await;
}

async fn repeat_deck(deck_name: &str) {
    let path = format!("./decks_of_card/{}", deck_name); 
    let content = fs::read_to_string(path).unwrap();

    let mut vector: Vec<Data> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        } 

        let mut first_line = line.splitn(2, ". ");

        let _word_num: i32 = first_line.next().unwrap().trim().parse().unwrap();
        let words = first_line.next().unwrap().trim().to_string();

        vector.push(Data { value: words });
    }

    if vector.is_empty() {
        panic!("FATAL ERROR! IDK WHAT HAPENS WHIS YOUR PC!");
    }

    let mut rng = thread_rng();
    vector.shuffle(&mut rng);

    print!("Do u wanna learn 'normal world --> translate'[1] or 'translate --> normal world'[2]? ");
    io::stdout().flush().unwrap();

    let mut answ = String::new();

    io::stdin()
        .read_line(&mut answ)
        .unwrap();

    let answ1 = answ.trim();
    if answ1 == "1" || answ1 == "2" {
        
    } else {
        println!("Write 1 OR 2 [!!!]");
    }

    let choise: i32 = answ1.parse().unwrap();

    for card in &vector {
        let mut org_and_translate = card.value.splitn(2, ":");

        let org = org_and_translate.next().unwrap().trim();
        let translate = org_and_translate.next().unwrap().trim();

        if choise == 1 {
            let first = org;
            let second = translate;

            
            print!("What is {first}? ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .unwrap();

            if second == answer.trim() {
                println!("{}", "You are right! Move on!".green().bold());
            } else {
                println!("{}{second}", "No! right answer - ".red().bold());
        
            }
        } else {
            let first = translate;
            let second = org;

            
            print!("What is {first}? ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .unwrap();
            if second == answer.trim() {
                println!("{}", "You are right! Move on!".green().bold());
            } else {
                println!("{}{second}", "No! right answer - ".red().bold());
            }
        }
    }
}
