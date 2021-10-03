use rand::thread_rng;
use rand::seq::SliceRandom;

use std::cell::RefCell;

use super::{card, dialog, player, publisher, square};

const BOARD_SIZE: u32 = 40; // 40 squares on the board

mod actions {
    use super::Game;
    use super::player::Player;
    use super::dialog;

    pub fn sell_street(game: &Game, orig_owner: &mut Player, new_owner: &mut Player,
                       street_idx: usize, purchase_price: u32) {
        let square = game.board.get(street_idx)
                         .expect("Street should exist");
        {
            let asset = square.asset.borrow();
            if asset.has_buildings() {
                println!("The street has buildings. Sell them first");
                return;
            }
            if new_owner.cash() < purchase_price {
                println!("{} cannot afford the street", new_owner.name());
                return;
            }
        }

        game.sell_property(&mut *orig_owner, &mut *new_owner,
                           square, purchase_price);
    }

    pub fn mortgage_street(game: &Game, owner: &mut Player, street_idx: usize) {
        let square = game.board.get(street_idx)
                         .expect("Street should exist");
        let mut asset = square.asset.borrow_mut();
        if asset.has_buildings() {
            println!("The street has buildings. Sell them first");
            return;
        }

        owner.transact_cash(square.get_street_details().unwrap().mortgage() as i32);
        asset.mortgage();
    }

    pub fn unmortgage_street(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let mut asset = street.asset.borrow_mut();

        if !asset.is_mortgaged() {
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
            Some(s) => s.building_price()
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx() {
                    println!("You don't own this street");
                    return;
                }
            },
            None => {
                println!("This street is unowned");
                return;
            }
        };

        if owner.cash() < building_price {
            println!("You can't afford to buy a house here");
            return;
        }

        // get streets in suburb not owned by player
        if !game.player_owns_suburb(owner.turn_idx(), &street) {
            println!("You don't own all the streets in the suburb");
            return;
        }

        if !game.street_eligible_for_house(&street) {
            println!("Cannot buy a house here yet. Buy houses on other streets first");
            return;
        }

        // buy immediately for unit test
        if game.is_unit_test {
            street.asset.borrow_mut().buy_house();
            owner.transact_cash(-1 * (building_price as i32));
            return;
        }

        let msg = format!("Confirm: Buy a house on {} for ${} (Y/n)?",
                          street.name(), building_price);
        match dialog::yes_no(&msg) {
            false => {}, // do nothing
            yes => {
                // Now buy house
                street.asset.borrow_mut().buy_house();
                owner.transact_cash(-1 * (building_price as i32));
            }
        };
    }

    pub fn sell_house(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let building_price = match street.get_street_details()
                    .expect("Details should exist").get_suburb() {
            None => {
                println!("You cannot sell a building here");
                return;
            },
            Some(s) => s.building_price()
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx() {
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
            Some(s) => s.building_price()
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx() {
                    println!("You don't own this street");
                    return;
                }
            },
            None => {
                println!("This street is unowned");
                return;
            }
        };

        if owner.cash() < building_price {
            println!("You can't afford to buy a hotel here");
            return;
        }

        // get streets in suburb not owned by player
        if !game.player_owns_suburb(owner.turn_idx(), &street) {
            println!("You don't own all the streets in the suburb");
            return;
        }

        if !game.street_eligible_for_hotel(&street) {
            println!("Cannot buy a hotel here yet.");
            return;
        }

        let msg = format!("Confirm: Buy a hotel on {} for ${}?",
                          street.name(), building_price);

        // buy immediately for unit test
        if game.is_unit_test {
            street.asset.borrow_mut().buy_hotel();
            owner.transact_cash(-1 * (building_price as i32));
            return;
        }

        match dialog::yes_no(&msg) {
            false => {}, // do nothing
            yes => {
                // Now buy hotel
                street.asset.borrow_mut().buy_hotel();
                owner.transact_cash(-1 * (building_price as i32));
            }
        };
    }

    pub fn sell_hotel(game: &Game, owner: &mut Player, street_idx: usize) {
        let street = game.board.get(street_idx).expect("Street should exist");
        let building_price = match street.get_street_details()
                    .expect("Details should exist").get_suburb() {
            None => {
                println!("You cannot sell a building here");
                return;
            },
            Some(s) => s.building_price()
        };
        match street.asset.borrow().owner {
            Some(i) => {
                if i != owner.turn_idx() {
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

#[derive(Debug)]
pub struct Dice {
    roll: (u32, u32),
    num_rolls: u32, // number of times user has rolled dice
    cumulative_sum: u32
}

/// The structure, containing links to all parts of the game
pub struct Game {
    pub players: Vec<RefCell<player::Player>>,
    pub board: [square::Square; BOARD_SIZE as usize],
    chance_cards: RefCell<Vec<card::Card>>,
    community_cards: RefCell<Vec<card::Card>>,
    is_unit_test: bool
}

impl Dice {
    pub fn new(die1: u32, die2: u32) -> Self {
        Self {
            roll: (die1, die2),
            num_rolls: 1,
            cumulative_sum: die1 + die2
        }
    }

    pub fn total(&self) -> u32 {
        self.roll.0 + self.roll.1
    }

    pub fn cumulative_sum(&self) -> u32 {
        self.cumulative_sum
    }

    pub fn roll(&self) -> (u32, u32) {
        self.roll
    }

    /// Check if its a double, and if so, update inner values
    pub fn is_double(&self) -> bool {
        self.roll.0 == self.roll.1
    }

    /// This dice follows the roll of a dice passed in as a parameter
    pub fn reroll(&mut self, dice: Dice) {
        self.roll = dice.roll;
        self.num_rolls += 1;
        self.cumulative_sum += dice.roll.0 + dice.roll.1;
    }
}

impl Game {
    /// Start the game
    pub fn start(self) {
        loop {
            for p_ref in self.players.iter() {
                {
                    let mut player = p_ref.borrow_mut();
                    if player.left_game() {
                        continue;
                    }
                    println!("\n=== {}, Your turn ===", player.name());
                    self.jail_time(&mut player);

                    print!("Roll dice: ");
                    let mut dice = dialog::capture_dice_roll();

                    self.execute_turn(&mut *player, dice);
                }

                if self.is_unit_test {
                    continue;
                }
                // present options of other transactions user can make

                let mut is_in_trouble;
                let mut left_game;
                let mut turn_idx;
                {
                    let player = p_ref.borrow();
                    is_in_trouble = player.is_in_trouble();
                    left_game = player.left_game();
                    turn_idx = player.turn_idx();
                }
                let was_in_trouble = is_in_trouble;

                while is_in_trouble {
                    // player couldn't pay for the turn.
                    // Player must now sell assets for cash
                    println!("Uh oh! You don't have enough money to continue.");
                    println!("You can sell assets, or leave the game");

                    self.execute_user_action(turn_idx, is_in_trouble);

                    {
                        let player = p_ref.borrow();
                        left_game = player.left_game();
                    }

                    // player still in the game, lets try run this turn again
                    if !left_game {
                        let mut player = p_ref.borrow_mut();
                        // stay on same square, to see if player can pay debts
                        self.execute_turn(&mut *player, Dice::new(0, 0));
                    } else {
                        break; // player has exited game
                    }
                    {
                        let player = p_ref.borrow();
                        is_in_trouble = player.is_in_trouble();
                    }
                }
                self.execute_user_action(turn_idx, false);
            }
        }
    }

    fn execute_user_action(&self, turn_idx: usize, is_in_trouble: bool) {
        loop {
            publisher::publish(&self);
            let option = match is_in_trouble {
                true  => dialog::trouble_user_actions(),
                false => dialog::additional_user_actions()
            };
            match option {
                dialog::UserAction::EndGame => {
                    // liquify assets
                    let squares = self.board.iter()
                        .filter(|s| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == turn_idx
                            }
                        })
                        .collect::<Vec<&square::Square>>();
                    for square in squares {
                        square.asset.borrow_mut().liquify();
                    }

                    let mut player = self.players.get(turn_idx).unwrap().borrow_mut();
                    let pos_idx = player.position();
                    player.leave_game();

                    // If player is on a card square, move the card to the bottom
                    let current_square = &self.board[pos_idx];
                    match current_square.square_type() {
                        square::SquareType::CommunityCard => {
                            let mut cards = self.community_cards.borrow_mut();
                            let card = cards.remove(0);
                            cards.push(card);
                        },
                        square::SquareType::ChanceCard    => {
                            let mut cards = self.chance_cards.borrow_mut();
                            let card = cards.remove(0);
                            cards.push(card);
                        },
                        _ => {} // do nothing
                    };

                    println!("== Game Over! {} ==", player.name());
                    break;
                },
                dialog::UserAction::EndTurn => return,
                dialog::UserAction::SellStreet => {
                    let player_idx = match is_in_trouble {
                        true  => turn_idx,
                        false => {
                            match dialog::get_player_idx(
                                    self, None, "Select the current owner") {
                                Ok(s)  => s,
                                Err(_) => {
                                    println!("Back to the menu");
                                    continue;
                                }
                            }
                        }
                    };
                    
                    let mut orig_owner = self.players.get(player_idx)
                            .unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &square::Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == orig_owner.turn_idx()
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

                    let eligible_streets :Vec<(usize, &square::Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx()
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
                    let player_idx = match is_in_trouble {
                        true  => turn_idx,
                        false => {
                            match dialog::get_player_idx(
                                    self, None, "Select the current owner") {
                                Ok(s)  => s,
                                Err(_) => {
                                    println!("Back to the menu");
                                    continue;
                                }
                            }
                        }
                    };
                    
                    let mut owner = self.players.get(player_idx).unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &square::Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(o) => o == owner.turn_idx()
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

                    let eligible_streets :Vec<(usize, &square::Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx()
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
                    let player_idx = match is_in_trouble {
                        true  => turn_idx,
                        false => {
                            match dialog::get_player_idx(
                                    self, None, "Select the current owner") {
                                Ok(s)  => s,
                                Err(_) => {
                                    println!("Back to the menu");
                                    continue;
                                }
                            }
                        }
                    };
                    
                    let mut owner = self.players.get(player_idx).unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &square::Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx()
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
                    let player_idx = match is_in_trouble {
                        true  => turn_idx,
                        false => {
                            match dialog::get_player_idx(
                                    self, None, "Select the current owner") {
                                Ok(s)  => s,
                                Err(_) => {
                                    println!("Back to the menu");
                                    continue;
                                }
                            }
                        }
                    };
                    
                    let mut owner = self.players.get(player_idx).unwrap().borrow_mut();

                    let eligible_streets :Vec<(usize, &square::Square)> = 
                            self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx()
                            }
                        })
                        .filter(|(_, s)| !s.asset.borrow().is_mortgaged() )
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

                    let eligible_streets :Vec<(usize, &square::Square)> = self.board.iter().enumerate()
                        .filter(|(_, s)| { match s.asset.borrow().owner {
                                None => false,
                                Some(u) => u == owner.turn_idx()
                            }
                        })
                        .filter(|(_, s)| s.asset.borrow().is_mortgaged() )
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

    /// Capture player name, and price, and complete purchase
    fn auction(&self, player: &player::Player, square: &square::Square) 
            -> Result<(), ()> {
        println!("Auction!!");
        let owner_idx = match dialog::get_player_idx(self, None,
                                                     "Select the new owner") {
            Ok(o)  => o,
            Err(_) => {
                println!("Back to the menu");
                return Ok(());
            }
        };

        let purchase_price = match dialog::get_purchase_price(square) {
            Ok(p) => p,
            Err(_) => {
                println!("Back to the menu");
                return Ok(());
            }
        };

        let mut owner = self.players.get(owner_idx).expect("Player should exist")
                            .borrow_mut();
        self.buy_property(&mut *owner, square, purchase_price)
    }

    /// Give player in jail options to get out
    pub fn jail_time(&self, player: &mut player::Player) {
        if !player.is_in_jail(){
            return;
        }
        if player.redeem_jail_free_card().is_ok() {
            println!("Yay, No More Jail, thanks to your get-out-of-jail-free card");
            return;
        };
        let pay_cash = match self.is_unit_test {
            true  => true,
            false => dialog::yes_no(
                "Bribe the guards $50 to get out of jail? (Y/n)")
        };
        if pay_cash {
            player.bribe_guards();
        }
    }

    /// Update game state to unit test
    // This mode will eliminate questions to the user that require keyboard input
    fn set_unit_test(&mut self) {
        self.is_unit_test = true;
    }

    /// Execute action on card
    fn execute_card(&self, player: &mut player::Player, card: &card::Card)
            -> Result <(), ()> {
        println!("{}", card.description());
        match card.action() {
            card::CardAction::Movement =>  {
                // calculate the dice number based on square
                let target = card.square().expect("Target square should exist");
                let p_pos = player.position() as u32;
                let dice = match target > p_pos {
                    true  => Dice::new(target - p_pos, 0),
                    false => Dice::new(target + BOARD_SIZE - p_pos, 0)
                };
                return self.execute_turn(player, dice);
            },
            card::CardAction::Payment => {
                return player.transact_cash(-1 * card.amount()
                        .expect("Amount should exist"));
            },
            card::CardAction::Jail => {
                player.go_to_jail();
            },
            card::CardAction::JailRelease => {
                player.receive_jail_free_card();
            }
        }
        Ok(())
    }

    /// Get all squares owned by a player
    fn get_player_owned_squares(&self, player_idx: usize) -> Vec<&square::Square> {
        let mut squares = Vec::<&square::Square>::new();
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
    pub fn calculate_rent(&self, s: &square::Square, dice: Dice) -> Option<u32> {
        let owner = match s.asset.borrow().owner {
            None => {
                // Nobody owns this square
                return None;
            },
            Some(r) => r
        };
        if s.asset.borrow().is_mortgaged() {
            return None;
        };

        // Need owner of this square
        // get all squares owner owns of the same type
        let rent: u32 = match s.square_type() {
            square::SquareType::Utility => {
                let owned_squares = self.get_player_owned_squares(owner);
                let utility_num = owned_squares.into_iter()
                    .filter(|&x| x.square_type() == square::SquareType::Utility)
                    .collect::<Vec<&square::Square>>().len();
                let utility_num = utility_num as u32;
                match utility_num {
                    1 => (dice.cumulative_sum * 4) as u32,
                    2 => (dice.cumulative_sum * 10) as u32,
                    _ => 0 // Error, no rent
                }
            },
            square::SquareType::Station => {
                // See how many stations user has
                let owned_squares = self.get_player_owned_squares(owner);
                let station_num = owned_squares.into_iter()
                    .filter(|&x| x.square_type() == square::SquareType::Station)
                    .collect::<Vec<&square::Square>>().len();
                let station_num = station_num as u32;

                match station_num {
                    1 => 25,
                    2 => 50,
                    3 => 100, // $100 for 3 stations
                    4 => 200,  // $200 for 4 stations
                    _ => 0 // Error, no rent 
                }
            },
            square::SquareType::Street => {
                let street_details = s.get_street_details().expect("Details expected");
                if !self.player_owns_suburb(owner, s) {
                    return Some(street_details.rent());
                }
                let a = s.asset.borrow();
                if a.has_hotel() {
                    return Some(street_details.rent_suburb()[5]);
                }
                match a.house_num() {
                    0 | 1 | 2 | 3 | 4 => {
                        let idx = a.house_num() as usize;
                        street_details.rent_suburb()[idx]
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
    fn execute_square_corner(&self, player: &mut player::Player, square: &square::Square)
            -> Result<(), ()> {
        if player.position() == 30 {
            player.go_to_jail();
        } else {
            println!("{}", square.name());
        }
        Ok(())
    }

    fn execute_square_tax(&self, player: &mut player::Player) -> Result<(), ()> {
        match player.position() {
            4 => {
                println!("Oh No! Pay $200 in Income Tax!");
                player.transact_cash(-200)
            },
            38 => {
                println!("Oh No! Pay $100 in Luxury Tax!");
                player.transact_cash(-100)
            }
            _ => {println!("Error, undefined Tax"); Ok(()) }
        }
    }

    fn execute_square_community(&self, player: &mut player::Player) -> Result<(), ()> {
        println!("COMMUNITY CHEST!");
        let mut cards = self.community_cards.borrow_mut();
        let card = cards.remove(0);
        match self.execute_card(player, &card) {
            Ok(_) => {
                // All good
                cards.push(card);
                Ok(())
            },
            Err(_) => {
                // player couldn't pay. return card to its original position
                cards.insert(0, card);
                Err(())
            }
        }
    }

    fn execute_square_chance(&self, player: &mut player::Player) -> Result<(), ()> {
        println!("CHANCE!");
        let mut cards = self.chance_cards.borrow_mut();
        let card = cards.remove(0);
        match self.execute_card(player, &card) {
            Ok(_) => {
                // All good
                cards.push(card);
                Ok(())
            },
            Err(_) => {
                // player couldn't pay. return card to its original position
                cards.insert(0, card);
                Err(())
            }
        }
    }

    /// Sell property to another player
    fn sell_property(&self, orig_owner: &mut player::Player, new_owner: &mut player::Player,
                     square: &square::Square, price: u32) {

        if new_owner.cash() < price {
            println!("{} has insufficient funds", &new_owner.name());
            return;
        }

        // new_owner has enough cash
        println!("{} sells {} to {} for ${}",
                 orig_owner.name(), square.name(), new_owner.name(), price);
        orig_owner.transact_cash(price as i32);
        new_owner.transact_cash(-1 * (price as i32));
        let mut asset = square.asset.borrow_mut();
        asset.owner = Some(new_owner.turn_idx());
    }

    /// Purchase the property
    fn buy_property(&self, new_owner: &mut player::Player,
                    square: &square::Square, price: u32) -> Result<(), ()> {
        // buying from scratch
        println!("You buy {} for ${}", square.name(), price);
        if new_owner.transact_cash(-1 * (price as i32)).is_err() {
            return Err(()); // should never happen, since price check was done already
        };
        let mut asset = square.asset.borrow_mut();
        asset.owner = Some(new_owner.turn_idx());
        Ok(())
    }

    /// Landed on a square that can be bought
    fn execute_square_property(&self, player: &mut player::Player,
                               square: &square::Square, dice: Dice) 
            -> Result<(), ()> {
        println!("You landed on {}", square.name());
        let has_owner = match square.asset.borrow().owner {
            Some(_) => true,
            None => false
        };
        match has_owner {
            false => { // Unowned asset
                // For unit tests, purchase automatically, with no auction option
                if self.is_unit_test {
                    self.buy_property(player, square, square.get_price());
                    return Ok(());
                }

                if player.cash() < square.get_price() {
                    println!("You can't afford to buy this street.");
                    return self.auction(player, square);
                }
                let message = format!("Do you want to buy {} for ${}? (Y/n)",
                                      square.name(), square.get_price());
                match super::dialog::yes_no(&message) {
                    true => self.buy_property(player, square, square.get_price()),
                    false => self.auction(player, square)
                }
            },
            true => { // Owned asset
                let asset = square.asset.borrow();
                let owner_idx = asset.owner.unwrap();
                if owner_idx == player.turn_idx() {
                    println!("Phew! Luckily it's yours");
                    return Ok(());
                }

                if asset.is_mortgaged() {
                    println!("Phew! {} is mortgaged, so no rent is due", square.name());
                    return Ok(());
                }
                let rent = self.calculate_rent(square, dice).expect("Rent should exist");

                let mut owner = self.players.get(owner_idx)
                    .expect("Owner should exist").borrow_mut();
                println!("Oh no! You pay ${} to {}", rent, owner.name());
                if player.transact_cash(-1 * (rent as i32)).is_err() {
                    return Err(()); // player is now in trouble
                };
                owner.transact_cash(rent as i32);
                Ok(())
            }
        }
    }

    /// Execute the turn of a player
    // The turn starts with a player moving. Then, once the player is on the new square,
    // the rules for that new square execute. Lastly, other players may want to execute 
    // transactions
    fn execute_turn(&self, player: &mut player::Player, mut dice: Dice) 
            -> Result<(), ()> {

        // rolling double has special rules
        while dice.is_double() {
            if player.is_in_jail(){
                println!("YAY, you're released from jail");
                player.leave_jail();
                break;
            }

            if dice.num_rolls == 3 {
                player.go_to_jail();
                return Ok(());
            }

            // rolled a double
            println!("A double. Roll again");
            let d = match self.is_unit_test {
                true => Dice::new(10, 20), // random roll
                false => dialog::capture_dice_roll()
            };
            dice.reroll(d);
        }

        // player doesn't advance if in jail and didn't roll double
        if player.is_in_jail() {
            return Ok(());
        }

        let old_pos = player.position();
        player.advance(dice.cumulative_sum(), BOARD_SIZE);

        if player.position() < old_pos {
            println!("Yay! You pass begin and collect $200");
            player.transact_cash(200);
        }
        
        let square = self.board.get(player.position()).unwrap();
        let r = match square.square_type() {
            square::SquareType::Utility |
            square::SquareType::Station |
            square::SquareType::Street        => self.execute_square_property(player, &square,
                                                                      dice),
            square::SquareType::Corner        => self.execute_square_corner(player, &square),
            square::SquareType::Tax           => self.execute_square_tax(player),
            square::SquareType::CommunityCard => self.execute_square_community(player),
            square::SquareType::ChanceCard    => self.execute_square_chance(player)
        };

        // Player is in trouble if the transaction failed
        match r {
            Ok(_)  => player.set_in_trouble(false),
            Err(_) => player.set_in_trouble(true)
        };
        Ok(())
    }

    /// Calculate if a player owns all streets in the suburb
    fn player_owns_suburb(&self, player_idx: usize, street: &square::Square) -> bool {
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
            .collect::<Vec<&square::Square>>()
            .len();

        // player owns all, if none are unowned or owned by others
        streets_missing == 0
    }

    /// Can this street have another property, relative to other streets in the suburb
    fn street_eligible_for_house(&self, street: &square::Square) -> bool {
        let street_details = street.get_street_details().unwrap();
        let suburb = street_details.get_suburb();
        let asset = street.asset.borrow();
        let building_num = asset.house_num();

        // current building num cannot be > 1 of lowest number of buildings on street
        // ie, if we're building house no 2, all other streets must have at least 1 house
        self.board.iter()
            .filter(|s| s.name() != street.name()) // ignore own street
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            // if any street has less houses, cannot buy house
            .fold(true, |acc, s| { 
                acc && building_num <= s.asset.borrow().house_num()
            })
    }

    /// Can this street have its property sold
    fn street_eligible_for_house_sale(&self, street: &square::Square) -> bool {
        let street_details = street.get_street_details().unwrap();
        let suburb = street_details.get_suburb();
        let asset = street.asset.borrow();
        let building_num = asset.house_num();

        // current building num cannot be > 1 of lowest number of buildings on street
        // ie, if we're building house no 2, all other streets must have at least 1 house
        self.board.iter()
            .filter(|s| s.name() != street.name()) // ignore own street
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            // if any street has more houses, cannot sell house
            .fold(true, |acc, s| { 
                acc && building_num >= s.asset.borrow().house_num()
            })
    }

    /// All streets must have 4 houses or a hotel
    fn street_eligible_for_hotel(&self, street: &square::Square) -> bool {
        let street_details = street.get_street_details().unwrap();
        let suburb = street_details.get_suburb();
        let asset = street.asset.borrow();
        let building_num = asset.house_num();

        // all streets in suburb must have 4 houses or a hotel
        self.board.iter()
            .filter(|s| s.name() != street.name()) // ignore own street
            .filter(|s| match s.get_street_details() {
                    Some(sd) => sd.get_suburb() == suburb,
                    None => false
                })
            .fold(true, |acc, s| { 
                let a = s.asset.borrow();
                acc && (a.has_hotel() || a.house_num() == 4 )
            })
    }
}

/// Shuffle the deck of chance or community chest cards
fn shuffle_cards(cards: &mut Vec<card::Card>) {
    let mut idxs: Vec<usize> = (0..cards.len()).collect();

    idxs.shuffle(&mut thread_rng());
    for i in idxs.iter() {
        let c = cards.remove(0);
        cards.insert(*i, c);
    }
}

/// Load the chance cards
fn load_chance_cards() -> Vec<card::Card> {
    let mut cards = Vec::new();

    cards.push(card::Card::new("GO TO JAIL!", card::CardAction::Jail, None, None));
    cards.push(card::Card::new("Advance to St. Charles Place",
                         card::CardAction::Movement, None, Some(11)));
////        cards.push(card::Card::new("Make general repairs on all your property. House, $25 each; Hotel, $100 each", card::CardAction::PaymentDice, Some(25), None), // TODO: calculate amoun);
////       cards.push(card::Card::new("Advance to the next railroad. If unowned, you can buy it. if owned, pay twice the rent", card::CardAction::Unknown, None, None), // TODO: calculate amoun);
      cards.push(card::Card::new("You have been elected chairman of the board. Pay $50",
                           card::CardAction::Payment, Some(50), None));
//                           card::CardAction::PaymentPlayers, Some(50), None));
      cards.push(card::Card::new("Take a trip to Reading Railroad.",
                           card::CardAction::Movement, None, Some(5)));
      cards.push(card::Card::new("Speeding fine. Pay $15",
                           card::CardAction::Payment, Some(15), None));
      cards.push(card::Card::new("Your building load matures. Receive $150",
                           card::CardAction::Payment, Some(-150), None));
      cards.push(card::Card::new("Advance to Boardwalk",
                           card::CardAction::Movement, None, Some(39)));
//        card::Card::new("Go back three spaces", card::CardAction::MovementRelative, None, Some(-3)), // TODO: move relative to current square
      cards.push(card::Card::new("Advance to Illinois Avenue",
                           card::CardAction::Movement, None, Some(24)));
      cards.push(card::Card::new("Advance to GO. Collect $200",
                           card::CardAction::Movement, None, Some(0)));
      cards.push(card::Card::new("GET OUT OF JAIL FREE.",
                           card::CardAction::JailRelease, None, None));
//        card::Card::new("Take all $100 bills from the Bank and throw them in the air", card::CardAction::Unknown, None, None), // TODO: how to model this? Random allocation?
//        card::Card::new("Advance to the nearest railroad. If unowned, you can buy it. If owned, pay twice the rent", card::CardAction::Unknown, None, None), // TODO: go to closest 5,15,25,35. 2x amount
//        card::Card::new("Advance to the nearest utility. If unowned, you can buy it. If owned, roll the dice, and pay the owner 10x the roll", card::CardAction::Unknown, None, None), // TODO: pay relative to roll
    shuffle_cards(&mut cards);
    cards
}

/// Load the community chest cards
fn load_community_chest_cards() -> Vec<card::Card> {
    let mut cards = Vec::new();
//        card::Card::new("You are assessed for Street repairs: $40 per House, $115 per Hotel", card::CardAction::Payment, Some(0), None),
    cards.push(card::Card::new("GET OUT OF JAIL FREE.", card::CardAction::JailRelease, None, None));
    cards.push(card::Card::new("You have won second prize in a beauty contest. Collect $10",
                         card::CardAction::Payment, Some(-10), None));
    cards.push(card::Card::new("Life insurance matures. Collect $100",
                         card::CardAction::Payment, Some(-100), None));
    cards.push(card::Card::new("It's your birthday. Collect $40",
                         card::CardAction::Payment, Some(-40), None));
//                         card::CardAction::PaymentPlayers, Some(-40), None));
    cards.push(card::Card::new("Advance to GO. Collect $200",
                         card::CardAction::Movement, None, Some(0)));
    cards.push(card::Card::new("You inherit $100",
                         card::CardAction::Payment, Some(-100), None));
    cards.push(card::Card::new("Bank error in your favor. Collect $200",
                         card::CardAction::Payment, Some(-200), None));
    cards.push(card::Card::new("From sale of stock, you get $50",
                         card::CardAction::Payment, Some(-50), None));
    cards.push(card::Card::new("Collect $25 consultancy fee",
                         card::CardAction::Payment, Some(-25), None));
    cards.push(card::Card::new("Holiday fund matures. Collect $100",
                         card::CardAction::Payment, Some(-100), None));
    cards.push(card::Card::new("Doctor's fees. Pay $50", card::CardAction::Payment, Some(50), None));
    cards.push(card::Card::new("Hospital fees. Pay $100",
                         card::CardAction::Payment, Some(100), None));
    cards.push(card::Card::new("GO TO JAIL!", card::CardAction::Jail, None, None));
    cards.push(card::Card::new("School fees. Pay $50", card::CardAction::Payment, Some(50), None));
    cards.push(card::Card::new("Income tax refund. Collect $20",
                         card::CardAction::Payment, Some(-20), None));

    shuffle_cards(&mut cards);
    cards
}

/// Initialize the game
// Initializes the game by setting up the necessary data structures.
pub fn init(player_names: Vec::<String>) -> Game {
    let mut players = Vec::<RefCell<player::Player>>::new();
    // Create player objects
    for (i, p) in player_names.iter().enumerate() {
        players.push(RefCell::new(player::Player::new(p.to_string(), i)));
    }

    Game {
        players,
        chance_cards: RefCell::new(load_chance_cards()),
        community_cards: RefCell::new(load_community_chest_cards()),
        board: load_squares(),
        is_unit_test: false
    }
}

/// Load the squares of the board into memory
fn load_squares() -> [square::Square; BOARD_SIZE as usize] {
    [
        square::Square::new("Just chillin' at the start", square::SquareType::Corner, None),
        square::Square::new("Mediterranean Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'B', 60, 2, [4, 10, 30, 90, 160, 250], 30))),
        square::Square::new("Community Chest", square::SquareType::CommunityCard, None),
        square::Square::new("Baltic Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'B', 60, 4, [8, 20, 60, 180, 320, 450], 30))),
        square::Square::new("Income Tax", square::SquareType::Tax, None),
        square::Square::new("Reading Railroad", square::SquareType::Station,
            Some(square::StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        square::Square::new("Oriental Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'L', 100, 6, [12, 30, 90, 270, 400, 550], 50))),
        square::Square::new("Chance", square::SquareType::ChanceCard, None),
        square::Square::new("Vermont Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'L', 100, 6, [12, 30, 90, 270, 440, 550], 50))),
        square::Square::new("Connecticut Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'L', 120, 8, [16, 40, 100, 300, 450, 600], 60))),
        square::Square::new("Visiting Jail", square::SquareType::Corner, None),
        square::Square::new("St. Charles Place", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'P', 140, 10, [20, 50, 150, 450, 625, 750], 70))),
        square::Square::new("Electric Company", square::SquareType::Utility,
            Some(square::StreetDetails::new('N', 150, 0, [0,0,0,0,0,0], 75))),
        square::Square::new("States Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'P', 140, 10, [20, 50,150, 450, 625, 750], 70))),
        square::Square::new("Virginia Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'P', 160, 12, [24, 60, 180, 500, 700, 900], 80))),
        square::Square::new("Pennsylvania Railroad", square::SquareType::Station,
            Some(square::StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        square::Square::new("St. James Place", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'O', 180, 14, [28, 70, 200, 550, 750, 950], 90))),
        square::Square::new("Community Chest", square::SquareType::CommunityCard, None),
        square::Square::new("Tennessee Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'O', 180, 14, [28, 70, 200, 550, 750, 950], 90))),
        square::Square::new("New York Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'O', 200, 16, [32, 80, 220, 600, 800, 1000], 110))),
        square::Square::new("Yay! Free Parking", square::SquareType::Corner, None),
        square::Square::new("Kentucky Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'R', 220, 18, [36, 90, 250, 700, 875, 1050], 110))),
        square::Square::new("Chance", square::SquareType::ChanceCard, None),
        square::Square::new("Indiana Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'R', 220, 18, [36, 90, 250, 700, 875, 1050], 110))),
        square::Square::new("Illinois Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'R', 240, 20, [40, 100, 300, 750, 925, 1100], 120))),
        square::Square::new("B. & O. Railroad", square::SquareType::Station,
            Some(square::StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        square::Square::new("Atlantic Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'Y', 260, 22, [44, 110, 330, 800, 975, 1150], 130))),
        square::Square::new("Ventnor Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'Y', 260, 22, [44, 110, 330, 800, 975, 1150], 130))),
        square::Square::new("Water Works", square::SquareType::Utility,
            Some(square::StreetDetails::new('N', 150, 0, [0,0,0,0,0,0], 75))),
        square::Square::new("Marvin Gardens", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'Y', 280, 24, [48, 120, 360, 850, 1025, 1200], 140))),
        square::Square::new("Go To Jail", square::SquareType::Corner, None),
        square::Square::new("Pacific Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'G', 300, 26, [52, 130, 390, 900, 1100, 1275], 150))),
        square::Square::new("North Carolina Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'G', 300, 26, [52, 130, 390, 900, 1100, 1275], 150))),
        square::Square::new("Community Chest", square::SquareType::CommunityCard, None),
        square::Square::new("Pennsylvania Avenue", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'G', 320, 28, [56, 150, 450, 1000, 1200, 1400], 160))),
        square::Square::new("Short Line", square::SquareType::Station,
            Some(square::StreetDetails::new('N', 200, 0, [0,0,0,0,0,0], 100))),
        square::Square::new("Chance", square::SquareType::ChanceCard, None),
        square::Square::new("Park Place", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'I', 350, 35, [70, 175, 500, 1100, 1300, 1500], 175))),
        square::Square::new("Luxury Tax", square::SquareType::Tax, None),
        square::Square::new("Boardwalk", square::SquareType::Street, 
            Some(square::StreetDetails::new(
                    'I', 400, 50, [100, 200, 600, 1400, 1700, 2000], 200)))
    ]
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
        assert_eq!(g.players.get(0).unwrap().borrow().name(), "Bob", "First player");
        assert_eq!(g.players.get(1).unwrap().borrow().name(), "Joe", "Middle player");
        assert_eq!(g.players.get(2).unwrap().borrow().name(), "Sally", "Last player");
    }

    #[test]
    fn pay_income_tax() {
        let g = init(vec!["Test".to_string()]);
        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.cash(), 1500);
        g.execute_turn(&mut p, Dice::new(4, 0)); // income tax, $200
        assert_eq!(p.cash(), 1300);
    }

    #[test]
    fn test_pass_go() {
        let g = init(vec!["Test".to_string()]);

        // advance on top of GO
        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.cash(), 1500);
        g.execute_turn(&mut p, Dice::new(10, 0)); // visiting jail
        g.execute_turn(&mut p, Dice::new(30, 0)); // on the go square
        assert_eq!(p.cash(), 1700);

        // advance past GO
        g.execute_turn(&mut p, Dice::new(20, 0)); // free parking
        assert_eq!(p.cash(), 1700);
        g.execute_turn(&mut p, Dice::new(30, 0)); // pass go, to visiting jail
        assert_eq!(p.cash(), 1900);
    }

    #[test]
    fn jail_time() {
        let mut g = init(vec!["Test".to_string()]);
        g.set_unit_test(); // skip dialog and pay $50 to be released from jail

        // go to jail
        let mut p = g.players.get(0).unwrap().borrow_mut();
        p.receive_jail_free_card();
        assert_eq!(p.is_in_jail(), false);

        g.execute_turn(&mut p, Dice::new(30, 0));
        assert_eq!(p.position(), 10);
        assert_eq!(p.is_in_jail(), true);
        assert_eq!(p.cash(), 1500);
        assert_eq!(p.num_get_out_of_jail_cards(), 1);

        // now release, using card
        g.jail_time(&mut p);
        assert_eq!(p.num_get_out_of_jail_cards(), 0);
        assert_eq!(p.is_in_jail(), false);
        assert_eq!(p.cash(), 1500);

        // back in jail
       g.execute_turn(&mut p, Dice::new(20, 0));
       assert_eq!(p.is_in_jail(), true);
       assert_eq!(p.num_get_out_of_jail_cards(), 0);
       assert_eq!(p.cash(), 1500);

       // now release, paying $50
       g.jail_time(&mut p);
       assert_eq!(p.is_in_jail(), false);
       assert_eq!(p.num_get_out_of_jail_cards(), 0);
       assert_eq!(p.cash(), 1450);
    }

    #[test]
    fn three_2_doubles_not_in_jail() {
        let mut g = init(vec!["Test".to_string()]);
        g.set_unit_test();

        // go to jail
        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.is_in_jail(), false);

        let mut dice = Dice::new(2, 2);
        dice.reroll(Dice::new(2, 2));
        assert_eq!(dice.num_rolls, 2);
        g.execute_turn(&mut p, dice);
        assert_eq!(p.is_in_jail(), false);
    }

    #[test]
    fn three_doubles_in_jail() {
        let mut g = init(vec!["Test".to_string()]);
        g.set_unit_test();

        // go to jail
        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.is_in_jail(), false);

        let mut dice = Dice::new(2, 2);
        dice.reroll(Dice::new(2, 2));
        dice.reroll(Dice::new(2, 2));
        assert_eq!(dice.num_rolls, 3);
        g.execute_turn(&mut p, dice);
        assert_eq!(p.is_in_jail(), true);
        assert_eq!(p.cash(), 1500);
    }

    #[test]
    fn player_in_jail_does_not_advance() {
        let mut g = init(vec!["Jailbird".to_string()]);
        g.set_unit_test();

        // go to jail
        let mut p = g.players.get(0).unwrap().borrow_mut();
        p.transact_cash(-1480);
        let mut dice = Dice::new(2, 2);
        dice.reroll(Dice::new(2, 2));
        dice.reroll(Dice::new(2, 2));
        g.execute_turn(&mut p, dice);
        assert_eq!(p.is_in_jail(), true);
        assert_eq!(p.cash(), 20); // now can't pay $50 to be free from jail

        // stay in jail when not rolling double or paying $50 to get free
        g.execute_turn(&mut p, Dice::new(3, 2));
        assert_eq!(p.is_in_jail(), true);
        assert_eq!(p.position(), 10);
        assert_eq!(p.cash(), 20);

        // player is free when rolling double
        g.execute_turn(&mut p, Dice::new(3, 3));
        assert_eq!(p.is_in_jail(), false);
        assert_eq!(p.position(), 16);
    }

    #[test]
    fn calculate_rent_unowned() {
        let mut g = init(vec!["Test".to_string()]);
        g.set_unit_test();
        // Unowned square | No Rent
        let s = g.board.get(1).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, None);
        
        // Income tax | No rent
        let s = g.board.get(4).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, None);
       
        // no-rent square
        let s = g.board.get(10).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, None);
    }

    #[test]
    fn calculate_rent_mortgaged() {
        let mut g = init(vec!["StreetOwner".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, Dice::new(3, 0)); // Owner moves to Baltic Avenue

        s.asset.borrow_mut().mortgage();
        assert_eq!(g.calculate_rent(s, Dice::new(0, 0)), None);
        s.asset.borrow_mut().unmortgage();
        assert_eq!(g.calculate_rent(s, Dice::new(0, 0)), Some(4));
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
        g.execute_turn(&mut p, Dice::new(3, 0)); // Owner moves to Baltic Avenue
        g.execute_turn(&mut p, Dice::new(3, 0)); // Owner moves to Oriental Avenue
        g.execute_turn(&mut p, Dice::new(2, 0)); // Vermont Avenue
        g.execute_turn(&mut p, Dice::new(3, 0)); // St. Charles place
        g.execute_turn(&mut p, Dice::new(2, 0)); // States Ave
        g.execute_turn(&mut p, Dice::new(1, 0)); // Virginia Ave
        g.execute_turn(&mut p, Dice::new(23, 0)); // Park place
        g.execute_turn(&mut p, Dice::new(2, 0)); // Boardwalk

        // Rent for 1 of 2 set 
        let s = g.board.get(3).unwrap(); // Baltic
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(4));

        // Rent for 2 of 3 set 
        let s = g.board.get(6).unwrap(); // Oriental
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(6));
        let s = g.board.get(8).unwrap(); // Vermont
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(6));

        // rent for 3 of 3 set 
        let s = g.board.get(11).unwrap(); // St. Charles 
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(20));
        let s = g.board.get(13).unwrap(); // States Ave
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(20));
        let s = g.board.get(14).unwrap(); // Virginia Ave
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(24));
        
        // rent for 2 of 2 set 
        let s = g.board.get(37).unwrap(); // Park Ave
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(70));
        let s = g.board.get(39).unwrap(); // Boardwalk
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(100));
    }

    #[test]
    fn calculate_rent_utility() {
        // Buy 1 utility, then buy the second
        let mut g = init(vec!["TestOwner".to_string(), "TestRenter".to_string()]);
        g.set_unit_test();

        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, Dice::new(12, 0)); // Electric
        let s = g.board.get(12).unwrap(); // Electric
        let r = g.calculate_rent(s, Dice::new(1, 2));
        assert_eq!(r, Some(12));  // 3 * 4

        g.execute_turn(&mut p, Dice::new(16, 0)); // Water
        let s = g.board.get(28).unwrap();
        let r = g.calculate_rent(s, Dice::new(3, 0));
        assert_eq!(r, Some(30)); // 3 * 10

        // cumulative roll
        let mut dice = Dice::new(3, 0);
        dice.reroll(Dice::new(6, 0)); // cumulative sum is 9
        let r = g.calculate_rent(s, dice);
        assert_eq!(r, Some(90)); // 9 * 10
    }

    #[test]
    fn calculate_rent_station() {
        // Buy stations one at a time
        let mut g = init(vec!["StationOwner".to_string(),
                              "StationRenter".to_string()]);
        g.set_unit_test();

        let mut p = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut p, Dice::new(5, 0)); // Reading Railroad
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(25)); 

        g.execute_turn(&mut p, Dice::new(10, 0)); // Pennsylvania Railroad
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(50)); 

        g.execute_turn(&mut p, Dice::new(10, 0)); // B.O. Railroad
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
        assert_eq!(r, Some(100)); 

        g.execute_turn(&mut p, Dice::new(10, 0)); // Short line
        let s = g.board.get(5).unwrap();
        let r = g.calculate_rent(s, Dice::new(0, 0));
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
            assert_eq!(owner.cash(), 1500);
            g.execute_turn(&mut owner, Dice::new(3, 0)); // Owner moves to Baltic Avenue
            assert_eq!(owner.cash(), 1440); // bought street
            assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx());
        }

        let mut renter = g.players.get(1).unwrap().borrow_mut();
        g.execute_turn(&mut renter, Dice::new(3, 0)); // Renter on Baltic Ave
        assert_eq!(renter.cash(), 1496);
        let owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash(), 1444);
    }

    #[test]
    fn buy_each_property_type() {
        let mut g = init(vec!["Mongul".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut p = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(p.cash(), 1500);
        
        g.execute_turn(&mut p, Dice::new(3, 0)); // Mongul moves to Baltic Avenue
        assert_eq!(p.cash(), 1440); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), p.turn_idx());

        g.execute_turn(&mut p, Dice::new(9, 0)); // Mongul moves to Electric Company
        assert_eq!(p.cash(), 1290); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), p.turn_idx());

        g.execute_turn(&mut p, Dice::new(3, 0)); // Mongul moves to Pennsylvania Railroad
        assert_eq!(p.cash(), 1090); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), p.turn_idx());
    }

    #[test]
    fn sell_a_property() {
        let mut g = init(vec!["Seller".to_string(), "NewOwner".to_string()]);
        g.set_unit_test();
        let s = g.board.get(3).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut seller = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(seller.cash(), 1500);
        
        g.execute_turn(&mut seller, Dice::new(3, 0)); // Seller moves to Baltic Avenue
        assert_eq!(seller.cash(), 1440); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), seller.turn_idx());

        let mut owner = g.players.get(1).unwrap().borrow_mut();
        g.sell_property(&mut seller, &mut owner, &s, 20);
        assert_eq!(owner.cash(), 1480);
        assert_eq!(seller.cash(), 1460);
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx());
    }

    #[test]
    fn mortgage_then_unmortgage() {
        let mut g = init(vec!["M".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 3;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash(), 1500);
        
        g.execute_turn(&mut owner, Dice::new(3, 0)); // Seller moves to Baltic Avenue
        assert_eq!(owner.cash(), 1440); // bought street
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx());

        actions::mortgage_street(&g, &mut owner, street_idx);
        assert_eq!(owner.cash(), 1470); // mortgage of 30
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx());
        assert_eq!(s.asset.borrow().is_mortgaged(), true);

        actions::unmortgage_street(&g, &mut owner, street_idx);
        assert_eq!(owner.cash(), 1437); // unmortgage for 33
        assert_eq!(s.asset.borrow().owner.unwrap(), owner.turn_idx());
        assert_eq!(s.asset.borrow().is_mortgaged(), false);
    }

    #[test]
    fn shuffle_fake_cards() {
        let mut cards = Vec::new();
        cards.push(card::Card::new("Card 1", card::CardAction::Payment, Some(-1), None));
        cards.push(card::Card::new("Card 2", card::CardAction::Payment, Some(2), None));
        cards.push(card::Card::new("Card 3", card::CardAction::Payment, Some(10), None));
        cards.push(card::Card::new("Card 4", card::CardAction::Payment, Some(100), None));
        cards.push(card::Card::new("Card 5", card::CardAction::Payment, Some(-200), None));
        cards.push(card::Card::new("Card 6", card::CardAction::Payment, Some(-80), None));
        shuffle_cards(&mut cards);
        assert_eq!(cards.len(), 6);
        assert_eq!(false, // there must be a change in order
            cards.get(0).unwrap().description() == "Card 1" &&
            cards.get(1).unwrap().description() == "Card 2" &&
            cards.get(2).unwrap().description() == "Card 3" &&
            cards.get(3).unwrap().description() == "Card 4" &&
            cards.get(4).unwrap().description() == "Card 5" &&
            cards.get(4).unwrap().description() == "Card 6"
        );
    }

    #[test]
    fn unmortgage_cost_calculation() {
        assert_eq!(33, square::StreetDetails::new('B', 60, 2, [4,0,0,0,0,0], 30)
                   .get_unmortgage_amount());
        assert_eq!(193, square::StreetDetails::new('B', 60, 2, [4,0,0,0,0,0], 175)
                   .get_unmortgage_amount());
        assert_eq!(220, square::StreetDetails::new('B', 60, 2, [4,0,0,0,0,0], 200)
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
        assert_eq!(owner.cash(), 1500);
        
        g.execute_turn(&mut owner, Dice::new(1, 0)); // Seller moves to Mediterranean Avenue
        assert_eq!(owner.cash(), 1440); // bought street
        actions::buy_house(&g, &mut owner, street_idx); // whole suburb isn't owned
        assert_eq!(s.asset.borrow().house_num(), 0);

        g.execute_turn(&mut owner, Dice::new(2, 0)); // Seller moves to Baltic Avenue and buys it
        assert_eq!(owner.cash(), 1380); // bought street
        actions::buy_house(&g, &mut owner, street_idx); // buy Mediterranean
        assert_eq!(s.asset.borrow_mut().house_num(), 1);
        assert_eq!(owner.cash(), 1330); // bought house for 50

        // cannot buy second house on mediterranean
        actions::buy_house(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow_mut().house_num(), 1);
        assert_eq!(owner.cash(), 1330); // bought house for 50

        // sell house
        actions::sell_house(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow_mut().house_num(), 0);
        assert_eq!(owner.cash(), 1380); // sell house for 50
    }

    #[test]
    fn buy_houses_then_hotel() {
        // buy all possible houses for suburb
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);
        let rs = s.get_street_details().unwrap().rent_suburb();

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        assert_eq!(owner.cash(), 1500);
        
        g.execute_turn(&mut owner, Dice::new(1, 0)); // Seller moves to Mediterranean Avenue
        assert_eq!(owner.cash(), 1440); // bought street
        assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(),
                   2); // rent for Mediterranean w/o suburb

        g.execute_turn(&mut owner, Dice::new(2, 0)); // Seller moves to Baltic Avenue
        assert_eq!(owner.cash(), 1380); // bought street
        assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(),
                   rs[0]); // rent for suburb

        // tycoon now owns all streets, lets iteratively buy houses up to 4
        for i in 0..4 {
            // buy another house in Mediterranean then Baltic
            let street_idx = 1;
            let s = g.board.get(street_idx).unwrap();
            actions::buy_house(&g, &mut owner, street_idx);
            assert_eq!(s.asset.borrow().house_num(), (i+1));
            let r: u32 = rs[(i+1) as usize];
            assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(),
                       r); // rent for houses

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
        g.execute_turn(&mut owner, Dice::new(1, 0)); // Seller moves to Mediterranean Avenue

        // try buy hotel
        actions::buy_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().house_num(), 0);
        assert_eq!(s.asset.borrow().has_hotel(), false);

        g.execute_turn(&mut owner, Dice::new(2, 0)); // Seller moves to Baltic Avenue
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
        g.execute_turn(&mut owner, Dice::new(1, 0));
        g.execute_turn(&mut owner, Dice::new(2, 0));

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
        assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(), 250);

        let street_idx = 3;
        let s = g.board.get(street_idx).unwrap();
        actions::buy_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), true);
        assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(), 450);

        // Selling hotels succeeds
        actions::sell_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), false);
        assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(), 320);

        let street_idx = 1;
        let s = g.board.get(street_idx).unwrap();
        actions::sell_hotel(&g, &mut owner, street_idx);
        assert_eq!(s.asset.borrow().has_hotel(), false);
        assert_eq!(g.calculate_rent(s, Dice::new(3, 0)).unwrap(), 160);
    }

    #[test]
    fn check_house_eligibility() {
        // buy all possible houses for suburb
        let mut g = init(vec!["Tycoon".to_string()]);
        g.set_unit_test();
        let street_idx: usize = 1;
        let s = g.board.get(street_idx).unwrap();
        assert_eq!(s.asset.borrow().owner, None);
        let rs = s.get_street_details().unwrap().rent_suburb();

        let mut owner = g.players.get(0).unwrap().borrow_mut();
        
        g.execute_turn(&mut owner, Dice::new(1, 0)); // buys mediterranean
        assert_eq!(g.player_owns_suburb(owner.turn_idx(), &s), false);
        assert_eq!(g.street_eligible_for_house(&s), true);
        assert_eq!(g.street_eligible_for_hotel(&s), false);
        let s = g.board.get(3).unwrap();
        assert_eq!(g.player_owns_suburb(owner.turn_idx(), &s), false);
        assert_eq!(g.street_eligible_for_house(&s), true);
        assert_eq!(g.street_eligible_for_hotel(&s), false);

        g.execute_turn(&mut owner, Dice::new(2, 0)); // buys Baltic
        assert_eq!(g.player_owns_suburb(owner.turn_idx(), &s), true);
        assert_eq!(g.street_eligible_for_house(&s), true); // can now have houses
        assert_eq!(g.street_eligible_for_hotel(&s), false);
        let s = g.board.get(1).unwrap();
        assert_eq!(g.player_owns_suburb(owner.turn_idx(), &s), true);
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

    #[test]
    fn roll_dice() {
        let mut dice = Dice::new(2, 3);
        assert_eq!(dice.roll, (2,3)); 
        assert_eq!(dice.num_rolls, 1);
        assert_eq!(dice.cumulative_sum, 5);
        assert_eq!(dice.is_double(), false);

        let mut dice = Dice::new(3, 3);
        assert_eq!(dice.roll, (3,3)); 
        assert_eq!(dice.num_rolls, 1);
        assert_eq!(dice.cumulative_sum, 6);
        assert_eq!(dice.is_double(), true);

        dice.reroll(Dice::new(5, 2));
        assert_eq!(dice.roll, (5, 2)); 
        assert_eq!(dice.num_rolls, 2);
        assert_eq!(dice.cumulative_sum, 13);
        assert_eq!(dice.is_double(), false);
    }

    #[test]
    fn player_in_trouble() {
        // player is in trouble when can't pay their bill
        // move player to income tax 8 times. on the last time, they're in trouble
        let mut g = init(vec!["Chancer".to_string()]);
        g.set_unit_test();
        let mut player = g.players.get(0).unwrap().borrow_mut();
        g.execute_turn(&mut player, Dice::new(4, 0)); // income tax pay 200
        for i in 1..8 {
            assert_eq!(player.cash(), (1500 - (200*i)));
            g.execute_turn(&mut player, Dice::new(40, 0)); // income tax, pay 200
        }
        assert_eq!(player.cash(), 100); // didn't pay yet
        assert_eq!(player.is_in_trouble(), true); // now in trouble
        // do this another 
    }

    #[test]
    fn player_on_mortgaged_property() {
        let mut g = init(vec!["A".to_string(), "B".to_string()]);
        g.set_unit_test();
        let street_idx = 3;
        let street = g.board.get(street_idx).unwrap();

        {
            let mut player = g.players.get(0).unwrap().borrow_mut();
            g.execute_turn(&mut player, Dice::new(1, 2)); // buy baltic
            assert_eq!(street.asset.borrow().owner.unwrap(), 0);
            // mortgage Baltic
            actions::mortgage_street(&g, &mut player, street_idx);
            assert_eq!(street.asset.borrow().is_mortgaged(), true);
        }

        // Player B lands on baltic and pays no rent
        let mut player = g.players.get(1).unwrap().borrow_mut();
        assert_eq!(player.cash(), 1500);
        g.execute_turn(&mut player, Dice::new(1, 2)); // land on baltic
        assert_eq!(street.asset.borrow().owner.unwrap(), 0);
        assert_eq!(player.cash(), 1500);
    }
}
