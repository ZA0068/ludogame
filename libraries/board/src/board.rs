mod board {

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PlayerID {
        None,
        Player0,
        Player1,
        Player2,
        Player3,
    }
    #[derive(Copy, Clone)]
    pub struct BoardState {
        pub position: i8,
        pub number_of_pieces: u8,
        pub player_id: Option<PlayerID>,
    }

    impl BoardState {
        pub fn new() -> BoardState {
            BoardState {
                position: 0,
                number_of_pieces: 0,
                player_id: None,
            }
        }

        pub fn create(
            position: i8,
            number_of_pieces: u8,
            player_id: Option<PlayerID>,
        ) -> BoardState {
            BoardState {
                position,
                number_of_pieces,
                player_id,
            }
        }
    }

    impl Default for BoardState {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct Board {
        home: [BoardState; 16],
        goal: [BoardState; 4],
        outside: [BoardState; 52],
        inside: [BoardState; 20],
        globe: [BoardState; 4],
        invincible: [BoardState; 4],
        star: [BoardState; 8],
    }

    impl Board {
        pub fn new() -> Self {
            let mut home = [BoardState::new(); 16];
            (0..4).for_each(|i| {
                home[i] = BoardState::create(-1, 1, Some(PlayerID::Player0));
            });
            (4..8).for_each(|i| {
                home[i] = BoardState::create(-1, 1, Some(PlayerID::Player1));
            });
            (8..12).for_each(|i| {
                home[i] = BoardState::create(-1, 1, Some(PlayerID::Player2));
            });
            (12..16).for_each(|i| {
                home[i] = BoardState::create(-1, 1, Some(PlayerID::Player3));
            });
            Self {
                home,
                goal: [BoardState::new(); 4],
                outside: [BoardState::new(); 52],
                inside: [BoardState::new(); 20],
                globe: [BoardState::new(); 4],
                invincible: [BoardState::new(); 4],
                star: [BoardState::new(); 8],
            }
        }

        pub fn home(&self) -> &[BoardState; 16] {
            &self.home
        }

        pub fn goal(&self) -> &[BoardState; 4] {
            &self.goal
        }

        pub fn outside(&self) -> &[BoardState; 52] {
            &self.outside
        }

        pub fn inside(&self) -> &[BoardState; 20] {
            &self.inside
        }

        pub fn globe(&self) -> &[BoardState; 4] {
            &self.globe
        }

        pub fn invincible(&self) -> &[BoardState; 4] {
            &self.invincible
        }

        pub fn star(&self) -> &[BoardState; 8] {
            &self.star
        }

        // pub fn occupy(&mut self, space_number: i8, player: PlayerID) -> Option<i8> {
        //     // Your logic for occupying a space
        //     None
        // }
    }

    impl Default for Board {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use board::Board;
pub use board::BoardState;
pub use board::PlayerID;
