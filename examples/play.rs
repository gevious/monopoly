use std::io;
use std::io::{Write};
use std::convert::TryInto;

use monopoly::{game};

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
    for p in 1..(player_num+1) {
        print!("Enter name for Player {}: ", p);
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                players.push(user_input);
            },
            Err(_) => {
                println!("Invalid name");
                // TODO: if this happens, figure out how to repeat the user
            }
        };
    }
    players
}

/// Capture the amount of players
fn capture_player_num() -> i32 {
    loop {
        print!("How many players are there? ");
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

fn main() {
    let player_num = capture_player_num();
    let players = capture_names(player_num);
    let game = game::init(players);
    game.start();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_player_num() {
        assert_eq!(get_player_num("Yo".to_string()), Err(()));
        assert_eq!(get_player_num("2.3".to_string()), Err(()));
        assert_eq!(get_player_num("1".to_string()), Err(()));
        assert_eq!(get_player_num("2".to_string()), Ok(2));
        assert_eq!(get_player_num("8".to_string()), Ok(8));
        assert_eq!(get_player_num("9".to_string()), Err(()));
    }
}
