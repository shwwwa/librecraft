/// display module.
pub mod display;
/// fps module.
pub mod fps;
/// setup module.
pub mod setup;

use bevy::state::state::States;
// Scope is small enough.
pub use display::*;
pub use fps::*;
pub use setup::*;

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum DebugGUIState {
    Opened,
    Closed,
}
