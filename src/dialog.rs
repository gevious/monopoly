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
pub fn additional_user_actions() -> UserAction {
    println!("1. Sell street to another player");
    println!("2. Buy house");
    println!("3. Sell house");
    println!("4. Buy hotel");
    println!("5. Sell hotel");
    println!("6. Mortgage street");
    println!("7. Unmortgage street");
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
            Some(roll) => {
                return roll;
            },
            None       => {
                println!("Enter a number between 2 and 12");
                continue;
            }
        };
    }
}

/// Get the dice roll from the user
// This method gets a user input, validates its a number, and the number is within range
// of 2 dice (ie between 2 and 12)
fn get_dice_roll(user_input: String) -> Option<u32> {
    // Convert to number
    let dice_roll = match user_input.parse::<u32>() {
        Ok(d) => d,
        Err(_e) => {
            return None;
        }
    };
    match dice_roll {
        2..=12 => Some(dice_roll),
        _ => {
            None
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
                    "N" | "n" | "no"  => return false,
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
pub fn get_player_idx(game: &Game, player: Option<&Player>, msg: &str) 
        -> Result<usize, ()> {
    // Do not print current player
    let mut valid_options = Vec::<usize>::new();
    for i in 0..game.players.len() {
        match player {
            None  => {},
            Some(p) => if p.turn_idx == i { continue; }
        }
        valid_options.push(i+1);
        println!("{}: {}", i+1, game.players.get(i).unwrap().borrow().name);
    }
    println!("q: Quit, and return to the menu");

    loop { // repeat until player enters a valid selection
        print!("{}: ", &msg);
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                if user_quit(&user_input) {
                    return Err(());
                }

                let player_no = match user_input.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input. Try again");
                        continue;
                    }
                };
                match valid_options.contains(&player_no) {
                    true => {
                        return Ok(player_no-1); // user menu starts from 1 (not 0)
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
pub fn get_purchase_price(square: &Square) -> Result<u32, ()> {
    loop { // repeat until player enters a valid selection
        print!("Enter the purchase price for {}: ", square.name);
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                if user_quit(&user_input) {
                    return Err(());
                }
                match user_input.parse::<u32>() {
                    Ok(n) => return Ok(n),
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
pub fn get_street(eligible_streets: Vec<(usize, &Square)>) -> Result<usize, ()> {
    if eligible_streets.len() == 0 {
        println!("No matching streets");
        return Err(());
    }
    loop { // repeat until player enters a valid selection
        for (i, s) in eligible_streets.iter().enumerate() {
            let title = format!("{}. {}", i+1, s.1.name);
            let extra: String;
            let sd = s.1.get_street_details()
                      .expect("owned streets doesn't have details");
            let extra = match s.1.asset.borrow().is_mortgaged() {
                true  => format!("unmortgage for ${}", sd.get_unmortgage_amount()),
                false => format!("mortgage for ${}", sd.mortgage),
            };
            println!("{} ({})", title, &extra);
        }
        println!("q: Quit, and return to the menu");
        print!("Enter the street: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                if user_quit(&user_input) {
                    return Err(());
                }
                match user_input.parse::<usize>() {
                    Ok(n)  => {
                        if n <= eligible_streets.len() {
                            return Ok(
                                eligible_streets.get(n-1).expect("Street expected").0);
                        }
                        println!("Invalid option selected. Try again");
                    },
                    Err(_) => println!("Invalid street selected. Try again")
                }
            },
            Err(_) => {
                println!("Invalid selection. Try again");
            }
        }
    }
}

/// Get an amount from the user
pub fn get_amount() -> Result<u32, ()> {
    loop { // repeat until player enters a valid selection
        print!("Enter the amount (or 'q' to return to the menu): ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                if user_quit(&user_input) {
                    return Err(());
                }
                match user_input.parse::<u32>() {
                    Ok(n) => return Ok(n),
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

fn user_quit(user_input: &str) -> bool {
    match user_input.trim() {
        "Q" | "q" => true,
        _         => false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn did_user_quit() {
        assert_eq!(user_quit("q"), true);
        assert_eq!(user_quit("Q"), true);
        assert_eq!(user_quit("i"), false);
        assert_eq!(user_quit("0"), false);
    }

    #[test]
    fn roll_dice() {
        assert_eq!(get_dice_roll("Yo".to_string()), None);
        assert_eq!(get_dice_roll("2.3".to_string()), None);
        assert_eq!(get_dice_roll("1".to_string()), None);
        assert_eq!(get_dice_roll("2".to_string()), Some(2));
        assert_eq!(get_dice_roll("12".to_string()), Some(12));
        assert_eq!(get_dice_roll("13".to_string()), None);
    }
}
