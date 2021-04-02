use gts::abstractions::Environment;

use super::*;

/// Implements environment for tic tac toe board
impl Environment<Action, AgentId> for Board {
    type ActionIter = Actions;

    /// Implements an empty tic tac board with X player starting
    fn initial_state() -> Self {
        Board {
            moves_x: 0,
            moves_o: 0,
            turn: AgentId::X,
        }
    }

    /// Updates a board by filling the position given by the action.
    /// Returns true iff the board was updated.
    fn update(&mut self, a: &Action) -> bool {
        if !self.is_valid(a) {
            false
        } else {
            match self.turn {
                AgentId::X => {
                    self.moves_x = set_bit(&mut self.moves_x, a);
                    self.turn = AgentId::O;
                }
                AgentId::O => {
                    self.moves_o = set_bit(&mut self.moves_o, a);
                    self.turn = AgentId::X;
                }
            }
            true
        }
    }

    /// Produces a new board assuming that action happens
    fn what_if(&self, a: &Action) -> Self {
        let mut board = *self;
        board.update(a);
        return board;
    }

    /// Produces iterator with valid actions
    fn valid_actions(&self) -> Self::ActionIter {
        Actions::new(self)
    }

    /// Checks if an action is valid
    fn is_valid(&self, action: &Action) -> bool {
        if action > &8 {
            false
        } else {
            !(read_bit(&self.moves_x, action) || read_bit(&self.moves_o, action))
        }
    }

    /// Checks if a board is in a terminal position
    fn is_terminal(&self) -> bool {
        is_winning(self.moves_x) || is_winning(self.moves_o) || is_filled(self)
    }

    /// Returns the identity of the agent with the right to play.
    fn turn(&self) -> AgentId {
        self.turn
    }

    /// Returns the winner of a game, if one exists.
    fn winner(&self) -> Option<AgentId> {
        if is_winning(self.moves_x) {
            Some(AgentId::X)
        } else if is_winning(self.moves_o) {
            Some(AgentId::O)
        } else {
            None
        }
    }
}
