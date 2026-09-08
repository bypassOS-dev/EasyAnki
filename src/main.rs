use std::{io::{self, Write}, mem::transmute};

#[tokio::main]
async fn main() {   
    let choice = String::new();
    let result = hello_menu(choice).await;
    println!("{result}")
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
    
    loop {
        name_file.clear();
        print!("\n\nMake name to your new deck of card: ");
        std::io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut name_file)
            .expect("Read error");
        let ok_name = name_file.trim();

        let full_name = format!("{}.txt", ok_name);

        match tokio::fs::OpenOptions::new().write(true).create_new(true).open(&full_name).await {
            Ok(_) => {
                println!("\n File successfully created!");
                break;
            }
            Err(err) => {
                println!("Error: {err}");
                continue;
            }
        }
    }
    print!("\n\nDo u want to add some world in your new deck? ");
    std::io::stdout().flush().unwrap();

    let mut yes_no = String::new();
    io::stdin()
        .read_line(&mut yes_no)
        .expect("Read error");
    let quest = yes_no.trim();
    let lower_string = quest.to_lowercase();

    if lower_string == "yes" || lower_string == "y" ||  lower_string == "" {
        print!("Great!");
        let mut counter = 1;
        loop {
            let mut word = String::new();
            print!("Enter {counter} word or 'stop' if u wanna end: ");
            io::stdin()
                .read_line(&mut word)
                .expect("Read error... Why I am writing it? It will never happen anyway....");
            let normal_word = word.trim();
            let normal_word = normal_word.to_lowercase();
            std::io::stdout().flush().unwrap();

            if let "stop" = normal_word.as_str() {
                break;
            }

            print!("\nEnter translate this ({normal_word}) word: ");
            let mut translate = String::new();

            io::stdin()
                .read_line(&mut translate)
                .expect("Read error... Why I am writing it? It will never happen anyway....");
            let normal_translate = translate.trim();
            let normal_translate = normal_translate.to_lowercase();
            std::io::stdout().flush().unwrap();

            let text_to_append = format!("{counter}. {normal_word}: {normal_translate}\n");

            counter += 1;
        }
    } else {
        println!("Ok! u can change your choice later!");
    }
}
async fn choice_2() {
    println!("That's right! Repeatition is most important in education!");
    println!("Your decks of card:")
    
}
async fn choice_3() {

}
