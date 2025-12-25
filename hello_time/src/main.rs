use std::io;
use chrono::Local;

fn main() {
    // Ask for user's name
    println!("Enter your name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim();

    // Get local system time
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");

    // Print output
    println!(
        "Hello {}, right now the time is {}",
        name, current_time
    );
}

