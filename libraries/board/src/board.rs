mod board {

    use std::{cell::RefCell, rc::Rc};

    use pieces::Piece;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PlayerID {
        Player0,
        Player1,
        Player2,
        Player3,
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

    #[derive(Clone, Debug, PartialEq)]

    pub struct BoardState {
        pub position: i8,
        pub pieces: Vec<Rc<RefCell<Piece>>>,
        pub player_id: Option<PlayerID>,
        pub state: State,
    }

    impl BoardState {
        pub fn new() -> BoardState {
            BoardState {
                position: -1,
                pieces: Vec::default(),
                player_id: None,
                state: State::Home,
            }
        }

        pub fn create(
            position: i8,
            pieces: Vec<Rc<RefCell<Piece>>>,
            player_id: Option<PlayerID>,
            state: State,
        ) -> BoardState {
            BoardState {
                position,
                pieces,
                player_id,
                state,
            }
        }
        pub fn set(
            &mut self,
            position: i8,
            pieces: Vec<Rc<RefCell<Piece>>>,
            player_id: Option<PlayerID>,
            state: State,
        ) {
            self.position = position;
            self.pieces = pieces;
            self.player_id = player_id;
            self.state = state;
        }

        pub fn get(&self) -> &BoardState {
            self
        }

        pub fn piece(&mut self, piece_id: i8) -> Rc<RefCell<Piece>> {
            let piece_idx = self
                .pieces
                .iter()
                .position(|piece| piece.borrow_mut().id() == piece_id)
                .unwrap();
            self.pieces[piece_idx].clone()
        }
    }

    impl Default for BoardState {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
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
            let mut home: [BoardState; 4] = [
                BoardState::new(),
                BoardState::new(),
                BoardState::new(),
                BoardState::new(),
            ];
            let player_ids = [
                PlayerID::Player0,
                PlayerID::Player1,
                PlayerID::Player2,
                PlayerID::Player3,
            ];

            for (position, player_id) in home.iter_mut().zip(player_ids.iter()) {
                position.set(
                    -1,
                    create_vector_of_pieces().clone(),
                    Some(*player_id),
                    State::Home,
                );
            }
            home
        }

        fn initialize_goal() -> [BoardState; 4] {
            let mut goal = [
                BoardState::new(),
                BoardState::new(),
                BoardState::new(),
                BoardState::new(),
            ];
            (0..4).for_each(|position| {
                goal[position] = BoardState::create(99, Vec::default(), None, State::Goal);
            });
            goal
        }

        fn initialize_outside() -> [BoardState; 52] {
            let mut outside: Vec<BoardState> = vec![];
            (0..52).for_each(|position| {
                outside.push(BoardState::create(
                    position as i8,
                    Vec::default(),
                    None,
                    State::Outside,
                ));
            });
            outside.try_into().unwrap()
        }

        fn initialize_inside() -> [BoardState; 20] {
            let mut inside: Vec<BoardState> = vec![];
            (0..20).for_each(|position| {
                inside.push(BoardState::create(
                    (position + 52) as i8,
                    Vec::default(),
                    None,
                    State::Inside,
                ));
            });
            inside.try_into().unwrap()
        }

        fn initialize_globe(outside: &mut [BoardState; 52]) -> [usize; 4] {
            let globes = [8, 21, 34, 47];
            for &position in globes.iter() {
                outside[position].set(position as i8, Vec::default(), None, State::Globe);
            }
            globes
        }

        fn initialize_invincible(outside: &mut [BoardState; 52]) -> [usize; 4] {
            let invincibles = [0, 13, 26, 39];
            for &position in invincibles.iter() {
                outside[position].set(position as i8, Vec::default(), None, State::Invincible);
            }
            invincibles
        }

        fn initialize_star(outside: &mut [BoardState; 52]) -> [usize; 8] {
            let stars = [5, 11, 18, 24, 31, 37, 44, 50];
            for &position in stars.iter() {
                outside[position].set(position as i8, Vec::default(), None, State::Star);
            }
            stars
        }

        pub fn home(&mut self, player_id: i8) -> &mut BoardState {
            &mut self.home[player_id as usize]
        }

        pub fn goal(&mut self, player_id: i8) -> &mut BoardState {
            &mut self.goal[player_id as usize]
        }

        pub fn outside(&mut self, position: i8) -> &mut BoardState {
            &mut self.outside[position as usize]
        }

        pub fn inside(&mut self, position: i8) -> &mut BoardState {
            &mut self.inside[(position - 52) as usize]
        }
        pub fn invincible(&mut self, position: i8) -> &mut BoardState {
            &mut self.outside[self.invincible[position as usize]]
        }

        pub fn star(&mut self, position: i8) -> &mut BoardState {
            &mut self.outside[self.star[position as usize]]
        }

        pub fn globe(&mut self, position: i8) -> &mut BoardState {
            &mut self.outside[self.globe[position as usize]]
        }

        pub fn is_globe(&self, position: i8) -> bool {
            self.globe.contains(&(position as usize))
        }

        pub fn is_invincible(&self, position: i8) -> bool {
            // Return None if position is not in invincible
            self.invincible.contains(&(position as usize))
        }

        pub fn is_star(&self, position: i8) -> bool {
            // Return None if position is not in star
            self.star.contains(&(position as usize))
        }

        pub fn move_from_home(&mut self, player_id: i8, piece_id: i8, new_position: i8) {
            let (piece, piece_idx) = self.get_home_piece_and_index(player_id, piece_id);
            self.add_piece_to_outside_position(new_position, player_id, piece);
            self.remove_piece_from_home_position(player_id, piece_idx);
        }

        fn add_piece_to_outside_position(
            &mut self,
            new_position: i8,
            player_id: i8,
            piece: Rc<RefCell<Piece>>,
        ) {
            self.outside(new_position).player_id = Some(get_player_id(player_id).unwrap());
            self.outside(new_position).pieces.push(piece);
        }

        fn get_home_piece_and_index(
            &mut self,
            player_id: i8,
            piece_id: i8,
        ) -> (Rc<RefCell<Piece>>, usize) {
            let piece_idx = self.get_home_piece_index(player_id, piece_id);
            let piece = self.get_home_piece(player_id, piece_idx);
            (piece, piece_idx)
        }

        fn get_home_piece(&mut self, player_id: i8, piece_idx: usize) -> Rc<RefCell<Piece>> {
            self.home(player_id).pieces[piece_idx].clone()
        }

        fn get_home_piece_index(&mut self, player_id: i8, piece_id: i8) -> usize {
            self.home(player_id)
                .pieces
                .iter()
                .position(|piece| piece.borrow().id() == piece_id)
                .unwrap()
        }

        fn remove_piece_from_home_position(&mut self, player_id: i8, piece_idx: usize) {
            self.remove_home_piece_if_not_empty(player_id, piece_idx);
            self.set_home_player_id_to_none_if_empty(player_id);
        }

        fn set_home_player_id_to_none_if_empty(&mut self, player_id: i8) {
            if self.home(player_id).pieces.is_empty() {
                self.home(player_id).player_id = None;
            }
        }

        fn remove_home_piece_if_not_empty(&mut self, player_id: i8, piece_idx: usize) {
            if !self.home(player_id).pieces.is_empty() {
                self.home(player_id).pieces.remove(piece_idx);
            }
        }

        pub fn move_into_home(&mut self, player_id: i8, piece_id: i8, old_position: i8) {
            let (piece, piece_idx) = self.get_outside_piece_and_index(old_position, piece_id);
            self.add_piece_to_home_position(player_id, piece);
            self.remove_piece_from_outside_position(old_position, piece_idx);
        }

        fn add_piece_to_home_position(&mut self, player_id: i8, piece: Rc<RefCell<Piece>>) {
            self.home[player_id as usize].pieces.push(piece);
            self.home[player_id as usize].player_id = Some(get_player_id(player_id).unwrap());
        }

        pub fn get_outside_piece_and_index(
            &mut self,
            old_position: i8,
            piece_id: i8,
        ) -> (Rc<RefCell<Piece>>, usize) {
            let piece_idx = self.get_outside_piece_index(old_position, piece_id);
            let piece = self.get_outside_piece(old_position, piece_idx);
            (piece, piece_idx)
        }

        pub fn get_outside_piece(
            &mut self,
            old_position: i8,
            piece_idx: usize,
        ) -> Rc<RefCell<Piece>> {
            self.outside(old_position).pieces[piece_idx].clone()
        }

        fn get_outside_piece_index(&mut self, old_position: i8, piece_id: i8) -> usize {
            let piece_idx = self
                .outside(old_position)
                .pieces
                .iter()
                .position(|piece| piece.borrow().id() == piece_id)
                .unwrap();
            piece_idx
        }

        pub fn update_outside(
            &mut self,
            player_id: i8,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) {
            let (piece, piece_idx) = self.get_outside_piece_and_index(old_position, piece_id);
            self.remove_piece_from_outside_position(old_position, piece_idx);
            self.add_piece_to_outside_position(new_position, player_id, piece);
        }

        fn remove_piece_from_outside_position(&mut self, old_position: i8, piece_idx: usize) {
            self.remove_outside_piece_if_not_empty(old_position, piece_idx);
            self.set_outside_player_id_to_none_if_empty(old_position);
        }

        fn set_outside_player_id_to_none_if_empty(&mut self, old_position: i8) {
            if self.outside(old_position).pieces.is_empty() {
                self.outside(old_position).player_id = None;
            }
        }

        fn remove_outside_piece_if_not_empty(&mut self, old_position: i8, piece_idx: usize) {
            if !self.outside(old_position).pieces.is_empty() {
                self.outside(old_position).pieces.remove(piece_idx);
            }
        }

        pub fn move_inside(
            &mut self,
            player_id: i8,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) {
            let (piece, piece_idx) = self.get_outside_piece_and_index(old_position, piece_id);
            self.add_piece_to_inside_position(new_position, piece, player_id);
            self.remove_piece_from_outside_position(old_position, piece_idx);
        }

        fn add_piece_to_inside_position(
            &mut self,
            new_position: i8,
            piece: Rc<RefCell<Piece>>,
            player_id: i8,
        ) {
            self.inside(new_position).pieces.push(piece);
            self.inside(new_position).player_id = Some(get_player_id(player_id).unwrap());
        }

        pub fn update_inside(
            &mut self,
            player_id: i8,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) {
            let (piece, piece_idx) = self.get_inside_piece_and_index(old_position, piece_id);
            self.remove_piece_from_inside_position(old_position, piece_idx);
            self.add_piece_to_inside_position(new_position, piece, player_id);
        }

        fn get_inside_piece_and_index(
            &mut self,
            old_position: i8,
            piece_id: i8,
        ) -> (Rc<RefCell<Piece>>, usize) {
            let piece_idx = self.get_inside_piece_index(old_position, piece_id);
            let piece = self.get_inside_piece(old_position, piece_idx);
            (piece, piece_idx)
        }

        fn remove_piece_from_inside_position(&mut self, old_position: i8, piece_idx: usize) {
            self.remove_inside_piece_if_not_empty(old_position, piece_idx);
            self.set_inside_player_id_to_none_if_empty(old_position);
        }

        fn set_inside_player_id_to_none_if_empty(&mut self, old_position: i8) {
            if self.inside(old_position).pieces.is_empty() {
                self.inside(old_position).player_id = None;
            }
        }

        fn remove_inside_piece_if_not_empty(&mut self, old_position: i8, piece_idx: usize) {
            if !self.inside(old_position).pieces.is_empty() {
                self.inside(old_position).pieces.remove(piece_idx);
            }
        }

        fn get_inside_piece_index(&mut self, old_position: i8, piece_id: i8) -> usize {
            self.inside(old_position)
                .pieces
                .iter()
                .position(|piece| piece.borrow().id() == piece_id)
                .unwrap()
        }

        fn get_inside_piece(&mut self, old_position: i8, piece_idx: usize) -> Rc<RefCell<Piece>> {
            self.inside(old_position).pieces[piece_idx].clone()
        }

        pub fn enter_goal(&mut self, player_id: i8, piece_id: i8, old_position: i8) {
            match old_position {
                (0..=51) => {
                    let (piece, piece_idx) =
                        self.get_outside_piece_and_index(old_position, piece_id);
                    self.remove_piece_from_outside_position(old_position, piece_idx);
                    self.add_piece_to_goal_position(player_id, piece);
                }
                (52..=71) => {
                    let (piece, piece_idx) =
                        self.get_inside_piece_and_index(old_position, piece_id);
                    self.add_piece_to_goal_position(player_id, piece);
                    self.remove_piece_from_inside_position(old_position, piece_idx);
                }
                _ => panic!("Invalid position"),
            };
        }

        fn add_piece_to_goal_position(&mut self, player_id: i8, piece: Rc<RefCell<Piece>>) {
            self.goal(player_id).pieces.push(piece);
            self.goal(player_id).player_id = Some(get_player_id(player_id).unwrap());
        }

        pub fn is_occupied_more(&mut self, position: i8) -> bool {
            self.outside(position).pieces.len() > 1
        }

        pub fn is_occupied(&mut self, position: i8) -> bool {
            !self.outside(position).pieces.is_empty()
        }

        pub fn is_occupied_self(&mut self, player_id: i8, position: i8) -> bool {
            if !self.is_occupied(position) {
                return false;
            }
            self.outside(position).player_id == get_player_id(player_id)
        }

        pub fn is_occupied_by_more_self(&mut self, player_id: i8, position: i8) -> bool {
            if !self.is_occupied_more(position) {
                return false;
            }
            self.outside(position).player_id != get_player_id(player_id)
        }

        pub fn is_occupied_by_other(&mut self, player_id: i8, position: i8) -> bool {
            if !self.is_occupied(position) {
                return false;
            }
            self.outside(position).player_id != get_player_id(player_id)
        }

        pub fn is_occupied_by_more_other(&mut self, player_id: i8, position: i8) -> bool {
            if !self.is_occupied_more(position) {
                return false;
            }
            self.outside(position).player_id != get_player_id(player_id)
        }
    }

    fn create_vector_of_pieces() -> Vec<Rc<RefCell<Piece>>> {
        let mut pieces = Vec::default();
        for i in 0..4 {
            pieces.push(Rc::new(RefCell::new(Piece::new(i))));
        }
        pieces
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
