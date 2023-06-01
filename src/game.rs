mod game {
    use board::Board;
    use csv::Writer;
    use dice::Dice;
    use players::{Act, Player};
    use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
    use std::fs::File;

    use std::{cell::RefCell, rc::Rc};

    const NUM_GENERATIONS: usize = 50;
    const POPULATION_SIZE: usize = 10;

    static AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Goal,
        Act::Skip,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];
    static FAST_AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Goal,
        Act::Skip,
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
        Act::Skip,
        Act::Free,
        Act::Leave,
        Act::Nothing,
        Act::Die,
    ];
    static FAST_ACTIONS: [Act; 10] = [
        Act::Goal,
        Act::Skip,
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
        Act::Skip,
        Act::Nothing,
    ];
    #[derive(Clone, Debug, PartialEq)]
    pub struct IPlayer {
        player: Player,
        aggresive_actions: [Act; 10],
        fast_actions: [Act; 10],
        safe_actions: [Act; 10],
        fast_aggro_actions: [Act; 10],
        genetic_actions: Vec<Act>,
    }

    impl IPlayer {
        pub fn new(player: Player) -> Self {
            IPlayer {
                player,
                aggresive_actions: AGGRO_ACTIONS,
                fast_actions: FAST_ACTIONS,
                safe_actions: SAFE_ACTIONS,
                fast_aggro_actions: FAST_AGGRO_ACTIONS,
                genetic_actions: DEFAULT_ACTIONS.to_vec(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Game {
        iplayers: Vec<IPlayer>,
        board: Rc<RefCell<Board>>,
        dice: Rc<RefCell<Dice>>,
    }

    unsafe impl Send for Game {}

    unsafe impl Sync for Game {}

    trait Playstyles {
        fn aggressive(&mut self);
        fn fast(&mut self);
        fn random(&mut self);
        fn safe(&mut self);
        fn fast_aggressive(&mut self);
        fn genetic(&mut self, actions: Vec<Act>);
    }

    impl Playstyles for IPlayer {
        fn aggressive(&mut self) {
            self.player.my_turn();
            self.player
                .ordered_play(self.aggresive_actions.to_vec(), true);
        }
        fn fast(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(self.fast_actions.to_vec(), true);
        }
        fn random(&mut self) {
            self.player.my_turn();
            self.player.random_play(DEFAULT_ACTIONS.to_vec());
        }
        fn safe(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(self.safe_actions.to_vec(), false);
        }
        fn fast_aggressive(&mut self) {
            self.player.my_turn();
            self.player
                .ordered_play(self.fast_aggro_actions.to_vec(), true);
        }

        fn genetic(&mut self, actions: Vec<Act>) {
            self.player.my_turn();
            self.player.ordered_play(actions, true); // Pass `actions` to `ordered_play` instead of cloning `genetic_actions`
        }
    }

    impl Game {
        pub fn new() -> Self {
            let board = Rc::new(RefCell::new(Board::new()));
            let dice = Rc::new(RefCell::new(Dice::new()));
            let player0 = Player::new(0, board.clone(), Some(dice.clone()));
            let player1 = Player::new(1, board.clone(), Some(dice.clone()));
            let player2 = Player::new(2, board.clone(), Some(dice.clone()));
            let player3 = Player::new(3, board.clone(), Some(dice.clone()));
            Self {
                iplayers: vec![
                    IPlayer::new(player0),
                    IPlayer::new(player1),
                    IPlayer::new(player2),
                    IPlayer::new(player3),
                ],
                board,
                dice,
            }
        }
        pub fn actions_to_string(&mut self, actions: &[Act]) -> String {
            actions.iter().map(|act| format!("{}", act)).collect::<Vec<String>>().join(", ")
        }
        
        pub fn start_the_game(&mut self) {
            let result = self.genetic_algorithm();
        
            match result {
                Ok(all_generations) => {
                    let file_path = "generations.csv";
                    let file = File::create(file_path).expect("Unable to create file");
                    let mut wtr = Writer::from_writer(file);
        
                    wtr.write_record(&["generation", "individual", "fitness_score"])
                        .expect("Unable to write headers");
        
                    for (generation_num, (population, fitness_scores)) in
                        all_generations.iter().enumerate()
                    {
                        for (individual_num, fitness_score) in
                            fitness_scores.iter().enumerate()
                        {
                            wtr.write_record(&[
                                format!("{}", generation_num),
                                format!("{}", individual_num),
                                format!("{}", fitness_score),
                            ])
                            .expect("Unable to write record");
                        }
                    }
        
                    wtr.flush().expect("Unable to flush writer");
        
                    // Write out the action sequence of the winning last generation
                    let last_generation = all_generations.last().unwrap();
                    let last_fitness_scores = self.evaluate_population(&last_generation.0).unwrap();
                    let max_fitness_score_index = last_fitness_scores
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap()
                        .0;
        
                    let winning_sequence = &last_generation.0[max_fitness_score_index];
                    let file_path = "winning_sequence.csv";
                    let file = File::create(file_path).expect("Unable to create file");
                    let mut wtr = Writer::from_writer(file);
        
                    for act in winning_sequence {
                        wtr.write_record(&[self.actions_to_string(act)])
                            .expect("Unable to write record");
                    }
                    wtr.flush().expect("Unable to flush writer");
        
                    println!(
                        "The winning sequence of the last generation is: {:?}",
                        winning_sequence
                    );
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        

        fn genetic_algorithm(
            &mut self,
        ) -> Result<Vec<(Vec<Vec<Act>>, Vec<f32>)>, Box<dyn std::error::Error>> {
            // Initialization
            let mut population = self.initialize_population();
            let mut all_generations: Vec<(Vec<Vec<Act>>, Vec<f32>)> =
                Vec::with_capacity(NUM_GENERATIONS);

            for _ in 0..NUM_GENERATIONS {
                // Evaluation
                let fitness_scores = self.evaluate_population(&population)?;

                let parents = self.select_parents(&population, &fitness_scores);

                // Crossover and mutation
                let children = self.crossover_and_mutate(&parents);
                all_generations.push((children.clone(), fitness_scores.clone()));

                // Replacement
                population = self.replace_worst(&population, &children);
            }

            Ok(all_generations)
        }

        fn replace_worst(
            &mut self,
            population: &Vec<Vec<Act>>,
            children: &Vec<Vec<Act>>,
        ) -> Vec<Vec<Act>> {
            let mut rng = rand::thread_rng();
            let mut population = population.clone();
            for child in children {
                let index = rng.gen_range(0..POPULATION_SIZE);
                population[index] = child.clone();
            }
            population
        }

        fn select_parents(
            &mut self,
            population: &Vec<Vec<Act>>,
            fitness_scores: &Vec<f32>,
        ) -> Vec<Vec<Act>> {
            let mut parents = Vec::new();
            let mut rng = rand::thread_rng();

            for _ in 0..POPULATION_SIZE {
                let parent1_index = self.select_parent_index(fitness_scores, &mut rng);
                let parent2_index = self.select_parent_index(fitness_scores, &mut rng);

                let parent1 = population[parent1_index].clone();
                let parent2 = population[parent2_index].clone();

                parents.push(parent1);
                parents.push(parent2);
            }

            parents
        }

        fn select_parent_index(&mut self, fitness_scores: &Vec<f32>, rng: &mut ThreadRng) -> usize {
            let sum_fitness: f32 = fitness_scores.iter().sum();
            let selection_probabilities: Vec<f32> = fitness_scores
                .iter()
                .map(|&score| score / sum_fitness)
                .collect();

            let mut cumulative_probability = 0.0;
            let random_number = rng.gen_range(0.0..1.0);

            for (index, &probability) in selection_probabilities.iter().enumerate() {
                cumulative_probability += probability;
                if random_number <= cumulative_probability {
                    return index;
                }
            }

            0 // Default to the first parent if no index is found
        }

        fn crossover_and_mutate(&mut self, parents: &Vec<Vec<Act>>) -> Vec<Vec<Act>> {
            let mut children = Vec::new();
            let crossover_rate = 0.7;
            let mutation_rate = 0.001;

            self.crossover(parents, &mut children, crossover_rate);
            self.mutate_population(&mut children, mutation_rate);

            children
        }

        fn crossover(
            &self,
            parents: &Vec<Vec<Act>>,
            children: &mut Vec<Vec<Act>>,
            crossover_rate: f64,
        ) {
            let mut rng = rand::thread_rng();

            for i in 0..POPULATION_SIZE / 2 {
                let mut child = Vec::new();

                for j in 0..10 {
                    if rng.gen_range(0.0..1.0) < crossover_rate {
                        child.push(parents[i][j]);
                    } else {
                        child.push(parents[i + POPULATION_SIZE / 2][j]);
                    }
                }

                children.push(child);
            }
        }

        fn mutate_population(&mut self, children: &mut Vec<Vec<Act>>, mutation_rate: f64) {
            let mut rng = rand::thread_rng();

            for i in 0..children.len() {
                for j in 0..children[i].len() {
                    if rng.gen_range(0.0..1.0) < mutation_rate {
                        children[i][j] = DEFAULT_ACTIONS[rng.gen_range(0..DEFAULT_ACTIONS.len())];
                    }
                }
            }
        }

        fn initialize_population(&mut self) -> Vec<Vec<Act>> {
            let mut population = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0..POPULATION_SIZE {
                let mut actions = DEFAULT_ACTIONS.to_vec().clone();
                actions.shuffle(&mut rng);
                population.push(actions);
            }
            population
        }

        fn evaluate_population(
            &mut self,
            population: &[Vec<Act>],
        ) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
            let mut fitness_scores = Vec::new();

            for individual in population {
                let mut winrate = vec![0.0; 5];

                for _ in 0..1000 {
                    match self.play_game(individual) {
                        Ok(result) => winrate[result as usize] += 1.0,
                        Err(_) => continue, // if there is an error, we skip this game and move to the next
                    }
                }

                let total_games = winrate.iter().sum::<f32>();
                let normalized_winrate: Vec<f32> =
                    winrate.iter().map(|&score| score / total_games).collect();
                fitness_scores.push(normalized_winrate[0]); // Assuming fitness score is based on player 0's win rate
            }
            Ok(fitness_scores)
        }

        fn play_game(&mut self, genetic_action: &[Act]) -> Result<i8, Box<dyn std::error::Error>> {
            loop {
                self.iplayers[0].genetic(genetic_action.to_vec());
                self.iplayers[2].safe();
                self.iplayers[1].fast();
                self.iplayers[3].aggressive();

                match self
                    .iplayers
                    .iter_mut()
                    .find(|iplayer| iplayer.player.is_finished())
                {
                    Some(iplayer) => return Ok(iplayer.player.id()),
                    None => continue,
                }
            }
        }

        pub fn first_round(&mut self) {
            let mut rng = rand::thread_rng();
            for iplayer in &mut self.iplayers {
                let mut roll_count = 0;
                while roll_count < 3 {
                    if iplayer.player.roll_dice() == 6 {
                        iplayer.player.free_piece(rng.gen_range(0..4));
                        break;
                    }
                    roll_count += 1;
                }
            }
        }

        pub fn beginning(&mut self) {
            let mut scores = Vec::new();

            for (index, iplayer) in self.iplayers.iter_mut().enumerate() {
                scores.push((index, iplayer.player.roll_dice()));
            }

            scores.sort_by(|a, b| b.1.cmp(&a.1));

            let mut scores_with_duplicates = scores.clone();

            let mut duplicate_scores = std::collections::HashSet::new();
            for &(_, score) in &scores_with_duplicates {
                if self.count_score_occurrences(&scores_with_duplicates, score) > 1 {
                    duplicate_scores.insert(score);
                }
            }
            scores_with_duplicates.retain(|&(_, score)| !duplicate_scores.contains(&score));

            let new_order: Vec<usize> = scores_with_duplicates
                .iter()
                .map(|&(index, _)| index)
                .collect();

            self.iplayers.sort_by_key(|iplayer| {
                new_order
                    .iter()
                    .position(|&x| x == iplayer.player.id() as usize)
                    .unwrap_or(0)
            });
        }

        fn count_score_occurrences(&self, scores: &[(usize, i8)], score: i8) -> usize {
            scores.iter().filter(|&(_, s)| *s == score).count()
        }
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use game::Game;
