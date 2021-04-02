use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AgentId {
    X,
    O,
}

impl Display for AgentId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AgentId::X => write!(f, "Player X"),
            AgentId::O => write!(f, "Player O"),
        }
    }
}
