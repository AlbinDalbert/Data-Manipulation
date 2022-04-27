use std::process::exit;
use terminal_toolbox::*;

const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn vigenere_run(){
    
    // ask for input
    let text = input("Enter text to encrypt or decrypt: ");
    // ask for key
    let key = input("Enter key: ");
    // ask for mode
    let mode = input("Enter mode \n1) encrypt \n2) decrypt]: ");
    
    // let the user choose the mode of operation (encrypt or decrypt)
    println!("{}", match mode.to_lowercase().as_str() {
        "1" => vigenere_encrypt(&text, &key),
        "2" => vigenere_decrypt(&text, &key),
        _ => {
            println!("Invalid mode");
            exit(1);
        }
    });
    
}

fn vigenere_encrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    let mut key_index = 0;
    for c in text.chars() {
        let mut new_c = c;
        if c.is_ascii_lowercase() {
            // lowercase
            let index = ALPHABET_LOWER.find(c).unwrap();
            new_c = ALPHABET_LOWER.chars().nth((index + ALPHABET_LOWER.find(key.chars().nth(key_index).unwrap()).unwrap()) % 26).unwrap();
        }
        else if c.is_ascii_uppercase() {
            // uppercase
            let index = ALPHABET_UPPER.find(c).unwrap();
            new_c = ALPHABET_UPPER.chars().nth((index + ALPHABET_UPPER.find(key.chars().nth(key_index).unwrap()).unwrap()) % 26).unwrap();
        }
        key_index = (key_index + 1) % key.len();
        result.push(new_c);
    }
    result
}

fn vigenere_decrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    let mut key_index = 0;
    for c in text.chars() {
        let mut new_c = c;
        if c.is_ascii_lowercase() {
            // lowercase
            let index = ALPHABET_LOWER.find(c).unwrap();
        }
        else if c.is_ascii_uppercase() {
            // uppercase
            let index = ALPHABET_UPPER.find(c).unwrap();

        }
        key_index = (key_index + 1) % key.len();
        result.push(new_c);
    }
    result
}