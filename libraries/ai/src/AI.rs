mod ai {
    use std::collections::HashSet;
    use game::Game;
    use iplayers::{IPlayer, Playstyle, ACTIONS, SELECTIONS};
    use players::{Act, Select};
    use rand::{seq::SliceRandom, thread_rng};

    pub struct GeneticAlgorithm {
        population: Vec<IPlayer>,
        evaluator: Game,
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        elitism_count: usize,
        tournament_size: usize,
    }

    impl GeneticAlgorithm {
        pub fn new(
        ) -> GeneticAlgorithm {
            GeneticAlgorithm {
                population: Vec::new(),
                evaluator: Game::new(),
                population_size: 0,
                mutation_rate: 0.0,
                crossover_rate: 0.0,
                elitism_count: 0,
                tournament_size: 0,
            }
        }

        pub fn population(&self) -> &Vec<IPlayer> {
            &self.population
        }

        pub fn population_size(&self) -> usize {
            self.population_size
        }

        pub fn mutation_rate(&self) -> f64 {
            self.mutation_rate
        }

        pub fn crossover_rate(&self) -> f64 {
            self.crossover_rate
        }

        pub fn elitism_count(&self) -> usize {
            self.elitism_count
        }

        pub fn tournament_size(&self) -> usize {
            self.tournament_size
        }

        pub fn set_evaluator(&mut self, evaluator: Game) {
            self.evaluator = evaluator;
        }

        pub fn set_populations(&mut self) {
            if self.population_size == 0 {
                panic!("Population size is 0. Please set the population size");
            }

            self.population.clear();
            
            for _ in 0..self.population_size {
                let mut iplayer = IPlayer::new(0);
                iplayer.set_playstyle(Playstyle::GeneticAlgorithm);
                
                let mut actions = ACTIONS.to_vec();
                actions.shuffle(&mut rand::thread_rng());
                iplayer.set_actions(actions);
                
                let select = SELECTIONS.choose(&mut rand::thread_rng()).unwrap();
                iplayer.select_which_piece(*select);

                iplayer.select_which_piece(Select::Nearest);
                self.population.push(iplayer);
            }
        }

        pub fn set_population_size(&mut self, population_size: usize) {
            self.population_size = population_size;
        }

        pub fn set_mutation_rate(&mut self, mutation_rate: f64) {
            self.mutation_rate = mutation_rate;
        }

        pub fn set_crossover_rate(&mut self, crossover_rate: f64) {
            self.crossover_rate = crossover_rate;
        }

        pub fn set_elitism_count(&mut self, elitism_count: usize) {
            self.elitism_count = elitism_count;
        }

        pub fn set_tournament_size(&mut self, tournament_size: usize) {
            self.tournament_size = tournament_size;
        }

        pub fn evaluate_fitness_for_all_populations(&mut self) {
            let total_games = 100;
            for iplayer in &mut self.population {
                self.evaluator.set_iplayer(0, iplayer.clone());
                self.evaluator.start_game(total_games);
                iplayer.calculate_winrate(total_games);
                iplayer.print_winrate();
            }
        }

    }

    fn remove_duplicates(actions: &mut Vec<Act>) {
        let default_actions = ACTIONS.to_vec();

        let mut seen = HashSet::new();
        actions.retain(|&x| seen.insert(x));

        // If there are less actions than in the default_actions, fill in the missing ones
        if actions.len() < default_actions.len() {
            for action in &default_actions {
                if !seen.contains(action) {
                    actions.push(*action);
                }
            }
        }
    }
    impl Default for GeneticAlgorithm {
        fn default() -> Self {
            GeneticAlgorithm {
                population: Vec::new(),
                evaluator: Game::new(),
                population_size: 10,
                mutation_rate: 0.01,
                crossover_rate: 0.95,
                elitism_count: 2,
                tournament_size: 5,
            }
        }
    }
}

pub use ai::GeneticAlgorithm;
