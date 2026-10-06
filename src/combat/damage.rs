//! How hard attacks hit.

use super::Fighter;
// `RngExt` is a trait from the external crate `rand`. Methods like
// `random_range` can only be called while their trait is in scope.
// External crates need no `extern` line: once `rand` is listed in
// `Cargo.toml`, paths can start with `rand`.
use rand::RngExt;

pub fn calculate(attacker: &Fighter, defender: &Fighter) -> i32 {
    let mut amount = attacker.attack + variance();
    if defender.defending {
        amount /= 2;
    }
    amount.max(1)
}

fn variance() -> i32 {
    rand::rng().random_range(-2..=2)
}
