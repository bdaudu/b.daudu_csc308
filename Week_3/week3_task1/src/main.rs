use std::io;

fn main() {
    let message = String::from("Hello");
    
    show_message(&message);
    add_note(&message);
    
    println!("Final message: {}", message);
}

fn show_message(msg: &String) {
    println!("Current message: {}", msg);
}

fn add_note(msg: &String) {
    msg.push_str(", world!");
}

