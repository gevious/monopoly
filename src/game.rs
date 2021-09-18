use rand::thread_rng;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::io;
use std::io::{Write};
use std::fmt;
use std::cell::{Cell, RefCell};

const BOARD_SIZE: i32 = 40; // 40 squares on the board
// Change to a const, to avoid concurrency issues when running tests
static SQUARES: [Square; BOARD_SIZE as usize] = [
    Square::new("Just chillin' at the start", SquareType::Corner, None),
    Square::new("Mediterranean Avenue", SquareType::Street, 
        Some(StreetDetails::new('A', 60, 2, 4))),
    Square::new("Community Chest", SquareType::CommunityCard, None),
    Square::new("Baltic Avenue", SquareType::Street, 
        Some(StreetDetails::new('A', 60, 4, 8))),
    Square::new("Income Tax", SquareType::Tax, None),
    Square::new("Reading Railroad", SquareType::Station, None),
    Square::new("Oriental Avenue", SquareType::Street, 
        Some(StreetDetails::new('B', 100, 6, 12))),
    Square::new("Chance", SquareType::ChanceCard, None),
    Square::new("Vermont Avenue", SquareType::Street, 
        Some(StreetDetails::new('B', 100, 6, 12))),
    Square::new("Connecticut Avenue", SquareType::Street, 
        Some(StreetDetails::new('B', 120, 8, 16))),
    Square::new("Visiting Jail", SquareType::Corner, None),
    Square::new("St. Charles Place", SquareType::Street, 
        Some(StreetDetails::new('C', 140, 10, 20))),
    Square::new("Electric Company", SquareType::Utility, None),
    Square::new("States Avenue", SquareType::Street, 
        Some(StreetDetails::new('C', 140, 10, 20))),
    Square::new("Virginia Avenue", SquareType::Street, 
        Some(StreetDetails::new('C', 160, 12, 24))),
    Square::new("Pennsylvania Railroad", SquareType::Station, None),
    Square::new("St. James Place", SquareType::Street, 
        Some(StreetDetails::new('D', 180, 14, 28))),
    Square::new("Community Chest", SquareType::CommunityCard, None),
    Square::new("Tennessee Avenue", SquareType::Street, 
        Some(StreetDetails::new('D', 180, 14, 28))),
    Square::new("New York Avenue", SquareType::Street, 
        Some(StreetDetails::new('D', 200, 16, 32))),
    Square::new("Yay! Free Parking", SquareType::Corner, None),
    Square::new("Kentucky Avenue", SquareType::Street, 
        Some(StreetDetails::new('E', 220, 18, 36))),
    Square::new("Chance", SquareType::ChanceCard, None),
    Square::new("Indiana Avenue", SquareType::Street, 
        Some(StreetDetails::new('E', 220, 18, 36))),
    Square::new("Illinois Avenue", SquareType::Street, 
        Some(StreetDetails::new('E', 240, 20, 40))),
    Square::new("B. & O. Railroad", SquareType::Station, None),
    Square::new("Atlantic Avenue", SquareType::Street, 
        Some(StreetDetails::new('F', 260, 22, 44))),
    Square::new("Ventnor Avenue", SquareType::Street, 
        Some(StreetDetails::new('F', 260, 22, 44))),
    Square::new("Water Works", SquareType::Utility, None),
    Square::new("Marvin Gardens", SquareType::Street, 
        Some(StreetDetails::new('F', 280, 24, 48))),
    Square::new("Go To Jail", SquareType::Corner, None),
    Square::new("Pacific Avenue", SquareType::Street, 
        Some(StreetDetails::new('G', 300, 26, 52))),
    Square::new("North Carolina Avenue", SquareType::Street, 
        Some(StreetDetails::new('G', 300, 26, 52))),
    Square::new("Community Chest", SquareType::CommunityCard, None),
    Square::new("Pennsylvania Avenue", SquareType::Street, 
        Some(StreetDetails::new('G', 320, 28, 56))),
    Square::new("Short Line", SquareType::Station, None),
    Square::new("Chance", SquareType::ChanceCard, None),
    Square::new("Park Place", SquareType::Street, 
        Some(StreetDetails::new('H', 350, 35, 70))),
    Square::new("Luxury Tax", SquareType::Tax, None),
    Square::new("Boardwalk", SquareType::Street, 
        Some(StreetDetails::new('H', 400, 50, 100)))
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

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
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
const COMMUNITY_CARDS: [Card; 10 as usize] = [
//        Card::new("You are assessed for Street repairs: $40 per House, $115 per Hotel", CardAction::Payment, Some(0), None),
        Card::new("GET OUT OF JAIL FREE.", CardAction::JailRelease, None, None),
        Card::new("You have won second prize in a beauty contest. Collect $10", CardAction::Payment, Some(-10), None),
//        Card::new("Life insurance matures. Collect $100", CardAction::Payment, Some(-100), None),
 //       Card::new("It's your birthday. Collect $40", CardAction::PaymentPlayers, Some(-40), None),
        Card::new("Advance to GO. Collect $200", CardAction::Movement, None, Some(0)),
        Card::new("You inherit $100", CardAction::Payment, Some(-100), None),
//        Card::new("Bank error in your favor. Collect $200", CardAction::Payment, Some(-200), None),
        Card::new("From sale of stock, you get $50", CardAction::Payment, Some(-50), None),
        Card::new("Collect $25 consultancy fee", CardAction::Payment, Some(-25), None),
 //       Card::new("Holiday fund matures. Collect $100", CardAction::Payment, Some(-100), None),
        Card::new("Doctor's fees. Pay $50", CardAction::Payment, Some(50), None),
        Card::new("Hospital fees. Pay $100", CardAction::Payment, Some(100), None),
//        Card::new("GO TO JAIL!", CardAction::Jail, None, None),
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

#[derive(Clone, Copy)]
struct StreetDetails {
    group: char, // street group. No strong type, because it never changes
    price: i32,
    rent: i32,
    rent_group: i32
    
//    rent_full_street: i32,
//    rent_1house: i32,
//    rent_2house: i32,
//    rent_3house: i32,
//    rent_4house: i32,
//    rent_hotel: i32,
//    mortgage_cost: i32,
//    mortgage_redeem: i32,
}

pub struct Square {
    name: &'static str,
    square_type: SquareType,
    street_details: Option<StreetDetails>,
    asset: Asset
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
                        (cards, &CHANCE_CARDS)
                    }
                };
                let idx = cards.remove(0);
                let card = ref_deck.get(idx).expect("Card should exist");
                self.execute_card(player, card);
                cards.push(idx);
            },
            SquareType::Utility | SquareType::Station | SquareType::Street => {
                println!("You landed on {}", square.name);
                match square.calculate_rent(dice_roll) {
                    None => {
                        // No owner, therefore no rent
                        let price = square.get_price();
                        println!("You buy it for ${}", price);
                        if player.cash > price {
                            player.transact_cash(-1 * price);
                            square.asset.owner.replace(Some(self.active_player.get()));
                        } else {
                            println!("Not enough money. It stays on the market");
                            // TODO: Implement auction where bank-person inputs player and price
                        }
                    }, 
                    Some(rent) => {
                        let owner_idx = square.asset.owner
                            .get().expect("Somebody owns this street");
                        if owner_idx == self.active_player.get() {
                            println!("Phew! Luckily it's yours");
                            return;
                        }
                        let mut owner = self.players.get(owner_idx).unwrap().borrow_mut();
                        println!("Oh no! You pay ${} to {}", rent, owner.name);
                        player.transact_cash(-1 * rent);
                        owner.transact_cash(rent);
                    }
                }
            }
        }
    }
}

impl StreetDetails {
    const fn new(group: char, price: i32, rent: i32, rent_group: i32) -> Self {
        Self {
            group,
            price,
            rent,
            rent_group
        }
    }
}

impl Square {

    const fn new(name: &'static str, square_type: SquareType, street_details: Option<StreetDetails>) -> Self {
        let details = match street_details {
            None => None,
            Some(details) => Some(details)
        };
        Self {
            name,
            square_type,
            street_details: details,
            asset: Asset::new()
        }
    }


    /// Get purchase price of the street
    pub fn get_price(&self) -> i32 {
        match self.square_type {
            SquareType::Station => 200,
            SquareType::Utility => 150,
            SquareType::Street  => self.street_details.expect("Details should exist").price,
                _ => 0 // Error, should never happen
        }
    }

    /// Get all squares owned by a player
    fn get_player_owned_squares(player_idx: usize) -> Vec<Square> {
        let mut squares = Vec::<Square>::new();
        for s in SQUARES.iter() {
            match s.asset.owner.get() {
                Some(owner_idx) => {
                    if owner_idx == player_idx {
                        squares.push(
                            Square::new(s.name, s.square_type, s.street_details));
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
    pub fn calculate_rent(&self, dice_roll: i32) -> Option<i32> {
        let owner = match self.asset.owner.get() {
            None => {
                // Nobody owns this square
                return None;
            },
            Some(r) => r
        };

        // Need owner of this square
        // get all squares owner owns of the same type
        let rent = match self.square_type {
            SquareType::Utility => {
                let owned_squares = Square::get_player_owned_squares(owner);
                let utility_num = owned_squares.iter()
                    .filter(|&x| x.square_type == SquareType::Utility)
                    .collect::<Vec<&Square>>().len();
                let utility_num = utility_num as i32;
                match utility_num {
                    1 => dice_roll * 4,
                    2 => dice_roll * 10,
                    _ => 0 // Error, no rent
                }
            },
            SquareType::Station => {
                // See how many stations user has
                let owned_squares = Square::get_player_owned_squares(owner);
                let station_num = owned_squares.iter()
                    .filter(|&x| x.square_type == SquareType::Station)
                    .collect::<Vec<&Square>>().len();
                let station_num = station_num as i32;

                match station_num {
                    1 => 25,
                    2 => 50,
                    3 => 100, // $100 for 3 stations
                    4 => 200,  // $200 for 4 stations
                    _ => 0 // Error, no rent 
                }
            },
            SquareType::Street => {
                let owned_squares = Square::get_player_owned_squares(owner);
                let street_details = self.street_details.expect("Details expected");
                let street_group = street_details.group;
                let group_total = SQUARES.iter()
                    .filter(|&x| { 
                        match x.street_details {
                            None => false,
                            Some(d) => d.group == street_group
                        }})
                    .collect::<Vec<&Square>>().len();
                let group_owned = owned_squares.iter()
                    .filter(|&x| { 
                        match x.street_details {
                            None => false,
                            Some(d) => d.group == street_group
                        }})
                    .collect::<Vec<&Square>>().len();

                // println!("Street: {}", self.name);
                // println!("group: {}", street_group);
                // println!("group owned: {}", group_owned);
                // println!("group total: {}", group_total);

                match group_total == group_owned {
                    true => street_details.rent_group,
                    false => street_details.rent
                }
            }
            _ => 0
        };
        Some(rent)
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
    fn calculate_rent_unowned() {
        // Unowned square | No Rent
        let s = SQUARES.get(1).unwrap();
        let r = s.calculate_rent(3);
        assert_eq!(r, None);
        
        // Income tax | No rent
        let s = SQUARES.get(4).unwrap();
        let r = s.calculate_rent(4);
        assert_eq!(r, None);
       
        // no-rent square
        let s = SQUARES.get(10).unwrap();
        let r = s.calculate_rent(10);
        assert_eq!(r, None);
    }


    #[test]
    fn calculate_rent_street() {
        let g = init(vec!["StreetOwner".to_string(), "StreetRenter".to_string()]);
        let s = SQUARES.get(3).unwrap();
        println!(" --- {} : {:?}", s.name, s.asset.owner);
        assert_eq!(s.asset.owner.get(), None);

        // Buy the following squares:
        // Baltic Avenue[3] (1 of set of 2)
        // Oriental[6] & Vermont Ave[8] (2 of set of 3)
        // St. Charles place[11], States Ave[13], Virginia Ave[14] (3 of set of 3)
        // Park Place[37] & Boardwalk[39] (2 of set of 2)
        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 3); // Owner moves to Baltic Avenue
            g.execute_turn(&mut p, 3); // Owner moves to Oriental Avenue
            g.execute_turn(&mut p, 2); // Vermont Avenue
            g.execute_turn(&mut p, 3); // St. Charles place
            g.execute_turn(&mut p, 2); // States Ave
            g.execute_turn(&mut p, 1); // Virginia Ave
            g.execute_turn(&mut p, 23); // Park place
            g.execute_turn(&mut p, 2); // Boardwalk
        }

        g.active_player.set(1 as usize); // update active player to renter

        // Rent for 1 of 2 set 
        let s = SQUARES.get(3).unwrap(); // Baltic
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(4));

        // Rent for 2 of 3 set 
        let s = SQUARES.get(6).unwrap(); // Oriental
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(6));
        let s = SQUARES.get(8).unwrap(); // Vermont
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(6));

        // rent for 3 of 3 set 
        let s = SQUARES.get(11).unwrap(); // St. Charles 
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(20));
        let s = SQUARES.get(13).unwrap(); // States Ave
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(20));
        let s = SQUARES.get(14).unwrap(); // Virginia Ave
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(24));
        
        // rent for 2 of 2 set 
        let s = SQUARES.get(37).unwrap(); // Park Ave
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(70));
        let s = SQUARES.get(39).unwrap(); // Boardwalk
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(100));
    }

    #[test]
    fn calculate_rent_utility() {
        // Buy 1 utility, then buy the second
        let g = init(vec!["TestOwner".to_string(), "TestRenter".to_string()]);

        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 12); // Electric
        }
        g.active_player.set(1 as usize); // update active player to renter
        let s = SQUARES.get(12).unwrap(); // Electric
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(12)); 

        g.active_player.set(0 as usize); // update active player to renter
        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 16); // Water
        }
        g.active_player.set(1 as usize); // update active player to renter
        let s = SQUARES.get(28).unwrap(); // Electric
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(30)); 
    }

    #[test]
    fn calculate_rent_station() {
        // Buy stations one at a time
        let g = init(vec!["StationOwner".to_string(), "StationRenter".to_string()]);

        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 5); // Reading Railroad
        }
        g.active_player.set(1 as usize); // update active player to renter
        let s = SQUARES.get(5).unwrap();
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(25)); 

        g.active_player.set(0 as usize); // update active player to renter
        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 10); // Pennsylvania Railroad
        }
        g.active_player.set(1 as usize); // update active player to renter
        let s = SQUARES.get(5).unwrap();
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(50)); 

        g.active_player.set(0 as usize); // update active player to renter
        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 10); // B.O. Railroad
        }
        g.active_player.set(1 as usize); // update active player to renter
        let s = SQUARES.get(5).unwrap();
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(100)); 

        g.active_player.set(0 as usize); // update active player to renter
        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            g.execute_turn(&mut p, 10); // Short line
        }
        g.active_player.set(1 as usize); // update active player to renter
        let s = SQUARES.get(5).unwrap();
        let r = s.calculate_rent(3);
        assert_eq!(r, Some(200)); 
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

    #[test]
    fn buy_property() {
        let g = init(vec!["Mongul".to_string()]);
        let s = SQUARES.get(3).unwrap();
        assert_eq!(s.asset.owner.get(), None);

        { // scope the `mut p` reference, so we release it after this 'turn'
            let mut p = g.players.get(g.active_player.get()).unwrap().borrow_mut();
            assert_eq!(p.cash, 1500);
            
            g.execute_turn(&mut p, 3); // Mongul moves to Baltic Avenue
            assert_eq!(p.cash, 1440); // bought street
            assert_eq!(s.asset.owner.get().unwrap(), g.active_player.get());

            g.execute_turn(&mut p, 9); // Mongul moves to Electric Company
            assert_eq!(p.cash, 1290); // bought street
            assert_eq!(s.asset.owner.get().unwrap(), g.active_player.get());

            g.execute_turn(&mut p, 3); // Mongul moves to Pennsylvania Railroad
            assert_eq!(p.cash, 1090); // bought street
            assert_eq!(s.asset.owner.get().unwrap(), g.active_player.get());
        }
    }

}
