use terminal_toolbox::{*, system::*};

pub mod caesar;

fn main() {
    
    let mut system = System::new("Encode".to_string(), Some(TermColor::Green), None);
    
    system.add_program("Caesar".to_string(),caesar::caesar_run, None);
    
    system.menu();

    println!("bye");
}
