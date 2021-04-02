/// Actions will be a number from 0 to 8 representing a position in the tic tac toe board
pub type Action = u8;

mod agentid;
pub use agentid::AgentId;

mod board;
pub use board::Board;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
