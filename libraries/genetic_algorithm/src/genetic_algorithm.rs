mod genetic_algorithm {
    use game::Game;
    use iplayers::{IPlayer, Playstyle, ACTIONS, SELECTIONS};
    use players::{Act, Select};
    use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

    pub enum CrossoverType {
        SinglePoint,
        TwoPoint,
        Uniform,
    }

    pub struct GeneticAlgorithm {
        population: Vec<IPlayer>,
        data: Vec<(usize, Vec<IPlayer>)>,
        evaluator: Box<Game>,
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        elitism_count: usize,
        tournament_size: usize,
        total_games: u16,
        write_to_csv: bool,
        csv_name: String,
        rng: ThreadRng,
    }

    impl GeneticAlgorithm {
        pub fn new() -> GeneticAlgorithm {
            GeneticAlgorithm {
                population: Vec::new(),
                data: Vec::new(),
                evaluator: Box::default(),
                population_size: 0,
                mutation_rate: 0.0,
                crossover_rate: 0.0,
                elitism_count: 0,
                tournament_size: 0,
                total_games: 100,
                write_to_csv: false,
                csv_name: "GA data".to_string(),
                rng: thread_rng(),
            }
        }

        pub fn run_gentic_algorithm(&mut self) {
            self.initialize_all_populations();
            for tournament_size in 0..self.tournament_size {
                self.evaluate_fitness_for_all_populations(tournament_size);
                self.select_best_populations();
                self.create_children_and_replace_bad_populations();
            }
            self.evaluate_fitness_for_all_populations(self.tournament_size);
            if self.write_to_csv {
                self.export_2_csv();
            }
        }

        pub fn total_games(&mut self, total_games: u16){
            self.total_games = total_games;
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
            self.evaluator = Box::new(evaluator);
        }

        pub fn initialize_all_populations(&mut self) {
            if self.population_size == 0 {
                panic!("Population size is 0. Please set the population size");
            }

            self.population.clear();

            for _ in 0..self.population_size {
                let mut iplayer = IPlayer::new(0);
                iplayer.set_playstyle(Playstyle::GeneticAlgorithm);

                let mut actions = ACTIONS;
                actions.shuffle(&mut rand::thread_rng());
                iplayer.set_actions(ACTIONS);

                let select = SELECTIONS.choose(&mut rand::thread_rng()).unwrap();
                iplayer.select_which_piece(*select);

                iplayer.select_which_piece(Select::Nearest);
                self.population.push(iplayer);
            }
        }

        pub fn select_best_populations(&mut self) {
            if self.elitism_count == 0 {
                panic!("Elitism count must be greater than 0");
            }
            self.population
                .sort_unstable_by(|a, b| b.get_winrate().partial_cmp(a.get_winrate()).unwrap());
            self.population.truncate(self.elitism_count);
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

        pub fn set_csv_name(&mut self, csv_name: &str) {
            self.csv_name = csv_name.to_string();
        }

        pub fn set_write_to_csv(&mut self, write_to_csv: bool) {
            self.write_to_csv = write_to_csv;
        }

        pub fn set_tournament_size(&mut self, tournament_size: usize) {
            self.tournament_size = tournament_size;
        }

        pub fn evaluate_fitness_for_all_populations(&mut self, tournament_size: usize) {
            for population in self.population.iter_mut() {
                self.evaluator.set_iplayer(0, population);
                self.evaluator.start_game(self.total_games);
                self.evaluator.get_iplayer(0, population);
                population.calculate_winrate(self.total_games);
                population.print_winrate();
            }
            if self.write_to_csv {
                self.data.push((tournament_size, self.population.clone()));
            }
        }

        pub fn create_children_and_replace_bad_populations(&mut self) {
            let first_parent = self.population[0].clone();
            let second_parent = self.population.choose(&mut self.rng).unwrap().clone();
            let mut children: Vec<IPlayer> = Vec::new();
            for _i in 0..(self.population_size - self.elitism_count) {
                let parent_actions_1 = first_parent.get_actions();
                let parent_selector_1 = first_parent.get_piece_selector();
                let parent_selector_2 = first_parent.get_piece_selector();
                let parent_actions_2 = second_parent.get_actions();

                let child_action =
                    self.try_to_crossover_actions(parent_actions_1, parent_actions_2);
                let child_action_mutated = self.mutate_actions(child_action);
                let child_selector =
                    self.try_to_crossover_selector(parent_selector_1, parent_selector_2);
                let child_selector_mutated = self.try_to_mutate_selector(child_selector);
                children.push(self.create_child(child_action_mutated, child_selector_mutated));
            }
            self.population.append(&mut children);
        }

        pub fn export_2_csv(&mut self) {
            let mut wtr = csv::Writer::from_path(format!("data/{}.csv", self.csv_name)).unwrap();
            let headers: Vec<String> = (0..=self.population_size)
                .flat_map(|i| {
                    if i == 0 {
                        vec!["tournament:".to_string()]
                    } else {
                        vec![
                            format!("population {} winrate", i),
                            format!("population {} select", i),
                            format!("population {} actions", i),
                        ]
                    }
                })
                .collect();
        
            wtr.write_record(&headers).unwrap();
        
            // Write the data rows
            for (tournament_index, iplayers) in &self.data {
                let row: Vec<String> = std::iter::once(format!("{}:", tournament_index))
                    .chain(iplayers.iter().flat_map(|iplayer| {
                        vec![
                            iplayer.get_winrate().to_string(),
                            iplayer.select_which_piece.to_string(),
                            iplayer.actions.as_ref().map(|actions| {
                                actions.iter().map(Act::to_string).collect::<Vec<_>>().join(", ")
                            }).unwrap_or_default(),
                        ]
                    }))
                    .collect();
        
                wtr.write_record(&row).unwrap();
            }
        
            wtr.flush().unwrap();
        }
        

        pub fn try_to_crossover_selector(
            &mut self,
            selector1: &Select,
            selector2: &Select,
        ) -> Select {
            let mut rng = thread_rng();
            let crossover_rate = rng.gen_range(0.0..1.0);
            if crossover_rate < self.crossover_rate {
                self.crossover_selector(selector1, selector2)
            } else {
                self.inherit_from_parents_selector(selector1, selector2)
            }
        }

        pub fn inherit_from_parents_selector(
            &mut self,
            selector1: &Select,
            selector2: &Select,
        ) -> Select {
            let parent_inheritance = self.rng.gen_bool(0.5);
            if parent_inheritance {
                *selector1
            } else {
                *selector2
            }
        }

        pub fn crossover_selector(&mut self, selector1: &Select, selector2: &Select) -> Select {
            let crossover_point = self.rng.gen_range(0..3);
            match crossover_point {
                0 => *selector1,
                1 => *selector2,
                2 => Select::Random,
                _ => panic!("Invalid crossover point"),
            }
        }

        pub fn try_to_mutate_selector(&mut self, selector: Select) -> Select {
            let mut rng = thread_rng();
            let mutation_rate = rng.gen_range(0.0..1.0);
            if mutation_rate < self.mutation_rate {
                self.mutate_selector(selector)
            } else {
                selector
            }
        }

        pub fn mutate_selector(&mut self, selector: Select) -> Select {
            let mut rng = thread_rng();
            loop {
                let new_selector_int = rng.gen_range(0..=2);
                let new_selector = match new_selector_int {
                    0 => Select::Nearest,
                    1 => Select::Furthest,
                    2 => Select::Random,
                    _ => panic!("Invalid selector"),
                };
                if new_selector != selector {
                    return new_selector;
                }
            }
        }
        

        pub fn create_child(&mut self, actions: [Act; 10], selector: Select) -> IPlayer {
            let mut iplayer = IPlayer::new(0);
            iplayer.set_playstyle(Playstyle::GeneticAlgorithm);
            iplayer.set_actions(actions);
            iplayer.select_which_piece(selector);
            iplayer
        }

        pub fn try_to_crossover_actions(
            &mut self,
            parent_actions_1: &[Act; 10],
            parent_actions_2: &[Act; 10],
        ) -> [Act; 10] {
            let mut rng = thread_rng();
            let crossover_rate = rng.gen_range(0.0..1.0);
            if crossover_rate < self.crossover_rate {
                self.crossover_actions(parent_actions_1, parent_actions_2)
            } else {
                self.inherit_from_parents(parent_actions_2, parent_actions_1)
            }
        }

        fn two_point_crossover(&mut self, parent1: &[Act; 10], parent2: &[Act; 10]) -> [Act; 10] {
            let crossover_point1: usize = self.rng.gen_range(0..10);
            let crossover_point2 = self.rng.gen_range(crossover_point1..10);
            let mut child = ACTIONS;
            child[0..crossover_point1].copy_from_slice(&parent1[0..crossover_point1]);
            child[crossover_point1..crossover_point2].copy_from_slice(&parent2[crossover_point1..crossover_point2]);
            child[crossover_point2..10].copy_from_slice(&parent1[crossover_point2..10]);
            child
        }
        
        fn single_point_crossover(&mut self, parent1: &[Act; 10], parent2: &[Act; 10]) -> [Act; 10] {
            let crossover_point = self.rng.gen_range(0..10);
            let mut child = ACTIONS;
            child[0..crossover_point].copy_from_slice(&parent1[0..crossover_point]);
            child[crossover_point..10].copy_from_slice(&parent2[crossover_point..10]);
            child
        }
        
        pub fn uniform_crossover(
            &mut self,
            first_parent_actions: &[Act; 10],
            second_parent_actions: &[Act; 10],
        ) -> [Act; 10] {
            let bitmask: u16 = self.rng.gen_range(0..1024);
            let bitmask_array: [u8; 10] = [
                ((bitmask >> 9) & 1) as u8,
                ((bitmask >> 8) & 1) as u8,
                ((bitmask >> 7) & 1) as u8,
                ((bitmask >> 6) & 1) as u8,
                ((bitmask >> 5) & 1) as u8,
                ((bitmask >> 4) & 1) as u8,
                ((bitmask >> 3) & 1) as u8,
                ((bitmask >> 2) & 1) as u8,
                ((bitmask >> 1) & 1) as u8,
                (bitmask & 1) as u8,
            ];
            let mut result: [Act; 10] = ACTIONS; // Initialize with any valid Act.
            for i in 0..10 {
                result[i] = if bitmask_array[i] == 1 {
                    first_parent_actions[i]
                } else {
                    second_parent_actions[i]
                };
            }
            result
        }

        pub fn inherit_from_parents(
            &mut self,
            parent_actions_2: &[Act; 10],
            parent_actions_1: &[Act; 10],
        ) -> [Act; 10] {
            let parent_inheritance = self.rng.gen_bool(0.5);
            if parent_inheritance {
                *parent_actions_2
            } else {
                *parent_actions_1
            }
        }

        fn crossover_actions(
            &mut self,
            parent_actions_1: &[Act; 10],
            parent_actions_2: &[Act; 10],
        ) -> [Act; 10] {
            let crossover_type = self.rng.gen_range(0..=2);
            match crossover_type {
                0 => self.single_point_crossover(parent_actions_1, parent_actions_2),
                1 => self.uniform_crossover(parent_actions_1, parent_actions_2),
                2 => self.two_point_crossover(parent_actions_1, parent_actions_2),
                _ => panic!("Invalid crossover type"),
            }
        }

        pub fn mutate_actions(&mut self, actions: [Act; 10]) -> [Act; 10] {
            let mut mutated_actions = actions;
            for action in mutated_actions.iter_mut() {
                let mutation_rate = self.rng.gen_range(0.0..1.0);
                if mutation_rate < self.mutation_rate {
                    *action = *ACTIONS.choose(&mut self.rng).unwrap();
                }
            }
            mutated_actions
        }
        
    }

    impl Default for GeneticAlgorithm {
        fn default() -> Self {
            let mut game = Game::new();
            game.setup_game();
            game.give_iplayer_a_playstyle(0, Playstyle::GeneticAlgorithm);
            game.give_iplayer_a_playstyle(1, Playstyle::Random);
            game.give_iplayer_a_playstyle(2, Playstyle::Fast);
            game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
            let evaluator = Box::new(game);
            GeneticAlgorithm {
                population: Vec::new(),
                data: Vec::new(),
                evaluator,
                population_size: 10,
                mutation_rate: 0.01,
                crossover_rate: 0.95,
                elitism_count: 2,
                tournament_size: 5,
                total_games: 100,
                write_to_csv: false,
                csv_name: "GA data".to_string(),
                rng: thread_rng(),
            }
        }
    }
}

pub use genetic_algorithm::{CrossoverType, GeneticAlgorithm};
