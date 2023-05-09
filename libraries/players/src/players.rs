mod players {
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
    }
    pub enum Act {
        Move,
        Free,
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
            }
        }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn piece(&mut self, id: i8) -> &mut Piece {
            &mut self.pieces[id as usize]
        }

        pub fn pieces(&mut self) -> &mut Vec<Piece> {
            &mut self.pieces
        }

        pub fn make_move(&mut self, id: i8, dice_number: i8) {
            match self.valid_moves(id, dice_number) {
                Act::Move => {
                    self.move_piece(id, dice_number);
                }
                Act::Free => {
                    self.free_piece(id);
                }
                Act::Nothing => (),
            }
        }

        fn move_piece(&mut self, id: i8, dice_number: i8) {
            let position = self.update_position(id, dice_number);
            self.update_piece(id, position);
        }

        fn update_position(&mut self, id: i8, dice_number: i8) -> i8 {
            let initial_position = self.pieces[id as usize].position();
            let pos = initial_position + dice_number;
            self.adjust_position(pos)
        }

        fn adjust_position(&mut self, pos: i8) -> i8 {
            let pos = match (pos, self.id) {
                (51..=56, 0) => pos + 1,
                _ => pos,
            };
            let pos = match (pos, self.id) {
                (57, 0) => 99,
                _ => pos,
            };
            match (pos, self.id) {
                (58..=62, 0) => 52 + (62 - pos),
                _ => pos,
            }
        }

        pub fn free_piece(&mut self, id: i8) {
            self.pieces[id as usize].free();
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

        pub fn valid_moves(&self, id: i8, dice: i8) -> Act {
            match (
                self.pieces[id as usize].is_goal(),
                self.pieces[id as usize].is_home(),
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
                if !self.pieces[i as usize].is_goal() {
                    available_pieces.push(i);
                }
            }

            if available_pieces.is_empty() {
                return 0; // Return 0 as a default value if no piece is available
            }

            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..available_pieces.len());
            available_pieces[index]
        }

        pub fn update_piece_state(&mut self, id: i8) {
            let pos = self.piece(id).position();
            match pos {
                -1..=99 => {
                    if self.board.home().contains(&pos) {
                        self.piece(id).home();
                    } else if self.board.invincible()[self.id() as usize] == pos {
                        self.piece(id).dangerous();
                    } else if self.board.globe().contains(&pos) || self.board.inside().contains(&pos) {
                        self.piece(id).safe();
                    } else if self.board.goal().contains(&pos) {
                        self.piece(id).goal();
                    } else {
                        self.piece(id).not_safe();
                    }
                }
                _ => panic!("Invalid position"),
            }
        }

        pub fn play_random(&mut self) {
            while self.is_player_turn() {
                let dice = self.roll_dice();
                let id = self.choose_piece();
                self.make_move(id, dice);
                self.can_continue();
            }
        }

        pub fn is_winner(&self) -> bool {
            self.pieces.iter().all(|p| p.is_goal())
        }

        fn update_piece(&mut self, id: i8, pos: i8) {
            self.piece(id).set_position(pos);
            self.update_piece_state(id);
        }
    }
}

pub use players::Player;
