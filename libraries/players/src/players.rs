mod players {
    use std::{cell::RefCell, rc::Rc};

    use board::Board;
    use dice::Dice;
    use pieces::Piece;
    #[derive(PartialEq, Debug)]
    pub struct Player {
        id: i8,
        turn: bool,
        dice: Option<Rc<RefCell<Dice>>>,
        board: Rc<RefCell<Board>>,
        pieces: Vec<Rc<RefCell<Piece>>>,
    }

    #[derive(PartialEq, Debug)]
    pub enum Act {
        Move,
        Free,
        Kill,
        Join,
        Leave,
        Nothing,
    }

    // Playstyles {
    // [x] aggro_play;
    // [x] fast_aggro_play
    // [x] random_play
    // [x] safe_play
    // [x] fast_play
    // }

    impl Player {
        pub fn new(player_id: i8, board: Rc<RefCell<Board>>, dice: Option<Rc<RefCell<Dice>>>) -> Player {
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
            if self.id > 3 {
                panic!("Player id must be between 0 and 3");
            }
            self.id
        }
        pub fn piece(&mut self, piece_id: i8) -> Rc<RefCell<Piece>> {
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

        pub fn make_move(&mut self, piece_id: i8, dice_number: i8, choice: Act) {
            match choice {
                Act::Move => {
                    self.move_piece(piece_id, dice_number);
                }
                Act::Free => {
                    self.free_piece(piece_id);
                }
                Act::Kill => {
                    self.kill_piece(piece_id, dice_number);
                }
                Act::Join => {
                    self.join_piece(piece_id, dice_number);
                }
                Act::Leave => {
                    self.leave_piece(piece_id, dice_number);
                }
                Act::Nothing => (),
            }
        }


        pub fn kill_piece(&mut self, piece_id: i8, dice_number: i8) {
            let pos = self.piece(piece_id).borrow_mut().position() + dice_number;
            let other_player_id = self.board.borrow_mut().outside(pos).player_id.unwrap() as i8;
            self.board().borrow_mut().outside(pos).piece(other_player_id).borrow_mut().dead();
            self.move_piece(piece_id, dice_number);
        }

        pub fn leave_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.move_piece(piece_id, dice_number);
              
            for i in 0..4 {
                if self.piece(i).borrow_mut().is_home() {
                    continue;
                }
        
                let pos = self.piece(i).borrow_mut().position();
        
                if self.board().borrow_mut().is_globe(pos) || self.board().borrow_mut().is_occupied_more(pos) {
                    self.piece(i).borrow_mut().dangerous();
                } else {
                    self.piece(i).borrow_mut().not_safe();
                }
            }
        }
        

        pub fn join_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.move_piece(piece_id, dice_number);
            let piece_position = self.piece(piece_id).borrow_mut().position();
        
            for i in 0..4 {
                let piece = self.piece(i);
                let pos = piece.borrow_mut().position();
        
                if pos == piece_position {
                    piece.borrow_mut().dangerous();
                }
            }
        }
        

        pub fn move_piece(&mut self, piece_id: i8, dice_number: i8) {
            let (old_position, new_position) = self.update_position(piece_id, dice_number);
            self.update_piece_state(piece_id, old_position, new_position);
        }

        fn update_position(&mut self, piece_id: i8, dice_number: i8) -> (i8, i8) {
            let old_position = self.piece(piece_id).borrow_mut().position();
            let new_position = old_position + dice_number;
            (old_position, new_position)
        }

        fn update_piece_state(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            let _ = self.try_enter_goal(piece_id, old_position, new_position)
                .or_else(|err| self.try_enter_inside(piece_id, old_position, err))
                .or_else(|err| self.try_move_back(piece_id, old_position, err))
                .or_else(|err| self.try_enter_globe(piece_id, old_position, err))
                .or_else(|err| self.try_starjump(piece_id, old_position, err))
                .or_else(|err| self.try_update_outside(piece_id, old_position, err));
        }
        
        

        fn try_starjump(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            let star_position = match (old_position, new_position) {
                (51, 5) | (0..=4, 5) => 11,
                (5..=10, 11)  if self.id() != 1 => 18,
                (12..=17, 18) => 24,
                (18..=23, 24) if self.id() != 2 => 31,
                (25..=30, 31) => 37,
                (31..=36, 37) if self.id() != 3 => 44,
                (38..=43, 44) => 50,
                (44..=49, 50) if self.id() != 0 => 5,
                _ => return Err(new_position),
            };
            self.starjump(piece_id, star_position, old_position);
            Ok(())
        }

        fn starjump(&mut self, piece_id: i8, new_position: i8, old_position: i8) {
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.piece(piece_id).borrow_mut().not_safe();
            self.board().borrow_mut().update_outside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        fn try_enter_globe(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            if self.board().borrow_mut().is_globe(new_position) {
                self.piece(piece_id).borrow_mut().set_position(new_position);
                self.piece(piece_id).borrow_mut().dangerous();
                self.board().borrow_mut().update_outside(
                    self.id(),
                    piece_id,
                    old_position,
                    new_position,
                );
                return Ok(());
            }
            Err(new_position)
        }

        fn try_update_outside(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            let new_position = match (self.id(), new_position) {
                (1..=3, 52..=58) => new_position - 52,
                _ => new_position,
            };
            self.update_outside(piece_id, new_position, old_position);
            Ok(())
        }
        

        fn update_outside(&mut self, piece_id: i8, new_position: i8, old_position: i8) {
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.piece(piece_id).borrow_mut().not_safe();
            self.board().borrow_mut().update_outside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        fn try_move_back(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            let subtract = match (self.id, old_position, new_position) {
                (0, 52..=56, 58..=62) => 57,
                (1, 57..=61, 63..=67) => 62,
                (2, 62..=66, 68..=72) => 67,
                (3, 67..=71, 73..=77) => 72,
                _ => return Err(new_position),
            };
            self.move_back(old_position, new_position, piece_id, subtract);
            Ok(())
        }

        fn move_back(&mut self, old_position: i8, new_position: i8, piece_id: i8, subtract: i8) {
            let new_position = 2 * subtract - new_position;
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.board().borrow_mut().update_inside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        fn try_enter_goal(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            match (self.id(), old_position, new_position) {
                (_, _, 99)
                | (0, 50, 56)
                | (0, 52..=56, 57)
                | (0, 44..=49, 50)
                | (1, 11, 17)
                | (1, 57..=61, 62)
                | (1, 5..=10, 11)
                | (2, 24, 30)
                | (2, 62..=66, 67)
                | (2, 18..=23, 24)
                | (3, 37, 43)
                | (3, 67..=71, 72)
                | (3, 31..=36, 37) => {
                    self.enter_goal(piece_id, old_position);
                    Ok(())
                }
                _ => Err(new_position),
            }
        }

        fn try_enter_inside(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            match (self.id, old_position, new_position) {
                (0, 45..=50, 51..=55) => {
                    let new_position: i8 = new_position + 1;
                    self.enter_inside(piece_id, old_position, new_position);
                    Ok(())
                }
                (1, 6..=11, 12..=16) => {
                    let new_position: i8 = new_position + 45;
                    self.enter_inside(piece_id, old_position, new_position);
                    Ok(())
                }
                (2, 19..=24, 25..=29) => {
                    let new_position: i8 = new_position + 37;
                    self.enter_inside(piece_id, old_position, new_position);
                    Ok(())
                }
                (3, 32..=37, 38..=42) => {
                    let new_position: i8 = new_position + 29;
                    self.enter_inside(piece_id, old_position, new_position);
                    Ok(())
                }
                _ => Err(new_position),
            }
        }

        fn enter_inside(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            self.piece(piece_id).borrow_mut().set_position(new_position);
            self.board().borrow_mut().move_inside(
                self.id(),
                piece_id,
                old_position,
                new_position,
            );
        }

        fn enter_goal(&mut self, piece_id: i8, old_position: i8) {
            self.piece(piece_id).borrow_mut().goal();
            self.board()
                .borrow_mut()
                .enter_goal(self.id(),
                            piece_id,
                            old_position);
        }

        pub fn free_piece(&mut self, piece_id: i8) {
            let position = match self.id() {
                0 => 0,
                1 => 13,
                2 => 26,
                3 => 39,
                _ => panic!("invalid move!"),
            };
            self.piece(piece_id).borrow_mut().free();
            self.piece(piece_id).borrow_mut().set_position(position);
            self.board()
                .borrow_mut()
                .move_from_home(
                    self.id(), 
                    piece_id, position);
        }

        pub fn roll_dice(&mut self) -> i8 {
            if let Some(dice) = &self.dice {
                dice.borrow_mut().roll();
                dice.borrow_mut().get_value()
            } else {
                0
            }
        }

        pub fn die(&mut self, piece_id: i8) {
            let old_position = self.piece(piece_id).borrow_mut().position();
            self.piece(piece_id).borrow_mut().dead();
            self.board()
                .borrow_mut()
                .move_into_home(self.id(), 
                piece_id,
                old_position);
        }

        pub fn is_player_turn(&self) -> bool {
            self.turn
        }

        pub fn my_turn(&mut self) {
            self.turn = true;
        }

        pub fn can_continue(&mut self) {
            self.turn = self.dice.clone().unwrap().borrow_mut().get_value() == 6;
        }

        pub fn valid_moves(&mut self, piece_id: i8, dice: i8) -> bool {
            if piece_id > 3 {
                return false;
            }
            let binding = self.piece(piece_id);
            let binding = binding.borrow_mut();
            let is_home = binding.is_home();
            let is_goal = binding.is_goal();
            match (
                is_goal,
                is_home,
                dice,
            ) {
                (true, _, _) | (_, true, 1..=5) => false,
                (_, true, 6) => true,
                (false, false, 1..=6) => true,
                _ => false,
            }
        }

        pub fn select_choice(&mut self, piece_id: i8, dice_number: i8, action : Act) -> Act {
        let valid_moves = self.valid_moves(piece_id, dice_number);
        match (action, valid_moves) {
            (Act::Free, true) => {
                Act::Free
            }
            (Act::Move, true) => {
                Act::Move
            }
            (Act::Join, true) => {
                let pos = self.piece(piece_id).borrow_mut().position();
                let new_pos = pos + dice_number;
                let is_occupied = self.board().borrow_mut().is_occupied(new_pos);
                if is_occupied {
                    Act::Join
                } else {
                    Act::Nothing
                }
            }
            (Act::Kill, true) => {
                let pos = self.piece(piece_id).borrow_mut().position();
                let new_pos = pos + dice_number;
                let occupied_by_other = self.board().borrow_mut().is_occupied_by_other(self.id(), new_pos);
                if occupied_by_other {
                    Act::Kill
                } else {
                    Act::Nothing
                }
            }
            _ => Act::Nothing,
        }
    }

    // pub fn random_play(&mut self) {
    //     while self.is_player_turn() {
    //         let dice_number = self.roll_dice();
    //         {
    //             // self.make_choice(dice_number, Act::Free);
    //             // self.make_choice(dice_number, Act::Free);
    //         }
    //         self.can_continue();
    //     }
    // }

    pub fn is_finished(&self) -> bool {
            self.pieces.iter().all(|piece| piece.borrow_mut().is_goal())
        }
    }
}

pub use players::{Act, Player};
