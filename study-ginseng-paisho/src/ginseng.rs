use synthesis::prelude::*;
pub const NUM_MAX_TURNS: usize = 256;
const MAX_NUM_POSSIBLE_MOVES: usize = 128; // TODO: Calculate this!

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
pub struct Move {
    player: PlayerID,
    piece: Piece,
    from: i8,
    to: i8,
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
    fn from(value: usize) -> Self {
        todo!()
    }
}

impl From<Move> for usize {
    #[inline]
    fn from(value: Move) -> Self {
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
        for (i, row) in self.board.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                if let Some(piece) = square {
                    if let Piece::Lotus { player } = piece {
                        if j > 8 && player == &PlayerID::Host {
                            return Some(PlayerID::Host);
                        } else if j < 8 && player == &PlayerID::Guest {
                            return Some(PlayerID::Guest);
                        }
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
            board: [[None; 17]; 17],
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
    fn step(&mut self, action: &Self::Action) -> bool {
        todo!()
    }

    #[inline]
    fn features(&self) -> Self::Features {
        todo!()
    }

    #[inline]
    fn print(&self) {
        todo!()
    }
}
