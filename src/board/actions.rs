use gts::abstractions::Environment;

use super::*;

/// Iterator with valid actions
pub struct Actions {
    occupied: Position,
    current: Action,
}

/// Implements actions
impl Actions {
    /// Initializes a new structure with valid actions
    pub fn new(board: &Board) -> Self {
        let occupied = board.moves_o | board.moves_x;
        let current = if board.is_terminal() { 9 } else { 0 };
        Actions { occupied, current }
    }
}

/// Implements interator
impl Iterator for Actions {
    type Item = Action;

    /// Defines next element to iterate
    fn next(&mut self) -> Option<Self::Item> {
        while self.current < 9 && read_bit(&self.occupied, &self.current) {
            self.current += 1;
        }

        let output = if self.current >= 9 {
            None
        } else {
            Some(self.current)
        };

        self.current += 1;

        return output;
    }
}
