mod players {
    use board::Board;
    use dice::Dice;
    use pieces::Piece;
    use rand::prelude::SliceRandom;
    use prettytable::{Table, row};
    // use std::fmt;


    use std::{cell::RefCell, rc::Rc};

    #[derive(PartialEq, Debug, Clone)]
    pub struct Player {
        id: i8,
        color: Color,
        turn: bool,
        dice: Option<Dice>,
        board: Rc<RefCell<Board>>,
        pieces: Vec<Rc<RefCell<Piece>>>,
        pub action: (Act, i8, i8),
        pub old_position: i8,
        pub new_position: i8,
    }

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Color {
        Green,
        Yellow,
        Blue,
        Red,
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
            board: Rc<RefCell<Board>>
        ) -> Player {
            let pieces = Self::init_pieces(board.clone(), player_id);
            let id = player_id;
            let color = get_color_from_player_id(player_id);
            Player {
                id,
                color,
                turn: false,
                board,
                dice: None,
                pieces,
                action: (Act::Nothing, player_id, 57),
                old_position: -1,
                new_position: -1,
            }
        }

        fn init_pieces(board: Rc<RefCell<Board>>, player_id: i8) -> Vec<Rc<RefCell<Piece>>> {
            board.borrow_mut().home(player_id).pieces.clone()
        }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn piece(&self, piece_id: i8) -> Rc<RefCell<Piece>> {
            if piece_id > 3 {
                panic!("The highest piece id cannot be higher than 3, else you have more than 4 pieces!");
            }
            self.pieces[piece_id as usize].clone()
        }

        pub fn board(&self) -> &Rc<RefCell<Board>> {
            &self.board
        }

        pub fn take_dice(&mut self, dice: Dice) {
            self.dice = Some(dice);
        }

        pub fn give_dice(&mut self) {
            self.dice = None;
        }

        pub fn make_move(&mut self, piece_id: i8, dice_number: i8, choice: Act) {
            match choice {
                Act::Move => {
                    self.move_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Safe => {
                    self.save_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Starjump => {
                    self.starjump_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Goal => {
                    self.win_piece(piece_id, dice_number);
                    self.my_turn();
                }
                Act::Free => {
                    self.free_piece(piece_id);
                    self.my_turn();
                }
                Act::Kill => {
                    self.kill_piece(piece_id, dice_number);
                    self.my_turn();
                }
                Act::Join => {
                    self.join_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Leave => {
                    self.leave_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Die => {
                    self.die_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Nothing => {
                    self.turn = false;
                }
            }
        }

        pub fn win_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            if self.is_occupied_by_others(self.new_position).0 {
                self.send_other_piece_home(self.new_position);
            }
            match self.goal_positions(self.old_position, self.new_position) {
                99 => self.enter_goal(piece_id, self.old_position),
                _ => panic!("New position is not a goal!"),
            };
        }

        pub fn send_other_piece_home(&mut self, new_position: i8) {
            let pieces = self.board().borrow_mut().outside(new_position).pieces.clone();
            for piece in pieces {
                let position = piece.borrow().position();
                piece.borrow_mut().dead();
                let (other_player_id, other_piece_id) = get_piece_and_player_id(piece);
                self.board().borrow_mut().move_into_home(other_player_id, other_piece_id, position);
            }
        }

        pub fn kill_piece(&mut self, piece_id: i8, dice_number: i8) {
            if self.piece(piece_id).borrow().is_home() {
                let invincible_position = self.invincible_positions(self.id());
                self.send_other_piece_home(invincible_position);
                self.free_piece(piece_id);
            } else {
                self.update_position(piece_id, dice_number);
                self.kill(piece_id, self.old_position, self.new_position);
                if self.is_goal_position() {
                    self.enter_goal(piece_id, self.new_position);
                }
            }
        }

        pub fn kill(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            self.send_other_piece_home(new_position);
            let occupied_starspaces = self.is_star_occupied_by_others(old_position, new_position);
            match occupied_starspaces {
                (true, false) => {
                    let star_position = self.star_position(old_position, new_position);
                    self.send_other_piece_home(star_position);
                    self.starjump(piece_id, old_position, new_position);
                }
                (_, true) => {
                    self.die(piece_id);
                }
                _ => {
                    self.join(piece_id, old_position, new_position);
                }
            }
        }

        pub fn leave_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            self.leave(piece_id, self.old_position, self.new_position);
        }

        pub fn leave(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            self.update_piece(piece_id, old_position, new_position);
        }

        pub fn join_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            self.join(piece_id, self.old_position, self.new_position);
        }

        pub fn join(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            let new_position = self.star_position(old_position, new_position);
            self.update_outside(piece_id, old_position, new_position);
        }

        pub fn move_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            self.update_piece(piece_id, self.old_position, self.new_position);
        }

        pub fn save_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            match (self.id, self.old_position, self.new_position) {
                (0, 45..=50, 51..=56) => {
                    self.correct_position();
                    self.enter_inside(piece_id, self.old_position, self.new_position);
                },
                // (1, 6..=11, 12..=17) => self.enter_inside(piece_id, self.old_position, self.new_position),
                // (2, 62..=66, 62..=72) => self.enter_inside(piece_id, self.old_position, self.new_position),
                // (3, 67..=71, 67..=77) => self.enter_inside(piece_id, self.old_position, self.new_position),
                _ => {
                    if !self.board().borrow_mut().is_globe(self.new_position) {
                        panic!("New position is not a Globe!");	
                    };
                    self.enter_globe(piece_id, self.old_position, self.new_position);
                },
            };
        }

        pub fn update_position(&mut self, piece_id: i8, dice_number: i8) {
            self.old_position = self.piece(piece_id).borrow_mut().position();
            if self.piece(piece_id).borrow().is_home() {
                self.new_position = self.invincible_positions(self.id());
            } else {
                self.new_position = self.old_position + dice_number;
            }
        }

        pub fn update_piece(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            match (self.id(), old_position, new_position) {
                (0, 52..=56, 52..=62) |
                (1, 57..=61, 57..=67) |
                (2, 62..=66, 62..=72) |
                (3, 67..=71, 67..=77) => self.update_inside(piece_id, old_position, new_position),
                (0, 45..=50, 51..=56) |
                (1,  6..=11, 12..=17) |
                (2, 19..=24, 25..=30) |
                (3, 32..=37, 38..=43) => {
                    self.correct_position();
                    self.enter_inside(piece_id, self.old_position, self.new_position);
                },
                _ => self.update_outside(piece_id, old_position, new_position),
            };
        }

        pub fn update_inside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            let new_position = self.move_back_if_overshoot(old_position, new_position);
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.board().borrow_mut().update_inside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        pub fn star_position(&mut self, old_position: i8, new_position: i8) -> i8 {
            match (old_position, new_position) {
                (51, 5) | (0..=4, 5) => 11,
                (5..=10, 11) if self.id() != 1 => 18,
                (12..=17, 18) => 24,
                (18..=23, 24) if self.id() != 2 => 31,
                (25..=30, 31) => 37,
                (31..=36, 37) if self.id() != 3 => 44,
                (38..=43, 44) => 50,
                (44..=49, 50) if self.id() != 0 => 5,
                _ => new_position,
            }
        }

        pub fn starjump_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            match self.goal_positions(self.old_position, self.new_position) {
                99 => self.enter_goal(piece_id, self.old_position),
                _ => self.starjump(piece_id, self.old_position, self.new_position),
            };
            
        }

        pub fn starjump(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            self.new_position = self.star_position(old_position, new_position);
            self.update_outside(piece_id, old_position, self.new_position)
        }

        pub fn enter_globe(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.board().borrow_mut().update_outside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        pub fn circumvent_player_0(&mut self, old_position: i8, new_position: i8) -> i8 {
            match (self.id(), old_position, new_position) {
                (1..=3, 0..=51, 52..=57) => new_position - 52,
                _ => new_position,
            }
        }

        pub fn update_outside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            let new_position = self.circumvent_player_0(old_position, new_position);
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.board().borrow_mut().update_outside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        pub fn move_back_if_overshoot(
            &mut self,
            old_position: i8,
            new_position: i8,
        ) -> i8 {
            let subtract = self.get_subtraction_position(old_position, new_position);
            subtract - (new_position - subtract)
        }

        pub fn get_subtraction_position(&mut self, old_position: i8, new_position: i8) -> i8 {
            match (self.id, old_position, new_position) {
                (0, 52..=56, 58..=62) => 57,
                (1, 57..=61, 63..=67) => 62,
                (2, 62..=66, 68..=72) => 67,
                (3, 67..=71, 73..=77) => 72,
                _ => new_position,
            }
        }

        fn goal_positions(&mut self, old_position: i8, new_position: i8) -> i8 {
            match (self.id(), old_position, new_position) {
                (_, _, 99)
                | (0, 50, 56)
                | (0, 52..=56, 57)
                | (0, 44..=49, 50)
                | (0, 57, _)
                | (1, 11, 17)
                | (1, 57..=61, 62)
                | (1, 5..=10, 11)
                | (1, 62, _)
                | (2, 24, 30)
                | (2, 62..=66, 67)
                | (2, 18..=23, 24)
                | (2, 67, _)
                | (3, 37, 43)
                | (3, 67..=71, 72)
                | (3, 31..=36, 37)
                | (3, 72, _) => 99,
                _ => new_position,
            }
        }

        pub fn correct_position(&mut self) {
            self.new_position = match (self.id, self.old_position, self.new_position) {
                (0, 45..=50, 51..=55) => self.new_position + 1,
                (1, 6..=11, 12..=16)  => self.new_position + 45,
                (2, 19..=24, 25..=29) => self.new_position + 37,
                (3, 32..=37, 38..=42) => self.new_position + 29,
                _ => self.new_position,
            };
        }

        pub fn enter_inside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.board()
                .borrow_mut()
                .move_inside(self.id(), piece_id, old_position, new_position);
        }

        pub fn enter_goal(&mut self, piece_id: i8, old_position: i8) {
            self.piece(piece_id).borrow_mut().goal();
            self.board()
                .borrow_mut()
                .enter_goal(self.id(), piece_id, old_position);
        }

        pub fn free_piece(&mut self, piece_id: i8) {
            self.new_position = self.invincible_positions(self.id());
            self.piece(piece_id).borrow_mut().free();
            self.piece(piece_id).borrow_mut().set_position(self.new_position);
            self.board()
                .borrow_mut()
                .move_from_home(self.id(), piece_id, self.new_position);
        }

        fn invincible_positions(&self, id: i8) -> i8 {
            match id {
                0 => 0,
                1 => 13,
                2 => 26,
                3 => 39,
                _ => panic!("invalid player ID!"),
            }
        }

        pub fn roll_dice(&mut self) -> i8 {
            if let Some(dice) = &mut self.dice {
                dice.roll();
                dice.get_value()
            } else {
                0
            }
        }      

        pub fn die_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.update_position(piece_id, dice_number);
            if self.is_occupied_by_others(self.new_position).0 & !self.is_other_piece_invincible() {
                self.send_other_piece_home(self.new_position);
            }
            self.die(piece_id);
        }

        fn is_other_piece_invincible(&mut self) -> bool {
            self.invincible_positions(self.get_other_player_id()) == self.new_position
        }

        pub fn die(&mut self, piece_id: i8) {
            let old_position = self.piece(piece_id).borrow_mut().position();
            self.piece(piece_id).borrow_mut().dead();
            self.board()
                .borrow_mut()
                .move_into_home(self.id(), piece_id, old_position);
        }

        pub fn is_player_turn(&self) -> bool {
            self.turn
        }

        pub fn my_turn(&mut self) {
            self.turn = true;
        }

        pub fn can_continue(&mut self) {
            if let Some(dice) = &mut self.dice {
                self.turn = dice.get_value() == 6;
            }
        }

        pub fn valid_moves(&mut self, piece_id: i8, dice: i8) -> bool {
            if piece_id > 3 {
                return false;
            }
            let binding = self.piece(piece_id);
            let binding = binding.borrow_mut();
            let is_home = binding.is_home();
            let is_goal = binding.is_goal();
            matches!((is_goal, is_home, dice), (_, true, 6) | (false, false, 1..=6))
        }

        pub fn play_random(&mut self, actions: Vec<Act>) {
                let dice_number = self.roll_dice();
                let movesets = self.generate_vector_of_random_actions(actions, dice_number);
                self.action = self.select_random_piece(movesets);
                println!("-------------------");
                println!("prior play:");
                self.print_status();
                self.make_move( self.action.1, dice_number,  self.action.0);
                println!("-------------------");
                println!("posterior play:");
                self.print_status();
                println!("\n");
        }

        pub fn generate_vector_of_random_actions(&mut self, actions: Vec<Act>, dice_number: i8) ->  Vec<(Act, i8, i8)> {
            let mut movesets: Vec<(Act, i8, i8)> = Vec::new();
            for action in actions {
                let mut action_vector = self.generate_action_vector(dice_number, action);
                movesets.append(&mut action_vector);
            }
            movesets
        }

        pub fn select_random_piece(&mut self, action_vector: Vec<(Act, i8, i8)>) -> (Act, i8, i8) {
            *action_vector
                .choose(&mut rand::thread_rng())
                .unwrap_or(&(Act::Nothing, self.id, 57))
        }

        pub fn play_ordered(&mut self, actions: Vec<Act>, take_nearest_piece: bool) {
            let dice_number = self.roll_dice();
            let movesets = self.generate_vector_of_ordered_actions(actions, dice_number, take_nearest_piece);
            self.action = movesets.first().copied().unwrap_or((Act::Nothing, self.id, 57));
            println!("-------------------");
            println!("prior play");
            self.print_status();
            self.make_move(self.action.1, dice_number, self.action.0);
            println!("-------------------");
            println!("posterior play");
            self.print_status();
        }

        fn generate_vector_of_ordered_actions(&mut self, actions: Vec<Act>, dice_number: i8, take_nearest_piece: bool) -> Vec<(Act, i8, i8)> {
            let mut movesets: Vec<(Act, i8, i8)> = Vec::new();
            for action in actions {
                let moveset = self.make_ordered_choice(dice_number, action, take_nearest_piece);
                if moveset.0 != Act::Nothing {
                movesets.push(moveset);
                }
            }
            movesets
        }

        pub fn make_ordered_choice(
            &mut self,
            dice_number: i8,
            action: Act,
            take_nearest_piece: bool,
        ) -> (Act, i8, i8) {
            let action_vector = self.generate_action_vector(dice_number, action);
            self.select_ordered_piece(action_vector, take_nearest_piece)
        }

        pub fn select_ordered_piece(
            &mut self,
            mut action_vector: Vec<(Act, i8, i8)>,
            take_closest: bool,
        ) -> (Act, i8, i8) {
            action_vector.sort_by(|a, b| self.compare_heuristics(a, b, take_closest));
            match action_vector.first() {
                Some(&first_element) => first_element,
                None => (Act::Nothing, self.id, 57)
            }
        }
        
        fn compare_heuristics(
            &self,
            a: &(Act, i8, i8),
            b: &(Act, i8, i8),
            ascending: bool,
        ) -> std::cmp::Ordering {
            let order = a.2.cmp(&b.2);
            if ascending {
                order
            } else {
                order.reverse()
            }
        }

        pub fn generate_action_vector(
            &mut self,
            dice_number: i8,
            act: Act,
        ) -> Vec<(Act, i8, i8)> {
            let mut action_vector: Vec<(Act, i8, i8)> = Vec::new();
            for piece_id in 0..4 {
                let action = self.valid_choices(piece_id, dice_number, act);
                if action != Act::Nothing {
                    let heuristic = self.get_heuristics(piece_id);
                    action_vector.push((action, piece_id, heuristic));
                }
            }
            action_vector
        }

        pub fn valid_choices(&mut self, piece_id: i8, dice_number: i8, action: Act) -> Act {
            if !self.valid_moves(piece_id, dice_number)
            {
                return Act::Nothing;
            }
            match action {
                    Act::Free => self.try_to_free(piece_id, dice_number),
                    Act::Move => self.try_to_move(piece_id, dice_number),
                    Act::Join => self.try_to_join(piece_id, dice_number),
                    Act::Kill => self.try_to_kill(piece_id, dice_number),
                    Act::Die => self.try_to_die(piece_id, dice_number),
                    Act::Goal => self.try_to_win(piece_id, dice_number),
                    Act::Leave => self.try_to_leave(piece_id, dice_number),
                    Act::Safe => self.try_to_safe(piece_id, dice_number),
                    Act::Starjump => self.try_to_starjump(piece_id, dice_number),
                _ => Act::Nothing,
            }
        }

        pub fn try_to_free(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let binding = self.piece(piece_id);
            let binding = binding.borrow_mut();
            let is_home = binding.is_home();
            let invincible_position = self.invincible_positions(self.id());
            let is_occupied_by_others = self.is_occupied_by_others(invincible_position).0;
            match (is_home, is_occupied_by_others, dice_number) {
                (true, false, 6) => Act::Free,
                _ => Act::Nothing,
            }
        }

        pub fn try_to_starjump(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let is_star = self.board().borrow_mut().is_star(self.new_position);
            let is_home = self.piece(piece_id).borrow_mut().is_home();
            let is_occupied = self.is_occupied_by_others(self.new_position);
            let is_star_occupied = self.is_star_occupied(self.old_position, self.new_position);
            match (is_star, is_home, is_occupied.0, is_star_occupied.0) {
                (true, false, false, false) => Act::Starjump,
                _ => Act::Nothing,
            }
        }

        pub fn try_to_safe(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let is_globe = self.board().borrow_mut().is_globe(self.new_position);
            let is_occupied_by_others = self.is_occupied_by_others(self.new_position);
            self.correct_position();
            if (is_globe & !is_occupied_by_others.0) | (self.new_position >= 52 && self.old_position < 52) {
                return Act::Safe;
            }
            Act::Nothing
        }

        pub fn try_to_leave(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let occupied_by_selves = self.is_occupied_by_selves(self.old_position);
            let is_occupied = self.is_occupied_or_more(self.new_position);
            let is_star = self.board().borrow_mut().is_star(self.new_position);
            let is_globe = self.board().borrow_mut().is_globe(self.new_position);

            match (occupied_by_selves.1, is_star, is_globe, is_occupied.0) {
                (true, false, false, false) => Act::Leave,
                _ => Act::Nothing,
            }
        }

        pub fn try_to_move(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            self.correct_position();
            let binding = self.piece(piece_id);
            let binding = binding.borrow_mut();
            let is_home = binding.is_home();
            let is_goalpos = self.is_goal_position();
            let is_starpos = self.board().borrow_mut().is_star(self.new_position);
            let is_globepos = self.board().borrow_mut().is_globe(self.new_position);
            let is_self_occupied = self.is_occupied_by_selves(self.old_position).1;
            let is_newpos_occupied = self.is_occupied_or_more(self.new_position).0;
            let is_outside = self.new_position < 52;

            if is_home | is_goalpos | is_starpos | (is_self_occupied | is_newpos_occupied) & is_outside | is_globepos {
                return Act::Nothing;
            }
            Act::Move
        }

        fn is_goal_position(&mut self) -> bool {
            self.goal_positions(self.old_position, self.new_position) == 99
        }

        fn is_occupied_or_more(
            &mut self,
            position: i8,
        ) -> (bool, bool) {
            
            let is_occupied_more = self
                .board()
                .borrow_mut()
                .is_occupied_more(position);
            let is_occupied = self.board().borrow_mut().is_occupied(position);
            (is_occupied, is_occupied_more)
        }

        pub fn try_to_join(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let is_starpos = self.board().borrow_mut().is_star(self.new_position);
            let is_self_occupied = self.is_occupied_by_selves(self.new_position);
            let is_star_self_occupied = self.is_star_occupied_by_selves();
            let is_other_occupied = self.is_occupied_by_others(self.new_position);
            let is_goalpos = self.is_goal_position();
            let is_home = self.piece(piece_id).borrow_mut().is_home();
            match (is_self_occupied.0, is_starpos, is_star_self_occupied.0, is_other_occupied.0, is_goalpos, is_home) {
                (true, false, _, _, false, false) | (_, _, true, false, false, false) => Act::Join,
                (_, _, _, _, true, true) => Act::Nothing,
                _ => Act::Nothing,
            }
        }

        fn is_occupied_by_selves(&mut self, position: i8) -> (bool, bool) {
            let occupied_by_self = self
                .board()
                .borrow_mut()
                .is_occupied_self(self.id(), position);
            let occupied_by_self_more = self
                .board()
                .borrow_mut()
                .is_occupied_by_more_self(self.id(), position);
            (occupied_by_self, occupied_by_self_more)
        }

        pub fn try_to_die(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let occupied_by_others = self.is_occupied_by_others(self.new_position);
            let is_star_occupied_by_others = self.is_star_occupied_by_others(self.old_position, self.new_position);
            let is_globepos = self.board().borrow_mut().is_globe(self.new_position);
            let is_home = self.piece(piece_id).borrow_mut().is_home();
            let other_player_id = self.get_other_player_id();
            let invincible_position = self.invincible_positions(other_player_id);
            let is_other_player_invincible = self.is_occupied_by_others(invincible_position).0 & (self.new_position == invincible_position);

            match (is_home, occupied_by_others.0, occupied_by_others.1, is_globepos, is_star_occupied_by_others.1, is_other_player_invincible) {
                (false, _, true, _, _, _) | 
                (false, _, _, _, true, _) | 
                (false, _, _, _, _, true) |
                (false, true, _, true, _, _) => Act::Die,
                _ => Act::Nothing,
            }
        }

        pub fn is_occupied_by_others(&mut self, new_position: i8) -> (bool, bool) {
            let occupied_by_other_more = self
                .board()
                .borrow_mut()
                .is_occupied_by_other_more(self.id(), new_position);
            let occupied_by_other = self
                .board()
                .borrow_mut()
                .is_occupied_by_other(self.id(), new_position);
            (occupied_by_other, occupied_by_other_more)
        }

        pub fn try_to_win(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let is_occupied_by_others = self.is_occupied_by_others(self.new_position);
            let is_goalpos =  self.is_goal_position();
            match (is_occupied_by_others.1, is_goalpos) {
                (false, true) => Act::Goal,
                _ => Act::Nothing,
            }
        }

        pub fn try_to_kill(&mut self, piece_id: i8, dice_number: i8) -> Act {
            self.update_position(piece_id, dice_number);
            let piece = self.piece(piece_id);
            let is_home = piece.borrow().is_home();
            let other_player_id = self.get_other_player_id();
            let invincible_position = self.invincible_positions(other_player_id);
            let is_invincible_position_occupied_by_others = self.is_occupied_by_others(invincible_position).0;
            let is_new_position_same_as_invincible_position = self.new_position == invincible_position;
            let is_new_position_occupied_by_others = self.is_occupied_by_others(self.new_position);
            let is_star_position_occupied_by_others = self.is_star_occupied_by_others(self.old_position, self.new_position);
            let is_globe = self.board().borrow_mut().is_globe(self.new_position);
        
            match (is_home,
                   is_invincible_position_occupied_by_others,
                   is_new_position_same_as_invincible_position,
                   is_globe,
                   is_new_position_occupied_by_others.0,
                   is_new_position_occupied_by_others.1,
                   is_star_position_occupied_by_others.0, 
                   is_star_position_occupied_by_others.1, 
                   dice_number) {
                (false, _, false, false, true, false, _, false, _) |
                (false, _, false, false, _, false, true, false, _) |
                (false, _, false, false, true, false, true, _, _) |
                (true, _, _, _, true, _, _, _, 6) => Act::Kill,
                (false, true, false, _, true, _, _, _, _) => Act::Nothing,
                _ => Act::Nothing,
            }
        }

        pub fn get_other_player_id(&self) -> i8 {
            let other_piece = self.board().borrow_mut().outside(self.new_position).pieces.get(0).cloned();
            if let Some(id ) = other_piece.map(|piece| piece.borrow().color() as i8) {
                id
            } else {
                0
            }
        }

        pub fn is_star_occupied_by_selves(
            &mut self,
        ) -> (bool, bool) {
            let is_star = self.board().borrow_mut().is_star(self.new_position);
            if is_star {
                let star_position = self.star_position(self.old_position, self.new_position);
                let is_star_occupied_by_self = self
                    .board()
                    .borrow_mut()
                    .is_occupied_self(self.id(), star_position);
                let is_star_occupied_by_more_selves = self
                    .board()
                    .borrow_mut()
                    .is_occupied_by_more_self(self.id(), star_position);
                (is_star_occupied_by_self, is_star_occupied_by_more_selves)
            } else {
                (false, false)
            }
        }

        pub fn is_star_occupied(
            &mut self,
            old_position: i8,
            new_position: i8,
        ) -> (bool, bool) {
            let is_star = self.board().borrow_mut().is_star(new_position);
            if is_star {
                let star_position = self.star_position(old_position, new_position);
                let is_star_occupied = self.board().borrow_mut().is_occupied(star_position);
                let is_star_occupied_more =
                    self.board().borrow_mut().is_occupied_more(star_position);
                (is_star_occupied, is_star_occupied_more)
            } else {
                (false, false)
            }
        }

        pub fn is_star_occupied_by_others(
            &mut self,
            old_position: i8,
            new_position: i8,
        ) -> (bool, bool) {
            let is_star = self.board().borrow_mut().is_star(new_position);
            match is_star {
                true => {
                    let star_position = self.star_position(old_position, new_position);
                    let occupied_by_other_star = self
                        .board()
                        .borrow_mut()
                        .is_occupied_by_other(self.id(), star_position);
                    let occupied_by_other_more_star = self
                        .board()
                        .borrow_mut()
                        .is_occupied_by_other_more(self.id(), star_position);
                    (occupied_by_other_star, occupied_by_other_more_star)
                }
                _ => (false, false),
            }
        }

        pub fn get_heuristics(&mut self, piece_id: i8) -> i8{
            if self.piece(piece_id).borrow_mut().is_home() {
                return 57
            }
            if self.piece(piece_id).borrow_mut().is_goal() {
                return 0
            }
            let position = self.piece(piece_id).borrow_mut().position();
            match (self.id, position) {
                (0, 51..=55) => 57 - position,
                (1, _)  => 69 - position,
                (2, _) =>  82 - position,
                (3, _) =>  95 - position,
                _ => 56 - position,
            }
        }

        pub fn is_finished(&self) -> bool {
            self.pieces.iter().all(|piece| piece.borrow_mut().is_goal())
        }

        pub fn print_status(&mut self) {
            let mut table = Table::new();
            table.add_row(row![
                "Player",
                "Dice number",
                "Old position",
                "New position",
                "Piece id",
                "Act",
                "Piece 0",
                "Piece 1",
                "Piece 2",
                "Piece 3"
            ]);
            table.add_row(row![
                format!("{:<8}", self.id()),
                format!("{:<12}", self.dice.as_ref().unwrap().get_value()),
                format!("{:<13}", self.old_position),
                format!("{:<13}", self.new_position),
                format!("{:<9}", self.action.1),
                format!("{:<10}", format!("{:?}", self.action.0)),
                format!("{:<8}", self.piece(0).borrow().position()),
                format!("{:<8}", self.piece(1).borrow_mut().position()),
                format!("{:<8}", self.piece(2).borrow_mut().position()),
                format!("{:<8}", self.piece(3).borrow_mut().position())
            ]);
            table.printstd();
        }
        
        


    }

    fn get_color_from_player_id(player_id: i8) -> Color {
       match player_id {
            0 => Color::Green,
            1 => Color::Yellow,
            2 => Color::Blue,
            3 => Color::Red,
            _ => panic!("invalid player id!"),
        }
    }

    fn get_piece_and_player_id(piece: Rc<RefCell<Piece>>) -> (i8, i8) {
        let (other_player_id, other_piece_id) = {
            let piece_borrow = piece.borrow_mut();
            (piece_borrow.color() as i8, piece_borrow.id())
        };
        (other_player_id, other_piece_id)
    }
}

pub use players::{Act, Player};