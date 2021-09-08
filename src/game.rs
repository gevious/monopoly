use rand::thread_rng;
use rand::Rng;

use std::collections::HashMap;
use std::io;
use std::io::{Write};
use std::fmt;
use std::cell::{Cell, RefCell};

const BOARD_SIZE: i32 = 40; // 40 squares on the board
static SQUARES: [Square; BOARD_SIZE as usize] = [
    Square::new("Just chillin' at the start", None, None),
    Square::new("Mediterranean Avenue", Some(60), None),
    Square::new("Community Chest", None, None),
    Square::new("Baltic Avenue", Some(60), Some(4)),
    Square::new("Income Tax", None, None),
    Square::new("Reading Railroad", Some(200), None),
    Square::new("Oriental Avenue", Some(100), None),
    Square::new("Chance", None, None),
    Square::new("Vermont Avenue", Some(100), None),
    Square::new("Connecticut Avenue", Some(120), None),
    Square::new("Visiting Jail", None, None),
    Square::new("St. Charles Place", Some(140), None),
    Square::new("Electric Company", Some(150), None),
    Square::new("States Avenue", Some(140), None),
    Square::new("Virginia Avenue", Some(160), None),
    Square::new("Pennsylvania Railroad", Some(200), None),
    Square::new("St. James Place", Some(180), None),
    Square::new("Community Chest", None, None),
    Square::new("Tennessee Avenue", Some(180), None),
    Square::new("New York Avenue", Some(200), None),
    Square::new("Free Parking", None, None),
    Square::new("Kentucky Avenue", Some(220), None),
    Square::new("Chance", None, None),
    Square::new("Indiana Avenue", Some(220), None),
    Square::new("Illinois Avenue", Some(240), None),
    Square::new("B. & O. Railroad", Some(200), None),
    Square::new("Atlantic Avenue", Some(260), None),
    Square::new("Ventnor Avenue", Some(260), None),
    Square::new("Water Works", Some(150), None),
    Square::new("Marvin Gardens", Some(280), None),
    Square::new("Go To Jail", None, None),
    Square::new("Pacific Avenue", Some(300), None),
    Square::new("North Carolina Avenue", Some(300), None),
    Square::new("Community Chest", None, None),
    Square::new("Pennsylvania Avenue", Some(320), None),
    Square::new("Short Line", Some(200), None),
    Square::new("Chance", None, None),
    Square::new("Park Place", Some(350), None),
    Square::new("Luxury Tax", None, None),
    Square::new("Boardwalk", Some(400), None),
];

enum CardAction {
    Unknown
    // TODO: add other actions
}

static CHANCE_CARDS: [Card; 16 as usize] = [
        Card::new("GO TO JAIL", CardAction::Unknown, None, Some(10)),
        Card::new("Advance to St. Charles Place", CardAction::Unknown, None, Some(11)), // TODO: ensure turn 'restarts' after advancing 
        Card::new("Make general repairs on all your property. House, $25 each; Hotel, $100 each", CardAction::Unknown, None, None), // TODO: calculate amount
        Card::new("Advance to the next railroad. If unowned, you can buy it. if owned, pay twice the rent", CardAction::Unknown, None, None), // TODO: calculate amount
        Card::new("You have been elected chairman of the board. Pay each player $50", CardAction::Unknown, Some(50), None), // TODO: calculate amount
        Card::new("Take a trip to Reading Railroad.", CardAction::Unknown, None, Some(5)),
        Card::new("Speeding fine. Pay $15", CardAction::Unknown, Some(15), None),
        Card::new("Your building load matures. Receive $150", CardAction::Unknown, Some(-150), None),
        Card::new("Advance to Boardwalk", CardAction::Unknown, None, Some(39)),
        Card::new("Go back three spaces", CardAction::Unknown, None, Some(-3)), // TODO: move relative to current square
        Card::new("Advance to Illinois Avenue", CardAction::Unknown, None, Some(24)),
        Card::new("Advance to GO. Collect $200", CardAction::Unknown, Some(-200), Some(0)),
        Card::new("GET OUT OF JAIL FREE.", CardAction::Unknown, None, None), // TODO: player keeps this card
        Card::new("Take all $100 bills from the Bank and throw them in the air", CardAction::Unknown, None, None), // TODO: how to model this? Random allocation?
        Card::new("Advance to the nearest railroad. If unowned, you can buy it. If owned, pay twice the rent", CardAction::Unknown, None, None), // TODO: go to closest 5,15,25,35. 2x amount
        Card::new("Advance to the nearest utility. If unowned, you can buy it. If owned, roll the dice, and pay the owner 10x the roll", CardAction::Unknown, None, None), // TODO: pay relative to roll
];
static COMMUNITY_CHEST_CARDS: [Card; 16 as usize] = [
        Card::new("You are assessed for Street repairs: $40 per House, $115 per Hotel", CardAction::Unknown, None, None),
        Card::new("GET OUT OF JAIL FREE", CardAction::Unknown, None, None),
        Card::new("You have won second prize in a beauty contest. Collect $10", CardAction::Unknown, Some(-10), None),
        Card::new("Life insurance matures. Collect $100", CardAction::Unknown, Some(-100), None),
        Card::new("It's your birthday. Collect $10 from each player", CardAction::Unknown, Some(-10), None), // TODO: calculate amount
        Card::new("Advance to GO. Collect $200", CardAction::Unknown, Some(-200), Some(0)), // TODO: calculate amount
        Card::new("You inherit $100", CardAction::Unknown, Some(-100), None),
        Card::new("Bank error in your favor. Collect $200", CardAction::Unknown, Some(-200), None),
        Card::new("From sale of stock, you get $50", CardAction::Unknown, Some(-50), None),
        Card::new("Collect $25 consultancy fee", CardAction::Unknown, Some(-25), None),
        Card::new("Holiday fund matures. Collect $100", CardAction::Unknown, Some(-100), None),
        Card::new("Doctor's fees. Pay 50", CardAction::Unknown, Some(50), None),
        Card::new("Hospital fees. Pay 1000", CardAction::Unknown, Some(100), None),
        Card::new("GO TO JAIL", CardAction::Unknown, None, Some(10)),
        Card::new("School fees. Pay $50", CardAction::Unknown, None, Some(50)),
        Card::new("Income tax refund. Collect $20", CardAction::Unknown, None, Some(-20))
];

struct Asset {
    owner: Cell<Option<usize>>, // FIXME: hack so we don't store the Player object
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
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Player {
    name: String,
    position: usize, // the index of the board square
    turn_idx: usize, // idx in the group of players. need this to match asset
    cash: i32,
    is_in_jail: bool,
}

pub struct Square {
    name: &'static str,
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
                self.execute_turn(dice_roll);
                println!("");
            }
            self.print_summary();
        }

        // TODO: shuffle chance and community chest cards
    }

    /// Print the game summary
    // Prints out stats for each player
    pub fn print_summary(&self) {
        for p in self.players.iter() {
            println!("{}", p.borrow());
        }
        println!("");
    }

    /// Execute the turn of a player
    // The turn starts with a player moving. Then, once the player is on the new square,
    // the rules for that new square execute. Lastly, other players may want to execute 
    // transactions
    fn execute_turn(&self, dice_roll: i32) {
        let mut player = self.players.get(self.active_player.get()).unwrap().borrow_mut();
        let old_pos = player.position;
        player.advance(dice_roll);
        if player.position < old_pos {
            println!("Yay! You pass begin and collect $200");
            player.receive_cash(200);
        }
        // TODO: If 3 doubles, go to jail
        
        let square = SQUARES.get(player.position).unwrap();
        match player.position {
            0 | 10 | 20 => { // GO, Visiting Jail, Free Parking
                println!("{}", square.name);
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
                let mut rng = thread_rng();
                let r = rng.gen_range(0,16);
                println!("{}", COMMUNITY_CHEST_CARDS.get(r).expect("card expected").description);
                // TODO: don't just choose randomly. shuffle then keep in order
            },
            7 | 22 | 36 => {
                println!("Chance");
                let mut rng = thread_rng();
                let r = rng.gen_range(0,16);
                println!("{}", CHANCE_CARDS.get(r).expect("card expected").description);
                // TODO: don't just choose randomly. shuffle then keep in order
            },
            _ => {
                println!("You landed on {}", square.name);
                match square.asset.owner.get() {
                    Some(owner_idx) => { // this street is owned by somebody
                        // check if active player is the owner
                        if owner_idx == self.active_player.get() {
                            println!("Phew! you own this");
                            return;
                        }
                        let mut owner = self.players.get(owner_idx).unwrap().borrow_mut();
                        let rent = square.calculate_rent();
                        println!("Oh no! {} already owns it. Pay ${}", owner.name, rent);
                        player.spend_cash(rent);
                        owner.receive_cash(rent);
                    },
                    None => {
                        // FIXME: check if price exist
                        let price = square.price.unwrap();
                        println!("Nobody owns it yet. It costs ${}", price);
                        if player.cash > price {
                            player.spend_cash(price);
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
    pub const fn new(name: &'static str, price: Option<i32>, rent: Option<i32>) -> Self {
        Self {
            name,
            price,
            rent,
            asset: Asset::new()
        }
    }

    /// Calculate rent
    // Calculate rent, taking into account if a player owns all streets, and the number of
    // properties on the street.
    pub fn calculate_rent(&self) -> i32 {
        // TODO: Write test to calculate rent
        match self.rent {
            Some(r) => {
                r
                // TODO: Calculate using properties and owner's portfolio
            },
            None => 0
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
            Streets: {:?}",
            self.name, square.name, self.cash, self.is_in_jail, streets)
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

    /// Receive cash from bank or player
    pub fn receive_cash(&mut self, amount: i32) {
        self.cash += amount;
    }

    /// Pay cash to bank or player
    pub fn spend_cash(&mut self, amount: i32) {
        self.cash -= amount;
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
    Game {
        active_player: Cell::new(0),
        players
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
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1500);
        g.execute_turn(4); // income tax, $200
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1300);
    }

    #[test]
    fn test_pass_go() {
        let g = init(vec!["Test".to_string()]);

        // advance on top of GO
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1500);
        g.execute_turn(10); // visiting jail
        g.execute_turn(30); // on the go square
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1700);

        // advance past GO
        g.execute_turn(20); // free parking
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1700);
        g.execute_turn(30); // pass go, to visiting jail
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1900);
    }

    #[test]
    fn test_go_to_jail() {
        let g = init(vec!["Test".to_string()]);

        // go to jail
        assert_eq!(g.players.get(0).unwrap().borrow().is_in_jail, false);
        g.execute_turn(30);
        assert_eq!(g.players.get(0).unwrap().borrow().position, 10);
        assert_eq!(g.players.get(0).unwrap().borrow().is_in_jail, true);
    }

    #[test]
    fn test_calculate_rent() {
        // Baltic avenue
        let s = SQUARES.get(3).unwrap();
        let r = s.calculate_rent();
        assert_eq!(r, 4);

        // no-rent square
        let s = SQUARES.get(10).unwrap();
        let r = s.calculate_rent();
        assert_eq!(r, 0);

        // Income tax
        let s = SQUARES.get(4).unwrap();
        let r = s.calculate_rent();
        assert_eq!(r, 0);
    }

    #[test]
    fn test_purchase_and_pay_rent() {
        let g = init(vec!["TestA".to_string(), "TestB".to_string()]);
        let s = SQUARES.get(3).unwrap();
        assert_eq!(s.asset.owner.get(), None);

        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1500);
        g.execute_turn(3); // TestA moves to Baltic Avenue
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1440); // bought street
        assert_eq!(s.asset.owner.get().unwrap(), g.active_player.get());

        g.active_player.set(1 as usize); // update active player
        g.execute_turn(3); // TestB moves to Baltic Avenue and pays rent
        assert_eq!(g.players.get(0).unwrap().borrow().cash, 1444);
        assert_eq!(g.players.get(1).unwrap().borrow().cash, 1496);
    }
}
