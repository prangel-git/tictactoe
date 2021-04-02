use tictactoe::*;

/// Tic tac toe game (Alphabeta vs Alphabeta)
fn main() {
    let mut board = Board::initial_state();

    let mut player_x = AlphabetaAgent::new(AgentId::X, &depth_first, 10);
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
