use super::*;

const WINNING: [u16; 8] = [
    0b111u16,
    0b111000u16,
    0b111000000u16,
    0b1001001u16,
    0b10010010u16,
    0b100100100u16,
    0b100010001u16,
    0b1010100u16,
];

const FULL: u16 = 0b111111111u16;

/// Checks whehter a position is winning
pub(super) fn is_winning(pos: Position) -> bool {
    for &wins in &WINNING {
        if pos & wins == wins {
            return true;
        }
    }
    return false;
}

/// Checks whether the board is completelly full
pub(super) fn is_filled(board: &Board) -> bool {
    let fill = board.moves_o | board.moves_x;
    return fill == FULL;
}

/// Indexing Position bitwise
pub(super) fn read_bit(pos: &Position, act: &Action) -> bool {
    let mask = 1 << act;
    mask & pos == mask
}

/// Set a bit
pub(super) fn set_bit(&pos: &Position, act: &Action) -> Position {
    let mask = 1 << act;
    pos | mask
}
