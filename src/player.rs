pub struct Asset {
    pub owner: Option<usize>, // usize is a reference to a players turn_idx
    house_num: u32,
    has_hotel: bool,
    is_mortgaged: bool
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Player {
    name: String,
    position: usize, // the index of the board square
    turn_idx: usize, // idx in the suburb of players. need this to match asset
    cash: u32,
    is_in_jail: bool,
    num_get_out_of_jail_cards: u32,
    is_in_trouble: bool, // true if player cannot pay bills, and needs to sell
    left_game: bool // true if player has left the game
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

    /// Calculate if a street has houses built on it
    pub fn has_buildings(&self) -> bool {
        self.has_hotel() || self.house_num() > 0
    }

    pub fn liquify(&mut self) {
        self.owner = None;
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
            is_in_trouble: false,
            left_game: false,
            num_get_out_of_jail_cards: 0,
        }
    }


    //name: String,
    //position: usize, // the index of the board square
    //turn_idx: usize, // idx in the suburb of players. need this to match asset
    //cash: u32,
    //is_in_jail: bool,
    //num_get_out_of_jail_cards: u32,
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn cash(&self) -> u32 {
        self.cash
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn turn_idx(&self) -> usize {
        self.turn_idx
    }

    pub fn is_in_jail(&self) -> bool {
        self.is_in_jail
    }

    pub fn is_in_trouble(&self) -> bool {
        self.is_in_trouble
    }
    
    pub fn left_game(&self) -> bool {
        self.left_game
    }

    pub fn num_get_out_of_jail_cards(&self) -> u32 {
        self.num_get_out_of_jail_cards
    }

    pub fn set_in_trouble(&mut self, in_trouble: bool) {
        self.is_in_trouble = in_trouble;
    }
    
    pub fn leave_game(&mut self) {
        self.left_game = true;
    }

    /// Advance player
    // Move player to next square
    pub fn advance(&mut self, steps: u32, board_size: u32) {
        let target_square = ((self.position as u32) + steps) % board_size;
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

    pub fn leave_jail(&mut self) {
        self.is_in_jail = false;
    }

    pub fn bribe_guards(&mut self) -> Result<(), ()> {
        if self.cash < 50 {
            println!("Oh no. You don't have enough cash to bribe the guards");
            return Err(());
        }
        println!("Yay, No More Jail, since you bribed the guards $50");
        self.transact_cash(-50);
        self.is_in_jail = false;
        Ok(())
    }

    pub fn redeem_jail_free_card(&mut self) -> Result<(), ()> {
        if self.num_get_out_of_jail_cards < 1 {
            return Err(());
        }
        self.num_get_out_of_jail_cards -= 1;
        self.is_in_jail = false;
        Ok(())
    }

    pub fn receive_jail_free_card(&mut self) {
        self.num_get_out_of_jail_cards += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn advance_player() {
        let ref mut p = Player::new("Test".to_string(), 1);
        assert_eq!(p.position, 0);
        p.advance(37, 40);
        assert_eq!(p.position, 37);
        p.advance(5, 40); // wrap around BOARD_SIZE (=40)
        assert_eq!(p.position, 2);
    }

    fn check_bankrupt() {
        let ref mut p = Player::new("Test".to_string(), 1);
        assert_eq!(p.transact_cash(500), Ok(()));
        assert_eq!(p.transact_cash(1000), Ok(()));
        assert_eq!(p.transact_cash(1), Err(()));
    }
}
