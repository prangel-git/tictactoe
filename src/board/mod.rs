mod display;
mod environment;

mod actions;
mod utils;

use self::actions::Actions;
use super::Action;
use super::AgentId;

use self::utils::*;

type Position = u16;

/// Represents a tictactoe board
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    moves_x: Position, // a binary array with 1 iff a position is caputured by X
    moves_o: Position, // a binary array with 1 iff a position is caputured by X
    turn: AgentId,     // Player with the right to play the next move
}
