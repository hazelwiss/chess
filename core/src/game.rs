use crate::{
    board::{Board, BoardError, BoardPos, Piece, PieceID},
    layout::Layout,
    ruleset::RuleSet,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, marker::PhantomData};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerSlot {
    White,
    Black,
}

#[derive(Clone)]
pub struct PlayerInfo {
    pub ty: PlayerSlot,
    pub name: String,
    pub rating: u32,
}

pub struct Player {
    pub info: PlayerInfo,
    pub pieces: Vec<PieceID>,
    pub lost: Vec<Piece>,
    pub gained: Vec<Piece>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerID(PlayerSlot);

#[derive(Debug)]
pub enum GameError {
    PlayerSlotOccupied(PlayerID),
    InvalidPlayerID(PlayerID),
    BoardError(BoardError),
}

impl From<BoardError> for GameError {
    fn from(err: BoardError) -> Self {
        Self::BoardError(err)
    }
}

pub type Result<T> = std::result::Result<T, GameError>;

pub struct Game<L: Layout, R: RuleSet> {
    players: BTreeMap<PlayerID, Player>,
    board: Board,
    _l: PhantomData<L>,
    _gr: PhantomData<R>,
}

impl<L: Layout, R: RuleSet> Game<L, R> {
    pub fn new(layout_data: L::Input) -> Self {
        L::layout(layout_data)
    }

    pub(crate) fn with_board(board: Board) -> Self {
        Self {
            players: BTreeMap::new(),
            board,
            _l: PhantomData,
            _gr: PhantomData,
        }
    }

    pub fn add_player(&mut self, info: PlayerInfo) -> Result<PlayerID> {
        let id = PlayerID(info.ty);
        if let Some(_) = self.players.get(&id) {
            Err(GameError::PlayerSlotOccupied(id))
        } else {
            self.players.insert(
                id,
                Player {
                    info,
                    pieces: vec![],
                    lost: vec![],
                    gained: vec![],
                },
            );
            Ok(id)
        }
    }

    pub fn remove_player(&mut self, id: PlayerID) -> Result<Player> {
        self.players
            .remove(&id)
            .ok_or(GameError::InvalidPlayerID(id))
    }

    pub fn add_piece(&mut self, pos: BoardPos, piece: Piece) -> Result<PieceID> {
        let player_id = piece.owner;
        let piece_id = self.board.add_piece(pos, piece)?;
        if let Some(player) = self.get_player_mut(player_id) {
            player.pieces.push(piece_id);
            Ok(piece_id)
        } else {
            Err(GameError::InvalidPlayerID(player_id))
        }
    }

    pub fn get_player(&self, id: PlayerID) -> Option<&Player> {
        self.players.get(&id)
    }

    fn get_player_mut(&mut self, id: PlayerID) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }
}
