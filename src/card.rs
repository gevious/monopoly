pub enum CardAction {
    Movement,
    RelativeMovement,
    Payment,
    Jail, 
    JailRelease, 
    Repairs 
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
