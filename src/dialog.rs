use std::io;
use std::io::{Write};

use super::game::{Game, Dice};
use super::square::Square;
use super::player::Player;

pub enum UserAction {
    BuyHouse,
    SellHouse,
    BuyHotel,
    SellHotel,
    Mortgage,
    Unmortgage,
    SellStreet,
    EndTurn,
    EndGame
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

/// Print options a player has that lacks cash to continue
pub fn trouble_user_actions() -> UserAction {
    println!("1. Sell street to another player");
    println!("2. Sell house");
    println!("3. Sell hotel");
    println!("4. Mortgage street");
    println!("5. Continue");
    println!("0. QUIT (LEAVE GAME)");
    loop {
        print!("Select a valid option: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                match user_input.trim() {
                    "1" => { return UserAction::SellStreet; },
                    "2" => { return UserAction::SellHouse; },
                    "3" => { return UserAction::SellHotel; },
                    "4" => { return UserAction::Mortgage; },
                    "5" => { return UserAction::EndTurn; },
                    "0" => { return UserAction::EndGame; },
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
pub fn capture_dice_roll() -> Dice {
    loop {
        let _= io::stdout().flush();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Did not enter valid numbers");
        user_input.pop(); // Remove newline

        // split 
        match get_dice_roll(user_input) {
            Some(d) => {
                return Dice::new(d.0, d.1);
            },
            None       => {
                println!("Enter 2 numbers between 1 and 6");
                continue;
            }
        };
    }
}

/// Get the dice roll from the user
// This method gets a user input, validates its a number, and the number is within range
// of 2 dice (ie between 2 and 12)
fn get_dice_roll(user_input: String) -> Option<(u32, u32)> {
    // Split by whitespace, and convert each to a number

    let dice: Vec<&str> = user_input.split_whitespace().collect();
    if dice.len() != 2 {
        return None;
    }

    let mut roll = dice.iter()
                       // get string to u32, and throw out invalid inputs
                       .map(|x| x.parse::<u32>())
                       .filter(|x| x.is_ok())
                       .map(|x| x.unwrap())
                       // validate the u32 number is between 1 and 6
                       .filter(|x| x >= &1 && x <= &6)
                       .collect::<Vec<u32>>();
    if roll.len() != 2 {
        return None;
    }
    let last = roll.pop().unwrap();
    Some((roll.pop().unwrap(), last))
}

/// Capture yes/no answer from the user
pub fn yes_no(message: &str) -> bool {
    loop {
        println!("{} (Y/n)", message);
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
pub fn get_player_idx(game: &Game, player: Option<usize>, msg: &str) 
        -> Result<usize, ()> {
    // Do not print current player

    let mut valid_options = Vec::<usize>::new();
    for (i, p) in game.players.iter().enumerate() {
        if p.borrow().left_game() { continue; }; // ignore players who've left the game
        if player.is_some() {
            if player.unwrap() == i { continue; };
        }
        valid_options.push(i+1);
        // Fails because current player is already borrowed somewhere else
        println!("{}: {}", i+1, p.borrow().name());
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
        print!("Enter the purchase price for {}: ", square.name());
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
            let title = format!("{}. {}", i+1, s.1.name());
            let extra: String;
            let sd = s.1.get_street_details()
                      .expect("owned streets doesn't have details");
            let extra = match s.1.asset.borrow().is_mortgaged() {
                true  => format!("unmortgage for ${}", sd.get_unmortgage_amount()),
                false => format!("mortgage for ${}", sd.mortgage()),
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
        assert_eq!(get_dice_roll("13".to_string()), None);
        assert_eq!(get_dice_roll("a 4".to_string()), None);
        assert_eq!(get_dice_roll("0 4".to_string()), None);
        assert_eq!(get_dice_roll("7 4".to_string()), None);
        
        assert_eq!(get_dice_roll("1 1".to_string()), Some(Dice::new(1, 1).roll()));
        assert_eq!(get_dice_roll("6 6".to_string()), Some(Dice::new(6, 6).roll()));
        assert_eq!(get_dice_roll("3 4".to_string()), Some(Dice::new(3, 4).roll()));
    }
}
