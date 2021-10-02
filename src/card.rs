pub enum CardAction {
    Movement,
//    MovementRelative, // move relative to starting square
    Payment,
//    PaymentDice, // payment calculated based on dice roll
//    PaymentPlayers, // payment calculated based on dice roll
    Jail, 
    JailRelease, 
}

/// Chance or Community chest card
pub struct Card {
    description: String,
    action: CardAction,
    amount: Option<i32>, //negative means player receives cash
    square: Option<u32>
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

    /// Return the amount the player has to pay. Player receives cash for negative amounts
    pub fn amount(&self) -> Option<i32> {
        self.amount
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn square(&self) -> Option<u32> {
        self.square
    }

    pub fn action(&self) -> &CardAction {
        &self.action
    }
}
