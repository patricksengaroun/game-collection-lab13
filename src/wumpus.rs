// Hunt the Wumpus on an 8-room cube.
use crate::input::{read_int, read_line_trimmed};
use rand::RngExt;

pub fn exits(room: u32) -> (u32, u32, u32) {
    match room {
        1 => (2, 4, 5),
        2 => (1, 3, 6),
        3 => (2, 4, 7),
        4 => (1, 3, 8),
        5 => (1, 6, 8),
        6 => (2, 5, 7),
        7 => (3, 6, 8),
        _ => (4, 5, 7),
    }
}

pub fn is_exit(room: u32, target: u32) -> bool {
    let (a, b, c) = exits(room);
    target == a || target == b || target == c
}

pub fn print_warnings(room: u32, pit: u32, bats: u32, wumpus: u32) {
    if is_exit(room, pit) {
        println!("You feel a draft.");
    }
    if is_exit(room, bats) {
        println!("You hear rustling.");
    }
    if is_exit(room, wumpus) {
        println!("You smell a Wumpus.");
    }
}

// Returns player, pit, bats, Wumpus: four distinct rooms.
pub fn place_hazards() -> (u32, u32, u32, u32) {
    let mut rng = rand::rng();
    loop {
        let player = rng.random_range(1..=8);
        let pit = rng.random_range(1..=8);
        let bats = rng.random_range(1..=8);
        let wumpus = rng.random_range(1..=8);
        if player != pit && player != bats && player != wumpus && pit != bats && pit != wumpus && bats != wumpus {
            return (player, pit, bats, wumpus);
        }
    }
}

pub fn shoot(target: u32, wumpus: u32) -> bool {
    target == wumpus
}

pub fn bat_destination() -> u32 {
    rand::rng().random_range(1..=8)
}

// With probability 3/4 the Wumpus moves to a random exit; otherwise it stays.
pub fn wake_wumpus(wumpus: u32) -> u32 {
    let mut rng = rand::rng();
    if rng.random_range(0..4) == 0 {
        return wumpus;
    }
    let (a, b, c) = exits(wumpus);
    match rng.random_range(0..3) {
        0 => a,
        1 => b,
        _ => c,
    }
}

// Returns Some(message) if the player dies in this room.
fn deadly(room: u32, pit: u32, wumpus: u32) -> Option<&'static str> {
    if room == pit {
        Some("You fell into a pit. You lose.")
    } else if room == wumpus {
        Some("The Wumpus ate you. You lose.")
    } else {
        None
    }
}

pub fn play() {
    let (mut room, pit, bats, mut wumpus) = place_hazards();
    let mut arrows = 3;
    loop {
        let (a, b, c) = exits(room);
        println!("You are in room {room}. Exits: {a}, {b}, {c}.");
        print_warnings(room, pit, bats, wumpus);
        println!("Arrows: {arrows}");
        let action = loop {
            print!("(m)ove or (s)hoot? ");
            std::io::Write::flush(&mut std::io::stdout()).expect("failed to flush");
            let answer = read_line_trimmed();
            if answer == "m" || answer == "s" {
                break answer;
            }
            println!("Type m or s.");
        };
        let target = loop {
            let t = read_int("Which room? ", 1, 8);
            if is_exit(room, t) {
                break t;
            }
            println!("That is not an exit.");
        };
        if action == "m" {
            room = target;
            if room == bats {
                println!("Bats carry you away!");
                room = bat_destination();
            }
            if let Some(msg) = deadly(room, pit, wumpus) {
                println!("{msg}");
                return;
            }
            if room == bats {
                println!("Bats flutter around you but you are unharmed.");
            }
        } else if shoot(target, wumpus) {
            println!("You hit the Wumpus. You win.");
            return;
        } else {
            arrows -= 1;
            println!("You missed.");
            if arrows == 0 {
                println!("You are out of arrows. You lose.");
                return;
            }
            wumpus = wake_wumpus(wumpus);
            if wumpus == room {
                println!("The Wumpus found you. You lose.");
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_exits_are_symmetric() {
        for a in 1..=8 {
            let (x, y, z) = exits(a);
            for b in [x, y, z] {
                assert!(is_exit(b, a));
            }
        }
    }

    #[test]
    fn hazards_are_distinct() {
        for _ in 0..100 {
            let (p, q, r, s) = place_hazards();
            assert!(p != q && p != r && p != s && q != r && q != s && r != s);
        }
    }
}
