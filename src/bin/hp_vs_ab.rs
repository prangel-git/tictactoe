use tictactoe::*;

/// Two agents playing tic tac toe (human vs ab)
fn main() {
    let mut board = Board::initial_state();

    let mut player_x = HumanPlayer::new(AgentId::X);
    let mut player_o = AlphabetaAgent::new(AgentId::O, &depth_first, 10);

    let log = play(&mut board, &mut player_x, &mut player_o);

    for (agent, mv) in log {
        println!("Player: {:?}, moved {}", agent, mv);
    }

    println!("Last board \n{}", board);

    let winner = board.winner();

    match winner {
        Some(x) => println!("Player {:?} wins.", x),
        None => println!("\nThe game ended in a draw"),
    }
}
