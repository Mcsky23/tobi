use std::io;
use colored::Colorize;

pub fn are_you_sure(verif: &String) -> bool {
    let mut input = String::new();
    println!("! This action cannot be undone !");
    println!("Type '{}' to confirm: ", verif);
    io::stdin().read_line(&mut input).unwrap();

    input.trim() == verif
}

pub fn progress_bar(done: usize, total: usize) -> String {
    let bar_len = 15;
    let done_normalized = (done as f32 / total as f32 * bar_len as f32).round() as usize;
    let progress_bar = format!("{}{}", "X".repeat(done_normalized).green().on_green(), "X".repeat(bar_len - done_normalized).red().on_red());
    progress_bar
}