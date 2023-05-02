use crate::piece::Piece;

mod players
{
    pub struct Player {
        id: u8,
        pieces: Vec<u8>,
    }

    impl Player {
        pub fn new(id: u8) -> Player {
            Player {
                id,
                pieces: (0..4).map(|i| Piece::new(i)).collect(),
            }
        }
    }
}

pub use players::Player;