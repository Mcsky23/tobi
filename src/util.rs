use std::io;

pub fn are_you_sure(verif: &String) -> bool {
    let mut input = String::new();
    println!("! This action cannot be undone !");
    println!("Type '{}' to confirm: ", verif);
    io::stdin().read_line(&mut input).unwrap();

    input.trim() == verif
}