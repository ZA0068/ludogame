mod pieces {

    pub struct State {
        home: bool,
        goal: bool,
        safe: bool,
        invincible: bool,        
        position: u8,
    }
    pub struct Piece {
        id: u8,
        state: State,
    }
    
    impl Piece {
        pub fn new(id: u8) -> Piece {
            Piece {
                id: id,
                state: State {
                    home: true,
                    goal: false,
                    safe: false,
                    invincible: false,
                    position: 0,
                }
            }
        }
    }
}

pub use pieces::Piece;