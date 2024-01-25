use colored::Colorize;
use std::fmt::{Display, Formatter};
use synthesis::prelude::*;

mod utils;

const _NUM_SQUARES: u8 = 249;
pub const NUM_MAX_TURNS: usize = 256;
const MAX_NUM_POSSIBLE_MOVES: usize = 2034; // TODO: Find a lower upper bound, if possible
/// Wheel: Max 8 per direction = 64
/// All usual tiles: (5 + 9 + 7 + 5 + 3 + 1) * 2 = 60
/// Tiles boosted by Dragon: (6 + 11 + 9 + 7 + 5 + 3 + 1) * 2 = 84
/// Maximum 6 tiles boosted by Dragon, total of 7 tiles, means 8 * 84 + 60 = 732
/// Tiles + Wheels = 796
/// Lotus: Can at most land on 1 out of 4 tiles, excluding itself and the enemy lotus = 62 squares
/// Tiles + Wheels + Lotus = 858
/// BM and SB tiles have actions - up to 8 directions
/// Tiles_new = 60 + 6 * 84 + 2 * 8 * 84 = 1908
/// Tiles_new + Wheels + Lotus = 2034

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Ginseng {
    board: [[Option<Piece>; 17]; 17],
    player: PlayerID,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PlayerID {
    Host,
    Guest,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    player: PlayerID,
    piece: Piece,
    from: i8,
    to: i8,
    effect: Option<Direction>,
    exchange_into: Option<MortalPiece>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Piece {
    Lotus { player: PlayerID },
    Ginseng { player: PlayerID },
    LionTurtle { player: PlayerID },
    Dragon { player: PlayerID },
    SkyBison { player: PlayerID },
    BadgerMole { player: PlayerID },
    Koi { player: PlayerID },
    Orchid { player: PlayerID },
    Wheel { player: PlayerID },
    OutOfBounds,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MortalPiece {
    Ginseng { player: PlayerID },
    LionTurtle { player: PlayerID },
    Dragon { player: PlayerID },
    SkyBison { player: PlayerID },
    BadgerMole { player: PlayerID },
    Koi { player: PlayerID },
    Orchid { player: PlayerID },
    Wheel { player: PlayerID },
}

pub struct GinsengIterator;

impl Iterator for GinsengIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl From<usize> for Move {
    #[inline]
    fn from(_value: usize) -> Self {
        todo!()
    }
}

impl From<Move> for usize {
    #[inline]
    fn from(_value: Move) -> Self {
        todo!()
    }
}

impl HasTurnOrder for PlayerID {
    #[inline]
    fn prev(&self) -> Self {
        match self {
            PlayerID::Host => PlayerID::Guest,
            PlayerID::Guest => PlayerID::Host,
        }
    }

    #[inline]
    fn next(&self) -> Self {
        self.prev()
    }
}

impl Ginseng {
    #[inline]
    pub fn winner(&self) -> Option<PlayerID> {
        for row in self.board.iter() {
            for (j, square) in row.iter().enumerate() {
                if let Some(Piece::Lotus { player }) = square {
                    if j > 8 && player == &PlayerID::Host {
                        return Some(PlayerID::Host);
                    } else if j < 8 && player == &PlayerID::Guest {
                        return Some(PlayerID::Guest);
                    }
                }
            }
        }
        None
    }
}

impl Game<MAX_NUM_POSSIBLE_MOVES> for Ginseng {
    type PlayerId = PlayerID;
    type Action = Move;
    type ActionIterator = GinsengIterator;
    type Features = ();
    const MAX_NUM_ACTIONS: usize = MAX_NUM_POSSIBLE_MOVES;
    const MAX_TURNS: usize = NUM_MAX_TURNS;
    const NAME: &'static str = "Ginseng Pai Sho";
    const NUM_PLAYERS: usize = 2;
    const DIMS: &'static [i64] = &[];

    #[inline]
    fn new() -> Self {
        Self {
            board: utils::STARTING_BOARD,
            player: PlayerID::Guest,
        }
    }

    #[inline]
    fn player(&self) -> Self::PlayerId {
        self.player
    }

    #[inline]
    fn is_over(&self) -> bool {
        self.winner().is_some()
    }

    #[inline]
    fn reward(&self, player_id: Self::PlayerId) -> f32 {
        match self.winner() {
            None => 0.0,
            Some(winner) => {
                if winner == player_id {
                    1.0
                } else {
                    -1.0
                }
            }
        }
    }

    #[inline]
    fn iter_actions(&self) -> Self::ActionIterator {
        GinsengIterator
    }

    #[inline]
    fn step(&mut self, _action: &Self::Action) -> bool {
        todo!()
    }

    #[inline]
    fn features(&self) -> Self::Features {
        todo!()
    }

    #[inline]
    fn print(&self) {
        for row in self.board {
            let mut row_string = String::new();
            for square in row {
                match square {
                    None => row_string += &"+",
                    Some(Piece::OutOfBounds) => row_string += " ",
                    Some(piece) => {
                        let letter = match piece {
                            Piece::Lotus { .. } => "L",
                            Piece::Ginseng { .. } => "G",
                            Piece::LionTurtle { .. } => "T",
                            Piece::Dragon { .. } => "D",
                            Piece::SkyBison { .. } => "S",
                            Piece::BadgerMole { .. } => "B",
                            Piece::Koi { .. } => "K",
                            Piece::Orchid { .. } => "O",
                            Piece::Wheel { .. } => "W",
                            _ => " ",
                        };
                        match piece {
                            Piece::OutOfBounds => {}
                            Piece::Lotus { player }
                            | Piece::Ginseng { player }
                            | Piece::LionTurtle { player }
                            | Piece::Dragon { player }
                            | Piece::SkyBison { player }
                            | Piece::BadgerMole { player }
                            | Piece::Koi { player }
                            | Piece::Orchid { player }
                            | Piece::Wheel { player } => match player {
                                PlayerID::Host => {
                                    row_string += &letter
                                        .black()
                                        .on_truecolor(189, 148, 102)
                                        .bold()
                                        .to_string()
                                }
                                PlayerID::Guest => {
                                    row_string +=
                                        &letter.on_truecolor(117, 112, 37).bold().to_string()
                                }
                            },
                        }
                    }
                }
                row_string += " ";
            }
            println!("{row_string}");
        }
    }
}

impl Display for PlayerID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerID::Host => {
                    "H"
                }
                PlayerID::Guest => {
                    "G"
                }
            }
        )
    }
}
