use std::io;
use std::io::{Write};
use std::convert::TryInto;

mod board {
    use std::fmt;
    use std::convert::TryInto;

    const BOARD_SIZE: usize = 40; // 40 squares on the board
    static BOARD: [Position; BOARD_SIZE] = [
            Position::new("Just chillin' at the start", 0, None, None),
            Position::new("Mediterranean Avenue", 1, Some(60), None),
            Position::new("Community Chest", 2, None, None),
            Position::new("Baltic Avenue", 3, Some(60), None),
            Position::new("Income Tax", 4, None, None),
            Position::new("Reading Railroad", 5, Some(200), None),
            Position::new("Oriental Avenue", 6, Some(100), None),
            Position::new("Chance", 7, None, None),
            Position::new("Vermont Avenue", 8, Some(100), None),
            Position::new("Connecticut Avenue", 9, Some(120), None),
            Position::new("Visiting Jail", 10, None, None),
            Position::new("St. Charles Place", 11, Some(140), None),
            Position::new("Electric Company", 12, Some(150), None),
            Position::new("States Avenue", 13, Some(140), None),
            Position::new("Virginia Avenue", 14, Some(160), None),
            Position::new("Pennsylvania Railroad", 15, Some(200), None),
            Position::new("St. James Place", 16, Some(180), None),
            Position::new("Community Chest", 17, None, None),
            Position::new("Tennessee Avenue", 18, Some(180), None),
            Position::new("New York Avenue", 19, Some(200), None),
            Position::new("Free Parking", 20, None, None),
            Position::new("Kentucky Avenue", 21, Some(220), None),
            Position::new("Chance", 22, None, None),
            Position::new("Indiana Avenue", 23, Some(220), None),
            Position::new("Illinois Avenue", 24, Some(240), None),
            Position::new("B. & O. Railroad", 25, Some(200), None),
            Position::new("Atlantic Avenue", 26, Some(260), None),
            Position::new("Ventnor Avenue", 27, Some(260), None),
            Position::new("Water Works", 28, Some(150), None),
            Position::new("Marvin Gardens", 29, Some(280), None),
            Position::new("Go To Jail", 30, None, None),
            Position::new("Pacific Avenue", 31, Some(300), None),
            Position::new("North Carolina Avenue", 32, Some(300), None),
            Position::new("Community Chest", 33, None, None),
            Position::new("Pennsylvania Avenue", 34, Some(320), None),
            Position::new("Short Line", 35, Some(200), None),
            Position::new("Chance", 36, None, None),
            Position::new("Park Place", 37, Some(350), None),
            Position::new("Luxury Tax", 38, None, None),
            Position::new("Boardwalk", 39, Some(400), None)
        ];

    static CHANCE: [Card; 16] = [
            Card::new("GO TO JAIL", None, Some(10)),
            Card::new("Advance to St. Charles Place", None, Some(11)), // TODO: ensure turn 'restarts' after advancing 
            Card::new("Make general repairs on all your property. House, $25 each; Hotel, $100 each", None, None), // TODO: calculate amount
            Card::new("Advance to the next railroad. If unowned, you can buy it. if owned, pay twice the rent", None, None), // TODO: calculate amount
            Card::new("You have been elected chairman of the board. Pay each player $50", Some(50), None), // TODO: calculate amount
            Card::new("Take a trip to Reading Railroad.", None, Some(5)),
            Card::new("Speeding fine. Pay $15", Some(15), None),
            Card::new("Your building load matures. Receive $150", Some(-150), None),
            Card::new("Advance to Boardwalk", None, Some(39)),
            Card::new("Go back three spaces", None, Some(-3)), // TODO: move relative to current position
            Card::new("Advance to Illinois Avenue", None, Some(24)),
            Card::new("Advance to GO. Collect $200", Some(-200), Some(0)),
            Card::new("GET OUT OF JAIL FREE.", None, None), // TODO: player keeps this card
            Card::new("Take all $100 bills from the Bank and throw them in the air", None, None), // TODO: how to model this? Random allocation?
            Card::new("Advance to the nearest railroad. If unowned, you can buy it. If owned, pay twice the rent", None, None), // TODO: go to closest 5,15,25,35. 2x amount
            Card::new("Advance to the nearest utility. If unowned, you can buy it. If owned, roll the dice, and pay the owner 10x the roll", None, None) // TODO: pay relative to roll
    ];

    static COMMUNITY_CHEST: [Card; 16] = [
            Card::new("You are assessed for Street repairs: $40 per House, $115 per Hotel", None, None),
            Card::new("GET OUT OF JAIL FREE", None, None),
            Card::new("You have won second prize in a beauty contest. Collect $10", Some(-10), None),
            Card::new("Life insurance matures. Collect $100", Some(-100), None),
            Card::new("It's your birthday. Collect $10 from each player", Some(-10), None), // TODO: calculate amount
            Card::new("Advance to GO. Collect $200", Some(-200), Some(0)), // TODO: calculate amount
            Card::new("You inherit $100", Some(-100), None),
            Card::new("Bank error in your favor. Collect $200", Some(-200), None),
            Card::new("From sale of stock, you get $50", Some(-50), None),
            Card::new("Collect $25 consultancy fee", Some(-25), None),
            Card::new("Holiday fund matures. Collect $100", Some(-100), None),
            Card::new("Doctor's fees. Pay 50", Some(50), None),
            Card::new("Hospital fees. Pay 1000", Some(100), None),
            Card::new("GO TO JAIL", None, Some(10)),
            Card::new("School fees. Pay $50", None, Some(50)),
            Card::new("Income tax refund. Collect $20", None, Some(-20))
    ];

    pub fn get_position<'a>(pos: usize) -> &'a Position<'a> {
        BOARD.get(pos).unwrap()
    }

    pub struct Card<'a> {
        pub description: &'a str,
        pub pay_amount: Option<i32>,
        pub position: Option<i32>

    }

    impl<'a> Card<'a> {
        pub const fn new(description: &'a str, pay_amount: Option<i32>, position: Option<i32>) -> Self {
            Self {
                description,
                pay_amount,
                position,
            }
        }
    }

    pub struct Position<'a> {
        pub name: &'a str,
        pub idx: i32,
        pub price: Option<i32>,
        pub rent: Option<i32>,
        pub owner: Option<&'a Player>
    //    rent_full_street: i32,
    //    rent_1house: i32,
    //    rent_2house: i32,
    //    rent_3house: i32,
    //    rent_4house: i32,
    //    rent_hotel: i32,
    //    mortgage_cost: i32,
    //    mortgage_redeem: i32,
    }

    impl<'a> Position<'a> {
        pub const fn new(name: &'a str, idx: i32,
                   price: Option<i32>, rent: Option<i32>) -> Self {
            Self {
                name,
                idx,
                price,
                rent,
                owner: None

            }
        }

        /// A player buys the property
        pub fn buy(&mut self, player: &'a mut Player) {
            self.owner = Some(player);
            // FIXME: uncomment
//            player.spend_cash(self.price.unwrap());
        }

        /// Calculate rent
        // Calculate rent, taking into account if a player owns all streets, and the number of
        // properties on the street.
        pub fn calculate_rent(&self) -> i32 {
            match self.rent {
                Some(r) => {
                    r
                    // TODO: Calculate using properties and owner's portfolio
                },
                None => 0
            }
        }
    }

    pub struct Player {
        pub name: String,
        pub position: usize, // the index of the board position
        pub cash: i32,
        is_in_jail: bool
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let pos = BOARD.get(self.position).unwrap();
            write!(f, "--- {} ---
                Position: {}
                Cash: {}
                In Jail?: {}
                Streets:",
                self.name, pos.name, self.cash, self.is_in_jail)
            // TODO: if streets have houses/hotels, print them on the same line
            // TODO: if has get-out-of-jail card, then print it
            // TODO: if has mortgagages, print them
        }
    }

    impl Player {
        pub fn new(name: String) -> Self {
            Self {
                name,
                position: 0,
                cash: 1500, // 2x500, 4x100, 1x50, 1x20, 2x10, 1x5, 5x1
                is_in_jail: false
            }
        }

        /// Advance player
        // Move player to next position
        pub fn advance(&mut self, steps: i32) {
            let steps : usize = steps.try_into().unwrap();
            self.position = (self.position + steps) % BOARD_SIZE;
        }

        /// Go to jail
        // Player doesn't collect 200, and goes straight to jail
        pub fn go_to_jail(&mut self) {
            println!("GO TO JAIL!");
            self.position = 10;
            self.is_in_jail = true;
        }

        /// Receive cash from bank or player
        pub fn receive_cash(&mut self, amount: i32) {
            self.cash += amount;
        }

        /// Pay cash to bank or player
        pub fn spend_cash(&mut self, amount: i32) {
            self.cash -= amount;
        }
    }

    // TODO: Test the player
    // test_advance, to ensure numbers wrap around BOARD_SIZE;
}

/// Print the game summary
// Prints out stats for each player
fn print_game_summary(players: &Vec<board::Player>) {
    for p in players.iter() {
        println!("{}", &p);
    }
    println!("");
}

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
fn capture_names<'a>(player_num: i32) -> Vec<board::Player> {
    let mut players = Vec::<board::Player>::with_capacity(player_num.try_into().unwrap());
    for p in 1..(player_num+1) {
        print!("Enter name for Player {}: ", p);
        let _= io::stdout().flush();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                user_input.pop(); // Remove newline
                players.push(board::Player::new(user_input));
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

/// Capture the roll of the dice
fn capture_dice_roll() -> i32 {
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

/// Execute the turn of a player
// The turn starts with a player moving. Then, once the player is on the new position,
// the rules for that new position execute. Lastly, other players may want to execute 
// transactions
fn execute_turn<'a>(player: &'a mut board::Player) {
    print!("Player {}, roll dice: ", player.name);
    let dice_roll = capture_dice_roll();

    let old_pos_idx = player.position;
    player.advance(dice_roll);
    if player.position < old_pos_idx {
        println!("Yay! You pass begin and collect $200");
        player.receive_cash(200);
    }
    // TODO: If 3 doubles, go to jail
    //
    
    let p = board::get_position(player.position);
    match player.position {
        0 | 10 | 20 => {
            println!("{}", p.name);
        },
        4 => {
            println!("Income Tax! Pay $200");
            player.spend_cash(200);
        },
        38 => {
            println!("Luxury Tax! Pay $100");
            player.spend_cash(100);
        },
        30 => {
            player.go_to_jail();
        },
        2 | 17 | 33 => {
            println!("Community Chest");
            // TODO: Print card
        },
        7 | 22 | 36 => {
            println!("Chance");
            // TODO: Print card
        },
        _ => {
            println!("You landed on {}", p.name);
            match p.owner {
                Some(owner) => {
                    let rent = p.calculate_rent();
                    println!("Oh no! {} already owns it. Pay ${}", owner.name, rent);
                    player.spend_cash(rent);
//                    // FIXME: uncomment
//                    owner.receive_cash(rent);
                },
                None => {
                    let price = p.price.unwrap();
                    println!("Nobody owns it yet. You can buy it for ${}", price);
                    if player.cash > price {
                        // TODO: Buy
                        // FIXME: uncomment
//                        p.buy(player);
                    } else {
                        println!("Not enough money. It stays on the market");
                        // TODO: Implement auction where bank-person inputs player and price
                    }
                }
            }
        }
    }

    // TODO: Allow other users to execute transactions
    // buy/sell houses 
    // buy/sell streets from other players
    // mortgage/unmortgage streets
}

fn main() {
    let player_num = capture_player_num();
    let mut players = capture_names(player_num);
    loop {
        // TODO: Figure out how to mutate players in the loop, or avoid mutating player objects
        for player in players.iter_mut() {
            execute_turn(player);
            println!("");
        }
        print_game_summary(&players);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_roll() {
        assert_eq!(get_dice_roll("Yo".to_string()), Err(()));
        assert_eq!(get_dice_roll("2.3".to_string()), Err(()));
        assert_eq!(get_dice_roll("1".to_string()), Err(()));
        assert_eq!(get_dice_roll("2".to_string()), Ok(2));
        assert_eq!(get_dice_roll("12".to_string()), Ok(12));
        assert_eq!(get_dice_roll("13".to_string()), Err(()));
    }

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
