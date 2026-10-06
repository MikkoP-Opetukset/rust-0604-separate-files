#![allow(dead_code, unused_variables, unused_mut)]

//! Library crate of the `arena` turn-based combat game.
//!
//! The module tree and the code are exactly the same as in the previous demo.
//! Only where the code lives has changed. `mod combat;` with a semicolon
//! instead of a block tells the compiler to load the module's body from a
//! file. The module is declared once, in its parent, and the file path
//! follows the module path:
//!
//! ```text
//! src/lib.rs                 crate root
//! src/combat.rs              crate::combat
//! src/combat/damage.rs       crate::combat::damage
//! src/combat/status.rs       crate::combat::status
//! src/items.rs               crate::items
//! src/enemy_ai.rs            crate::enemy_ai
//! src/ui.rs                  crate::ui
//! ```
//!
//! A module with children keeps its own code in `src/combat.rs` and puts the
//! children into a folder with the same name. The older style,
//! `src/combat/mod.rs`, still works but is not recommended, because a project
//! with many `mod.rs` files is hard to navigate in an editor.

mod combat;
mod enemy_ai;
mod items;
pub mod ui;

// `pub use` re-exports names. The private modules above are an implementation
// detail now; users of the library see this flat API instead, and the module
// layout can change without breaking them.
//
// Nested paths work here too, and `as` renames the re-exported item.
pub use combat::status::tick as tick_status;
pub use combat::{Action, Fighter, resolve_action};
pub use enemy_ai::choose_action;
pub use items::Item;
