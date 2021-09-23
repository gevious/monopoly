use std::io;
use std::io::{Write};

use super::game::{Game, Player, Square};

pub enum UserAction {
    BuyHouse,
    SellHouse,
    BuyHotel,
    SellHotel,
    Mortgage,
    Unmortgage,
    SellStreet,
    EndTurn
}

/// Print actions a player can make outside of their turn
pub fn additional_user_actions(game: &Game) -> UserAction {
    println!("1. Sell street to another player");
    println!("2. Buy house (coming soon)");
    println!("3. Sell house (coming soon)");
    println!("4. Buy hotel (coming soon)");
    println!("5. Sell hotel (coming soon)");
    println!("6. Mortgage street (coming soon)");
    println!("7. Unmortgage street (coming soon)");
    println!("0. End turn");
    loop {
        print!("Select a valid option: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                match user_input.trim() {
                    "0" => return UserAction::EndTurn,
                    "1" => {
                        return UserAction::SellStreet;
                    },
                    "2" => {
                        return UserAction::BuyHouse;
                    },
                    "3" => {
                        return UserAction::SellHouse;
                    },
                    "4" => {
                        return UserAction::BuyHotel;
                    }
                    "5" => {
                        return UserAction::SellHotel;
                    },
                    "6" => {
                        return UserAction::Mortgage;
                    },
                    "7" => {
                        return UserAction::Unmortgage;
                    },
                    _  => println!("Invalid option. Try again")
                }
            },
            Err(_) => {
                println!("Invalid Input. Try again");
            }
        }
    }
}

/// Capture the roll of the dice
pub fn capture_dice_roll() -> u32 {
    loop {
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

/// Get the dice roll from the user
// This method gets a user input, validates its a number, and the number is within range
// of 2 dice (ie between 2 and 12)
fn get_dice_roll(user_input: String) -> Result<u32, ()> {
    // Convert to number
    let dice_roll = match user_input.parse::<u32>() {
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


/// Capture yes/no answer from the user
pub fn want_to_buy_property(square: &Square) -> bool {
    loop {
        println!("Do you want to buy {} for ${}? (Y/n)", square.name, square.get_price());
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                match user_input.trim() {
                    "Y" | "y" | ""  => return true,
                    "N" | "n" | "no"  => return true,
                    _ => println!("Invalid Input. Try again")
                }
            },
            Err(_) => {
                println!("Invalid Input. Try again");
            }
        }
    }
}

/// Capture the idx of a player from the user
// This method is useful for out-of-band transactions. These include auctions and ad-hoc selling of property to others
pub fn get_player_idx(game: &Game, player: Option<&Player>) -> usize {
    // Do not print current player
    let mut valid_options = Vec::<usize>::new();
    for i in 0..game.players.len() {
        if player != None && i == player.unwrap().turn_idx {
            continue;
        }
        valid_options.push(i);
        println!("{}: {}", i, game.players.get(i).unwrap().borrow().name);
    }

    loop { // repeat until player enters a valid selection
        print!("Enter the player number: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                let player_no = match user_input.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input. Try again");
                        continue;
                    }
                };
                match valid_options.contains(&player_no) {
                    true => {
                        return player_no;
                    },
                    false => {
                        println!("Invalid selection. Try again");
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("Invalid selection. Try again");
            }
        }
    }
}

/// Capture purchase price for property from the user
pub fn get_purchase_price(square: &Square) -> u32 {
    loop { // repeat until player enters a valid selection
        print!("Enter the purchase price for {}: ", square.name);
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                let player_no = match user_input.parse::<u32>() {
                    Ok(n) => return n,
                    Err(_) => {
                        println!("Invalid input. Try again");
                        continue;
                    }
                };
            },
            Err(_) => {
                println!("Invalid selection. Try again");
            }
        }
    }
}

/// Get the street name from user, and return the index on the board where the street is
pub fn get_street(game: &Game) -> String {
    loop { // repeat until player enters a valid selection
        print!("Enter the street: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                return String::from(user_input.trim());
            },
            Err(_) => {
                println!("Invalid selection. Try again");
            }
        }
    }
}

/// Get an amount from the user
pub fn get_amount() -> u32 {
    loop { // repeat until player enters a valid selection
        print!("Enter the amount: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                let player_no = match user_input.parse::<u32>() {
                    Ok(n) => return n,
                    Err(_) => {
                        println!("Invalid input. Try again");
                        continue;
                    }
                };
            },
            Err(_) => {
                println!("Invalid selection. Try again");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_dice() {
        assert_eq!(get_dice_roll("Yo".to_string()), Err(()));
        assert_eq!(get_dice_roll("2.3".to_string()), Err(()));
        assert_eq!(get_dice_roll("1".to_string()), Err(()));
        assert_eq!(get_dice_roll("2".to_string()), Ok(2));
        assert_eq!(get_dice_roll("12".to_string()), Ok(12));
        assert_eq!(get_dice_roll("13".to_string()), Err(()));
    }
}
