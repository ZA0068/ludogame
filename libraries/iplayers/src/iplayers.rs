mod iplayers {
    use std::cell::RefCell;
    use std::rc::Rc;
    use board::Board;
    use players::{Act, Player, Select};
    use dice::Dice;


    static AGGRO_ACTIONS: [Act; 10] = [
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
    
    static FAST_AGGRO_ACTIONS: [Act; 10] = [
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
    
    static SAFE_ACTIONS: [Act; 10] = [
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
    
    static FAST_ACTIONS: [Act; 10] = [
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
    
    static DEFAULT_ACTIONS: [Act; 10] = [
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



    #[derive(Clone, Debug, PartialEq)]
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
        actions: Option<Vec<Act>>,
        dice_number: i8,
    }

    pub trait Behavior {
        fn play(&mut self, is_in_debug_mode: bool);
        fn take_dice(&mut self, dice: Dice);
        fn roll_dice(&mut self);
        fn give_dice(&mut self, iplayer: &mut IPlayer);
    }

    impl Behavior for IPlayer {
        fn play(&mut self, shall_debug: bool) {
            self.roll_dice();
            match self.playstyle {
                Some(Playstyle::Aggressive) => self.aggro(),
                // Playstyle::Fast => self.fast(),
                Some(Playstyle::Random) => self.random(),
                // Playstyle::Safe => self.safe(),
                Some(Playstyle::FastAggressive) => self.fast_aggro(),
                // Playstyle::GeneticAlgorithm => self.genetic(),
                Some(_) => self.random(),
                None => panic!("Player has no playstyle. Please give it with some playstyle"),
            }
            self.take_action(shall_debug);
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
    }
    
    impl IPlayer {

        pub fn create(id: i8, playstyle: Playstyle) -> Self {
            IPlayer {
                player : Player::new(id),
                playstyle: Some(playstyle.clone()),
                actions: get_action_from_playstyle(playstyle),
                dice_number: 0,
            }
        }

        pub fn new(id: i8) -> Self {
            let player = Player::new(id);
            IPlayer {
                player,
                playstyle : None,
                actions : None,
                dice_number: 0,
            }
        }

        pub fn set_actions(&mut self, actions: Vec<Act>) {
            self.actions = Some(actions);
        }

        pub fn set_playstyle(&mut self, playstyle: Playstyle) {
            self.actions = get_action_from_playstyle(playstyle.clone());
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

        pub fn get_actions(&self) -> &Vec<Act> {
            if let Some(actions) = &self.actions {
                actions
            } else {
                panic!("Player has no actions. Please give it with some actions")
            }
        }



        fn aggro(&mut self) {
            self.choose_ordered_action(players::Select::Random);
        }

        fn fast_aggro(&mut self) {
            self.choose_ordered_action(players::Select::Nearest);
        }

        fn choose_ordered_action(&mut self, select: Select) {
            self.player.action = self.player.get_ordered_action(self.get_actions().clone(), self.dice_number, select);
        }

        fn random(&mut self) {
            self.choose_random_action();
        }

        fn choose_random_action(&mut self) {
            let movesets = self.player.generate_vector_of_random_actions(self.get_actions().clone(), self.dice_number);
            self.player.action = self.player.select_random_piece(movesets);
        }

        fn take_action(&mut self, debug: bool) {
            if debug {
                self.log_moves();
            } else {
                self.player.make_move(self.player.action.1, self.dice_number, self.player().action.0);
            }
        }

        fn log_moves(&mut self) {
            println!("\n\n------------------------");
            println!("Prior play\n");
            self.player.print_status();
            self.player.make_move(self.player.action.1, self.dice_number, self.player().action.0);
            println!("Posterior play\n");
            self.player.print_status();
        }
    }

    fn get_action_from_playstyle(playstyle: Playstyle) -> Option<Vec<Act>> {
        match playstyle {
            Playstyle::Aggressive => Some(AGGRO_ACTIONS.to_vec()),
            Playstyle::Fast => Some(FAST_ACTIONS.to_vec()),
            Playstyle::Random => Some(DEFAULT_ACTIONS.to_vec()),
            Playstyle::Safe => Some(SAFE_ACTIONS.to_vec()),
            Playstyle::FastAggressive => Some(FAST_AGGRO_ACTIONS.to_vec()),
            Playstyle::GeneticAlgorithm => None,
        }
    }

}

pub use iplayers::{IPlayer, Playstyle, Behavior};