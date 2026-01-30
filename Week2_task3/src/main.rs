use std::io;

fn add_surname_to_firstname(name: &mut String){
    name.push_str(" Daudu");
}


fn main() {
    let mut name = String::from("Basil ");
    add_surname_to_firstname(&mut name);
    println!("{}", name);
}
