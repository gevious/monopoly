use std::fs;
use async_process::Command;


use super::game::{Game, Square};

const TEMP_FILE :&str = "/tmp/index.html";

/// Publish game summary to www.gevious.com/monopoly
pub fn publish(game: &Game) {

    let mut sb = String::from("<h1>Monopoly</h1>");
    for p_ref in game.players.iter() {
        let p = p_ref.borrow();
        let occupying_square = game.board.get(p.position)
            .expect("Player is not on the board");
        sb.push_str(&format!("{} : ${}", p.name, p.cash));
        sb.push_str("<ul>");
        match p.is_in_jail {
            true  => sb.push_str(&format!("<li> is IN JAIL, but still has ${}</li>",
                                          p.cash)),
            false => sb.push_str(&format!("<li>is on {} with ${}</li>",
                                          occupying_square.name, p.cash))
        };
        if p.num_get_out_of_jail_cards > 0 {
            sb.push_str(&format!("<li>has {} get-out-of-jail cards</li>",
                                 p.num_get_out_of_jail_cards));
      }
        let owned_streets = game.board.iter()
            .filter(|&x| { match x.asset.owner.get() {
                None => false,
                Some(owner_idx) => owner_idx == p.turn_idx
            }})
            .collect::<Vec<&Square>>();
        match owned_streets.len() {
            0 => sb.push_str(&format!("<li>owns nothing :(</li>")),
            _ => {
                sb.push_str(&format!("<li>owns: <ul>"));
                for s in owned_streets.iter() {
                    sb.push_str(&format!("<li>{}</li>", s.name));
                }
                sb.push_str("</ul></li>");
            }
        };
        sb.push_str("</ul>");
    }


    let mut summary = format!("<html><body>{}</body></html>", sb);
    fs::write(TEMP_FILE, summary);
    upload(sb);
}

/// Upload summary to S3
fn upload(content: String) {
    // For now, i'm just calling a CLI command. 
    // TODO: Implement AWS SDK to make this more robust
    Command::new("./src/upload.sh")
        .output();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_summary() {
        upload(String::from("This is a test upload"));
    }
}
