use game::Game;
use genetic_algorithm::GeneticAlgorithm;

fn run_ga_control() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.total_games(100);
    ga.set_write_to_csv(true);
    ga.run_gentic_algorithm();
}

fn run_ga_1() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.1);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.total_games(100);
    ga.set_write_to_csv(true);
    ga.run_gentic_algorithm();
}

fn run_ga_2() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.1);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.total_games(100);
    ga.set_write_to_csv(true);
    ga.run_gentic_algorithm();
}

fn run_ga_3() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.total_games(200);
    ga.set_write_to_csv(true);
    ga.run_gentic_algorithm();
}

fn run_ga_4() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.1);
    ga.set_crossover_rate(0.9);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.total_games(100);
    ga.set_write_to_csv(true);
    ga.run_gentic_algorithm();
}

fn run_ga_5() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.9);
    ga.set_crossover_rate(0.1);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.total_games(100);
    ga.run_gentic_algorithm();
}

fn run_ga_6() {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(10);
    ga.total_games(100);
    ga.run_gentic_algorithm();
}

fn main() {
    run_ga_control();
    // run_ga_1();
    // // run_ga_2();
    // // run_ga_3();
    // // run_ga_4();
    // // run_ga_5();
    // run_ga_6();
}
