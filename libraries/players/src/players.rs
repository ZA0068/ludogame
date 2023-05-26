mod players {
    use std::{cell::RefCell, rc::Rc};

    use board::Board;
    use dice::Dice;
    use pieces::Piece;

    #[derive(PartialEq, Debug)]
    pub struct Player {
        id: i8,
        pieces: Vec<Piece>,
        turn: bool,
        dice: Option<Rc<RefCell<Dice>>>,
        board: Rc<RefCell<Board>>,
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

    impl Player {
        pub fn new(id: i8, board: Rc<RefCell<Board>>, dice: Option<Rc<RefCell<Dice>>>) -> Player {
            let mut pieces = vec![];
            for i in 0..4 {
                pieces.push(Piece::new(i));
            }
            Player {
                id,
                pieces,
                turn: false,
                board,
                dice,
            }
        }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn piece(&mut self, piece_id: i8) -> &mut Piece {
            &mut self.pieces[piece_id as usize]
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

        pub fn make_choice(&mut self, piece_id: i8, dice_number: i8, choice: Act) {
            match choice {
                Act::Move => {
                    self.move_piece(piece_id, dice_number);
                }
                Act::Free => {
                    self.free_piece(piece_id);
                }
                Act::Kill => {
                    // self.kill_piece();
                }
                Act::Join => {
                    self.join_piece(piece_id, dice_number);
                }
                Act::Nothing => (),
                _ => (),
            }
        }

        pub fn join_piece(&mut self, piece_id: i8, dice_number: i8){
            let old_position = self.piece(piece_id).position();
            let new_position = old_position + dice_number;
            self.piece(piece_id).set_position(new_position);
            self.piece(piece_id).dangerous();
            self.piece(0).dangerous();
            self.board().borrow_mut().update_outside(
                self.id(),
                old_position.into(),
                new_position.into(),
            );
        }

        pub fn move_piece(&mut self, piece_id: i8, dice_number: i8) {
            let old_position = self.piece(piece_id).position();
            let new_position = old_position + dice_number;
            self.update_piece_state(piece_id, old_position, new_position);
        }

        fn update_piece_state(&mut self, piece_id: i8, old_position: i8, new_position: i8) {
            let res = self.try_enter_inside(piece_id, old_position, new_position);
            if res == Ok(()) {
                return;
            }
            let res = self.try_enter_goal(piece_id, old_position, res.unwrap_err());
            if res == Ok(()) {
                return;
            }
            let res = self.try_move_back(piece_id, old_position, res.unwrap_err());
            if res == Ok(()) {
                return;
            }
            let res = self.try_enter_globe(piece_id, old_position, res.unwrap_err());
            if res == Ok(()) {
                return;
            }
            let res = self.try_starjump(piece_id, old_position, res.unwrap_err());
            if res == Ok(()) {
                return;
            }
            let _ = self.try_update_outside(piece_id, old_position, res.unwrap_err());
        }

        fn try_starjump(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            if self.board().borrow().is_star(new_position as usize) {
                self.starjump(piece_id, new_position, old_position);
                return Ok(());
                }
                Err(new_position)
            }

        fn starjump(&mut self, piece_id: i8, new_position: i8, old_position: i8) {
            let new_position = match new_position {
                5 => 11,
                11 => 18,
                18 => 24,
                24 => 31,
                31 => 37,
                37 => 44,
                44 => 50,
                50 => 5,
                _ => new_position,
            };
            self.piece(piece_id).set_position(new_position);
            self.piece(piece_id).not_safe();
            self.board().borrow_mut().update_outside(
                self.id(),
                old_position.into(),
                new_position.into(),
            );
        }

        fn try_enter_globe(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            if self.board().borrow().is_globe(new_position as usize) {
                self.piece(piece_id).set_position(new_position);
                self.piece(piece_id).dangerous();
                self.board().borrow_mut().update_outside(
                    self.id(),
                    old_position.into(),
                    new_position.into(),
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
            self.piece(piece_id).set_position(new_position);
            self.piece(piece_id).not_safe();
            self.board().borrow_mut().update_outside(
                self.id(),
                old_position.into(),
                new_position.into(),
            );
            Ok(())
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
            self.move_back(old_position, new_position, subtract, piece_id);
            Ok(())
        }

        fn move_back(&mut self, old_position: i8, new_position: i8, subtract: i8, piece_id: i8) {
            let corrected_position = 2 * subtract - new_position;
            self.piece(piece_id).set_position(corrected_position);
            self.board().borrow_mut().update_inside(
                self.id(),
                old_position as isize,
                corrected_position as isize,
            );
        }

        fn try_enter_goal(
            &mut self,
            piece_id: i8,
            old_position: i8,
            new_position: i8,
        ) -> Result<(), i8> {
            match (piece_id, old_position, new_position) {
                (0, 50, 56)
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
            self.piece(piece_id).set_position(new_position);
            self.board().borrow_mut().move_inside(
                self.id(),
                old_position as usize,
                new_position as usize,
            );
        }

        fn enter_goal(&mut self, piece_id: i8, old_position: i8) {
            self.piece(piece_id).goal();
            self.board()
                .borrow_mut()
                .enter_goal(self.id(), old_position.into());
        }

        pub fn free_piece(&mut self, piece_id: i8) {
            let position = match self.id() {
                0 => 0,
                1 => 13,
                2 => 26,
                3 => 39,
                _ => panic!("invalid move!"),
            };
            self.piece(piece_id).free();
            self.piece(piece_id).set_position(position);
            self.board()
                .borrow_mut()
                .move_from_home(self.id(), position.into());
        }

        pub fn roll_dice(&mut self) -> i8 {
            if let Some(dice) = &self.dice {
                dice.borrow_mut().roll();
                dice.borrow().get_value()
            } else {
                0
            }
        }

        pub fn die(&mut self, piece_id: i8) {
            let new_pos = self.piece(piece_id).position();
            self.piece(piece_id).dead();
            self.board()
                .borrow_mut()
                .move_into_home(self.id(), new_pos.into());
        }

        pub fn is_player_turn(&self) -> bool {
            self.turn
        }

        pub fn my_turn(&mut self) {
            self.turn = true;
        }

        pub fn can_continue(&mut self) {
            self.turn = self.dice.clone().unwrap().borrow().get_value() == 6;
        }

        pub fn valid_moves(&mut self, piece_id: i8, dice: i8) -> Act {
            if piece_id > 3 {
                return Act::Nothing;
            }
            match (
                self.piece(piece_id).is_goal(),
                self.piece(piece_id).is_home(),
                dice,
            ) {
                (true, _, _) | (_, true, 1..=5) => Act::Nothing,
                (_, true, 6) => Act::Free,
                (false, false, 1..=6) => Act::Move,
                _ => Act::Nothing,
            }
        }

        pub fn is_finished(&self) -> bool {
            self.pieces.iter().all(|p: &Piece| p.is_goal())
        }
    }
}

pub use players::{Act, Player};
