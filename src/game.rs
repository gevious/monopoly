use rand::thread_rng;
use rand::seq::SliceRandom;

use std::cell::RefCell;

use super::{dialog, publisher};

const BOARD_SIZE: u32 = 40; // 40 squares on the board
enum CardAction {
    Movement,
    MovementRelative, // move relative to starting square
    Payment,
    PaymentDice, // payment calculated based on dice roll
    PaymentPlayers, // payment calculated based on dice roll
    Jail, 
    JailRelease, 
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum SquareType {
    ChanceCard,
    CommunityCard,
    Corner,
    Station,
    Street,
    Tax,
    Utility
}

#[derive(Eq, PartialEq, Debug)]
pub struct Suburb { // Name and building price (per building)
    pub color: String,
    building_price: u32
}

pub struct Asset {
    pub owner: Option<usize>, // usize is a reference to a players turn_idx
    house_num: u32,
    has_hotel: bool,
    is_mortgaged: bool
}

struct Card {
    description: String,
    action: CardAction,
    amount: Option<i32>,
    square: Option<u32>
}

/// The structure, containing links to all parts of the game
pub struct Game {
    pub players: Vec<RefCell<Player>>,
    pub board: [Square; BOARD_SIZE as usize],
    chance_cards: RefCell<Vec<Card>>,
    community_cards: RefCell<Vec<Card>>,
    is_unit_test: bool
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Player {
    pub name: String,
    pub position: usize, // the index of the board square
    pub turn_idx: usize, // idx in the suburb of players. need this to match asset
    pub cash: u32,
    pub is_in_jail: bool,
    pub num_get_out_of_jail_cards: u32,
}


pub struct StreetDetails {
    pub suburb: Option<Suburb>,
    pub price: u32,
    pub rent: u32,
    pub rent_suburb: [u32; 6], // suburb, house 1..4, hotel
    pub mortgage: u32,
}

pub struct Square {
    pub name: String,
    pub square_type: SquareType,
    street_details: Option<StreetDetails>,
    pub asset: RefCell<Asset>
}

impl Asset {
    pub const fn new() -> Self {
        Self { 
            owner: None,
            house_num: 0,
            has_hotel: false,
            is_mortgaged: false
        }
    }

    pub fn is_mortgaged(&self) -> bool {
        self.is_mortgaged
    }

    pub fn mortgage(&mut self) {
        self.is_mortgaged = true;
    }

    pub fn unmortgage(&mut self) {
        self.is_mortgaged = false;
    }

    pub fn house_num(&self) -> u32 {
        self.house_num
    }

    pub fn has_hotel(&self) -> bool {
        self.has_hotel
    }

    pub fn buy_house(&mut self) -> Result<(), String> {
        match self.house_num < 4 {
            true => {
                self.house_num += 1;
                Ok(())
            },
            false => Err(String::from("This street cannot have more houses"))
        }
    }

    pub fn sell_house(&mut self) -> Result<(), String> {
        // validate street can sell house
        match self.house_num > 0 {
            true => { 
                self.house_num -= 1;
                Ok(())
            },
            false => Err(String::from("This street has no houses"))
        }
    }

    pub fn buy_hotel(&mut self) -> Result<(), String> {
        if self.has_hotel() {
            return Err(String::from("This street cannot have more hotels"));
        }
        if self.house_num != 4 {
            return Err(String::from("You need 4 houses before you can buy a hotel"));
        }
        self.has_hotel = true;

        Ok(())
    }

    pub fn sell_hotel(&mut self) {
        // street can always sell hotel
        self.has_hotel = false;
    }
}

impl Card {
    pub fn new(description: &str, action: CardAction, amount: Option<i32>, square: Option<u32>) -> Self {
        Self {
            description: description.to_string(),
            action,
            amount,
            square
        }
    }
}

impl Game {

    /// Start the game
    pub fn start(self) {
        loop {
            for p_ref in self.players.iter() {
                {
                    let mut player = p_ref.borrow_mut();
                    print!("\n{}, roll dice: ", player.name);
                    let dice_roll = dialog::capture_dice_roll();
                    self.execute_turn(&mut *player, dice_roll);
                }

                // present options of other transactions user can make
                if !self.is_unit_test {
                    self.execute_user_action();
                }
            }
        }
    }

    fn execute_user_action(&self) {
        loop {
            publisher::publish(&self);
            let option = dialog::additional_user_actions();
            match option {
                dialog::UserAction::EndTurn => return,
                dialog::UserAction::SellStreet => {
                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the current owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut orig_owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == orig_owner.turn_idx
                            }
                        })
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let player_idx = match dialog::get_player_idx(
                            self, Some(&*orig_owner), "Select the new owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut new_owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();


                    let purchase_price = match dialog::get_amount() {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    actions::sell_street(&self, &mut orig_owner, &mut new_owner,
                                         street_idx, purchase_price);
                },
                dialog::UserAction::BuyHouse => {
                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx
                            }
                        })
                        .filter(|(_, s)| s.asset.borrow().house_num() < 4 )
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    actions::buy_house(&self, &mut owner, street_idx);
                },
                dialog::UserAction::SellHouse => {
                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(o) => o == owner.turn_idx
                            }
                        })
                        .filter(|(_, s)| s.asset.borrow().house_num() > 0 )
                        .filter(|(_, s)| !s.asset.borrow().has_hotel())
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    actions::sell_house(&self, &mut owner, street_idx);
                },
                dialog::UserAction::BuyHotel => {
                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx
                            }
                        })
                        .filter(|(_, s)| s.asset.borrow().house_num() == 4)
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    actions::buy_hotel(&self, &mut owner, street_idx);
                },
                dialog::UserAction::SellHotel => {
                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx
                            }
                        })
                        .filter(|(_, s)| s.asset.borrow().has_hotel())
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    actions::sell_hotel(&self, &mut owner, street_idx);
                },
                dialog::UserAction::Mortgage => {
                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the current owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();


                    let eligible_streets :Vec<(usize, &Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx
                            }
                        })
                        .filter(|(_, s)| !s.asset.borrow().is_mortgaged )
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    actions::mortgage_street(&self, &mut owner, street_idx);
                },
                dialog::UserAction::Unmortgage => {

                    let player_idx = match dialog::get_player_idx(
                            self, None, "Select the current owner") {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };

                    let mut owner = self.players.get(player_idx)
                                        .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &Square)> = self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx
                            }
                        })
                        .filter(|(_, s)| s.asset.borrow().is_mortgaged )
                        .collect();
                    let street_idx = match dialog::get_street(eligible_streets) {
                        Ok(s)  => s,
                        Err(_) => {
                            println!("Back to the menu");
                            continue;
                        }
                    };


                    actions::unmortgage_street(&self, &mut owner, street_idx);
                }
            }
        }
    }

    /// Calculate if a street has houses built on it
    fn has_buildings(&self, street_idx: usize) -> bool {
        let a = &self.board.get(street_idx).expect("Street should exist")
                     .asset.borrow();
        a.has_hotel || a.house_num > 0
    }

    /// Capture player name, and price, and complete purchase
    fn auction(&self, player: &Player, square: &Square) {
        println!("Auction!!");
        let owner_idx = match dialog::get_player_idx(self, Some(player),
                                                     "Select the new owner") {
            Ok(o)  => o,
            Err(_) => {
                println!("Back to the menu");
                return;
            }
        };

        let purchase_price = match dialog::get_purchase_price(square) {
            Ok(p) => p,
            Err(_) => {
                println!("Back to the menu");
                return;
            }
        };

        let mut owner = self.players.get(owner_idx).expect("Player should exist")
                            .borrow_mut();
        self.buy_property(&mut *owner, square, purchase_price);
    }


    /// Update game state to unit test
    // This mode will eliminate questions to the user that require keyboard input
    pub fn set_unit_test(&mut self) {
        self.is_unit_test = true;
    }

    /// Execute action on card
    fn execute_card(&self, player: &mut Player, card: &Card) {
        println!("{}", card.description);
        match card.action {
            CardAction::Movement =>  {
                // calculate the dice number based on square
                let target = card.square.expect("Target square should exist");
                let p_pos = player.position as u32;
                let dice_roll: u32 = match target > p_pos {
                    true  => target - p_pos,
                    false => target + BOARD_SIZE - p_pos
                };
                self.execute_turn(player, dice_roll);
            },
            CardAction::Payment => {
                player.transact_cash(-1 * card.amount.expect("Amount should exist"));
            },
            CardAction::Jail => {
                player.go_to_jail();
            },
            CardAction::JailRelease => {
                player.num_get_out_of_jail_cards += 1
            },
            _ => {
                // TODO: implement others
            }
        }
    }

    /// Player tries to break out of jail
    fn break_out_of_jail(&self, player: &mut Player) {
        if !player.is_in_jail {
            return;
        }
        if player.num_get_out_of_jail_cards > 0 {
            println!("Yay, No More Jail, thanks to your get-out-of-jail-free card");
            player.num_get_out_of_jail_cards -= 1;
            player.is_in_jail = false;
        } else if player.cash >= 50 {
            println!("Yay, No More Jail, since you bribed the guards $50");
            player.transact_cash(-50);
            player.is_in_jail = false;
        }
        // TODO: Implement roll double digits to get out
    }

    /// Get all squares owned by a player
    fn get_player_owned_squares(&self, player_idx: usize) -> Vec<&Square> {
        let mut squares = Vec::<&Square>::new();
        for s in self.board.iter() {
            match s.asset.borrow().owner {
                Some(owner_idx) => {
                    if owner_idx == player_idx {
                        squares.push(&s);
                    }
                },
                None => {}
            }
        }
        squares
    }

    /// Calculate rent. If the square is unowned, there is no rent
    // Calculate rent, taking into account if a player owns all streets, and the number of
    // properties on the street.
    pub fn calculate_rent(&self, s: &Square, dice_roll: u32) -> Option<u32> {
        let owner = match s.asset.borrow().owner {
            None => {
                // Nobody owns this square
                return None;
            },
            Some(r) => r
        };
        if s.asset.borrow().is_mortgaged {
            println!("Phew! {} is mortgaged, so no rent is due", s.name);
            return None;
        };

        // Need owner of this square
        // get all squares owner owns of the same type
        let rent: u32 = match s.square_type {
            SquareType::Utility => {
                let owned_squares = self.get_player_owned_squares(owner);
                let utility_num = owned_squares.into_iter()
                    .filter(|&x| x.square_type == SquareType::Utility)
                    .collect::<Vec<&Square>>().len();
                let utility_num = utility_num as u32;
                match utility_num {
                    1 => (dice_roll * 4) as u32,
                    2 => (dice_roll * 10) as u32,
                    _ => 0 // Error, no rent
                }
            },
            SquareType::Station => {
                // See how many stations user has
                let owned_squares = self.get_player_owned_squares(owner);
                let station_num = owned_squares.into_iter()
                    .filter(|&x| x.square_type == SquareType::Station)
                    .collect::<Vec<&Square>>().len();
                let station_num = station_num as u32;

                match station_num {
                    1 => 25,
                    2 => 50,
                    3 => 100, // $100 for 3 stations
                    4 => 200,  // $200 for 4 stations
                    _ => 0 // Error, no rent 
                }
            },
            SquareType::Street => {
                let street_details = s.get_street_details().expect("Details expected");
                if !self.player_owns_suburb(owner, s) {
                    return Some(street_details.rent);
                }
                let a = s.asset.borrow();
                if a.has_hotel() {
                    return Some(street_details.rent_suburb[5]);
                }
                match a.house_num() {
                    0 | 1 | 2 | 3 | 4 => {
                        let idx = a.house_num() as usize;
                        street_details.rent_suburb[idx]
                    },
                    _ => {
                        // Panic!
                        println!("Oops! Error in calculating rent");
                        0
                    }
                }
            }
            _ => 0
        };
        Some(rent)
    }

    /// Actions on corner squares
    fn execute_square_corner(&self, player: &mut Player, square: &Square) {
        if player.position == 30 {
            println!("GO TO JAIL!");
            player.go_to_jail();
        } else {
            println!("{}", square.name);
        }
    }

    fn execute_square_tax(&self, player: &mut Player) {
        match player.position {
            4 => {
                println!("Oh No! Pay $200 in Income Tax!");
                player.transact_cash(-200);
            },
            38 => {
                println!("Oh No! Pay $100 in Luxury Tax!");
                player.transact_cash(-100);
            }
            _ => {println!("Error, undefined Tax"); }
        }
    }

    fn execute_square_community(&self, player: &mut Player) {
        println!("COMMUNITY CHEST!");
        let mut cards = self.community_cards.borrow_mut();
        let card = cards.remove(0);
        self.execute_card(player, &card);
        cards.push(card);
    }

    fn execute_square_chance(&self, player: &mut Player) {
        println!("CHANCE!");
        let mut cards = self.chance_cards.borrow_mut();
        let card = cards.remove(0);
        self.execute_card(player, &card);
        cards.push(card);
    }

    /// Sell property to another player
    fn sell_property(&self, orig_owner: &mut Player, new_owner: &mut Player,
                     square: &Square, price: u32) {

        if new_owner.cash < price {
            println!("{} has insufficient funds", &new_owner.name);
            return;
        }

        // new_owner has enough cash
        println!("{} sells {} to {} for ${}",
                 orig_owner.name, square.name, new_owner.name, price);
        orig_owner.transact_cash(price as i32);
        new_owner.transact_cash(-1 * (price as i32));
        let mut asset = square.asset.borrow_mut();
        asset.owner = Some(new_owner.turn_idx);
    }

    /// Purchase the property
    fn buy_property(&self, new_owner: &mut Player, square: &Square, price: u32) {
        // buying from scratch
        if new_owner.cash < price {
            println!("You do not have enouch cash. You'll have to auction it");
            self.auction(new_owner, square);
            return;
        }
        println!("You buy {} for ${}", square.name, price);
        new_owner.transact_cash(-1 * (price as i32));
        let mut asset = square.asset.borrow_mut();
        asset.owner = Some(new_owner.turn_idx);
    }

    fn execute_square_property(&self, player: &mut Player, square: &Square,
                               dice_roll: u32) {
        println!("You landed on {}", square.name);
        match self.calculate_rent(square, dice_roll) {
            None => {

                // For unit tests, purchase automatically, with no auction option
                if self.is_unit_test {
                    self.buy_property(player, square, square.get_price());
                    return;
                }

                match super::dialog::want_to_buy_property(square) {
                    true => self.buy_property(player, square, square.get_price()),
                    false => self.auction(player, square)
                }
            }, 
            Some(rent) => {
                let owner_idx = square.asset.borrow().owner
                    .expect("Somebody owns this street");
                if owner_idx == player.turn_idx {
                    println!("Phew! Luckily it's yours");
                } else {
                    let mut owner = self.players.get(owner_idx)
                        .expect("Owner should exist").borrow_mut();
                    println!("Oh no! You pay ${} to {}", rent, owner.name);
                    player.transact_cash(-1 * (rent as i32));
                    owner.transact_cash(rent as i32);
                }
            }
        }
    }

    /// Execute the turn of a player
    // The turn starts with a player moving. Then, once the player is on the new square,
    // the rules for that new square execute. Lastly, other players may want to execute 
    // transactions
    fn execute_turn(&self, player: &mut Player, dice_roll: u32) {
        self.break_out_of_jail(player); // does nothing if player is not in jail

        let old_pos = player.position;
        player.advance(dice_roll);
        if player.position < old_pos {
            println!("Yay! You pass begin and collect $200");
            player.transact_cash(200);
        }
        // TODO: If 3 doubles, go to jail
        
        let square = self.board.get(player.position).unwrap();
        match square.square_type {
            SquareType::Utility |
            SquareType::Station |
            SquareType::Street        => self.execute_square_property(player, &square,
                                                                      dice_roll),
            SquareType::Corner        => self.execute_square_corner(player, &square),
            SquareType::Tax           => self.execute_square_tax(player),
            SquareType::CommunityCard => self.execute_square_community(player),
            SquareType::ChanceCard    => self.execute_square_chance(player)
        }
    }

    /// Calculate if a player owns all streets in the suburb
    fn player_owns_suburb(&self, player_idx: usize, street: &Square) -> bool {
        match street.asset.borrow().owner {
            None    => return false, // nobody own this street
            Some(o) => {
                if o != player_idx {
                    return false; // player doesn't own this street
                }
            }
        };

        let suburb = street.get_street_details().unwrap().get_suburb();
        let streets_missing = self.board.iter()
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            .filter(|s| match s.asset.borrow().owner {
                    Some(o) => o != player_idx, // others own this
                    None    => true // nobody owns this
                })
            .collect::<Vec<&Square>>()
            .len();

        // player owns all, if none are unowned or owned by others
        streets_missing == 0
    }

    /// Can this street have another property, relative to other streets in the suburb
    fn street_eligible_for_house(&self, street: &Square) -> bool {
        let street_details = street.get_street_details().unwrap();
        let suburb = street_details.get_suburb();
        let asset = street.asset.borrow();
        let building_num = asset.house_num();

        // current building num cannot be > 1 of lowest number of buildings on street
        // ie, if we're building house no 2, all other streets must have at least 1 house
        self.board.iter()
            .filter(|s| s.name != street.name) // ignore own street
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            // if any street has less houses, cannot buy house
            .fold(true, |acc, s| { 
                acc && building_num <= s.asset.borrow().house_num
            })
    }

    /// Can this street have its property sold
    fn street_eligible_for_house_sale(&self, street: &Square) -> bool {
        let street_details = street.get_street_details().unwrap();
        let suburb = street_details.get_suburb();
        let asset = street.asset.borrow();
        let building_num = asset.house_num();

        // current building num cannot be > 1 of lowest number of buildings on street
        // ie, if we're building house no 2, all other streets must have at least 1 house
        self.board.iter()
            .filter(|s| s.name != street.name) // ignore own street
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            // if any street has more houses, cannot sell house
            .fold(true, |acc, s| { 
                acc && building_num >= s.asset.borrow().house_num
            })
    }

    /// All streets must have 4 houses or a hotel
    fn street_eligible_for_hotel(&self, street: &Square) -> bool {
        let street_details = street.get_street_details().unwrap();
        let suburb = street_details.get_suburb();
        let asset = street.asset.borrow();
        let building_num = asset.house_num();

        // all streets in suburb must have 4 houses or a hotel
        self.board.iter()
            .filter(|s| s.name != street.name) // ignore own street
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            .fold(true, |acc, s| { 
                let a = s.asset.borrow();
                acc && (a.has_hotel || a.house_num == 4 )
            })
    }
}

impl StreetDetails {
    fn new(color: char, price: u32, rent: u32, rent_suburb: [u32; 6],
           mortgage: u32) -> Self {

        // Create a suburb based on the street
        // TODO: this can be more elegant, with the Street details referencing suburbs.

        let suburb = match color {
            'B' => Some(Suburb::new("Brown", 50)),
            'L' => Some(Suburb::new("Blue", 50)),
            'P' => Some(Suburb::new("Pink", 100)), 
            'O' => Some(Suburb::new("Orange", 100)),
            'R' => Some(Suburb::new("Red", 150)),
            'Y' => Some(Suburb::new("Yellow", 150)),
            'G' => Some(Suburb::new("Green", 200)),
            'I' => Some(Suburb::new("Indigo", 200)),
            _   => None
        };

        Self {
            suburb,
            price,
            rent,
            rent_suburb,
            mortgage
        }
    }

    /// Calculate unmortgage amount
    pub fn get_unmortgage_amount(&self) -> u32 {
        (1.1 * (self.mortgage as f32)).round() as u32
    }

    pub fn get_suburb(&self) -> Option<&Suburb> {
        self.suburb.as_ref()
    }
}

impl Suburb {
    fn new(color: &str, building_price: u32) -> Self {
        Self {
            color: color.to_string(),
            building_price
        }
    }
}

impl Square {

    fn new(name: &str, square_type: SquareType,
           street_details: Option<StreetDetails>) -> Self {
        let details = match street_details {
            None => None,
            Some(details) => Some(details)
        };
        Self {
            name: name.to_string(),
            square_type,
            street_details: details,
            asset: RefCell::new(Asset::new())
        }
    }

    pub fn get_street_details(&self) -> Option<&StreetDetails> {
        self.street_details.as_ref()
    }

    /// Get purchase price of the street
    pub fn get_price(&self) -> u32 {
        match self.square_type {
            SquareType::Station => 200,
            SquareType::Utility => 150,
            SquareType::Street  => self.get_street_details()
                    .expect("Details should exist").price,
                _ => 0 // Error, should never happen
        }
    }

}

impl Player {
    pub fn new(name: String, idx: usize) -> Self {
        Self {
            name,
            position: 0,
            turn_idx: idx,
            cash: 1500, // 2x500, 4x100, 1x50, 1x20, 2x10, 1x5, 5x1
            is_in_jail: false,
            num_get_out_of_jail_cards: 0,
        }
    }

    /// Advance player
    // Move player to next square
    pub fn advance(&mut self, steps: u32) {
        let target_square = ((self.position as u32) + steps) % BOARD_SIZE;
        self.position = target_square as usize;
    }

    /// Go to jail
    // Player doesn't collect 200, and goes straight to jail
    pub fn go_to_jail(&mut self) {
        self.is_in_jail = true;
        self.position = 10;
    }

    /// Transact in cash.
    // Adds `amount` to players cash amount. Also works for negative numbers
    pub fn transact_cash(&mut self, amount: i32) -> Result<(), ()> {
        if amount < 0 {
            let a = amount.abs() as u32;
            if self.cash < a {
                return Err(());
            }
            self.cash -= a;
        } else {
            let a = amount as u32;
            self.cash += a;
        }
        Ok(())
    }
}

/// Shuffle the deck of chance or community chest cards
fn shuffle_cards(cards: &mut Vec<Card>) {
    let mut idxs: Vec<usize> = (0..cards.len()).collect();

    idxs.shuffle(&mut thread_rng());
    for i in idxs.iter() {
        let c = cards.remove(0);
        cards.insert(*i, c);
    }
}

/// Load the chance cards
fn load_chance_cards() -> Vec<Card> {
    let mut cards = Vec::new();

    cards.push(Card::new("GO TO JAIL!", CardAction::Jail, None, None));
    cards.push(Card::new("Advance to St. Charles Place",
                         CardAction::Movement, None, Some(11)));
////        cards.push(Card::new("Make general repairs on all your property. House, $25 each; Hotel, $100 each", CardAction::PaymentDice, Some(25), None), // TODO: calculate amoun);
////       cards.push(Card::new("Advance to the next railroad. If unowned, you can buy it. if owned, pay twice the rent", CardAction::Unknown, None, None), // TODO: calculate amoun);
      cards.push(Card::new("You have been elected chairman of the board. Pay $50",
                           CardAction::PaymentPlayers, Some(50), None));
      cards.push(Card::new("Take a trip to Reading Railroad.",
                           CardAction::Movement, None, Some(5)));
      cards.push(Card::new("Speeding fine. Pay $15",
                           CardAction::Payment, Some(15), None));
      cards.push(Card::new("Your building load matures. Receive $150",
                           CardAction::Payment, Some(-150), None));
      cards.push(Card::new("Advance to Boardwalk",
                           CardAction::Movement, None, Some(39)));
//        Card::new("Go back three spaces", CardAction::MovementRelative, None, Some(-3)), // TODO: move relative to current square
      cards.push(Card::new("Advance to Illinois Avenue",
                           CardAction::Movement, None, Some(24)));
      cards.push(Card::new("Advance to GO. Collect $200",
                           CardAction::Movement, None, Some(0)));
      cards.push(Card::new("GET OUT OF JAIL FREE.",
                           CardAction::JailRelease, None, None));
//        Card::new("Take all $100 bills from the Bank and throw them in the air", CardAction::Unknown, None, None), // TODO: how to model this? Random allocation?
//        Card::new("Advance to the nearest railroad. If unowned, you can buy it. If owned, pay twice the rent", CardAction::Unknown, None, None), // TODO: go to closest 5,15,25,35. 2x amount
//        Card::new("Advance to the nearest utility. If unowned, you can buy it. If owned, roll the dice, and pay the owner 10x the roll", CardAction::Unknown, None, None), // TODO: pay relative to roll
    shuffle_cards(&mut cards);
    cards
}

/// Load the community chest cards
fn load_community_chest_cards() -> Vec<Card> {
    let mut cards = Vec::new();
//        Card::new("You are assessed for Street repairs: $40 per House, $115 per Hotel", CardAction::Payment, Some(0), None),
    cards.push(Card::new("GET OUT OF JAIL FREE.", CardAction::JailRelease, None, None));
    cards.push(Card::new("You have won second prize in a beauty contest. Collect $10",
                         CardAction::Payment, Some(-10), None));
    cards.push(Card::new("Life insurance matures. Collect $100",
                         CardAction::Payment, Some(-100), None));
    cards.push(Card::new("It's your birthday. Collect $40",
                         CardAction::PaymentPlayers, Some(-40), None));
    cards.push(Card::new("Advance to GO. Collect $200",
                         CardAction::Movement, None, Some(0)));
    cards.push(Card::new("You inherit $100",
                         CardAction::Payment, Some(-100), None));
    cards.push(Card::new("Bank error in your favor. Collect $200",
                         CardAction::Payment, Some(-200), None));
    cards.push(Card::new("From sale of stock, you get $50",
                         CardAction::Payment, Some(-50), None));
    cards.push(Card::new("Collect $25 consultancy fee",
                         CardAction::Payment, Some(-25), None));
    cards.push(Card::new("Holiday fund matures. Collect $100",
                         CardAction::Payment, Some(-100), None));
    cards.push(Card::new("Doctor's fees. Pay $50", CardAction::Payment, Some(50), None));
    cards.push(Card::new("Hospital fees. Pay $100",
                         CardAction::Payment, Some(100), None));
    cards.push(Card::new("GO TO JAIL!", CardAction::Jail, None, None));
    cards.push(Card::new("School fees. Pay $50", CardAction::Payment, Some(50), None));
    cards.push(Card::new("Income tax refund. Collect $20",
                         CardAction::Payment, Some(-20), None));

    shuffle_cards(&mut cards);
    cards
}

/// Initialize the game
// Initializes the game by setting up the necessary data structures.
pub fn init(player_names: Vec::<String>) -> Game {
    let mut players = Vec::<RefCell<Player>>::new();
    // Create player objects
    for (i, p) in player_names.iter().enumerate() {
        players.push(RefCell::new(Player::new(p.to_string(), i)));
    }

    Game {
        players,
        chance_cards: RefCell::new(load_chance_cards()),
        community_cards: RefCell::new(load_community_chest_cards()),
        board: load_squares(),
        is_unit_test: false
    }
}

fn load_squares() -> [Square; BOARD_SIZE as usize] {
    [
        Square::new("Just chillin' at the start", SquareType::Corner, None),
        Square::new("Mediterranean Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'B', 60, 2, [4, 10, 30, 90, 160, 250], 30))),
        Square::new("Community Chest", SquareType::CommunityCard, None),
        Square::new("Baltic Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'B', 60, 4, [8, 20, 60, 180, 320, 450], 30))),
        Square::new("Income Tax", SquareType::Tax, None),
        Square::new("Reading Railroad", SquareType::Station,
            Some(StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        Square::new("Oriental Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'L', 100, 6, [12, 30, 90, 270, 400, 550], 50))),
        Square::new("Chance", SquareType::ChanceCard, None),
        Square::new("Vermont Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'L', 100, 6, [12, 30, 90, 270, 440, 550], 50))),
        Square::new("Connecticut Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'L', 120, 8, [16, 40, 100, 300, 450, 600], 60))),
        Square::new("Visiting Jail", SquareType::Corner, None),
        Square::new("St. Charles Place", SquareType::Street, 
            Some(StreetDetails::new(
                    'P', 140, 10, [20, 50, 150, 450, 625, 750], 70))),
        Square::new("Electric Company", SquareType::Utility,
            Some(StreetDetails::new('N', 150, 0, [0,0,0,0,0,0], 75))),
        Square::new("States Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'P', 140, 10, [20, 50,150, 450, 625, 750], 70))),
        Square::new("Virginia Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'P', 160, 12, [24, 60, 180, 500, 700, 900], 80))),
        Square::new("Pennsylvania Railroad", SquareType::Station,
            Some(StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        Square::new("St. James Place", SquareType::Street, 
            Some(StreetDetails::new(
                    'O', 180, 14, [28, 70, 200, 550, 750, 950], 90))),
        Square::new("Community Chest", SquareType::CommunityCard, None),
        Square::new("Tennessee Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'O', 180, 14, [28, 70, 200, 550, 750, 950], 90))),
        Square::new("New York Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'O', 200, 16, [32, 80, 220, 600, 800, 1000], 110))),
        Square::new("Yay! Free Parking", SquareType::Corner, None),
        Square::new("Kentucky Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'R', 220, 18, [36, 90, 250, 700, 875, 1050], 110))),
        Square::new("Chance", SquareType::ChanceCard, None),
        Square::new("Indiana Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'R', 220, 18, [36, 90, 250, 700, 875, 1050], 110))),
        Square::new("Illinois Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'R', 240, 20, [40, 100, 300, 750, 925, 1100], 120))),
        Square::new("B. & O. Railroad", SquareType::Station,
            Some(StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        Square::new("Atlantic Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'Y', 260, 22, [44, 110, 330, 800, 975, 1150], 130))),
        Square::new("Ventnor Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'Y', 260, 22, [44, 110, 330, 800, 975, 1150], 130))),
        Square::new("Water Works", SquareType::Utility,
            Some(StreetDetails::new('N', 150, 0, [0,0,0,0,0,0], 75))),
        Square::new("Marvin Gardens", SquareType::Street, 
            Some(StreetDetails::new(
                    'Y', 280, 24, [48, 120, 360, 850, 1025, 1200], 140))),
        Square::new("Go To Jail", SquareType::Corner, None),
        Square::new("Pacific Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'G', 300, 26, [52, 130, 390, 900, 1100, 1275], 150))),
        Square::new("North Carolina Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'G', 300, 26, [52, 130, 390, 900, 1100, 1275], 150))),
        Square::new("Community Chest", SquareType::CommunityCard, None),
        Square::new("Pennsylvania Avenue", SquareType::Street, 
            Some(StreetDetails::new(
                    'G', 320, 28, [56, 150, 450, 1000, 1200, 1400], 160))),
        Square::new("Short Line", SquareType::Station,
            Some(StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        Square::new("Chance", SquareType::ChanceCard, None),
        Square::new("Park Place", SquareType::Street, 
            Some(StreetDetails::new(
                    'I', 350, 35, [70, 175, 500, 1100, 1300, 1500], 175))),
        Square::new("Luxury Tax", SquareType::Tax, None),
        Square::new("Boardwalk", SquareType::Street, 
            Some(StreetDetails::new(
                    'I', 400, 50, [100, 200, 600, 1400, 1700, 2000], 200)))
    ]
}

mod actions {
    use super::{Game, Player, dialog};

    pub fn sell_street(game: &Game, orig_owner: &mut Player, new_owner: &mut Player,
                       street_idx: usize, purchase_price: u32) {
        println!("Sell {} to {} for ${}",
                 street_idx, new_owner.name, purchase_price);
        if game.has_buildings(street_idx) {
            println!("The street has buildings. Sell them first");
            return;
        }
        if new_owner.cash < purchase_price {
            println!("{} cannot afford the street", new_owner.name);
            return;
        }

        let square = game.board.get(street_idx)
                         .expect("Street should exist");
        game.sell_property(&mut *orig_owner, &mut *new_owner,
                           square, purchase_price);
    }

    pub fn mortgage_street(game: &Game, owner: &mut Player, street_idx: usize) {
        if game.has_buildings(street_idx) {
            println!("The street has buildings. Sell them first");
            return;
        }

        let street = game.board.get(street_idx).expect("Street should exist");
        let mut asset = street.asset.borrow_mut();

        owner.transact_cash(street.get_street_details().unwrap().mortgage as i32);
        asset.mortgage();
    }

    pub fn unmortgage_street(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let mut asset = street.asset.borrow_mut();

        if !asset.is_mortgaged {
            println!("Street isn't mortgaged");
        } else {
            let sd = street.get_street_details().unwrap();
            owner.transact_cash(-1 * (sd.get_unmortgage_amount() as i32));
            asset.unmortgage();
        }
    }

    pub fn buy_house(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let building_price = match street.get_street_details()
                    .expect("Details should exist").get_suburb() {
            None => {
                println!("You cannot buy a building here");
                return;
            },
            Some(s) => s.building_price
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx {
                    println!("You don't own this street");
                    return;
                }
            },
            None => {
                println!("This street is unowned");
                return;
            }
        };

        if owner.cash < building_price {
            println!("You can't afford to buy a house here");
            return;
        }

        // get streets in suburb not owned by player
        if !game.player_owns_suburb(owner.turn_idx, &street) {
            println!("You don't own all the streets in the suburb");
            return;
        }

        if !game.street_eligible_for_house(&street) {
            println!("Cannot buy a house here yet. Buy houses on other streets first");
            return;
        }

        // Now buy house
        street.asset.borrow_mut().buy_house();
        owner.transact_cash(-1 * (building_price as i32));
    }

    pub fn sell_house(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let building_price = match street.get_street_details()
                    .expect("Details should exist").get_suburb() {
            None => {
                println!("You cannot sell a building here");
                return;
            },
            Some(s) => s.building_price
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx {
                    println!("You don't own this street");
                    return;
                }
            },
            None => {
                println!("This street is unowned");
                return;
            }
        };
        
        if !game.street_eligible_for_house_sale(&street) {
            println!("Cannot sell a house. Sell other houses first");
            return;
        }
        // Player can always sell hotel
        street.asset.borrow_mut().sell_house();
        owner.transact_cash(building_price as i32);
    }

    pub fn buy_hotel(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let building_price = match street.get_street_details()
                    .expect("Details should exist").get_suburb() {
            None => {
                println!("You cannot buy a building here");
                return;
            },
            Some(s) => s.building_price
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx {
                    println!("You don't own this street");
                    return;
                }
            },
            None => {
                println!("This street is unowned");
                return;
            }
        };

        if owner.cash < building_price {
            println!("You can't afford to buy a hotel here");
            return;
        }

        // get streets in suburb not owned by player
        if !game.player_owns_suburb(owner.turn_idx, &street) {
            println!("You don't own all the streets in the suburb");
            return;
        }

        if !game.street_eligible_for_hotel(&street) {
            println!("Cannot buy a hotel here yet.");
            return;
        }

        // Now buy house
        street.asset.borrow_mut().buy_hotel();
        owner.transact_cash(-1 * (building_price as i32));
    }

    pub fn sell_hotel(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let building_price = match street.get_street_details()
                    .expect("Details should exist").get_suburb() {
            None => {
                println!("You cannot sell a building here");
                return;
            },
            Some(s) => s.building_price
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx {
                    println!("You don't own this street");
                    return;
                }
            },
            None => {
                println!("This street is unowned");
                return;
            }
        };
        // Player can always sell hotel
        street.asset.borrow_mut().sell_hotel();
        owner.transact_cash(building_price as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_game() {
        let v = vec!["Bob".to_string(),"Joe".to_string(),"Sally".to_string()];
        let len = v.len();
        let g = init(v);
        assert_eq!(g.players.len(), len, "All players created");
        assert_eq!(g.players.get(0).unwrap().borrow().name, "Bob", "First player");
        assert_eq!(g.players.get(1).unwrap().borrow().name, "Joe", "Middle player");
        assert_eq!(g.players.get(2).unwrap().borrow().name, "Sally", "Last player");
    }

    #[test]
    fn advance_player() {
        let ref mut p = Player::new("Test".to_string(), 1);
        assert_eq!(p.position, 0);
        p.advance(37);
        assert_eq!(p.position, 37);
        p.advance(5); // wrap around BOARD_SIZE
        assert_eq!(p.position, 2);
    }

    #[test]
    fn pay_income_tax() {
        let g = init(vec!["Test".to_string()]);
        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.cash, 1500);
        g.execute_turn(&mut p, 4); // income tax, $200
        assert_eq!(p.cash, 1300);
    }

    #[test]
    fn test_pass_go() {
        let g = init(vec!["Test".to_string()]);

        // advance on top of GO
        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.cash, 1500);
        g.execute_turn(&mut p, 10); // visiting jail
        g.execute_turn(&mut p, 30); // on the go square
        assert_eq!(p.cash, 1700);

        // advance past GO
        g.execute_turn(&mut p, 20); // free parking
        assert_eq!(p.cash, 1700);
        g.execute_turn(&mut p, 30); // pass go, to visiting jail
        assert_eq!(p.cash, 1900);
    }

    #[test]
    fn jail_time() {
        let g = init(vec!["Test".to_string()]);

        // go to jail
        let mut p = g.players.get(0).unwrap().borrow_mut();
        p.num_get_out_of_jail_cards = 1;
        assert_eq!(p.is_in_jail, false);

        g.execute_turn(&mut p, 30);
        assert_eq!(p.position, 10);
        assert_eq!(p.is_in_jail, true);
        assert_eq!(p.cash, 1500);
        assert_eq!(p.num_get_out_of_jail_cards, 1);

        // now release, using card
        g.execute_turn(&mut p, 10);
        assert_eq!(p.num_get_out_of_jail_cards, 0);
        assert_eq!(p.is_in_jail, false);
        assert_eq!(p.cash, 1500);

        // back in jail
        g.execute_turn(&mut p, 10);
        assert_eq!(p.is_in_jail, true);
        assert_eq!(p.num_get_out_of_jail_cards, 0);
        assert_eq!(p.cash, 1500);

        // now release, paying $50
        g.execute_turn(&mut p, 10);
        assert_eq!(p.is_in_jail, false);
        assert_eq!(p.num_get_out_of_jail_cards, 0);
        assert_eq!(p.cash, 1450);
    }

    #[test]
    fn calculate_rent_unowned() {
        let mut g = init(vec!["Test".to_string()]);
        g.set_unit_test();
        // Unowned square | No Rent
        let s = g.board.get(1).unwrap();
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, None);
        
        // Income tax | No rent
        let s = g.board.get(4).unwrap();
        let r = g.calculate_rent(s, 4);
        assert_eq!(r, None);
       
        // no-rent square
        let s = g.board.get(10).unwrap();
        let r = g.calculate_rent(s, 10);
        assert_eq!(r, None);
    }

    #[test]
    fn calculate_rent_mortgaged() {
        let mut g = init(vec!["StreetOwner".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, 3); // Owner moves to Baltic Avenue

        s.asset.borrow_mut().mortgage();
        assert_eq!(g.calculate_rent(s, 3), None);
        s.asset.borrow_mut().unmortgage();
        assert_eq!(g.calculate_rent(s, 3), Some(4));
    }

    #[test]
    fn calculate_rent_street() {
        let mut g = init(vec!["StreetOwner".to_string(),
                          "StreetRenter".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        // Buy the following squares:
        // Baltic Avenue[3] (1 of set of 2)
        // Oriental[6] & Vermont Ave[8] (2 of set of 3)
        // St. Charles place[11], States Ave[13], Virginia Ave[14] (3 of set of 3)
        // Park Place[37] & Boardwalk[39] (2 of set of 2)
        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, 3); // Owner moves to Baltic Avenue
        g.execute_turn(&mut p, 3); // Owner moves to Oriental Avenue
        g.execute_turn(&mut p, 2); // Vermont Avenue
        g.execute_turn(&mut p, 3); // St. Charles place
        g.execute_turn(&mut p, 2); // States Ave
        g.execute_turn(&mut p, 1); // Virginia Ave
        g.execute_turn(&mut p, 23); // Park place
        g.execute_turn(&mut p, 2); // Boardwalk

        // Rent for 1 of 2 set 
        let s = g.board.get(3).unwrap(); // Baltic
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(4));

        // Rent for 2 of 3 set 
        let s = g.board.get(6).unwrap(); // Oriental
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(6));
        let s = g.board.get(8).unwrap(); // Vermont
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(6));

        // rent for 3 of 3 set 
        let s = g.board.get(11).unwrap(); // St. Charles 
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(20));
        let s = g.board.get(13).unwrap(); // States Ave
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(20));
        let s = g.board.get(14).unwrap(); // Virginia Ave
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(24));
        
        // rent for 2 of 2 set 
        let s = g.board.get(37).unwrap(); // Park Ave
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(70));
        let s = g.board.get(39).unwrap(); // Boardwalk
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(100));
    }

    #[test]
    fn calculate_rent_utility() {
        // Buy 1 utility, then buy the second
        let mut g = init(vec!["TestOwner".to_string(), "TestRenter".to_string()]);
        g.set_unit_test();

        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, 12); // Electric
        let s = g.board.get(12).unwrap(); // Electric
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(12)); 

        g.execute_turn(&mut p, 16); // Water
        let s = g.board.get(28).unwrap(); // Electric
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(30)); 
    }

    #[test]
    fn calculate_rent_station() {
        // Buy stations one at a time
        let mut g = init(vec!["StationOwner".to_string(),
                              "StationRenter".to_string()]);
        g.set_unit_test();

        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, 5); // Reading Railroad
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(25)); 

        g.execute_turn(&mut p, 10); // Pennsylvania Railroad
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(50)); 

        g.execute_turn(&mut p, 10); // B.O. Railroad
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(100)); 

        g.execute_turn(&mut p, 10); // Short line
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, 3);
        assert_eq!(r, Some(200)); 
    }

    #[test]
    fn purchase_and_pay_rent() {
        let mut g = init(vec!["Owner".to_string(), "Renter".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        { // change scope so we can release owner when renter moves
            let mut owner = g.players.get(0).unwrap().borrow_mut();
            assert_eq!(owner.cash, 1500);
            g.execute_turn(&mut owner, 3); // Owner moves to Baltic Avenue
            assert_eq!(owner.cash, 1440); // bought street
            assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx);
        }

        let mut renter = g.players.get(1).unwrap().borrow_mut();
        g.execute_turn(&mut renter, 3); // Renter on Baltic Ave
        assert_eq!(renter.cash, 1496);
        let owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash, 1444);
    }

    #[test]
    fn buy_each_property_type() {
        let mut g = init(vec!["Mongul".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.cash, 1500);
        
        g.execute_turn(&mut p, 3); // Mongul moves to Baltic Avenue
        assert_eq!(p.cash, 1440); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), p.turn_idx);

        g.execute_turn(&mut p, 9); // Mongul moves to Electric Company
        assert_eq!(p.cash, 1290); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), p.turn_idx);

        g.execute_turn(&mut p, 3); // Mongul moves to Pennsylvania Railroad
        assert_eq!(p.cash, 1090); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), p.turn_idx);
    }

    #[test]
    fn sell_a_property() {
        let mut g = init(vec!["Seller".to_string(), "NewOwner".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut seller = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(seller.cash, 1500);
        
        g.execute_turn(&mut seller, 3); // Seller moves to Baltic Avenue
        assert_eq!(seller.cash, 1440); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), seller.turn_idx);

        let mut owner = g.players.get(1).unwrap().borrow_mut();
        g.sell_property(&mut seller, &mut owner, &s, 20);
        assert_eq!(owner.cash, 1480);
        assert_eq!(seller.cash, 1460);
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx);
    }

    #[test]
    fn mortgage_then_unmortgage() {
        let mut g = init(vec!["M".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 3;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash, 1500);
        
        g.execute_turn(&mut owner, 3); // Seller moves to Baltic Avenue
        assert_eq!(owner.cash, 1440); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx);

        actions::mortgage_street(&g, &mut owner, street_idx);
        assert_eq!(owner.cash, 1470); // mortgage of 30
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx);
        assert_eq!(s.asset.borrow().is_mortgaged, true);

        actions::unmortgage_street(&g, &mut owner, street_idx);
        assert_eq!(owner.cash, 1437); // unmortgage for 33
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx);
        assert_eq!(s.asset.borrow().is_mortgaged, false);
    }

    #[test]
    fn shuffle_fake_cards() {
        let mut cards = Vec::new();
        cards.push(Card::new("Card 1", CardAction::Payment, Some(-1), None));
        cards.push(Card::new("Card 2", CardAction::Payment, Some(2), None));
        cards.push(Card::new("Card 3", CardAction::Payment, Some(10), None));
        cards.push(Card::new("Card 4", CardAction::Payment, Some(100), None));
        cards.push(Card::new("Card 5", CardAction::Payment, Some(-200), None));
        cards.push(Card::new("Card 6", CardAction::Payment, Some(-80), None));
        shuffle_cards(&mut cards);
        assert_eq!(cards.len(), 6);
        assert_eq!(false, // there must be a change in order
            cards.get(0).unwrap().description == "Card 1" &&
            cards.get(1).unwrap().description == "Card 2" &&
            cards.get(2).unwrap().description == "Card 3" &&
            cards.get(3).unwrap().description == "Card 4" &&
            cards.get(4).unwrap().description == "Card 5" &&
            cards.get(4).unwrap().description == "Card 6"
        );
    }

    #[test]
    fn unmortgage_cost_calculation() {
        assert_eq!(33, StreetDetails::new('B', 60, 2, [4,0,0,0,0,0], 30)
                   .get_unmortgage_amount());
        assert_eq!(193, StreetDetails::new('B', 60, 2, [4,0,0,0,0,0], 175)
                   .get_unmortgage_amount());
        assert_eq!(220, StreetDetails::new('B', 60, 2, [4,0,0,0,0,0], 200)
                   .get_unmortgage_amount());
    }

    #[test]
    fn buy_first_house() {
        // cannot buy house unless all houses in suburb are owned 
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash, 1500);
        
        g.execute_turn(&mut owner, 1); // Seller moves to Mediterranean Avenue
        assert_eq!(owner.cash, 1440); // bought street
        actions::buy_house(&g, &mut owner, street_idx); // whole suburb isn't owned
        assert_eq!(s.asset.borrow().house_num(), 0);

        g.execute_turn(&mut owner, 2); // Seller moves to Baltic Avenue and buys it
        assert_eq!(owner.cash, 1380); // bought street
        actions::buy_house(&g, &mut owner, street_idx); // buy Mediterranean
        assert_eq!(s.asset.borrow_mut().house_num(), 1);
        assert_eq!(owner.cash, 1330); // bought house for 50

        // cannot buy second house on mediterranean
        actions::buy_house(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow_mut().house_num(), 1);
        assert_eq!(owner.cash, 1330); // bought house for 50

        // sell house
        actions::sell_house(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow_mut().house_num(), 0);
        assert_eq!(owner.cash, 1380); // sell house for 50
    }

    #[test]
    fn buy_houses_then_hotel() {
        // buy all possible houses for suburb
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);
        let rs = s.get_street_details().unwrap().rent_suburb;

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash, 1500);
        
        g.execute_turn(&mut owner, 1); // Seller moves to Mediterranean Avenue
        assert_eq!(owner.cash, 1440); // bought street
        assert_eq!(g.calculate_rent(s, 3).unwrap(), 2); // rent for Mediterranean w/o suburb

        g.execute_turn(&mut owner, 2); // Seller moves to Baltic Avenue
        assert_eq!(owner.cash, 1380); // bought street
        assert_eq!(g.calculate_rent(s, 3).unwrap(), rs[0]); // rent for suburb

        // tycoon now owns all streets, lets iteratively buy houses up to 4
        for i in 0..4 {
            // buy another house in Mediterranean then Baltic
            let street_idx = 1;
            let s = g.board.get(street_idx).unwrap();
            actions::buy_house(&g, &mut owner, street_idx);
            assert_eq!(s.asset.borrow().house_num(), (i+1));
            let r: u32 = rs[(i+1) as usize];
            assert_eq!(g.calculate_rent(s, 3).unwrap(), r); // rent for houses

            let street_idx = 3;
            let s = g.board.get(street_idx).unwrap();
            actions::buy_house(&g, &mut owner, street_idx);
            assert_eq!(s.asset.borrow().house_num(), (i+1));

            // not eligible for hotel
            if s.asset.borrow().house_num() != 4 {
                assert_eq!(g.street_eligible_for_hotel(&s), false);
            }
        }

        // double check, both streets should have 4 houses
        assert_eq!(g.board.get(1).unwrap().asset.borrow().house_num(), 4);
        assert_eq!(g.board.get(3).unwrap().asset.borrow().house_num(), 4);

        // buying 5th house fails
        let street_idx = 1;
        let s = g.board.get(street_idx).unwrap();
        actions::buy_house(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().house_num(), 4);
        assert_eq!(s.asset.borrow().has_hotel(), false);

        // buying hotel succeeds
        actions::buy_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().house_num(), 4);
        assert_eq!(s.asset.borrow().has_hotel(), true);
    }

    #[test]
    fn buy_hotels_fails() {
        // buy all possible houses for suburb
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        // buy Brown suburb
        let mut owner = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut owner, 1); // Seller moves to Mediterranean Avenue

        // try buy hotel
        actions::buy_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().house_num(), 0);
        assert_eq!(s.asset.borrow().has_hotel(), false);

        g.execute_turn(&mut owner, 2); // Seller moves to Baltic Avenue
        // try buy hotel
        actions::buy_hotel(&g, &mut owner, 3);
        assert_eq!(s.asset.borrow().house_num(), 0);
        assert_eq!(s.asset.borrow().has_hotel(), false);
    }

    #[test]
    fn buy_then_sell_hotel() {
        // buy all possible houses for suburb
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        
        // buy indigo squares
        g.execute_turn(&mut owner, 1);
        g.execute_turn(&mut owner, 2);

        // put houses on all squares
        for i in 0..4 {
            // buy another house in Mediterranean then Baltic
            let street_idx = 1;
            let s = g.board.get(street_idx).unwrap();
            actions::buy_house(&g, &mut owner, street_idx);

            let street_idx = 3;
            let s = g.board.get(street_idx).unwrap();
            actions::buy_house(&g, &mut owner, street_idx);
        }

        // double check, both streets should have 4 houses
        assert_eq!(g.board.get(1).unwrap().asset.borrow().house_num(), 4);
        assert_eq!(g.board.get(3).unwrap().asset.borrow().house_num(), 4);

        // buying hotels succeeds
        let street_idx = 1;
        let s = g.board.get(street_idx).unwrap();
        actions::buy_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), true);
        assert_eq!(g.calculate_rent(s, 3).unwrap(), 250);

        let street_idx = 3;
        let s = g.board.get(street_idx).unwrap();
        actions::buy_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), true);
        assert_eq!(g.calculate_rent(s, 3).unwrap(), 450);

        // Selling hotels succeeds
        actions::sell_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), false);
        assert_eq!(g.calculate_rent(s, 3).unwrap(), 320);

        let street_idx = 1;
        let s = g.board.get(street_idx).unwrap();
        actions::sell_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), false);
        assert_eq!(g.calculate_rent(s, 3).unwrap(), 160);
    }

    #[test]
    fn check_house_eligibility() {
        // buy all possible houses for suburb
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);
        let rs = s.get_street_details().unwrap().rent_suburb;

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        
        g.execute_turn(&mut owner, 1); // buys mediterranean
        assert_eq!(g.player_owns_suburb(owner.turn_idx, &s), false);
        assert_eq!(g.street_eligible_for_house(&s), true);
        assert_eq!(g.street_eligible_for_hotel(&s), false);
        let s = g.board.get(3).unwrap();
        assert_eq!(g.player_owns_suburb(owner.turn_idx, &s), false);
        assert_eq!(g.street_eligible_for_house(&s), true);
        assert_eq!(g.street_eligible_for_hotel(&s), false);

        g.execute_turn(&mut owner, 2); // buys Baltic
        assert_eq!(g.player_owns_suburb(owner.turn_idx, &s), true);
        assert_eq!(g.street_eligible_for_house(&s), true); // can now have houses
        assert_eq!(g.street_eligible_for_hotel(&s), false);
        let s = g.board.get(1).unwrap();
        assert_eq!(g.player_owns_suburb(owner.turn_idx, &s), true);
        assert_eq!(g.street_eligible_for_house(&s), true);
        assert_eq!(g.street_eligible_for_hotel(&s), false);

        // buy house on mediterranean
        let street_idx = 1;
        let s = g.board.get(street_idx).unwrap();
        actions::buy_house(&g, &mut owner, street_idx);
        assert_eq!(g.street_eligible_for_house(&s), false); // cannot buy 2nd house
        
        // buy house on baltic
        let street_idx = 3;
        let s = g.board.get(street_idx).unwrap();
        actions::buy_house(&g, &mut owner, street_idx);
        
        // buy second house on mediterranean
        let street_idx = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(g.street_eligible_for_house(&s), true); // now eligible
        actions::buy_house(&g, &mut owner, street_idx);
        assert_eq!(g.street_eligible_for_house_sale(&s), true);

        // cannot sell house on baltic
        let street_idx = 3;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(g.street_eligible_for_house_sale(&s), false);
    }
}
