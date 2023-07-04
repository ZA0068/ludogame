mod players {
    use board::Board;
    use dice::Dice;
    use pieces::Piece;
    
    // use std::fmt;
    use std::{cell::RefCell, rc::Rc};

    #[derive(PartialEq, Debug, Clone)]
    pub struct Player {
        id: i8,
        turn: bool,
        dice: Option<Rc<RefCell<Dice>>>,
        board: Rc<RefCell<Board>>,
        pieces: Vec<Rc<RefCell<Piece>>>,
    }

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Act {
        Move,
        Free,
        Kill,
        Join,
        Leave,
        Die,
        Goal,
        Safe,
        Starjump,
        Nothing,
    }

    // impl fmt::Display for Act {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         match *self {
    //             Act::Move => write!(f, "Move"),
    //             Act::Free => write!(f, "Free"),
    //             Act::Kill => write!(f, "Kill"),
    //             Act::Join => write!(f, "Join"),
    //             Act::Leave => write!(f, "Leave"),
    //             Act::Die => write!(f, "Die"),
    //             Act::Goal => write!(f, "Goal"),
    //             Act::Safe => write!(f, "Safe"),
    //             Act::Starjump => write!(f, "Starjump"),
    //             Act::Nothing => write!(f, "Nothing"),
    //         }
    //     }
    // }

    // #[derive(PartialEq, Debug, Clone)]
    // pub struct ActionSequence(Vec<Act>);

    // impl fmt::Display for ActionSequence {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         for (index, action) in self.0.iter().enumerate() {
    //             if index > 0 {
    //                 write!(f, ", ")?;
    //             }
    //             write!(f, "{}", action)?;
    //         }
    //         Ok(())
    //     }
    // }

    // Playstyles {
    // [x] aggro_play;
    // [x] fast_aggro_play
    // [x] random_play
    // [x] safe_play
    // [x] fast_play
    // }

    impl Player {
        pub fn new(
            player_id: i8,
            board: Rc<RefCell<Board>>,
            dice: Option<Rc<RefCell<Dice>>>,
        ) -> Player {
            let pieces = Self::init_pieces(board.clone(), player_id);
            let id = player_id;
            Player {
                id,
                turn: false,
                board,
                dice,
                pieces,
            }
        }

        fn init_pieces(board: Rc<RefCell<Board>>, player_id: i8) -> Vec<Rc<RefCell<Piece>>> {
            board.borrow_mut().home(player_id).pieces.clone()
        }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn piece(&self, piece_id: i8) -> Rc<RefCell<Piece>> {
            self.pieces[piece_id as usize].clone()
        }

        pub fn board(&self) -> &Rc<RefCell<Board>> {
            &self.board
        }

        pub fn take_dice(&mut self, dice: Rc<RefCell<Dice>>) {
            self.dice = Some(dice);
        }

        pub fn give_dice(&mut self) {
            self.dice = None;
        }

        // pub fn make_move(&mut self, piece_id: i8, dice_number: i8, choice: Act) {
        //     match choice {
        //         Act::Move => {
        //             self.move_piece(piece_id, dice_number);
        //             self.can_continue();
        //         }
        //         Act::Safe => {
        //             self.move_to_safety(piece_id, dice_number);
        //             self.can_continue();
        //         }
        //         Act::Starjump => {
        //             self.skip(piece_id, dice_number);
        //             self.can_continue();
        //         }
        //         Act::Goal => {
        //             self.goal(piece_id);
        //             self.my_turn();
        //         }
        //         Act::Free => {
        //             self.free_piece(piece_id);
        //             self.my_turn();
        //         }
        //         Act::Kill => {
        //             self.kill_piece(piece_id, dice_number);
        //             self.my_turn();
        //         }
        //         Act::Join => {
        //             self.join_piece(piece_id, dice_number);
        //             self.can_continue();
        //         }
        //         Act::Leave => {
        //             self.leave_piece(piece_id, dice_number);
        //             self.can_continue();
        //         }
        //         Act::Die => {
        //             self.die(piece_id);
        //             self.can_continue();
        //         }
        //         Act::Nothing => {
        //             self.turn = false;
        //         }
        //     }
        // }

        // pub fn goal(&mut self, piece_id: i8) {
        //     let old_position = self.piece(piece_id).borrow_mut().position();
        //     self.enter_goal(piece_id, old_position);
        // }

        // pub fn skip(&mut self, piece_id: i8, dice_number: i8) {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     self.starjump(piece_id, old_position, new_position);
        // }

        // pub fn kill_piece(&mut self, piece_id: i8, dice_number: i8) {
        //     if self.piece(piece_id).borrow().is_home() {
        //         let invincible_position = self.invincible_positions();
        //         self.send_other_piece_home(invincible_position);
        //         self.free_piece(piece_id);
        //     } else {
        //         let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //         self.kill_occupying_piece(piece_id, old_position, new_position);
        //     }
        // }

        // fn kill_occupying_piece(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     self.send_other_piece_home(new_position);
        //     let mut star_position: i8 = new_position;
        //     if self
        //         .is_star_occupied_by_others_or_more(old_position, new_position)
        //         .0
        //     {
        //         star_position = self.star_position(old_position, new_position);
        //         self.send_other_piece_home(star_position);
        //     }
        //     self.update_outside(piece_id, old_position, star_position);
        // }

        // fn send_other_piece_home(&mut self, position: i8) {
        //     let other_player_id = match self.board().borrow_mut().outside(position).player_id {
        //         Some(player_id) => player_id as i8,
        //         None => {
        //             return;
        //         }
        //     };
        //     let other_piece_vec = self.board().borrow_mut().outside(position).pieces.clone();
        //     for other_piece in other_piece_vec {
        //         let other_piece_id = other_piece.borrow_mut().id();
        //         self.board()
        //             .borrow_mut()
        //             .outside(position)
        //             .piece(other_piece_id)
        //             .borrow_mut()
        //             .dead();
        //         self.board()
        //             .borrow_mut()
        //             .move_into_home(other_player_id, other_piece_id, position);
        //     }
        // }

        // pub fn leave_piece(&mut self, piece_id: i8, dice_number: i8) {
        //     self.move_piece(piece_id, dice_number);
        //     for i in 0..4 {
        //         let piece = self.piece(i).clone();
        //         let pos = piece.borrow_mut().position();
        //         if piece.borrow_mut().is_home() || piece.borrow_mut().is_goal() || pos > 51 {
        //             continue;
        //         }

        //         if self.board().borrow_mut().is_globe(pos)
        //             || self.board().borrow_mut().is_occupied_more(pos)
        //             || (self.invincible_positions() == pos)
        //         {
        //             self.piece(i).borrow_mut().dangerous();
        //         } else {
        //             self.piece(i).borrow_mut().not_safe();
        //         }
        //     }
        // }

        // pub fn join_piece(&mut self, piece_id: i8, dice_number: i8) {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     let new_position = self.star_position(old_position, new_position);
        //     self.update_outside(piece_id, old_position, new_position);

        //     for i in 0..4 {
        //         let piece = self.piece(i);
        //         let pos = piece.borrow_mut().position();

        //         if pos == new_position {
        //             piece.borrow_mut().dangerous();
        //         }
        //     }
        // }

        // pub fn move_piece(&mut self, piece_id: i8, dice_number: i8) {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     self.update_piece(piece_id, old_position, new_position);
        // }

        // pub fn move_to_safety(&mut self, piece_id: i8, dice_number: i8) {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     self.enter_globe(piece_id, old_position, new_position);
        // }

        // fn update_position(&mut self, piece_id: i8, dice_number: i8) -> (i8, i8) {
        //     let old_position = self.piece(piece_id).borrow_mut().position();
        //     let new_position = old_position + dice_number;
        //     let new_position = self.correct_position_for_player_1_to_3(old_position, new_position);
        //     (old_position, new_position)
        // }

        // fn update_piece(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     let _ = self
        //         .try_enter_inside(piece_id, old_position, new_position)
        //         .or_else(|err| self.try_update_inside(piece_id, old_position, err))
        //         .or_else(|err| self.try_move_back(piece_id, old_position, err))
        //         .or_else(|err| self.try_enter_goal(piece_id, old_position, err))
        //         .or_else(|err| self.try_update_outside(piece_id, old_position, err));
        // }

        // pub fn try_enter_goal(
        //     &mut self,
        //     piece_id: i8,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<(), i8> {
        //     let goal_position = self.goal_positions(old_position, new_position);
        //     match goal_position {
        //         99 => {
        //             self.enter_goal(piece_id, old_position);
        //             Ok(())
        //         }
        //         _ => Err(new_position),
        //     }
        // }

        // pub fn try_update_inside(
        //     &mut self,
        //     piece_id: i8,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<(), i8> {
        //     match (self.id(), old_position, new_position) {
        //         (0, 52..=56, 52..=56)
        //         | (1, 57..=61, 57..=61)
        //         | (2, 62..=66, 62..=66)
        //         | (3, 67..=71, 67..=71) => {
        //             self.update_inside(piece_id, old_position, new_position);
        //             Ok(())
        //         }
        //         _ => Err(new_position),
        //     }
        // }

        // pub fn update_inside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     self.piece(piece_id).borrow_mut().set_position(new_position);
        //     self.board().borrow_mut().update_inside(
        //         self.id(),
        //         piece_id,
        //         old_position,
        //         new_position,
        //     );
        // }

        // pub fn star_position(&mut self, old_position: i8, new_position: i8) -> i8 {
        //     match (old_position, new_position) {
        //         (51, 5) | (0..=4, 5) => 11,
        //         (5..=10, 11) if self.id() != 1 => 18,
        //         (12..=17, 18) => 24,
        //         (18..=23, 24) if self.id() != 2 => 31,
        //         (25..=30, 31) => 37,
        //         (31..=36, 37) if self.id() != 3 => 44,
        //         (38..=43, 44) => 50,
        //         (44..=49, 50) if self.id() != 0 => 5,
        //         _ => new_position,
        //     }
        // }

        // pub fn starjump(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     let new_position = self.star_position(old_position, new_position);
        //     self.update_outside(piece_id, old_position, new_position)
        // }

        // pub fn enter_globe(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     self.piece(piece_id).borrow_mut().set_position(new_position);
        //     self.piece(piece_id).borrow_mut().dangerous();
        //     self.board().borrow_mut().update_outside(
        //         self.id(),
        //         piece_id,
        //         old_position,
        //         new_position,
        //     );
        // }

        // fn try_update_outside(
        //     &mut self,
        //     piece_id: i8,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<(), i8> {
        //     let new_position = self
        //         .correct_position(old_position, new_position)
        //         .unwrap_or(new_position);
        //     self.update_outside(piece_id, old_position, new_position);
        //     Ok(())
        // }

        // fn correct_position_for_player_1_to_3(&mut self, old_position: i8, new_position: i8) -> i8 {
        //     match (self.id(), old_position, new_position) {
        //         (1..=3, 0..=51, 52..=58) => new_position - 52,
        //         _ => new_position,
        //     }
        // }

        // fn update_outside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     self.piece(piece_id).borrow_mut().set_position(new_position);
        //     self.piece(piece_id).borrow_mut().not_safe();
        //     self.board().borrow_mut().update_outside(
        //         self.id(),
        //         piece_id,
        //         old_position,
        //         new_position,
        //     );
        // }

        // fn try_move_back(
        //     &mut self,
        //     piece_id: i8,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<(), i8> {
        //     let subtract = match self.subtract_if_overshoot(old_position, new_position) {
        //         Ok(value) => value,
        //         Err(value) => return value,
        //     };
        //     self.move_back(old_position, new_position, piece_id, subtract);
        //     Ok(())
        // }

        // fn subtract_if_overshoot(
        //     &mut self,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<i8, Result<(), i8>> {
        //     let subtract: i8 = match (self.id, old_position, new_position) {
        //         (0, 52..=56, 58..=62) => 57,
        //         (1, 57..=61, 63..=67) => 62,
        //         (2, 62..=66, 68..=72) => 67,
        //         (3, 67..=71, 73..=77) => 72,
        //         _ => return Err(Err(new_position)),
        //     };
        //     Ok(subtract)
        // }

        // fn move_back(&mut self, old_position: i8, new_position: i8, piece_id: i8, subtract: i8) {
        //     let new_position = subtract - (new_position - subtract);
        //     self.piece(piece_id).borrow_mut().set_position(new_position);
        //     self.board().borrow_mut().update_inside(
        //         self.id(),
        //         piece_id,
        //         old_position,
        //         new_position,
        //     );
        // }

        // fn goal_positions(&mut self, old_position: i8, new_position: i8) -> i8 {
        //     match (self.id(), old_position, new_position) {
        //         (_, _, 99)
        //         | (0, 50, 56)
        //         | (0, 52..=56, 57)
        //         | (0, 44..=49, 50)
        //         | (0, 57, _)
        //         | (1, 11, 17)
        //         | (1, 57..=61, 62)
        //         | (1, 5..=10, 11)
        //         | (1, 62, _)
        //         | (2, 24, 30)
        //         | (2, 62..=66, 67)
        //         | (2, 18..=23, 24)
        //         | (2, 67, _)
        //         | (3, 37, 43)
        //         | (3, 67..=71, 72)
        //         | (3, 31..=36, 37)
        //         | (3, 72, _) => 99,
        //         _ => new_position,
        //     }
        // }

        // fn try_enter_inside(
        //     &mut self,
        //     piece_id: i8,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<(), i8> {
        //     let new_position = match self.correct_position(old_position, new_position) {
        //         Ok(value) => value,
        //         Err(value) => return value,
        //     };
        //     self.enter_inside(piece_id, old_position, new_position);
        //     Ok(())
        // }

        // fn correct_position(
        //     &mut self,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> Result<i8, Result<(), i8>> {
        //     let new_position: i8 = match (self.id, old_position, new_position) {
        //         (0, 45..=51, 51..=55) => new_position + 1,
        //         (1, 6..=12, 12..=16) => new_position + 45,
        //         (2, 19..=25, 25..=29) => new_position + 37,
        //         (3, 32..=38, 38..=42) => new_position + 29,
        //         _ => return Err(Err(new_position)),
        //     };
        //     Ok(new_position)
        // }

        // fn enter_inside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
        //     self.piece(piece_id).borrow_mut().set_position(new_position);
        //     self.piece(piece_id).borrow_mut().safe();
        //     self.board()
        //         .borrow_mut()
        //         .move_inside(self.id(), piece_id, old_position, new_position);
        // }

        // fn enter_goal(&mut self, piece_id: i8, old_position: i8) {
        //     self.piece(piece_id).borrow_mut().goal();
        //     self.board()
        //         .borrow_mut()
        //         .enter_goal(self.id(), piece_id, old_position);
        // }

        // pub fn free_piece(&mut self, piece_id: i8) {
        //     let position = self.invincible_positions();
        //     self.piece(piece_id).borrow_mut().free();
        //     self.piece(piece_id).borrow_mut().set_position(position);
        //     self.board()
        //         .borrow_mut()
        //         .move_from_home(self.id(), piece_id, position);
        // }

        // fn invincible_positions(&mut self) -> i8 {
        //     match self.id() {
        //         0 => 0,
        //         1 => 13,
        //         2 => 26,
        //         3 => 39,
        //         _ => panic!("invalid move!"),
        //     }
        // }

        pub fn roll_dice(&mut self) -> i8 {
            if let Some(dice) = &self.dice {
                dice.borrow_mut().roll();
                dice.borrow_mut().get_value()
            } else {
                0
            }
        }

        // pub fn die(&mut self, piece_id: i8) {
        //     let old_position = self.piece(piece_id).borrow_mut().position();
        //     self.piece(piece_id).borrow_mut().dead();
        //     self.board()
        //         .borrow_mut()
        //         .move_into_home(self.id(), piece_id, old_position);
        // }

        // pub fn is_player_turn(&self) -> bool {
        //     self.turn
        // }

        // pub fn my_turn(&mut self) {
        //     self.turn = true;
        // }

        // pub fn can_continue(&mut self) {
        //     self.turn = self.dice.clone().unwrap().borrow_mut().get_value() == 6;
        // }

        // pub fn valid_moves(&mut self, piece_id: i8, dice: i8) -> (bool, bool) {
        //     if piece_id > 3 {
        //         return (false, false);
        //     }
        //     let binding = self.piece(piece_id);
        //     let binding = binding.borrow_mut();
        //     let is_home = binding.is_home();
        //     let is_goal = binding.is_goal();
        //     let is_valid = match (is_goal, is_home, dice) {
        //         (true, _, _) | (_, true, 1..=5) => false,
        //         (_, true, 6) => true,
        //         (false, false, 1..=6) => true,
        //         _ => false,
        //     };
        //     (is_home, is_valid)
        // }

        // pub fn make_random_choice(&mut self, dice_number: i8, action: Act) -> (Act, i8) {
        //     let action_vector = self.generate_action_and_piece_id_vector(dice_number, action);
        //     self.select_random_piece(action_vector)
        // }

        // pub fn make_ordered_choice(
        //     &mut self,
        //     dice_number: i8,
        //     action: Act,
        //     take_closest: bool,
        // ) -> (Act, i8) {
        //     let action_vector = self.generate_action_and_piece_id_vector(dice_number, action);
        //     self.select_ordered_piece(action_vector, take_closest)
        // }

        // pub fn select_ordered_piece(
        //     &mut self,
        //     action_vector: Vec<(Act, i8)>,
        //     take_closest: bool,
        // ) -> (Act, i8) {
        //     if action_vector.is_empty() {
        //         return (Act::Nothing, 0);
        //     }
        //     let mut ordered_vector = action_vector;
        //     if take_closest {
        //         ordered_vector.sort_by(|a, b| self.cmp_positions_asc(a, b));
        //     } else {
        //         ordered_vector.sort_by(|a, b| self.cmp_positions_desc(a, b));
        //     }
        //     ordered_vector[0]
        // }

        // fn cmp_positions_asc(&mut self, a: &(Act, i8), b: &(Act, i8)) -> std::cmp::Ordering {
        //     self.piece(a.1)
        //         .borrow_mut()
        //         .position()
        //         .cmp(&self.piece(b.1).borrow_mut().position())
        // }

        // fn cmp_positions_desc(&mut self, a: &(Act, i8), b: &(Act, i8)) -> std::cmp::Ordering {
        //     self.piece(a.1)
        //         .borrow_mut()
        //         .position()
        //         .cmp(&self.piece(b.1).borrow_mut().position())
        //         .reverse()
        // }

        // pub fn generate_action_and_piece_id_vector(
        //     &mut self,
        //     dice_number: i8,
        //     action: Act,
        // ) -> Vec<(Act, i8)> {
        //     let mut action_vector: Vec<(Act, i8)> = Vec::new();
        //     for idx in 0..4 {
        //         let reaction = self.valid_choices(idx, dice_number, action);
        //         if reaction != Act::Nothing {
        //             action_vector.push((reaction, idx));
        //         }
        //     }
        //     action_vector
        // }

        // pub fn valid_choices(&mut self, piece_id: i8, dice_number: i8, action: Act) -> Act {
        //     let (is_home, is_valid) = self.valid_moves(piece_id, dice_number);
        //     match (action, is_home, is_valid) {
        //         (Act::Free, true, true) => self.try_to_free(),
        //         (Act::Move, false, true) => self.try_to_move(piece_id, dice_number),
        //         (Act::Join, false, true) => self.try_to_join(piece_id, dice_number),
        //         (Act::Kill, _, true) => self.try_to_kill(piece_id, dice_number),
        //         (Act::Die, false, true) => self.try_to_die(piece_id, dice_number),
        //         (Act::Goal, false, true) => self.try_to_win(piece_id, dice_number),
        //         (Act::Leave, false, true) => self.try_to_leave(piece_id, dice_number),
        //         (Act::Safe, false, true) => self.try_to_safe(piece_id, dice_number),
        //         (Act::Skip, false, true) => self.try_to_skip(piece_id, dice_number),
        //         _ => Act::Nothing,
        //     }
        // }

        // pub fn try_to_free(&mut self) -> Act {
        //     let invincible_position = self.invincible_positions();
        //     let is_invincible_space_occupied_by_others =
        //         self.is_occupied_by_others_or_more(invincible_position).0;
        //     if !is_invincible_space_occupied_by_others {
        //         return Act::Free;
        //     }
        //     Act::Nothing
        // }

        // pub fn try_to_skip(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     if new_position > 51 || old_position > 51 {
        //         return Act::Nothing;
        //     }
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     let is_occupied = self.is_occupied_or_more(new_position, new_position).0;
        //     let is_star_occupied = self.is_star_occupied_or_more(old_position, new_position).0;
        //     if is_star && !(is_occupied || is_star_occupied) {
        //         return Act::Skip;
        //     }
        //     Act::Nothing
        // }

        // pub fn try_to_safe(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     if new_position > 51 || old_position > 51 {
        //         return Act::Nothing;
        //     }
        //     let is_globe = self.board().borrow_mut().is_globe(new_position);
        //     let is_occupied_by_others = self.is_occupied_by_others_or_more(new_position).0;
        //     if is_globe && !is_occupied_by_others {
        //         return Act::Safe;
        //     }
        //     Act::Nothing
        // }

        // pub fn try_to_leave(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);

        //     if new_position > 51 || old_position > 51 {
        //         return Act::Nothing;
        //     }
        //     let is_globe = self.board().borrow_mut().is_globe(new_position);
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     let (is_occupied_new, is_occupied_old) =
        //         self.is_occupied_or_more(new_position, old_position);
        //     if !(is_globe || is_star || is_occupied_new) && is_occupied_old {
        //         return Act::Leave;
        //     }
        //     Act::Nothing
        // }

        // pub fn try_to_move(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     let is_goal = self.goal_positions(old_position, new_position) == 99;
        //     if is_goal {
        //         return Act::Nothing;
        //     }
        //     if new_position > 51 || old_position > 51 {
        //         return Act::Move;
        //     }
        //     let is_globe = self.board().borrow_mut().is_globe(new_position);
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     let (is_occupied_old, is_occupied_new) =
        //         self.is_occupied_or_more(new_position, old_position);
        //     if is_globe || is_star || is_occupied_old || is_occupied_new {
        //         Act::Nothing
        //     } else {
        //         Act::Move
        //     }
        // }

        // fn is_occupied_or_more(
        //     &mut self,
        //     occupying_position: i8,
        //     multi_occupying_position: i8,
        // ) -> (bool, bool) {
        //     let is_occupied_more = self
        //         .board()
        //         .borrow_mut()
        //         .is_occupied_more(multi_occupying_position);
        //     let is_occupied = self.board().borrow_mut().is_occupied(occupying_position);
        //     (is_occupied, is_occupied_more)
        // }

        // pub fn try_to_join(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     if new_position > 51 {
        //         return Act::Nothing;
        //     }
        //     let occupied_by_self = self.is_occupied_by_self_or_more(new_position).0;
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     let occupied_star_by_self = self
        //         .is_star_occupied_by_self_or_more(old_position, new_position)
        //         .0;
        //     if (occupied_by_self && !is_star) || occupied_star_by_self {
        //         return Act::Join;
        //     }
        //     Act::Nothing
        // }

        // fn is_occupied_by_self_or_more(&mut self, new_position: i8) -> (bool, bool) {
        //     let occupied_by_self = self
        //         .board()
        //         .borrow_mut()
        //         .is_occupied_self(self.id(), new_position);
        //     let occupied_by_self_more = self
        //         .board()
        //         .borrow_mut()
        //         .is_occupied_by_more_self(self.id(), new_position);
        //     (occupied_by_self, occupied_by_self_more)
        // }

        // pub fn try_to_die(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     if new_position > 51 || old_position > 51 {
        //         return Act::Nothing;
        //     }
        //     let (occupied_by_other, occupied_by_other_more) =
        //         self.is_occupied_by_others_or_more(new_position);
        //     let occupied_by_other_more_star = self
        //         .is_star_occupied_by_others_or_more(old_position, new_position)
        //         .1;
        //     let is_globe = self.board().borrow_mut().is_globe(new_position);
        //     let is_invincible = self.board().borrow_mut().is_invincible(new_position);
        //     let is_dangerous = occupied_by_other && (is_globe || is_invincible);

        //     if occupied_by_other_more || occupied_by_other_more_star || is_dangerous {
        //         return Act::Die;
        //     }
        //     Act::Nothing
        // }

        // fn is_occupied_by_others_or_more(&mut self, new_position: i8) -> (bool, bool) {
        //     let occupied_by_other_more = self
        //         .board()
        //         .borrow_mut()
        //         .is_occupied_by_more_other(self.id(), new_position);
        //     let occupied_by_other = self
        //         .board()
        //         .borrow_mut()
        //         .is_occupied_by_other(self.id(), new_position);
        //     (occupied_by_other, occupied_by_other_more)
        // }

        // pub fn try_to_win(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     if self.goal_positions(old_position, new_position) == 99 {
        //         return Act::Goal;
        //     }
        //     Act::Nothing
        // }

        // pub fn try_to_kill(&mut self, piece_id: i8, dice_number: i8) -> Act {
        //     let invincible_position = self.invincible_positions();
        //     let invincikill = self.is_occupied_by_others_or_more(invincible_position).0
        //         && self.piece(piece_id).borrow_mut().is_home();
        //     if invincikill {
        //         return Act::Kill;
        //     }
        //     let (old_position, new_position) = self.update_position(piece_id, dice_number);
        //     if new_position > 51 || old_position > 51 {
        //         return Act::Nothing;
        //     }

        //     let (occupied_by_other, occupied_by_other_more) =
        //         self.is_occupied_by_others_or_more(new_position);
        //     let (occupied_by_other_star, occupied_by_other_more_star) =
        //         self.is_star_occupied_by_others_or_more(old_position, new_position);
        //     let are_others_invincible = self.board().borrow().is_invincible(new_position);
        //     let is_globe = self.board().borrow().is_globe(new_position);
        //     let is_dangerous = (is_globe || are_others_invincible) && occupied_by_other;
        //     if (occupied_by_other && !(occupied_by_other_more || is_dangerous))
        //         || (occupied_by_other_star && !occupied_by_other_more_star)
        //     {
        //         return Act::Kill;
        //     }
        //     Act::Nothing
        // }

        // pub fn is_star_occupied_by_self_or_more(
        //     &mut self,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> (bool, bool) {
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     if is_star {
        //         let star_position = self.star_position(old_position, new_position);
        //         let is_star_occupied_by_self = self
        //             .board()
        //             .borrow_mut()
        //             .is_occupied_self(self.id(), star_position);
        //         let is_star_occupied_by_more_selves = self
        //             .board()
        //             .borrow_mut()
        //             .is_occupied_by_more_self(self.id(), star_position);
        //         (is_star_occupied_by_self, is_star_occupied_by_more_selves)
        //     } else {
        //         (false, false)
        //     }
        // }

        // pub fn is_star_occupied_or_more(
        //     &mut self,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> (bool, bool) {
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     if is_star {
        //         let star_position = self.star_position(old_position, new_position);
        //         let is_star_occupied = self.board().borrow_mut().is_occupied(star_position);
        //         let is_star_occupied_more =
        //             self.board().borrow_mut().is_occupied_more(star_position);
        //         (is_star_occupied, is_star_occupied_more)
        //     } else {
        //         (false, false)
        //     }
        // }

        // pub fn is_star_occupied_by_others_or_more(
        //     &mut self,
        //     old_position: i8,
        //     new_position: i8,
        // ) -> (bool, bool) {
        //     let is_star = self.board().borrow_mut().is_star(new_position);
        //     match is_star {
        //         true => {
        //             let star_position = self.star_position(old_position, new_position);
        //             let occupied_by_other_star = self
        //                 .board()
        //                 .borrow_mut()
        //                 .is_occupied_by_other(self.id(), star_position);
        //             let occupied_by_other_more_star = self
        //                 .board()
        //                 .borrow_mut()
        //                 .is_occupied_by_more_other(self.id(), star_position);
        //             (occupied_by_other_star, occupied_by_other_more_star)
        //         }
        //         _ => (false, false),
        //     }
        // }

        // pub fn random_play(&mut self, actions: Vec<Act>) {
        //     while self.is_player_turn() && !self.is_finished() {
        //         let dice_number = self.roll_dice();
        //         let choices = self.create_choice_vector_random(actions.clone(), dice_number);
        //         self.choose_random_move(choices, dice_number);
        //     }
        // }

        // fn create_choice_vector_random(
        //     &mut self,
        //     actions: Vec<Act>,
        //     dice_number: i8,
        // ) -> Vec<(Act, i8)> {
        //     let mut choices: Vec<(Act, i8)> = vec![];
        //     for action in actions {
        //         let valid_choice = self.make_random_choice(dice_number, action);
        //         if valid_choice.0 != Act::Nothing {
        //             choices.push(valid_choice);
        //         }
        //     }
        //     choices
        // }

        // fn choose_random_move(&mut self, choices: Vec<(Act, i8)>, dice_number: i8) {
        //     if let Some(choice) = choices.choose(&mut rand::thread_rng()) {
        //         self.make_move(choice.1, dice_number, choice.0);
        //     } else {
        //         self.make_move(0, dice_number, Act::Nothing);
        //     }
        // }

        // pub fn ordered_play(&mut self, actions: Vec<Act>, take_closest: bool) {
        //     while self.is_player_turn() && !self.is_finished() {
        //         let dice_number = self.roll_dice();
        //         let choices =
        //             self.create_choice_vector_ordered(actions.clone(), dice_number, take_closest);
        //         self.choose_prefered_move(choices, dice_number);
        //     }
        // }

        // fn choose_prefered_move(&mut self, choices: Vec<(Act, i8)>, dice_number: i8) {
        //     if let Some(choice) = choices.first() {
        //         self.make_move(choice.1, dice_number, choice.0);
        //     } else {
        //         self.make_move(0, dice_number, Act::Nothing);
        //     }
        // }

        // fn create_choice_vector_ordered(
        //     &mut self,
        //     actions: Vec<Act>,
        //     dice_number: i8,
        //     take_closest: bool,
        // ) -> Vec<(Act, i8)> {
        //     let mut choices: Vec<(Act, i8)> = vec![];
        //     for action in actions {
        //         let valid_choice = self.make_ordered_choice(dice_number, action, take_closest);
        //         if valid_choice.0 != Act::Nothing {
        //             choices.push(valid_choice);
        //         }
        //     }
        //     choices
        // }

        // pub fn is_finished(&self) -> bool {
        //     self.pieces.iter().all(|piece| piece.borrow_mut().is_goal())
        // }

        // pub fn print_status(&mut self) {
        //     println!("Player {} pieces:", self.id());
        //     println!(
        //         "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
        //         self.piece(0).borrow().position(),
        //         self.piece(1).borrow_mut().position(),
        //         self.piece(2).borrow_mut().position(),
        //         self.piece(3).borrow_mut().position()
        //     );
        // }

        // pub fn select_random_piece(&mut self, action_vector: Vec<(Act, i8)>) -> (Act, i8) {
        //     *action_vector
        //         .choose(&mut rand::thread_rng())
        //         .unwrap_or(&(Act::Nothing, 0))
        // }
    }
}

pub use players::{Act, Player};
