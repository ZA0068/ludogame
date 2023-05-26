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

    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct BoardState {
        pub position: i8,
        pub number_of_pieces: i8,
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
            number_of_pieces: i8,
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
            number_of_pieces: i8,
            player_id: Option<PlayerID>,
            state: State,
        ) {
            self.position = position;
            self.number_of_pieces = number_of_pieces;
            self.player_id = player_id;
            self.state = state;
        }

        pub fn get(&self) -> &BoardState {
            self
        }
    }

    impl Default for BoardState {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Board {
        pub home: [BoardState; 4],
        pub goal: [BoardState; 4],
        pub outside: [BoardState; 52],
        pub inside: [BoardState; 20],
        pub globe: [usize; 4],
        pub invincible: [usize; 4],
        pub star: [usize; 8],
    }

    impl Board {
        pub fn new() -> Self {
            let home = Self::initialize_home();
            let goal = Self::initialize_goal();
            let mut outside = Self::initialize_outside();
            let inside = Self::initialize_inside();
            let globe = Self::initialize_globe(&mut outside);
            let invincible = Self::initialize_invincible(&mut outside);
            let star = Self::initialize_star(&mut outside);

            Self {
                home,
                goal,
                outside,
                inside,
                globe,
                invincible,
                star,
            }
        }

        fn initialize_home() -> [BoardState; 4] {
            let mut home = [BoardState::new(); 4];
            let player_ids = [
                PlayerID::Player0,
                PlayerID::Player1,
                PlayerID::Player2,
                PlayerID::Player3,
            ];
            for (position, player_id) in home.iter_mut().zip(player_ids.iter()) {
                position.set(-1, 4, Some(*player_id), State::Home);
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

        fn initialize_globe(outside: &mut [BoardState; 52]) -> [usize; 4] {
            let globes = [8, 21, 34, 47];
            for &position in globes.iter() {
                outside[position].set(position as i8, 0, None, State::Globe);
            }
            globes
        }

        fn initialize_invincible(outside: &mut [BoardState; 52]) -> [usize; 4] {
            let invincibles = [0, 13, 26, 39];
            for &position in invincibles.iter() {
                outside[position].set(position as i8, 0, None, State::Invincible);
            }
            invincibles
        }

        fn initialize_star(outside: &mut [BoardState; 52]) -> [usize; 8] {
            let stars = [5, 11, 18, 24, 31, 37, 44, 50];
            for &position in stars.iter() {
                outside[position].set(position as i8, 0, None, State::Star);
            }
            stars
        }

        pub fn home(&self, id: i8) -> Option<&BoardState> {
            self.home.get(id as usize)
        }

        pub fn goal(&self, id: i8) -> Option<&BoardState> {
            self.goal.get(id as usize)
        }

        pub fn outside(&self, position: usize) -> Option<&BoardState> {
            self.outside.get(position)
        }

        pub fn inside(&self, position: usize) -> Option<&BoardState> {
            // Subtract 52 from position to account for position starting at 52
            self.inside.get(position.checked_sub(52)?)
        }

        pub fn is_globe(&self, position: usize) -> bool {
            self.globe.contains(&position)
        }

        pub fn is_invincible(&self, position: usize) -> bool {
            // Return None if position is not in invincible
            self.invincible
                .contains(&position)
        }

        pub fn is_star(&self, position: usize) -> bool {
            // Return None if position is not in star
            self.star.contains(&position)
        }

        pub fn move_from_home(&mut self, id: i8, new_pos: isize) {
            self.home[id as usize].number_of_pieces -= 1;
            if self.home[id as usize].number_of_pieces == 0 {
                self.home[id as usize].player_id = None;
            }

            self.outside[new_pos as usize].player_id = Some(get_player_id(id).unwrap());
            self.outside[new_pos as usize].number_of_pieces += 1;
        }

        pub fn move_into_home(&mut self, id: i8, new_pos: isize) {
            self.home[id as usize].number_of_pieces += 1;
            self.home[id as usize].player_id = Some(get_player_id(id).unwrap());
            self.outside[new_pos as usize].number_of_pieces -= 1;
            if self.outside[new_pos as usize].number_of_pieces == 0 {
                self.outside[new_pos as usize].player_id = None;
            }
        }

        pub fn update_outside(&mut self, id: i8, old_pos: isize, new_pos: isize) {
            if self.outside[old_pos as usize].number_of_pieces > 0 {
                self.outside[old_pos as usize].number_of_pieces -= 1;
                if self.outside[old_pos as usize].number_of_pieces == 0 {
                    self.outside[old_pos as usize].player_id = None;
                }
            }
            self.outside[new_pos as usize].number_of_pieces += 1;
            self.outside[new_pos as usize].player_id = Some(get_player_id(id).unwrap());
        }

        pub fn move_inside(&mut self, id: i8, old_pos: usize, new_pos: usize) {
            if self.outside[old_pos].number_of_pieces > 0 {
                self.outside[old_pos].number_of_pieces -= 1;
                if self.outside[old_pos].number_of_pieces == 0 {
                    self.outside[old_pos].player_id = None;
                }
            }

            let index = self
                .inside
                .iter()
                .position(|&x| x.position == new_pos as i8)
                .unwrap();

            self.inside[index].number_of_pieces += 1;
            self.inside[index].player_id = Some(get_player_id(id).unwrap());
        }

        pub fn update_inside(&mut self, id: i8, old_pos: isize, new_pos: isize) {
            let old_index_option = self
                .inside
                .iter()
                .position(|&x| x.position == old_pos as i8);
            let new_index_option = self
                .inside
                .iter()
                .position(|&x| x.position == new_pos as i8);

            if old_index_option.is_none() || new_index_option.is_none() {
                panic!("Invalid position: old: {}, new: {}", old_pos, new_pos);
            }

            let old_index = old_index_option.unwrap();
            let new_index = new_index_option.unwrap();

            if self.inside[old_index].number_of_pieces > 0 {
                self.inside[old_index].number_of_pieces -= 1;
                if self.inside[old_index].number_of_pieces == 0 {
                    self.inside[old_index].player_id = None;
                }
            }

            self.inside[new_index].number_of_pieces += 1;
            self.inside[new_index].player_id = Some(get_player_id(id).unwrap());
        }

        pub fn enter_goal(&mut self, idx: i8, old_pos: isize) {
            let old_pos_usize = old_pos as usize;
            let id = get_player_id(idx).unwrap();
            if old_pos >= 52 {
                let old_inside_index = old_pos_usize - 52;

                if let Some(old_position) = self.inside.get_mut(old_inside_index) {
                    if old_position.player_id.as_ref() != Some(&id) {
                        panic!(
                            "Invalid move: Inside position {} is not owned by player {}",
                            old_pos, idx
                        );
                    }
                    old_position.number_of_pieces -= 1;
                    if old_position.number_of_pieces == 0 {
                        old_position.player_id = None;
                    }
                } else {
                    panic!("Invalid inside position: {}", old_pos);
                }
            } else if let Some(old_position) = self.outside.get_mut(old_pos_usize) {
                if old_position.player_id.as_ref() != Some(&id) {
                    panic!(
                        "Invalid move: Outside position {} is not owned by player {}",
                        old_pos, idx
                    );
                }
                old_position.number_of_pieces -= 1;
                if old_position.number_of_pieces == 0 {
                    old_position.player_id = None;
                }
            } else {
                panic!("Invalid outside position: {}", old_pos);
            }

            self.goal[id as usize].number_of_pieces += 1;
            self.goal[id as usize].player_id = Some(id);
        }

        pub fn get_state(&self, id: i8, pos: isize) -> &BoardState {
            if pos < 0 {
                self.home[id as usize].get()
            } else if pos >= 52 {
                let inside_index = pos as usize - 52;
                return self.inside[inside_index].get();
            } else {
                return self.outside[pos as usize].get();
            }
        }
    }

    fn get_player_id(id: i8) -> Option<PlayerID> {
        match id {
            0 => Some(PlayerID::Player0),
            1 => Some(PlayerID::Player1),
            2 => Some(PlayerID::Player2),
            3 => Some(PlayerID::Player3),
            _ => None,
        }
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
