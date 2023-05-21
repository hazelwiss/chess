use chess_core::board::{Board, BoardPos, PieceID};
use server::ClientID;

pub enum Error {
    InalidMove(ClientID, PieceID, BoardPos),
}

pub enum MsgClientTy {}

pub struct MsgClient {
    sender: ClientID,
    msg: MsgServerTy,
}

pub enum MsgServerTy {
    Error(Error),
    Board(Board),
}

pub struct MsgServer {
    recipient: ClientID,
    msg: MsgServerTy,
}
