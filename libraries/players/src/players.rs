mod players {
    use std::collections::HashMap;

    use board::Board;
    use dice::Dice;
    use pieces::Piece;
    use rand::Rng;

    pub struct Player {
        id: i8,
        pieces: Vec<Piece>,
        turn: bool,
        dice: Dice,
        board: Board,
        opponents: HashMap<i8, Player>,
    }
    pub enum Act {
        Move,
        Free,
        Kill,
        Nothing,
    }

    impl Player {
        pub fn new(id: i8) -> Player {
            Player {
                id,
                pieces: (0..4).map(Piece::new).collect(),
                turn: false,
                dice: Dice::new(),
                board: Board::new(),
                opponents: HashMap::new(),
            }
        }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn piece(&mut self, piece_id: i8) -> &mut Piece {
            &mut self.pieces[piece_id as usize]
        }

        pub fn pieces(&mut self) -> &mut Vec<Piece> {
            &mut self.pieces
        }

        pub fn make_move(&mut self, piece_id: i8, dice_number: i8) {
            match self.valid_moves(piece_id, dice_number) {
                Act::Move => {
                    self.move_piece(piece_id, dice_number);
                }
                Act::Free => {
                    self.free_piece(piece_id);
                }
                Act::Kill => {
                    self.kill_piece(self.opponents);
                }
                Act::Nothing => (),
            }
        }

        pub fn kill_piece(&mut self, piece_id: &i8) {
            self.piece(piece_id).kill();
        }

        fn move_piece(&mut self, piece_id: i8, dice_number: i8) {
            let position = self.update_position(piece_id, dice_number);
            self.update_piece(piece_id, position);
        }

        fn update_position(&mut self, piece_id: i8, dice_number: i8) -> i8 {
            let initial_position = self.piece(piece_id).position();
            let pos = initial_position + dice_number;
            let pos = self.enter_inside(pos, initial_position);
            self.starjump(pos)
        }

        fn enter_inside(&mut self, pos: i8, initial_position: i8) -> i8 {
            let pos = self.adjust_pos_when_enter_inside(pos, initial_position);
            let pos = self.adjust_pos_when_entering_goal(pos);
            let pos = self.adjust_pos_for_multiplayers(pos);
            self.go_back_when_overshoot(pos)
        }

        fn adjust_pos_for_multiplayers(&mut self, pos: i8) -> i8 {
            match (pos, self.id) {
                (52..=56, 1..=3) => pos - 52,
                _ => pos,
            }
        }

        fn go_back_when_overshoot(&mut self, pos: i8) -> i8 {
            match (pos, self.id) {
                (58..=62, 0) => 52 + (62 - pos),
                _ => pos,
            }
        }

        fn adjust_pos_when_entering_goal(&mut self, pos: i8) -> i8 {
             match (pos, self.id) {
                (57, 0) => 99,
                _ => pos,
            }
        }

        fn adjust_pos_when_enter_inside(&mut self, pos: i8, initial_position: i8) -> i8 {
             match (pos, initial_position, self.id) {
                (51..=56, 0..=51 , 0) => pos + 1,
                _ => pos,
            }
        }

        fn starjump(&mut self, pos: i8) -> i8 {
            let goal_positions = [(50, 0), (11, 1), (24, 2), (37, 3)];
            let star_positions = self.board.star();
            let next_star = |pos| {
                let pos_index = star_positions.iter().position(|&r| r == pos).unwrap();
                star_positions[(pos_index + 1) % star_positions.len()]
            };

            match pos {
                pos if goal_positions.contains(&(pos, self.id)) => 99,
                pos if star_positions.contains(&pos) => next_star(pos),
                _ => pos,
            }
        }

        pub fn free_piece(&mut self, piece_id: i8) {
            match self.id() {
                0 => self.piece(piece_id).free(),
                1 => {self.piece(piece_id).free(); self.piece(piece_id).set_position(13)},
                2 => {self.piece(piece_id).free(); self.piece(piece_id).set_position(26)},
                3 => {self.piece(piece_id).free(); self.piece(piece_id).set_position(39)},
                _ => panic!("invalid move!"),
            }
        }

        pub fn roll_dice(&mut self) -> i8 {
            self.dice.roll();
            self.dice.get_value() as i8
        }

        pub fn is_player_turn(&self) -> bool {
            self.turn
        }

        pub fn my_turn(&mut self) {
            self.turn = true;
        }

        pub fn can_continue(&mut self) {
            self.turn = self.dice.get_value() == 6;
        }

        pub fn valid_moves(&mut self, piece_id: i8, dice: i8) -> Act {
            if piece_id == -1 {
                return Act::Nothing;
            }
            match (
                self.piece(piece_id).is_goal(),
                self.piece(piece_id).is_home(),
                dice,
            ) {
                (true, _, _) | (_, true, 1..=5) => Act::Nothing,
                (false, true, 6) => Act::Free,
                (false, _, 1..=6) => Act::Move,
                _ => Act::Nothing,
            }
        }

        pub fn choose_piece(&mut self) -> i8 {
            let mut available_pieces = vec![];

            for i in 0..4 {
                let piece = self.piece(i);
                let not_in_goal = !piece.is_goal();
                let not_in_home_or_can_leave = !(piece.is_home() && self.dice.get_value() != 6);
                if not_in_goal && not_in_home_or_can_leave
                {
                    available_pieces.push(i);
                }
            }

            if available_pieces.is_empty() {
                return -1; // Return 0 as a default value if no piece is available
            }

            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..available_pieces.len());
            available_pieces[index]
        }

        pub fn update_piece_state(&mut self, piece_id: i8) {
            let pos = self.piece(piece_id).position();
            if self.board.home().contains(&pos) {
                self.piece(piece_id).home();
            } else if self.board.invincible()[self.id() as usize] == pos
            || self.board.globe().contains(&pos)
            {
                self.piece(piece_id).dangerous();
            } else if self.board.inside().contains(&pos) {
                self.piece(piece_id).safe();
            } else if self.board.goal().contains(&pos) {
                self.piece(piece_id).goal();
            } else {
                self.check_sharing_square();
            }
        }

        pub fn play_random(&mut self) {
            while self.is_player_turn() {
                let dice = self.roll_dice();
                let piece_id = self.choose_piece();
                self.make_move(piece_id, dice);
                self.can_continue();
            }
        }

        pub fn is_finished(&self) -> bool {
            self.pieces.iter().all(|p| p.is_goal())
        }

        fn update_piece(&mut self, piece_id: i8, pos: i8) {
            self.piece(piece_id).set_position(pos);
            self.update_piece_state(piece_id);
        }

        fn check_sharing_square(&mut self) {
            let mut position_map: HashMap<i8, i8> = HashMap::new();
            for i in 0..4 {
                let pos = self.piece(i).position();
                if pos == -1 || pos == 99 {
                    continue;
                }
                let count = position_map.entry(pos).or_insert(0);
                *count += 1;
            }
        
            for i in 0..4 {
                let pos = self.piece(i).position();
                if pos == -1 || pos == 99 {
                    continue;
                }
                match position_map.get(&pos) {
                    Some(&count) if count > 1 => self.piece(i).dangerous(),
                    _ => self.piece(i).not_safe(),
                }
            }
        }        
  
    }
}

pub use players::Player;
