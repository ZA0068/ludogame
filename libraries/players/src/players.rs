mod players {
    use pieces::Piece;
    pub struct Player {
        id: u8,
        pieces: Vec<Piece>,
    }

    impl Player {
        pub fn new(id: u8) -> Player {
            Player {
                id,
                pieces: (0..4).map(|i| Piece::new(i)).collect(),
            }
        }

        pub fn id(&self) -> u8 {
            self.id
        }

        pub fn pieces(&self) -> &Vec<Piece> {
            &self.pieces
        }
    }
}

pub use players::Player;
