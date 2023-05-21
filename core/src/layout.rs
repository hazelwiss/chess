use crate::{
    board::{Board, BoardPos, Piece, PieceTy},
    game::{Game, PlayerInfo, PlayerSlot},
    ruleset::RuleSet,
};

pub trait Layout: Sized {
    type Input;

    fn layout<R: RuleSet>(input: Self::Input) -> Game<Self, R>;
}

pub struct Default;

pub struct DefaultInput {
    white_name: String,
    white_rating: u32,
    black_name: String,
    black_rating: u32,
}

impl Layout for Default {
    type Input = DefaultInput;

    fn layout<R: RuleSet>(input: Self::Input) -> Game<Self, R> {
        let mut game = Game::with_board(Board::new(8, 8));

        let white_id = game
            .add_player(PlayerInfo {
                ty: PlayerSlot::White,
                name: input.white_name,
                rating: input.white_rating,
            })
            .expect("Unable to add white player");
        let black_id = game
            .add_player(PlayerInfo {
                ty: PlayerSlot::Black,
                name: input.black_name,
                rating: input.black_rating,
            })
            .expect("unable to add black player");

        macro_rules! add_pawn_line {
            ($board:expr, $row:literal, $id:expr) => {
                for col in 0..8 {
                    $board
                        .add_piece(
                            BoardPos { col, row: $row },
                            Piece {
                                owner: $id,
                                ty: PieceTy::Pawn,
                            },
                        )
                        .expect("failure on adding pawn");
                }
            };
        }
        macro_rules! add_misc_line {
            ($board:expr, $row:literal, $id:expr) => {
                $board
                    .add_piece(
                        BoardPos { col: 0, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Rook,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 1, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Knight,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 2, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Bishop,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 3, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Queen,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 4, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::King,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 5, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Bishop,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 6, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Knight,
                        },
                    )
                    .expect("failure to add rook");
                $board
                    .add_piece(
                        BoardPos { col: 7, row: $row },
                        Piece {
                            owner: $id,
                            ty: PieceTy::Rook,
                        },
                    )
                    .expect("failure to add rook");
            };
        }
        add_pawn_line!(game, 6, black_id);
        add_pawn_line!(game, 1, white_id);
        add_misc_line!(game, 0, white_id);
        add_misc_line!(game, 7, black_id);
        game
    }
}
