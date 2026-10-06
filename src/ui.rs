//! The text interface for the player.

use crate::combat::{Action, Fighter};
use crate::items::Item;
// Two types are both called `Result`. `as` gives each a different local
// name so that both can be imported. `self` in a nested path imports the
// module itself, so `io::stdin` can be written below.
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{self, Result as IoResult};

/// A health bar that can be printed with `{}`.
struct HealthBar {
    hp: i32,
    max_hp: i32,
}

impl Display for HealthBar {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let filled = (self.hp.max(0) * 20 / self.max_hp) as usize;
        write!(f, "[{}{}]", "#".repeat(filled), "-".repeat(20 - filled))
    }
}

fn read_line() -> IoResult<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn clear_screen() {
    // ANSI escape codes: clear the terminal and move the cursor to the top left.
    print!("\x1B[2J\x1B[1;1H");
}

pub fn show_status(player: &Fighter, enemy: &Fighter) {
    for fighter in [player, enemy] {
        let bar = HealthBar {
            hp: fighter.hp(),
            max_hp: fighter.max_hp(),
        };
        let poisoned = if fighter.is_poisoned() {
            " (poisoned)"
        } else {
            ""
        };
        println!(
            "{:<8} {bar} {}/{}{poisoned}",
            fighter.name,
            fighter.hp().max(0),
            fighter.max_hp()
        );
    }
}

/// Asks the player for an action. Returns `None` when the player quits.
pub fn prompt_action(player: &Fighter) -> Option<Action> {
    loop {
        println!(
            "\n1) Attack  2) Defend  3) Potion ({})  4) Poison flask ({})  q) Quit",
            player.potions, player.flasks
        );
        let input = read_line().unwrap_or_default();
        match input.trim() {
            "1" => return Some(Action::Attack),
            "2" => return Some(Action::Defend),
            "3" => return Some(Action::UseItem(Item::Potion)),
            "4" => return Some(Action::UseItem(Item::PoisonFlask)),
            // Empty input means that the input ended or could not be read.
            "q" | "" => return None,
            _ => println!("Unknown choice."),
        }
    }
}
