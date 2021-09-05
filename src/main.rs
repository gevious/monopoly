use std::io;
use std::io::{Write};

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

fn main() {
    loop {
        print!("Enter dice roll: ");
        let _= io::stdout().flush();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Did not enter a correct string");
        user_input.pop(); // Remove newline

        let dice_roll = match get_dice_roll(user_input) {
            Ok(d) => d,
            Err(s) => {
                println!("Enter a number between 2 and 12");
                continue;
            }
        };
        println!("You rolled {}", dice_roll);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_valid_number() {
        assert_eq!(get_dice_roll("Yo".to_string()), Err(()));
        assert_eq!(get_dice_roll("2.3".to_string()), Err(()));
    }

    #[test]
    fn test_dice_number_in_range() {
        assert_eq!(get_dice_roll("1".to_string()), Err(()));
        assert_eq!(get_dice_roll("2".to_string()), Ok((2)));
        assert_eq!(get_dice_roll("12".to_string()), Ok((12)));
        assert_eq!(get_dice_roll("13".to_string()), Err(()));
    }
}
