use std::fmt::{Display, Formatter, Result};

use super::*;

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for idx in 0..9 {
            let x = read_bit(&self.moves_x, &idx);
            let o = read_bit(&self.moves_o, &idx);

            if idx % 3 == 0 {
                write!(f, "\n|").ok();
            }
            match (x, o) {
                (true, false) => write!(f, " {} |", "x").ok(),
                (false, true) => write!(f, " {} |", "o").ok(),
                (false, false) => write!(f, " {} |", " ").ok(),
                (true, true) => write!(f, " {} |", "?").ok(),
            };
        }
        write!(f, "\nEnd of board")
    }
}
