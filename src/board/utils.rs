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
pub(super) fn is_winning(position: Position) -> bool {
    let pos: u16 = position.load();
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
    let fill: u16 = fill.load();
    return fill == FULL;
}
