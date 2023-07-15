use ai::GeneticAlgorithm;

#[cfg(test)]
mod GeneticAlgorithmTest {
    use super::*;

    #[test]
    fn init_test() {
        let mut ga = GeneticAlgorithm::new(10, 0.01, 0.95, 2, 5);
        assert_eq!(ga.population_size(), 10);
        assert_eq!(ga.mutation_rate(), 0.01);
        assert_eq!(ga.crossover_rate(), 0.95);
        assert_eq!(ga.elitism_count(), 2);
        assert_eq!(ga.tournament_size(), 5);
    }
}