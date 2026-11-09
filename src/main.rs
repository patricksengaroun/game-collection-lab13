mod game_of_life;
mod input;
mod wumpus;

use input::read_int;

fn main() {
    println!("=== GAME COLLECTION ===");
    loop {
        println!();
        println!("1. Mad Libs");
        println!("2. Hunt the Wumpus");
        println!("3. Bulls and Cows");
        println!("4. Tic-Tac-Toe");
        println!("5. Battleship");
        println!("6. Game of Life");
        println!("0. Quit");
        match read_int("Choice: ", 0, 6) {
            0 => break,
            2 => wumpus::play(),
            6 => game_of_life::play(),
            _ => println!("Coming soon."),
        }
    }
}
