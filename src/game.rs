use rand::thread_rng;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::io;
use std::io::{Write};
use std::fmt;
use std::cell::{Cell, RefCell};

const BOARD_SIZE: i32 = 40; // 40 squares on the board
static SQUARES: [Square; BOARD_SIZE as usize] = [
    Square::new("Just chillin' at the start", SquareType::Corner, None, None),
    Square::new("Mediterranean Avenue", SquareType::Street, Some(60), Some(2)),
    Square::new("Community Chest", SquareType::CommunityCard, None, None),
    Square::new("Baltic Avenue", SquareType::Street, Some(60), Some(4)),
    Square::new("Income Tax", SquareType::Tax, None, None),
    Square::new("Reading Railroad", SquareType::Station, Some(200), Some(25)),
    Square::new("Oriental Avenue", SquareType::Street, Some(100), Some(6)),
    Square::new("Chance", SquareType::ChanceCard, None, None),
    Square::new("Vermont Avenue", SquareType::Street, Some(100), Some(6)),
    Square::new("Connecticut Avenue", SquareType::Street, Some(120), Some(8)),
    Square::new("Visiting Jail", SquareType::Corner, None, None),
    Square::new("St. Charles Place", SquareType::Street, Some(140), Some(10)),
    Square::new("Electric Company", SquareType::Utility, Some(150), None),
    Square::new("States Avenue", SquareType::Street, Some(140), Some(10)),
    Square::new("Virginia Avenue", SquareType::Street, Some(160), Some(12)),
    Square::new("Pennsylvania Railroad", SquareType::Station, Some(200), Some(25)),
    Square::new("St. James Place", SquareType::Street, Some(180), Some(14)),
    Square::new("Community Chest", SquareType::CommunityCard, None, None),
    Square::new("Tennessee Avenue", SquareType::Street, Some(180), Some(14)),
    Square::new("New York Avenue", SquareType::Street, Some(200), Some(16)),
    Square::new("Yay! Free Parking", SquareType::Corner, None, None),
    Square::new("Kentucky Avenue", SquareType::Street, Some(220), Some(18)),
    Square::new("Chance", SquareType::ChanceCard, None, None),
    Square::new("Indiana Avenue", SquareType::Street, Some(220), Some(18)),
    Square::new("Illinois Avenue", SquareType::Street, Some(240), Some(20)),
    Square::new("B. & O. Railroad", SquareType::Station, Some(200), Some(25)),
    Square::new("Atlantic Avenue", SquareType::Street, Some(260), Some(22)),
    Square::new("Ventnor Avenue", SquareType::Street, Some(260), Some(22)),
    Square::new("Water Works", SquareType::Utility, Some(150), None),
    Square::new("Marvin Gardens", SquareType::Street, Some(280), Some(24)),
    Square::new("Go To Jail", SquareType::Corner, None, None),
    Square::new("Pacific Avenue", SquareType::Street, Some(300), Some(26)),
    Square::new("North Carolina Avenue", SquareType::Street, Some(300), Some(26)),
    Square::new("Community Chest", SquareType::CommunityCard, None, None),
    Square::new("Pennsylvania Avenue", SquareType::Street, Some(320), Some(28)),
    Square::new("Short Line", SquareType::Street, Some(200), Some(25)),
    Square::new("Chance", SquareType::ChanceCard, None, None),
    Square::new("Park Place", SquareType::Street, Some(350), Some(35)),
    Square::new("Luxury Tax", SquareType::Tax, None, None),
    Square::new("Boardwalk", SquareType::Street, Some(400), Some(50)),
];

enum CardAction {
    Movement,
    MovementRelative, // move relative to starting square
    Payment,
    PaymentDice, // payment calculated based on dice roll
    PaymentPlayers, // payment calculated based on dice roll
    Jail, 
    JailRelease, 
    Unknown
    // TODO: add other actions
}

#[derive(Debug)]
enum SquareType {
    ChanceCard,
    CommunityCard,
    Corner,
    Station,
    Street,
    Tax,
    Utility
}

const CHANCE_CARDS: [Card; 10 as usize] = [
        Card::new("GO TO JAIL!", CardAction::Jail, None, None),
        Card::new("Advance to St. Charles Place", CardAction::Movement, None, Some(11)),
//        Card::new("Make general repairs on all your property. House, $25 each; Hotel, $100 each", CardAction::PaymentDice, Some(25), None), // TODO: calculate amount
 //       Card::new("Advance to the next railroad. If unowned, you can buy it. if owned, pay twice the rent", CardAction::Unknown, None, None), // TODO: calculate amount
        Card::new("You have been elected chairman of the board. Pay $50", CardAction::PaymentPlayers, Some(50), None), // TODO: calculate amount to charge 'each player'
        Card::new("Take a trip to Reading Railroad.", CardAction::Movement, None, Some(5)),
        Card::new("Speeding fine. Pay $15", CardAction::Payment, Some(15), None),
        Card::new("Your building load matures. Receive $150", CardAction::Payment, Some(-150), None),
        Card::new("Advance to Boardwalk", CardAction::Movement, None, Some(39)),
//        Card::new("Go back three spaces", CardAction::MovementRelative, None, Some(-3)), // TODO: move relative to current square
        Card::new("Advance to Illinois Avenue", CardAction::Movement, None, Some(24)),
        Card::new("Advance to GO. Collect $200", CardAction::Movement, None, Some(0)),
        Card::new("GET OUT OF JAIL FREE.", CardAction::JailRelease, None, None),
 //       Card::new("Take all $100 bills from the Bank and throw them in the air", CardAction::Unknown, None, None), // TODO: how to model this? Random allocation?
//        Card::new("Advance to the nearest railroad. If unowned, you can buy it. If owned, pay twice the rent", CardAction::Unknown, None, None), // TODO: go to closest 5,15,25,35. 2x amount
//        Card::new("Advance to the nearest utility. If unowned, you can buy it. If owned, roll the dice, and pay the owner 10x the roll", CardAction::Unknown, None, None), // TODO: pay relative to roll
];
const COMMUNITY_CARDS: [Card; 15 as usize] = [
//        Card::new("You are assessed for Street repairs: $40 per House, $115 per Hotel", CardAction::Payment, Some(0), None),
        Card::new("GET OUT OF JAIL FREE.", CardAction::JailRelease, None, None),
        Card::new("You have won second prize in a beauty contest. Collect $10", CardAction::Payment, Some(-10), None),
        Card::new("Life insurance matures. Collect $100", CardAction::Payment, Some(-100), None),
        Card::new("It's your birthday. Collect $40", CardAction::PaymentPlayers, Some(-40), None),
        Card::new("Advance to GO. Collect $200", CardAction::Movement, None, Some(0)),
        Card::new("You inherit $100", CardAction::Payment, Some(-100), None),
        Card::new("Bank error in your favor. Collect $200", CardAction::Payment, Some(-200), None),
        Card::new("From sale of stock, you get $50", CardAction::Payment, Some(-50), None),
        Card::new("Collect $25 consultancy fee", CardAction::Payment, Some(-25), None),
        Card::new("Holiday fund matures. Collect $100", CardAction::Payment, Some(-100), None),
        Card::new("Doctor's fees. Pay $50", CardAction::Payment, Some(50), None),
        Card::new("Hospital fees. Pay $100", CardAction::Payment, Some(100), None),
        Card::new("GO TO JAIL!", CardAction::Jail, None, None),
        Card::new("School fees. Pay $50", CardAction::Payment, Some(50), None),
        Card::new("Income tax refund. Collect $20", CardAction::Payment, Some(-20), None)
];

struct Asset {
    owner: Cell<Option<usize>>, // usize is a ref to the index of the player object
    house_num: i32,
    has_hotel: bool,
    is_mortgaged: bool
}

struct Card {
    description: &'static str,
    action: CardAction,
    amount: Option<i32>,
    square: Option<i32>
}

/// The structure, containing links to all parts of the game
pub struct Game {
    players: Vec<RefCell<Player>>,
    active_player: Cell<usize>, // index of active player in the players vec
    chance_cards: RefCell<Vec<usize>>, // references to cards in CHANCE_CARDS
    community_cards: RefCell<Vec<usize>> // references to cards in COMMUNITY_CARDS
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Player {
    name: String,
    position: usize, // the index of the board square
    turn_idx: usize, // idx in the group of players. need this to match asset
    cash: i32,
    is_in_jail: bool,
    num_get_out_of_jail_cards: i32,
}

pub struct Square {
    name: &'static str,
    square_type: SquareType,
    price: Option<i32>,
    rent: Option<i32>,
    asset: Asset
    
//    rent_full_street: i32,
//    rent_1house: i32,
//    rent_2house: i32,
//    rent_3house: i32,
//    rent_4house: i32,
//    rent_hotel: i32,
//    mortgage_cost: i32,
//    mortgage_redeem: i32,
}

unsafe impl Sync for Square {}

impl Asset {
    pub const fn new() -> Self {
        Self { 
            owner: Cell::new(None),
            house_num: 0,
            has_hotel: false,
            is_mortgaged: false
        }
    }
}

impl Card {
    pub const fn new(description: &'static str, action: CardAction, amount: Option<i32>, square: Option<i32>) -> Self {
        Self {
            description,
            action,
            amount,
            square
        }
    }
}

impl Game {

    /// Start the game
    pub fn start(&self) {
        loop {
            for p in 0..self.players.len() {
                &self.active_player.set(p as usize); // update active player
                print!("{}, roll dice: ", &self.players.get(p).unwrap().borrow().name);
                let dice_roll = capture_dice_roll();

                let mut player = self.players.get(self.active_player.get()).unwrap().borrow_mut();
                self.execute_turn(&mut player, dice_roll);
                println!("");
            }
            self.print_summary();
        }
    }

    /// Print the game summary
    // Prints out stats for each player
    pub fn print_summary(&self) {
        for p in self.players.iter() {
            println!("{}", p.borrow());
        }
        println!("");
    }

    /// Execute action on card
    fn execute_card(&self, player: &mut Player, card: &Card) {
        println!("{}", card.description);
        match card.action {
            CardAction::Movement =>  {
                // calculate the dice number based on square
                let target = card.square.expect("Target square should exist");
                let mut dice_roll = target - (player.position as i32);
                if dice_roll < 0 {
                    dice_roll += BOARD_SIZE; // march around the board
                }
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

    /// Execute the turn of a player
    // The turn starts with a player moving. Then, once the player is on the new square,
    // the rules for that new square execute. Lastly, other players may want to execute 
    // transactions
    fn execute_turn(&self, player: &mut Player, dice_roll: i32) {
        self.break_out_of_jail(player); // does nothing if player is not in jail

        let old_pos = player.position;
        player.advance(dice_roll);
        if player.position < old_pos {
            println!("Yay! You pass begin and collect $200");
            player.transact_cash(200);
        }
        // TODO: If 3 doubles, go to jail
        
        let square = SQUARES.get(player.position).unwrap();
        match square.square_type {
            SquareType::Corner => {
                if player.position == 30 {
                    player.go_to_jail();
                } else {
                    println!("{}", square.name);
                }
            },
            SquareType::Tax => {
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
            },
            SquareType::CommunityCard | SquareType::ChanceCard => {
                let (mut cards, ref_deck) = match square.square_type {
                    SquareType::CommunityCard => {
                        print!("COMMUNITY CHEST: ");
                        let mut cards = self.community_cards.borrow_mut();
                        (cards, &COMMUNITY_CARDS)
                    },
                    _ => {
                        print!("CHANCE: ");
                        let mut cards = self.chance_cards.borrow_mut();
                        (cards, &COMMUNITY_CARDS)
                    }
                };
                let idx = cards.remove(0);
                let card = ref_deck.get(idx).expect("Card should exist");
                self.execute_card(player, card);
                cards.push(idx);
            },
            SquareType::Utility | SquareType::Station | SquareType::Street => {
                println!("You landed on {}", square.name);
                match square.asset.owner.get() {
                    Some(owner_idx) => { // this street is owned by somebody
                        // check if active player is the owner
                        if owner_idx == self.active_player.get() {
                            println!("Phew! Luckily it's yours");
                            return;
                        }

                        let mut owner = self.players.get(owner_idx).unwrap().borrow_mut();
                        let rent = square.calculate_rent(dice_roll);
                        println!("Oh no! You pay ${} to {}", rent, owner.name);
                        player.transact_cash(-1 * rent);
                        owner.transact_cash(rent);
                    },
                    None => {
                        let price = square.price.expect("Price of street should exist");
                        println!("You buy it for ${}", price);
                        if player.cash > price {
                            player.transact_cash(-1 * price);
                            square.asset.owner.replace(Some(self.active_player.get()));
                        } else {
                            println!("Not enough money. It stays on the market");
                            // TODO: Implement auction where bank-person inputs player and price
                        }
                    }
                }
            }
        }
    }
}

impl Square {
    const fn new(name: &'static str, square_type: SquareType, price: Option<i32>, rent: Option<i32>) -> Self {
        Self {
            name,
            price,
            rent,
            square_type,
            asset: Asset::new()
        }
    }

    /// Calculate rent
    // Calculate rent, taking into account if a player owns all streets, and the number of
    // properties on the street.
    pub fn calculate_rent(&self, dice_roll: i32) -> i32 {
        // TODO: check for owner owning street/stations, houses, hotels
        match self.square_type {
            SquareType::Utility => {
                dice_roll * 4
            },
            SquareType::Station => {
                self.rent.expect("Rent should exist")
            },
            SquareType::Street => {
                self.rent.expect("Rent should exist")
            }
            _ => 0
        }
    }
}

impl<'a> fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let square = SQUARES.get(self.position).unwrap();
        // get owned streets
        let mut streets = Vec::<&str>::new();
        for s in SQUARES.iter() {
            match s.asset.owner.get() {
                Some(owner_idx) => {
                    if owner_idx == self.turn_idx {
                        streets.push(&s.name);
                    }
                },
                None => {}
            }
        }
        write!(f, "--- {} ---
            Square: {}
            Cash: {}
            In Jail?: {}
            Get-Out-Of-Jail cards: {}
            Streets: {:?}",
            self.name, square.name, self.cash,
            self.is_in_jail, self.num_get_out_of_jail_cards,
            streets)
        // TODO: if streets have houses/hotels, print them on the same line
        // TODO: if has get-out-of-jail card, then print it
        // TODO: if has mortgagages, print them
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
    pub fn advance(&mut self, steps: i32) {
        let target_square = ((self.position as i32) + steps) % BOARD_SIZE;
        self.position = target_square as usize;
    }

    /// Go to jail
    // Player doesn't collect 200, and goes straight to jail
    pub fn go_to_jail(&mut self) {
        println!("GO TO JAIL!");
        self.is_in_jail = true;
        self.position = 10;
    }

    /// Transact in cash.
    // Adds `amount` to players cash amount. Also works for negative numbers
    pub fn transact_cash(&mut self, amount: i32) {
        self.cash += amount;
    }
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

/// Shuffle the deck of chance or community chest cards
fn shuffle_cards(card_num: usize) -> Vec::<usize> {
    // shuffle the indexes
    let mut idxs: Vec<usize> = (0..card_num).collect();
    idxs.shuffle(&mut thread_rng());
    //println!("{:?}", idxs);
    idxs
}

/// Initialize the game
// Initializes the game by setting up the necessary data structures.
pub fn init(player_names: Vec::<String>) -> Game {
    let mut players = Vec::<RefCell<Player>>::new();
    // Create player objects
    for (i, p) in player_names.iter().enumerate() {
        players.push(RefCell::new(Player::new(p.to_string(), i)));
    }

    // create players from String
    let asset_register = HashMap::<&Square, Asset>::new();

    // Deal with cards
    let chance_cards = RefCell::new(shuffle_cards(CHANCE_CARDS.len()));
    let community_cards = RefCell::new(shuffle_cards(COMMUNITY_CARDS.len()));

    Game {
        active_player: Cell::new(0),
        players,
        chance_cards,
        community_cards,
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
    fn test_game_init() {
        let v = vec!["Bob".to_string(),"Joe".to_string(),"Sally".to_string()];
        let len = v.len();
        let g = init(v);
        assert_eq!(g.players.len(), len, "All players created");
        assert_eq!(g.players.get(0).unwrap().borrow().name, "Bob", "First player");
        assert_eq!(g.players.get(1).unwrap().borrow().name, "Joe", "Middle player");
        assert_eq!(g.players.get(2).unwrap().borrow().name, "Sally", "Last player");
    }

    #[test]
    fn test_advance() {
        let ref mut p = Player::new("Test".to_string(), 1);
        assert_eq!(p.position, 0);
        p.advance(37);
        assert_eq!(p.position, 37);
        p.advance(5); // wrap around BOARD_SIZE
        assert_eq!(p.position, 2);
    }

    #[test]
    fn test_income_tax() {
        let g = init(vec!["Test".to_string()]);
        let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
        assert_eq!(p.cash, 1500);
        g.execute_turn(&mut p, 4); // income tax, $200
        assert_eq!(p.cash, 1300);
    }

    #[test]
    fn test_pass_go() {
        let g = init(vec!["Test".to_string()]);

        // advance on top of GO
        let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
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
        let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
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
    fn test_calculate_rent() {
        // Baltic avenue
        let s = SQUARES.get(3).unwrap();
        let r = s.calculate_rent(3);
        assert_eq!(r, 4);

        // no-rent square
        let s = SQUARES.get(10).unwrap();
        let r = s.calculate_rent(10);
        assert_eq!(r, 0);

        // Income tax
        let s = SQUARES.get(4).unwrap();
        let r = s.calculate_rent(4);
        assert_eq!(r, 0);

        // Utility
        let s = SQUARES.get(12).unwrap();
        let r = s.calculate_rent(8);
        assert_eq!(r, 32); // 4 * 8
    }

    #[test]
    fn test_purchase_and_pay_rent() {
        let g = init(vec!["TestA".to_string(), "TestB".to_string()]);
        let s = SQUARES.get(3).unwrap();
        assert_eq!(s.asset.owner.get(), None);

        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            assert_eq!(p.cash, 1500);
            g.execute_turn(&mut p, 3); // TestA moves to Baltic Avenue
            assert_eq!(p.cash, 1440); // bought street
            assert_eq!(s.asset.owner.get().unwrap(), g.active_player.get());
        }

        {
            g.active_player.set(1 as usize); // update active player
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 3); // TestB moves to Baltic Avenue and pays rent
        }
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1444);
        assert_eq!(g.players.get(1).unwrap().borrow().cash, 1496);
    }
}
