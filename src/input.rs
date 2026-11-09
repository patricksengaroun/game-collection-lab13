// Shared input helpers.
use std::io::{self, Write};

pub fn read_line_trimmed() -> String {
    let mut line = String::new();
    let bytes = io::stdin().read_line(&mut line).expect("failed to read input");
    if bytes == 0 {
        // End of input (Ctrl-D or a closed pipe): stop instead of re-prompting forever.
        println!();
        std::process::exit(0);
    }
    line.trim().to_string()
}

// Re-prompts until the player types a whole number from min to max.
pub fn read_int(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("failed to flush");
        match read_line_trimmed().parse::<u32>() {
            Ok(n) if n >= min && n <= max => return n,
            Ok(_) => println!("Enter a number from {min} to {max}."),
            Err(_) => println!("That is not a number."),
        }
    }
}
