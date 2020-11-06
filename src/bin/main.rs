use std::io;
use twitferr::Twitferr;

fn main() {
    let mut twt = Twitferr::new("redis://127.0.0.1:6379/");

    println!("Welcome for Twttr for Rust");
    let choice = ask("1)Login 2)Register. Choose 1 or 2");
    let login = String::from("1");

    println!("{}", choice);

    match choice.trim() {
        "1" => println!("Logging in"),
        "2" => println!("Registering"),
        _ => println!("Option not recognized"),
    }
}

fn ask(question: &str) -> String {
    println!("{}", question);
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice
}
