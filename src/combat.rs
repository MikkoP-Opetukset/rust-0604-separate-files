//! Rules of the fight: who the fighters are, what they can do, and what
//! happens when they do it.

// Idiomatic `use` paths:
// - For a function, bring its parent module into scope and call the
//   function through it (`items::use_item`). It is clear at the call site
//   that the function is not defined locally.
// - For structs, enums, and other types, bring the type itself into scope.
use self::status::StatusEffect;
use crate::items;
use crate::items::Item;

/// What a fighter chooses to do on its turn.
pub enum Action {
    Attack,
    Defend,
    UseItem(Item),
}

/// A character in the fight. The private fields can only be changed by
/// code inside `combat` and its child modules.
pub struct Fighter {
    pub name: String,
    pub potions: u32,
    pub flasks: u32,
    hp: i32,
    max_hp: i32,
    attack: i32,
    defending: bool,
    status: Option<StatusEffect>,
}

impl Fighter {
    pub fn new(name: &str, max_hp: i32, attack: i32, potions: u32, flasks: u32) -> Fighter {
        Fighter {
            name: String::from(name),
            potions,
            flasks,
            hp: max_hp,
            max_hp,
            attack,
            defending: false,
            status: None,
        }
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }

    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn is_poisoned(&self) -> bool {
        self.status.is_some()
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn set_status(&mut self, effect: StatusEffect) {
        self.status = Some(effect);
    }
}

/// Applies the action to the fighters and returns a line describing it.
pub fn resolve_action(actor: &mut Fighter, target: &mut Fighter, action: Action) -> String {
    actor.defending = false;
    match action {
        Action::Attack => {
            let amount = damage::calculate(actor, target);
            target.hp -= amount;
            format!("{} hits {} for {amount} damage.", actor.name, target.name)
        }
        Action::Defend => {
            actor.defending = true;
            format!("{} braces for impact.", actor.name)
        }
        Action::UseItem(item) => items::use_item(item, actor, target),
    }
}

pub mod damage;

pub mod status;
