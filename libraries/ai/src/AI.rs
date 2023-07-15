mod ai {
    use players::{Act, Select};
    use game::Game;

    pub struct GeneticAlgorithm {
        population: Vec<(Act, Select)>,
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        elitism_count: usize,
        tournament_size: usize,
    }
    
    impl GeneticAlgorithm {
        pub fn new(population_size: usize, mutation_rate: f64, crossover_rate: f64, elitism_count: usize, tournament_size: usize) -> GeneticAlgorithm {
            GeneticAlgorithm {
                population: Vec::new(),
                population_size,
                mutation_rate,
                crossover_rate,
                elitism_count,
                tournament_size,
            }
        }
    
        pub fn population(&self) -> &Vec<(Act, Select)> {
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
    
        pub fn set_population(&mut self, population: Vec<(Act, Select)>) {
            self.population = population;
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
    }
}

pub use ai::GeneticAlgorithm;