//! Effects that last for several turns.

use super::Fighter;

pub enum StatusEffect {
    Poisoned { turns_left: u32 },
}

/// Applies the damage over time of the fighter's status effect.
pub fn tick(fighter: &mut Fighter) -> Option<String> {
    if let Some(StatusEffect::Poisoned { turns_left }) = fighter.status {
        fighter.hp -= 3;
        fighter.status = if turns_left > 1 {
            Some(StatusEffect::Poisoned {
                turns_left: turns_left - 1,
            })
        } else {
            None
        };
        Some(format!("{} suffers 3 poison damage.", fighter.name))
    } else {
        None
    }
}
