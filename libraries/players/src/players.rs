mod players {
    use board::Board;
    use dice::Dice;
    use pieces::Piece;
    use rand::Rng;

    pub struct Player {
        id: u8,
        pieces: Vec<Piece>,
        turn: bool,
        dice: Dice,
    }

    impl Player {
        pub fn new(id: u8) -> Player {
            Player {
                id,
                pieces: (0..4).map(|i| Piece::new(i)).collect(),
                turn: false,
                dice: Dice::new(),
            }
        }

        pub fn id(&self) -> u8 {
            self.id
        }

        pub fn piece(&mut self, id: u8) -> &mut Piece {
            &mut self.pieces[id as usize]
        }

        pub fn pieces(&mut self) -> &mut Vec<Piece> {
            &mut self.pieces
        }

        pub fn move_piece(&mut self, id: u8, dice_number: u8) {
            if self.is_move_valid(id, dice_number) {
                let pos = self.pieces[id as usize].position() + dice_number as i8;
                self.pieces[id as usize].set_position(pos);
            }
        }

        pub fn free_piece(&mut self, id: u8) {
            self.pieces[id as usize].free();
        }

        pub fn roll_dice(&mut self) -> u8 {
            self.dice.roll();
            self.dice.get_value()
        }

        pub fn is_players_turn(&self) -> bool {
            self.turn
        }

        pub fn set_turn(&mut self, turn: bool) {
            self.turn = turn;
        }

        pub fn is_move_valid(&self, id: u8, dice: u8) -> bool {
            match (
                self.pieces[id as usize].is_goal(),
                self.pieces[id as usize].is_home(),
                dice,
            ) {
                (true, _, _) => false,
                (_, true, _) => false,
                (_, _, _p) if self.dice.get_value() != 6 => false,
                _ => true,
            }
        }
        pub fn can_free(&mut self, dice: u8, id: u8) -> bool {
            self.pieces[id as usize].is_home() && dice == 6
        }

        pub fn choose_piece(&mut self) -> u8 {
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

        pub fn play_random(&mut self) {
            if self.is_players_turn() {
                let dice = self.roll_dice();
                let id = self.choose_piece();
                if self.can_free(dice, id) {
                    self.free_piece(id);
                } else {
                    let id = self.choose_piece();
                    self.move_piece(id, dice);
                }
            }
        }

    }
}
pub use players::Player;
