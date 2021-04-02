mod actions;
mod display;
mod environment;
mod utils;

use self::actions::Actions;
use super::Action;
use super::AgentId;

use self::utils::{is_filled, is_winning};

use bitvec::prelude::*;

type Position = BitArray<Lsb0, [u16; 1]>;

/// Represents a tictactoe board
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    moves_x: Position, // a binary array with 1 iff a position is caputured by X
    moves_o: Position, // a binary array with 1 iff a position is caputured by X
    turn: AgentId,     // Player with the right to play the next move
}
