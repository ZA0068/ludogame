mod ai {
    use game::Game;
    use iplayers::{IPlayer, Playstyle, ACTIONS, SELECTIONS};
    use players::{Act, Select};
    use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
    use std::collections::HashSet;

    pub struct GeneticAlgorithm {
        population: Vec<IPlayer>,
        evaluator: Game,
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        elitism_count: usize,
        tournament_size: usize,
        rng: ThreadRng,
    }

    impl GeneticAlgorithm {
        pub fn new() -> GeneticAlgorithm {
            GeneticAlgorithm {
                population: Vec::new(),
                evaluator: Game::new(),
                population_size: 0,
                mutation_rate: 0.0,
                crossover_rate: 0.0,
                elitism_count: 0,
                tournament_size: 0,
                rng: thread_rng(),
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

                let mut actions = ACTIONS;
                actions.shuffle(&mut rand::thread_rng());
                iplayer.set_actions(ACTIONS);

                let select = SELECTIONS.choose(&mut rand::thread_rng()).unwrap();
                iplayer.select_which_piece(*select);

                iplayer.select_which_piece(Select::Nearest);
                self.population.push(iplayer);
            }
        }

        pub fn select_the_best_populations_among_all(&mut self) {
            let mut best_population: Option<IPlayer> = None;
            let mut second_best_population: Option<IPlayer> = None;

            for population in &self.population {
                if best_population.is_none()
                    || population.get_winrate() > best_population.as_ref().unwrap().get_winrate()
                {
                    second_best_population = best_population.take();
                    best_population = Some(population.clone());
                } else if second_best_population.is_none()
                    || population.get_winrate()
                        > second_best_population.as_ref().unwrap().get_winrate()
                {
                    second_best_population = Some(population.clone());
                }
            }

            self.population.clear();
            if let Some(best) = best_population {
                self.population.push(best);
            }
            if let Some(second_best) = second_best_population {
                self.population.push(second_best);
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
            let mut evaluator = std::mem::replace(&mut self.evaluator, Game::new());
            for population in self.population.iter_mut() {
                Self::evaluate(&mut evaluator, population, total_games);
                population.calculate_winrate(total_games);
            }
        }

        fn evaluate(evaluator: &mut Game, population: &mut IPlayer, total_games: u16) {
            evaluator.set_iplayer(0, population);
            evaluator.start_game(total_games);
            evaluator.get_iplayer(0, population);
        }

        pub fn create_children_and_replace_bad_populationa(&mut self) {
            let first_parent = self.population[0].clone();
            let second_parent = self.population[1].clone();
            let mut children: Vec<IPlayer> = Vec::new();
            let mut rng = thread_rng();

            for i in 0..(self.population_size - self.elitism_count) {
                let parent_actions_1 = first_parent.get_actions();
                let parent_actions_2 = second_parent.get_actions();

                let action3 = self.try_to_perform_crossover(parent_actions_1, parent_actions_2);
            }
        }

        pub fn try_to_perform_crossover(
            &mut self,
            parent_actions_1: &[Act; 10],
            parent_actions_2: &[Act; 10],
        ) -> [Act; 10] {
            let mut rng = thread_rng();
            let crossover_rate = rng.gen_range(0.0..1.0);
            if crossover_rate < self.crossover_rate {
                self.crossover(parent_actions_1, parent_actions_2)
            } else {
                self.inherit_from_parents(parent_actions_2, parent_actions_1)
            }
        }

        fn two_point_crossover(&mut self, parent1: &[Act; 10], parent2: &[Act; 10]) -> [Act; 10] {
            let crossover_point1 = self.rng.gen_range(0..10);
            let crossover_point2 = self.rng.gen_range(crossover_point1..10);
            let mut child = ACTIONS;
            for i in 0..crossover_point1 {
                child[i] = parent1[i];
            }
            for i in crossover_point1..crossover_point2 {
                child[i] = parent2[i];
            }
            for i in crossover_point2..10 {
                child[i] = parent1[i];
            }
            child
        }

        fn single_point_crossover(
            &mut self,
            parent1: &[Act; 10],
            parent2: &[Act; 10],
        ) -> [Act; 10] {
            let crossover_point = self.rng.gen_range(0..10);
            let mut child = ACTIONS;
            for i in 0..crossover_point {
                child[i] = parent1[i];
            }
            for i in crossover_point..10 {
                child[i] = parent2[i];
            }
            child
        }

        fn uniform_crossover(
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

        fn inherit_from_parents(
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

        fn crossover(
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

        pub fn remove_duplicates(&mut self, actions: [Act; 10]) -> [Act; 10] {
            let mut actions = actions.to_vec();
            let mut seen = HashSet::new();
            actions.retain(|&x| seen.insert(x));
            if actions.len() < ACTIONS.len() {
                for action in &ACTIONS {
                    if !seen.contains(action) {
                        actions.push(*action);
                    }
                }
            }
            actions.try_into().unwrap()
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
                rng: thread_rng(),
            }
        }
    }
}

pub use ai::GeneticAlgorithm;
