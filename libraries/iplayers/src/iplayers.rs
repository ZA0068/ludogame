mod iplayers {
    use board::Board;
    use dice::Dice;
    use players::{Act, Player, Select};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub static AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Goal,
        Act::Starjump,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    pub static FAST_AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Goal,
        Act::Starjump,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    pub static SAFE_ACTIONS: [Act; 10] = [
        Act::Join,
        Act::Safe,
        Act::Goal,
        Act::Move,
        Act::Kill,
        Act::Starjump,
        Act::Free,
        Act::Leave,
        Act::Nothing,
        Act::Die,
    ];

    pub static FAST_ACTIONS: [Act; 10] = [
        Act::Goal,
        Act::Starjump,
        Act::Leave,
        Act::Join,
        Act::Move,
        Act::Kill,
        Act::Free,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    pub static ACTIONS: [Act; 10] = [
        Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Starjump,
        Act::Nothing,
    ];

    pub static SELECTIONS: [Select; 3] = [
        Select::Nearest,
        Select::Furthest,
        Select::Random,
    ];

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Playstyle {
        Aggressive,
        Fast,
        Random,
        Safe,
        FastAggressive,
        GeneticAlgorithm,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct IPlayer {
        player: Player,
        playstyle: Option<Playstyle>,
        select_which_piece: Select,
        actions: Option<[Act; 10]>,
        dice_number: i8,
        wins: u16,
        winrate: f64,
        first_round: bool,
    }

    pub trait Behavior {
        fn play(&mut self, is_in_debug_mode: bool);
        fn take_dice(&mut self, dice: Dice);
        fn roll_dice(&mut self);
        fn give_dice(&mut self, iplayer: &mut IPlayer);
        fn hit_first_round(&mut self);
        fn my_turn(&mut self);
        fn is_my_turn(&self) -> bool;
        fn win(&mut self);
    }

    impl Behavior for IPlayer {
        fn play(&mut self, shall_debug: bool) {
            while self.is_my_turn() {
                if self.first_round {
                    self.hit_first_round();
                } else {
                    self.roll_dice();
                }
                match self.playstyle {
                    Some(Playstyle::Aggressive) => self.aggro(),
                    Some(Playstyle::Fast) => self.fast(),
                    Some(Playstyle::Random) => self.random(),
                    Some(Playstyle::Safe) => self.safe(),
                    Some(Playstyle::FastAggressive) => self.fast_aggro(),
                    Some(Playstyle::GeneticAlgorithm) => self.genetic(),
                    None => panic!("Player has no playstyle. Please give it with some playstyle"),
                }
                self.take_action(shall_debug);
            }
        }

        fn hit_first_round(&mut self) {
            let mut roll_count = 0;
            while roll_count < 3 {
                self.roll_dice();
                if self.dice_number == 6 {
                    self.first_round = false;
                }
                roll_count += 1;
            }
        }

        fn roll_dice(&mut self) {
            self.player.roll_dice();
            self.dice_number = self.player.get_dice_number();
            if self.dice_number == 0 {
                panic!("This player doesn't have the dice!");
            }
        }

        fn take_dice(&mut self, dice: Dice) {
            self.player.get_dice(dice);
        }

        fn give_dice(&mut self, iplayer: &mut IPlayer) {
            if let Some(dice) = self.player.dice() {
                iplayer.take_dice(dice.clone());
                self.player.drop_dice();
            } else {
                panic!("This player doesn't have the dice!");
            }
        }

        fn my_turn(&mut self) {
            self.player.my_turn();
        }

        fn is_my_turn(&self) -> bool {
            self.player.is_player_turn()
        }

        fn win(&mut self) {
            self.wins += 1;
        }
    }

    impl IPlayer {
        pub fn new(id: i8) -> Self {
            IPlayer {
                player: Player::new(id),
                playstyle: None,
                actions: None,
                select_which_piece: Select::Random,
                wins: 0,
                winrate: 0.0,
                dice_number: 0,
                first_round: true,
            }
        }

        pub fn create(id: i8, playstyle: Playstyle) -> Self {
            IPlayer {
                player: Player::new(id),
                playstyle: Some(playstyle),
                actions: get_action_from_playstyle(playstyle),
                select_which_piece: Select::Random,
                wins: 0,
                winrate: 0.0,
                dice_number: 0,
                first_round: true,
            }
        }

        pub fn substitute(&mut self, iplayer: &mut IPlayer) {
            self.playstyle = iplayer.playstyle;
            self.actions = iplayer.actions;
            self.select_which_piece = iplayer.select_which_piece;
            self.wins = iplayer.wins;
            self.winrate = iplayer.winrate;
            self.dice_number = iplayer.dice_number;
            self.first_round = iplayer.first_round;
        }

        pub fn set_actions(&mut self, actions: [Act; 10]) {
            self.actions = Some(actions);
        }

        pub fn select_which_piece(&mut self, select: Select) {
            self.select_which_piece = select;
        }

        pub fn set_playstyle(&mut self, playstyle: Playstyle) {
            if let Some(action) = get_action_from_playstyle(playstyle) {
                self.actions = Some(action);
            }
            self.playstyle = Some(playstyle);
        }

        pub fn setup_board(&mut self, board: Rc<RefCell<Board>>) {
            self.player.setup(board);
        }

        pub fn player(&self) -> &Player {
            &self.player
        }

        pub fn get_playstyle(&self) -> &Playstyle {
            if let Some(playstyle) = &self.playstyle {
                playstyle
            } else {
                panic!("Player has no playstyle. Please give it with some playstyle")
            }
        }

        pub fn get_actions(&self) -> &[Act; 10] {
            if let Some(actions) = &self.actions {
                actions
            } else {
                panic!("Player has no actions. Please give it with some actions")
            }
        }

        pub fn get_piece_selector(&self) -> &Select {
            &self.select_which_piece
        }

        pub fn calculate_winrate(&mut self, total_games: u16) {
            self.winrate = self.wins as f64 / total_games as f64 * 100.0;
        }

        pub fn get_winrate(&self) -> &f64 {
            &self.winrate
        }

        pub fn print_winrate(&self) {
            println!("Winrate: {}%", self.winrate);
        }

        pub fn reset_scores(&mut self) {
            self.wins = 0;
            self.winrate = 0.0;
        }

        pub fn reset_first_round(&mut self) {
            self.first_round = true;
        }

        pub fn dice_number(&self) -> i8 {
            self.dice_number
        }

        fn genetic(&mut self) {
            self.choose_ordered_action(self.select_which_piece);
        }

        fn safe(&mut self) {
            self.choose_ordered_action(Select::Furthest);
        }

        fn fast(&mut self) {
            self.choose_ordered_action(Select::Nearest);
        }

        fn aggro(&mut self) {
            self.choose_ordered_action(Select::Random);
        }

        fn fast_aggro(&mut self) {
            self.choose_ordered_action(Select::Nearest);
        }

        fn choose_ordered_action(&mut self, select: Select) {
            self.player.action = self.player.get_ordered_action(
                *self.get_actions(),
                self.dice_number,
                select,
            );
        }

        fn random(&mut self) {
            self.choose_random_action();
        }

        fn choose_random_action(&mut self) {
            let movesets = self
                .player
                .generate_vector_of_random_actions(*self.get_actions(), self.dice_number);
            self.player.action = self.player.select_random_piece(movesets);
        }

        fn take_action(&mut self, debug: bool) {
            if debug {
                self.log_moves();
            } else {
                self.player.make_move(
                    self.player.action.1,
                    self.dice_number,
                    self.player().action.0,
                );
            }
        }

        fn log_moves(&mut self) {
            println!("\n\n------------------------");
            println!("Prior play\n");
            self.player.print_status();
            self.player.make_move(
                self.player.action.1,
                self.dice_number,
                self.player().action.0,
            );
            println!("Posterior play\n");
            self.player.print_status();
        }
    }

    fn get_action_from_playstyle(playstyle: Playstyle) -> Option<[Act; 10]> {
        match playstyle {
            Playstyle::Aggressive => Some(AGGRO_ACTIONS),
            Playstyle::Fast => Some(FAST_ACTIONS),
            Playstyle::Random => Some(ACTIONS),
            Playstyle::Safe => Some(SAFE_ACTIONS),
            Playstyle::FastAggressive => Some(FAST_AGGRO_ACTIONS),
            Playstyle::GeneticAlgorithm => None,
        }
    }
}

pub use iplayers::{Behavior, IPlayer, Playstyle, ACTIONS, SELECTIONS};
