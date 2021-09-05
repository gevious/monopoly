use std::io;
use std::io::{Write};
use std::convert::TryInto;

/// Get the dice roll from the user
// This method gets a user input, validates its a number, and the number is within range
// of 2 dice (ie between 2 and 12)
fn get_dice_roll(user_input: String) -> Result<i32, ()> {
    // Convert to number
    let dice_roll = match user_input.parse::<i32>() {
        Ok(d) => d,
        Err(_e) => {
            return Err(());
        }
    };
    match dice_roll {
        2..=12 => Ok(dice_roll),
        _ => {
            Err(())
        }
    }
}

/// Get the number of players
fn get_player_num(user_input: String) -> Result<i32, ()> {
    // Convert to number
    let player_num = match user_input.parse::<i32>() {
        Ok(d) => d,
        Err(_e) => {
            return Err(());
        }
    };
    match player_num {
        2..=8 => Ok(player_num),
        _ => {
            Err(())
        }
    }
}

/// Capture the name of a player
fn capture_names(player_num: i32) -> Vec<String> {
    let mut players = Vec::<String>::with_capacity(player_num.try_into().unwrap());
    for mut p in 1..(player_num+1) {
        print!("Enter name for Player {}: ", p);
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(name) => {
                let mut player_name = name.to_string();
                player_name.pop(); // Remove newline
                players.push(player_name);
            },
            Err(_) => {
                println!("Invalid name");
                p = p - 1; // repeat player
            }
        };
    }
    players
}

/// Capture the amount of players
fn capture_player_num() -> i32 {
    loop {
        print!("Enter amount of players: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Did not enter a valid number");
        user_input.pop(); // Remove newline

        match get_player_num(user_input) {
            Ok(amt) => {
                return amt;
            },
            Err(_) => {
                println!("Enter a valid number between 1 and 8");
            }
        };
    }
}

/// Capture the roll of the dice
fn capture_dice_roll() -> i32 {
    loop {
        print!("Enter dice roll: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Did not enter a valid number");
        user_input.pop(); // Remove newline

        match get_dice_roll(user_input) {
            Ok(roll) => {
                return roll;
            },
            Err(_) => {
                println!("Enter a number between 2 and 12");
                continue;
            }
        };
    }
}

fn main() {
    let player_num = capture_player_num();
    let players = capture_names(player_num);
    let dice_roll = capture_dice_roll();
    println!("You rolled {}", dice_roll);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_valid_number() {
        assert_eq!(get_dice_roll("Yo".to_string()), Err(()));
        assert_eq!(get_dice_roll("2.3".to_string()), Err(()));
    }

    #[test]
    fn test_dice_number_in_range() {
        assert_eq!(get_dice_roll("1".to_string()), Err(()));
        assert_eq!(get_dice_roll("2".to_string()), Ok(2));
        assert_eq!(get_dice_roll("12".to_string()), Ok(12));
        assert_eq!(get_dice_roll("13".to_string()), Err(()));
    }

    #[test]
    fn test_player_num_in_range() {
        assert_eq!(get_player_num("1".to_string()), Err(()));
        assert_eq!(get_player_num("2".to_string()), Ok(2));
        assert_eq!(get_player_num("8".to_string()), Ok(8));
        assert_eq!(get_player_num("9".to_string()), Err(()));
    }
}
