mod players {
    use std::{cell::RefCell, rc::Rc, borrow::Borrow};

    use board::Board;
    use dice::Dice;
    use pieces::Piece;
    use rand::seq::SliceRandom;
    #[derive(PartialEq, Debug)]
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
        Win,
        Safe,
        Skip,
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

        pub fn actions(&mut self) -> Vec<Act> {
            vec![
                Act::Move,
                Act::Free,
                Act::Kill,
                Act::Join,
                Act::Leave,
                Act::Die,
                Act::Win,
                Act::Safe,
                Act::Skip,
                Act::Nothing,
            ]
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
                Act::Move | Act::Skip | Act::Safe => {
                    self.move_piece(piece_id, dice_number);
                    self.can_continue();
                }
                Act::Win => {
                    self.move_piece(piece_id, dice_number);
                    self.my_turn();
                }
                Act::Free => {
                    self.free_piece(piece_id);
                    self.can_continue();
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
                    self.die(piece_id);
                    self.can_continue();
                }
                Act::Nothing => {
                    self.turn = false;
                }
            }
        }

        pub fn kill_piece(&mut self, piece_id: i8, dice_number: i8) {
            let pre_pos = self.piece(piece_id).borrow_mut().position();
            let cur_pos = pre_pos + dice_number;
            let other_player_id = self
                .board()
                .borrow_mut()
                .outside(cur_pos)
                .player_id
                .unwrap() as i8;
            let other_piece_id = self
                .board()
                .borrow_mut()
                .get_outside_piece(cur_pos, 0)
                .borrow_mut()
                .id();
            self.board()
                .borrow_mut()
                .outside(cur_pos)
                .piece(other_piece_id)
                .borrow_mut()
                .dead();
            self.board()
                .borrow_mut()
                .move_into_home(other_player_id, other_piece_id, cur_pos);

            let is_star = self.board().borrow_mut().is_star(cur_pos);
            if is_star {
                let star_pos = self.star_position(pre_pos, cur_pos);
                let other_player_id = self
                    .board()
                    .borrow_mut()
                    .outside(star_pos)
                    .player_id
                    .unwrap() as i8;
                let other_piece_id = self
                    .board()
                    .borrow_mut()
                    .get_outside_piece(star_pos, 0)
                    .borrow_mut()
                    .id();
                self.board()
                    .borrow_mut()
                    .outside(star_pos)
                    .piece(other_piece_id)
                    .borrow_mut()
                    .dead();
                self.board()
                    .borrow_mut()
                    .move_into_home(other_player_id, other_piece_id, star_pos);
            }
            self.move_piece(piece_id, dice_number);
        }

        pub fn leave_piece(&mut self, piece_id: i8, dice_number: i8) {
            self.move_piece(piece_id, dice_number);

            for i in 0..4 {
                if self.piece(i).borrow_mut().is_home() {
                    continue;
                }

                let pos = self.piece(i).borrow_mut().position();

                if self.board().borrow_mut().is_globe(pos)
                    || self.board().borrow_mut().is_occupied_more(pos)
                {
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
            let _ = self
                .try_enter_goal(piece_id, old_position, new_position)
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
            if new_position > 51 {
                return Err(new_position);
            }
            let star_position = self.star_position(old_position, new_position);
            self.starjump(piece_id, star_position, old_position);
            Ok(())
        }

        fn star_position(&mut self, old_position: i8, new_position: i8) -> i8 {
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
            if self.goal_position(old_position, new_position) == 99 {
                self.enter_goal(piece_id, old_position);
                return Ok(());
            }
            Err(new_position)
        }

        fn goal_position(&mut self, old_position: i8, new_position: i8) -> i8 {
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
                | (3, 31..=36, 37) => 99,
                _ => new_position,
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
            self.board()
                .borrow_mut()
                .move_inside(self.id(), piece_id, old_position, new_position);
        }

        fn enter_goal(&mut self, piece_id: i8, old_position: i8) {
            self.piece(piece_id).borrow_mut().goal();
            self.board()
                .borrow_mut()
                .enter_goal(self.id(), piece_id, old_position);
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
                .move_from_home(self.id(), piece_id, position);
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
                .move_into_home(self.id(), piece_id, old_position);
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

        pub fn valid_moves(&mut self, piece_id: i8, dice: i8) -> (bool, bool) {
            if piece_id > 3 {
                return (false, false);
            }
            let binding = self.piece(piece_id);
            let binding = binding.borrow_mut();
            let is_home = binding.is_home();
            let is_goal = binding.is_goal();
            let is_valid = match (is_goal, is_home, dice) {
                (true, _, _) | (_, true, 1..=5) => false,
                (_, true, 6) => true,
                (false, false, 1..=6) => true,
                _ => false,
            };
            (is_home, is_valid)
        }

        pub fn select_choice(&mut self, dice_number: i8, action: Act) -> (Act, i8) {
            let mut reaction: Act = Act::Nothing;
            let mut piece_id: i8 = 0;
            for idx in 0..4 {
                reaction = self.valid_choices(idx, dice_number, action);
                if reaction != Act::Nothing {
                    piece_id = idx;
                    return (reaction, piece_id);
                }
            }
            (reaction, piece_id)
        }

        pub fn valid_choices(&mut self, piece_id: i8, dice_number: i8, action: Act) -> Act {
            let (is_home, is_valid) = self.valid_moves(piece_id, dice_number);
            match (action, is_home, is_valid) {
                (Act::Free, true, true) => Act::Free,
                (Act::Move, false, true) => self.try_to_move(piece_id, dice_number),
                (Act::Join, false, true) => self.try_to_join(piece_id, dice_number),
                (Act::Kill, false, true) => self.try_to_kill(piece_id, dice_number),
                (Act::Die, false, true) => self.try_to_die(piece_id, dice_number),
                (Act::Win, false, true) => self.try_to_win(piece_id, dice_number),
                (Act::Leave, false, true) => self.try_to_leave(piece_id, dice_number),
                (Act::Safe, false, true) => self.try_to_safe(piece_id, dice_number),
                (Act::Skip, false, true) => self.try_to_skip(piece_id, dice_number),
                _ => Act::Nothing,
            }
        }

        pub fn try_to_skip(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 51 {
                return Act::Nothing;
            }
            let is_star = self.board().borrow_mut().is_globe(new_pos);
            let is_occupied_new = self.board().borrow_mut().is_occupied_by_other(self.id(),new_pos);
            if is_star || !is_occupied_new {
                return Act::Skip;
            }
            Act::Nothing
        }

        pub fn try_to_safe(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 51 {
                return Act::Nothing;
            }
            let is_globe = self.board().borrow_mut().is_globe(new_pos);
            let is_occupied_new = self.board().borrow_mut().is_occupied_by_other(self.id(),new_pos);
            if is_globe || !is_occupied_new {
                return Act::Safe;
            }
            Act::Nothing
        }

        pub fn try_to_leave(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 51 {
                return Act::Nothing;
            }
            let is_occupied_old = self.board().borrow_mut().is_occupied(pos);
            if is_occupied_old {
                return Act::Leave;
            }
            Act::Nothing
        }

        pub fn try_to_move(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 51 {
                return Act::Move;
            }
            let is_occupied_old = self.board().borrow_mut().is_occupied_more(pos);
            let is_occupied_new = self.board().borrow_mut().is_occupied(new_pos);
            if is_occupied_old || is_occupied_new {
                return Act::Nothing;
            }
            Act::Move
        }

        pub fn try_to_join(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 51 {
                return Act::Nothing;
            }
            let occupied = self.board().borrow_mut().is_occupied(new_pos);

            let occupied_star = self.is_star_occupied(new_pos, pos);

            if occupied || occupied_star {
                return Act::Join;
            }
            Act::Nothing
        }

        pub fn try_to_die(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 51 {
                return Act::Nothing;
            }
            let occupied_by_other_more = self
                .board()
                .borrow_mut()
                .is_occupied_by_other_more(self.id(), new_pos);
            let occupied_by_other_more_star = self.is_star_occupied_by_others(pos, new_pos).1;
            let occupied_by_other = self.board().borrow_mut().is_occupied_by_other(self.id(), new_pos);
            let mut is_dangerous = false;
            if occupied_by_other {
                is_dangerous = self.board().borrow_mut().outside(new_pos).pieces[0].borrow_mut().is_dangerous();
            }
            if occupied_by_other_more || occupied_by_other_more_star || is_dangerous {
                return Act::Die;
            }
            Act::Nothing
        }

        pub fn try_to_win(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            let goal_pos = self.goal_position(pos, new_pos);

            if goal_pos != 99 {
                return Act::Nothing;
            }
            Act::Win
        }

        pub fn try_to_kill(&mut self, piece_id: i8, dice_number: i8) -> Act {
            let pos = self.piece(piece_id).borrow_mut().position();
            let new_pos = pos + dice_number;
            if new_pos > 52 {
                return Act::Nothing;
            }
            let occupied_by_other = self
                .board()
                .borrow_mut()
                .is_occupied_by_other(self.id(), new_pos);
            let occupied_by_other_more = self
                .board()
                .borrow_mut()
                .is_occupied_by_other_more(self.id(), new_pos);
            let (occupied_by_other_star, occupied_by_other_more_star) =
                self.is_star_occupied_by_others(pos, new_pos);

            let mut is_dangerous = false;
            if occupied_by_other {
                is_dangerous = self.board().borrow_mut().outside(new_pos).pieces[0].borrow_mut().is_dangerous();
            }            
            if (occupied_by_other && !occupied_by_other_more && !is_dangerous)
                || (occupied_by_other_star && !occupied_by_other_more_star)
            {
                return Act::Kill;
            }
            Act::Nothing
        }

        pub fn is_star_occupied(&mut self, new_pos: i8, pos: i8) -> bool {
            let star_position = self.star_position(pos, new_pos);
            self.board().borrow_mut().is_occupied(star_position)
        }

        pub fn is_star_occupied_by_others(&mut self, pos: i8, new_pos: i8) -> (bool, bool) {
            let is_star = self.board().borrow_mut().is_star(new_pos);
            match is_star {
                true => {
                    let star_position = self.star_position(pos, new_pos);
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

        pub fn random_play(&mut self) {
            while self.is_player_turn() {
                let dice_number = self.roll_dice();
                println!("dice_number: {}", dice_number);
                let mut choices: Vec<(Act, i8)> = vec![];
                for action in self.actions() {
                    let valid_choice = self.select_choice(dice_number, action);
                    if valid_choice.0 != Act::Nothing {
                        choices.push(valid_choice);
                    }
                }
                if let Some(choice) = choices.choose(&mut rand::thread_rng()) {
                    self.make_move(choice.1, dice_number, choice.0);
                }
            }
        }

        pub fn is_finished(&self) -> bool {
            self.pieces.iter().all(|piece| piece.borrow_mut().is_goal())
        }
    }
}

pub use players::{Act, Player};
