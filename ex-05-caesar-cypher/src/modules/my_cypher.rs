use serde_json::{json, Value};

pub fn caesar_encrypt(word: String, shift: i32) -> Value {
    let the_math = |letter: char| -> char {
        let char_code = letter as u32 + shift as u32;
        char::from_u32(((char_code - 97)%26) + 97).unwrap_or_default()
    };
    let new_word = caesar(word.to_ascii_lowercase(), the_math);

    // I needed to do this janky thing, because otherwise it adds " to the ends, making "\"[word]\"", ugly
    json!({ "word": new_word.as_str()[1..new_word.len()-1] })
}

pub fn caesar_decrypt(word: String, shift: i32) -> Value {
    let the_math = |letter: char| -> char {
        let char_code = letter as u32 - shift as u32;
        char::from_u32((((char_code+26) - 97)%26) + 97).unwrap_or_default()
    };
    let new_word = caesar(word.to_ascii_lowercase(), the_math);

    // I needed to do this janky thing, because otherwise it adds " to the ends, making "\"[word]\"", ugly
    json!({ "word": new_word.as_str()[1..new_word.len()-1] })
}

fn caesar(word: String, the_math: impl Fn(char) -> char) -> String {
    let mut new_word = String::new();
    for w in word.chars() {
        let letu8 = w as u8;
        if letu8 > 96 && letu8 < 123 {
            new_word.push(the_math(w));
        } else {
            new_word.push(w);
        }
    }
    new_word
}