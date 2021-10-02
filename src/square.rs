use std::cell::RefCell;

use super::player::{Asset};

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
    color: String,
    building_price: u32
}

pub struct StreetDetails {
    suburb: Option<Suburb>,
    price: u32,
    rent: u32,
    rent_suburb: [u32; 6], // suburb, house 1..4, hotel
    mortgage: u32,
}

pub struct Square {
    name: String,
    square_type: SquareType,
    street_details: Option<StreetDetails>,
    pub asset: RefCell<Asset>
//        FIXME: remove pub of Asset
}

impl StreetDetails {
    pub fn new(color: char, price: u32, rent: u32, rent_suburb: [u32; 6],
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

    pub fn rent(&self) -> u32 {
        self.rent
    }

    pub fn rent_suburb(&self) -> &[u32] {
        self.rent_suburb.as_ref()
    }

    pub fn mortgage(&self) -> u32 {
        self.mortgage
    }
}

impl Suburb {
    fn new(color: &str, building_price: u32) -> Self {
        Self {
            color: color.to_string(),
            building_price
        }
    }

    pub fn building_price(&self) -> u32 {
        self.building_price
    }

    pub fn color(&self) -> &str {
        self.color.as_ref()
    }
}

impl Square {
    pub fn new(name: &str, square_type: SquareType,
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

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn square_type(&self) -> SquareType {
        self.square_type
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

