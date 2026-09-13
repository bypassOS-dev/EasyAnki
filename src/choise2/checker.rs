use std::char;

pub async fn is_it_ok(word: &str, translate: &str) -> bool{
    if translate.is_empty() {
        return false;
    }

    let word_lettter: Vec<char> = word.chars().collect();
    let translate_letter: Vec<char> = translate.chars().collect();

    let len_word = word_lettter.len();

    let mut idk_how_give_name_this: Vec<(char, usize)> = Vec::new();
    
    for i in 0..len_word {
        let char = word_lettter[i];
        idk_how_give_name_this.push((char, i));
    }

    let mut coincidenes = 0;
    
    for (rigth_char, num_char) in idk_how_give_name_this {
        if let Some(&char)= translate_letter.get(num_char) {
            if rigth_char == char {
                coincidenes += 1;
            }
        }
    }
    
    let result = 100.0 / (len_word as f64);
    let sumary = result * (coincidenes as f64);

    if sumary >= 75.0 {
        return true;
    } else {
        return false;
    }
}