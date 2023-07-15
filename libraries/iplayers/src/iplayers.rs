mod iplayers {
    use players::{Act, Player};
    use dice::Dice;

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
        actions: Vec<Act>,
    }

    pub trait Behavior {
        fn play(&mut self, is_in_debug_mode: bool);
        fn take_dice(&mut self, dice: Dice);
        fn roll_dice(&mut self) -> i8;
    }

    impl Behavior for IPlayer {
        fn play(&mut self, is_in_debug_mode: bool) {
            match self.playstyle {
                // Playstyle::Aggressive => self.aggro(),
                // Playstyle::Fast => self.fast(),
                Some(Playstyle::Random) => self.random(is_in_debug_mode),
                // Playstyle::Safe => self.safe(),
                // Playstyle::FastAggressive => self.fast_aggro(),
                // Playstyle::GeneticAlgorithm => self.genetic(),
                Some(_) => self.random(is_in_debug_mode),
                None => panic!("Player has no playstyle. Please give it with some playstyle"),
            }
        }

        fn roll_dice(&mut self) -> i8 {
            self.player.roll_dice();
            let dice_number = self.player.get_dice_number();
            if dice_number == 0 {
                panic!("This player doesn't have the dice!");
            }
            dice_number
        }

        fn take_dice(&mut self, dice: Dice) {
            self.player.get_dice(dice);
        }
    }
    
    impl IPlayer {

        pub fn create(id: i8, playstyle: Playstyle, actions: Vec<Act>) -> Self {
            let player = Player::new(id);
            let playstyle = Some(playstyle);
            IPlayer {
                player,
                playstyle,
                actions,
            }
        }

        pub fn new(id: i8) -> Self {
            let player = Player::new(id);
            IPlayer {
                player,
                playstyle : None,
                actions : Vec::new(),
            }
        }

        pub fn add_actions(&mut self, actions: Vec<Act>) {
            self.actions = actions;
        }

        pub fn set_playstyle(&mut self, playstyle: Playstyle) {
            self.playstyle = Some(playstyle);
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
            &self.actions
        }

        pub fn random(&mut self, debug : bool) {
            let dice_number = self.roll_dice();
            let movesets = self.player.generate_vector_of_random_actions(self.actions.clone(), dice_number);
            self.player.action = self.player.select_random_piece(movesets);
            if debug {
                println!("\n\n------------------------");
                println!("Prior play\n");
                self.player.print_status();
                self.player.make_move(self.player.action.1, dice_number, self.player().action.0);
                println!("Posterior play\n");
                self.player.print_status();
            } else {
                self.player.make_move(self.player.action.1, dice_number, self.player().action.0);
            }
        }
    }

}

pub use iplayers::{IPlayer, Playstyle, Behavior};