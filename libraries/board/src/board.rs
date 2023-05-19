mod board {

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PlayerID {
        Player0,
        Player1,
        Player2,
        Player3,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum State {
        Home,
        Goal,
        Outside,
        Inside,
        Globe,
        Invincible,
        Star,
    }

    #[derive(Copy, Clone)]
    pub struct BoardState {
        pub position: i8,
        pub number_of_pieces: u8,
        pub player_id: Option<PlayerID>,
        pub state: State,
    }

    impl BoardState {
        pub fn new() -> BoardState {
            BoardState {
                position: -1,
                number_of_pieces: 0,
                player_id: None,
                state: State::Home,
            }
        }

        pub fn create(
            position: i8,
            number_of_pieces: u8,
            player_id: Option<PlayerID>,
            state: State,
        ) -> BoardState {
            BoardState {
                position,
                number_of_pieces,
                player_id,
                state,
            }
        }
        pub fn set(
            &mut self,
            position: i8,
            number_of_pieces: u8,
            player_id: Option<PlayerID>,
            state: State,
        ) {
            self.position = position;
            self.number_of_pieces = number_of_pieces;
            self.player_id = player_id;
            self.state = state;
        }
    }

    impl Default for BoardState {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct Board<'a> {
        pub home: [BoardState; 16],
        pub goal: [BoardState; 4],
        pub outside: [BoardState; 52],
        pub inside: [BoardState; 20],
        pub globe: Vec<&'a mut BoardState>,
        pub invincible: Vec<&'a mut BoardState>,
        pub star: [BoardState; 8],
    }

    impl Board<'a> {
        pub fn new() -> Self {
            let home = Self::initialize_home();
            let goal = Self::initialize_goal();
            let mut outside = Self::initialize_outside();
            let inside = Self::initialize_inside();
            let globe = Self::initialize_globe(&mut outside);
            let invincible = Self::initialize_invincible(&mut outside);

            Self {
                home,
                goal,
                outside,
                inside,
                globe,
                invincible,
                star: [BoardState::new(); 8],
            }
        }

        fn initialize_home() -> [BoardState; 16] {
            let mut home = [BoardState::new(); 16];
            let player_ids = [
                PlayerID::Player0,
                PlayerID::Player1,
                PlayerID::Player2,
                PlayerID::Player3,
            ];

            for (i, &player_id) in player_ids.iter().enumerate() {
                let start = i * 4;
                let end = start + 4;
                (start..end).for_each(|position| {
                    home[position] = BoardState::create(-1, 1, Some(player_id), State::Home);
                });
            }

            home
        }

        fn initialize_goal() -> [BoardState; 4] {
            let mut goal = [BoardState::new(); 4];
            (0..4).for_each(|position| {
                goal[position] = BoardState::create(99, 0, None, State::Goal);
            });

            goal
        }

        fn initialize_outside() -> [BoardState; 52] {
            let mut outside = [BoardState::new(); 52];
            (0..52).for_each(|position| {
                outside[position] = BoardState::create(position as i8, 0, None, State::Outside);
            });
            outside
        }

        fn initialize_inside() -> [BoardState; 20] {
            let mut inside = [BoardState::new(); 20];
            (0..20).for_each(|position| {
                inside[position] =
                    BoardState::create((position + 52) as i8, 0, None, State::Inside);
            });
            inside
        }

        fn initialize_globe(outside: &mut [BoardState; 52]) -> Vec<&mut BoardState> {
            let globes = [8, 21, 34, 47];
            let mut globe_states = Vec::new();

            for &position in globes.iter() {
                let board_state = &mut outside[position];
                board_state.set(position as i8, 0, None, State::Globe);
                globe_states.push(board_state);
            }
            globe_states
        }

        fn initialize_invincible(outside: &mut [BoardState; 52]) -> Vec<&mut BoardState> {
            let invincibles = [0, 13, 26, 39];
            let mut invincible_states = Vec::new();

            for &position in invincibles.iter() {
                let board_state = &mut outside[position];
                board_state.set(position as i8, 0, None, State::Invincible);
                invincible_states.push(board_state);
            }
            invincible_states
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

        pub fn globe(&self) -> &Vec<&mut BoardState> {
            &self.globe
        }

        pub fn invincible(&self) -> &Vec<&mut BoardState> {
            &self.invincible
        }

        //     pub fn star(&self) -> &[BoardState; 8] {
        //         &self.star
        //     }

        //     pub fn update(&mut self, old_position: i8, new_position: i8, player_id: Option<PlayerID>) {
        //         let old_pos_num = old_position as usize;
        //         let new_pos_num = new_position as usize;
        //         let player_id = player_id.unwrap();

        //         match (old_pos_num)
        //         {
        //              => {
        //                 self.home[0].number_of_pieces -= 1;
        //                 self.home[0].player_id = None;
        //             },
        //             0..=51 => {
        //                 self.outside[old_pos_num].number_of_pieces -= 1;
        //                 self.outside[old_pos_num].player_id = None;
        //             },
        //         }
        //         if self.outside[old_pos_num].number_of_pieces > 1
        //         {
        //             self.outside[old_pos_num].number_of_pieces -= 1;
        //         }
        //         self.outside[old_pos_num].player_id = None;

        //         self.outside[new_pos_num].number_of_pieces += 1;
        //         self.outside[new_pos_num].player_id = Some(player_id);

        //     }

        // }

        // impl Default for Board {
        //     fn default() -> Self {
        //         Self::new()
        //     }
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
pub use board::State;
