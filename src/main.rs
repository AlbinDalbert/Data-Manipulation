use terminal_toolbox::{*, system::*};

pub mod caesar;

fn main() {
    
    let mut system = System::new("Encode".to_string(), Some(TermColor::Green), None);

    //let p = program::Program::new("prog".to_string(), run, None, 0);
    
    system.add_program("Caesar".to_string(),caesar::caesar_run, None);
    
    system.menu();

    println!("bye");
}
