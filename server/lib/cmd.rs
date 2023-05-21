use chess_core::board::{BoardPos, PieceID};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    pub piece: PieceID,
    pub to: BoardPos,
}
