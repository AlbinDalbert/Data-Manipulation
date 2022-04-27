use terminal_toolbox::{*, system::*};

use crate::vigenere::vigenere_run;

pub mod caesar;
pub mod vigenere;

fn main() {
    
    let mut system = System::new("Encode".to_string(), Some(TermColor::Green), None);
    
    system.add_program("Caesar".to_string(),caesar::caesar_run, None);
    system.add_program("Vigenere".to_string(), vigenere_run, None);
    
    system.menu();

    println!("bye");
}
