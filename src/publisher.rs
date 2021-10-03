use std::fs;
use async_process::Command;


use super::game::{Game};
use super::square::Square;

const TEMP_FILE :&str = "/tmp/index.html";

/// Print the game summary
// Prints out stats for each player
fn print_summary(game: &Game) {
    println!("==== Summary ====");
    for p_ref in game.players.iter() {
        let p = p_ref.borrow();
        let occupying_square = game.board.get(p.position())
            .expect("Player is not on the board");
        print!("{} ", p.name());
        match p.is_in_jail() {
            true  => println!("\t is IN JAIL 🚧, but still has ${}", p.cash()),
            false => println!("\t is on {} with ${}", occupying_square.name(), p.cash()) 
        };
        if p.num_get_out_of_jail_cards() > 0 {
            println!("\t has {} get-out-of-jail cards", p.num_get_out_of_jail_cards());
        }
        let owned_streets = game.board.iter()
            .filter(|&x| {
                match x.asset.borrow().owner {
                    None => false,
                    Some(owner_idx) => owner_idx == p.turn_idx()
                }
            })
            .collect::<Vec<&Square>>();
        match owned_streets.len() {
            0 => println!("\t owns nothing :("),
            _ => {
                println!("\t owns {} assets:", owned_streets.len());
                for s in owned_streets.iter() {
                    let mut x = s.name().to_string();
                    let a = s.asset.borrow();
                    if a.is_mortgaged() {
                        x.push_str(" (mortgaged)");
                    } else if a.has_hotel() {
                        x.push_str(" (🏨)");
                    } else if a.house_num() > 0 {
                        match a.house_num() {
                            1 => x.push_str(&format!(" ({} 🏠)", a.house_num())),
                            2 => x.push_str(&format!(" ({} 🏡)", a.house_num())),
                            3 => x.push_str(&format!(" ({} 🏘️)", a.house_num())),
                            _ => x.push_str(&format!(" ({} 🏘️)", a.house_num()))
                        }
                    }
                    match s.get_street_details().unwrap().get_suburb() {
                        Some(s) => {
                            println!("\t\t {} ({:?})", x, s.color());
                        },
                        None => {
                            println!("\t\t {}", x);
                        }
                    }
                }
            }
        };
    }
    println!("=================");
}

/// Publish game summary to www.gevious.com/monopoly
pub fn publish(game: &Game) {
    print_summary(game);

    let mut sb = String::from("<h1>Monopoly</h1>");
    for p_ref in game.players.iter() {
        let p = p_ref.borrow();
        let occupying_square = game.board.get(p.position())
            .expect("Player is not on the board");
        sb.push_str(&format!("{} : ${}", p.name(), p.cash()));
        sb.push_str("<ul>");
        match p.is_in_jail() {
            true  => sb.push_str(&format!("<li> is IN JAIL 🚧, but still has ${}</li>",
                                          p.cash())),
            false => sb.push_str(&format!("<li>is on {} with ${}</li>",
                                          occupying_square.name(), p.cash()))
        };
        if p.num_get_out_of_jail_cards() > 0 {
            sb.push_str(&format!("<li>has {} get-out-of-jail cards</li>",
                                 p.num_get_out_of_jail_cards()));
      }
        let owned_streets = game.board.iter()
            .filter(|&x| { match x.asset.borrow().owner {
                None => false,
                Some(owner_idx) => owner_idx == p.turn_idx()
            }})
            .collect::<Vec<&Square>>();
        match owned_streets.len() {
            0 => sb.push_str(&format!("<li>owns nothing :(</li>")),
            _ => {
                sb.push_str(&format!("<li>owns: <ul>"));
                for s in owned_streets.iter() {
                    let mut x = s.name().to_string();
                    let a = s.asset.borrow();
                    if a.is_mortgaged() {
                        x.push_str(" (mortgaged)");
                    } else if a.has_hotel() {
                        x.push_str(" (🏨)");
                    } else if a.house_num() > 0 {
                        match a.house_num() {
                            1 => x.push_str(&format!(" ({} 🏠)", a.house_num())),
                            2 => x.push_str(&format!(" ({} 🏡)", a.house_num())),
                            3 => x.push_str(&format!(" ({} 🏘️)", a.house_num())),
                            _ => x.push_str(&format!(" ({} 🏘️)", a.house_num()))
                        }
                    }
                    match s.get_street_details().unwrap().get_suburb() {
                        Some(s) => {
                            sb.push_str(&format!("<li>{} ({:?})</li>", x, s.color()));
                        },
                        None => {
                            sb.push_str(&format!("<li>{}</li>", x));
                        }
                    }
                }
                sb.push_str("</ul></li>");
            }
        };
        sb.push_str("</ul>");
    }


    let summary = format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body>{}</body></html>", sb);
    fs::write(TEMP_FILE, summary);
    upload();
}

/// Upload summary to S3
fn upload() {
    // For now, i'm just calling a CLI command. 
    // TODO: Implement AWS SDK to make this more robust
    Command::new("./src/upload.sh")
        .output();
}

#[cfg(test)]
mod tests {
}
