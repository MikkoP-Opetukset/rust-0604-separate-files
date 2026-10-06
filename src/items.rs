//! Things fighters can spend during a fight.

// Nested paths: `crate::combat` appears once instead of twice.
use crate::combat::{Fighter, status::StatusEffect};

pub enum Item {
    Potion,
    PoisonFlask,
}

pub fn use_item(item: Item, user: &mut Fighter, target: &mut Fighter) -> String {
    match item {
        Item::Potion if user.potions > 0 => {
            user.potions -= 1;
            user.heal(15);
            format!("{} drinks a potion and recovers 15 HP.", user.name)
        }
        Item::PoisonFlask if user.flasks > 0 => {
            user.flasks -= 1;
            target.set_status(StatusEffect::Poisoned { turns_left: 3 });
            format!("{} throws a flask. {} is poisoned!", user.name, target.name)
        }
        _ => format!("{} fumbles around: nothing left to use.", user.name),
    }
}
