//! The enemy's behavior.

use crate::combat::{Action, Fighter};
use crate::items::Item;
use rand::RngExt;

/// A weighted random choice, with a bias to heal when low on HP.
pub fn choose_action(enemy: &Fighter) -> Action {
    let roll = rand::rng().random_range(0..100);
    if is_hurt(enemy) && enemy.potions > 0 && roll < 60 {
        Action::UseItem(Item::Potion)
    } else if roll < 20 {
        Action::Defend
    } else if enemy.flasks > 0 && roll < 35 {
        Action::UseItem(Item::PoisonFlask)
    } else {
        Action::Attack
    }
}

fn is_hurt(fighter: &Fighter) -> bool {
    fighter.hp() < fighter.max_hp() / 3
}
