use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game::PlayerID;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PieceTy {
    Pawn,
    Queen,
    King,
    Bishop,
    Rook,
    Knight,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Piece {
    pub owner: PlayerID,
    pub ty: PieceTy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceID(usize);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Square {
    Empty,
    Piece(PieceID),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct BoardPos {
    pub col: usize,
    pub row: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BoardError {
    OutsideOfBoard(BoardPos),
    OccupiedSquare(BoardPos, PieceID),
    InvalidPieceID(PieceID),
}

pub type Result<T> = std::result::Result<T, BoardError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    width: usize,
    height: usize,
    squares: Vec<Vec<Square>>,
    pieces: HashMap<PieceID, Piece>,
    id_counter: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            squares: vec![vec![Square::Empty; height]; width],
            pieces: HashMap::new(),
            id_counter: 0,
        }
    }

    /// Adds the piece to the board.
    /// Returns an error upon unsuccessfully adding a piece onto the
    /// board, or if the current square was already occupied.
    pub fn add_piece(&mut self, pos: BoardPos, piece: Piece) -> Result<PieceID> {
        let id = self.generate_new_piece(piece)?;
        self.move_piece(id, pos)?;
        Ok(id)
    }

    /// Adds the piece to the board.
    /// Returns an error upon unsuccessfully adding a piece onto the
    /// board. If a piece already exists on the given square,
    /// it is replaced.
    pub fn add_piece_forced(&mut self, pos: BoardPos, piece: Piece) -> Result<PieceID> {
        let id = self.generate_new_piece(piece)?;
        self.move_piece_force(id, pos)?;
        Ok(id)
    }

    pub fn remove_piece(&mut self, id: PieceID) -> Result<Piece> {
        self.pieces
            .remove(&id)
            .ok_or(BoardError::InvalidPieceID(id))
    }

    /// Moves a piece to the given square.
    /// Returns the moved to position on success.
    /// Errors if the piece was unsuccessfully moved, or in
    /// case another piece occupied the space.
    pub fn move_piece(&mut self, id: PieceID, pos: BoardPos) -> Result<BoardPos> {
        if let Some(square) = self.get_square_mut(pos) {
            match square {
                Square::Empty => {
                    *square = Square::Piece(id);
                    Ok(pos)
                }
                Square::Piece(id) => return Err(BoardError::OccupiedSquare(pos, *id)),
            }
        } else {
            Err(BoardError::OutsideOfBoard(pos))
        }
    }

    /// Moves a piece to the given square.
    /// Returns the moved to position on success.
    /// In case a piece already exists on the given square, it is
    /// replaced by the moved piece.
    /// Errors if the piece was unsuccessfully moved.
    pub fn move_piece_force(&mut self, id: PieceID, pos: BoardPos) -> Result<BoardPos> {
        if let Some(square) = self.get_square_mut(pos) {
            match *square {
                Square::Empty => {
                    *square = Square::Piece(id);
                    Ok(pos)
                }
                Square::Piece(old_id) => {
                    *square = Square::Piece(id);
                    self.remove_piece(old_id)?;
                    Ok(pos)
                }
            }
        } else {
            Err(BoardError::OutsideOfBoard(pos))
        }
    }

    pub fn get_piece(&self, id: PieceID) -> Option<&Piece> {
        self.pieces.get(&id)
    }

    fn get_piece_mut(&mut self, id: PieceID) -> Option<&mut Piece> {
        self.pieces.get_mut(&id)
    }

    pub fn get_square(&self, pos: BoardPos) -> Option<&Square> {
        if pos.col < self.width && pos.row < self.height {
            Some(&self.squares[pos.col][pos.row])
        } else {
            None
        }
    }

    fn get_square_mut(&mut self, pos: BoardPos) -> Option<&mut Square> {
        if pos.col < self.width && pos.row < self.height {
            Some(&mut self.squares[pos.col][pos.row])
        } else {
            None
        }
    }

    fn generate_new_piece(&mut self, piece: Piece) -> Result<PieceID> {
        let id = PieceID(self.id_counter);
        self.id_counter += 1;
        self.pieces.insert(id, piece);
        Ok(id)
    }
}
