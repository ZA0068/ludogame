use genetic_algorithm::GeneticAlgorithm;
use std::process::Command;


fn run_ga_control(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_write_to_csv(true);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_0(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(10);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(10);
    ga.set_total_games(10);
    ga.set_write_to_csv(true);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_1(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.1);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_write_to_csv(true);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_2(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.1);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_write_to_csv(true);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_3(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(1.0);
    ga.set_crossover_rate(1.0);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_write_to_csv(true);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_4(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(200);
    ga.set_write_to_csv(true);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_5(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.95);
    ga.set_crossover_rate(0.01);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_6(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.01);
    ga.set_crossover_rate(0.95);
    ga.set_elitism_count(2);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn run_ga_7(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.5);
    ga.set_crossover_rate(0.5);
    ga.set_elitism_count(10);
    ga.set_tournament_size(100);
    ga.set_total_games(100);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}


fn run_ga_8(filename: &str) {
    let mut ga = GeneticAlgorithm::default();
    ga.set_population_size(100);
    ga.set_mutation_rate(0.01);
    ga.set_crossover_rate(0.95);
    ga.set_elitism_count(10);
    ga.set_tournament_size(100);
    ga.set_total_games(1000);
    ga.set_csv_name(filename);
    ga.run_gentic_algorithm();
}

fn main() {

    let filename = "GA control test";
    run_ga_control(filename);
    plot_data(filename);

    let filename = "GA test 0 - 10pop10gen10game";
    run_ga_0(filename);
    plot_data(filename);
    
    let filename = "GA test 1 - 0.1mut";
    run_ga_1(filename);
    plot_data(filename);
    
    let filename = "GA test 2 - 0.1cross";
    run_ga_2(filename);
    plot_data(filename);

    let filename = "GA test 3 - 1mut1cross";
    run_ga_3(filename);
    plot_data(filename);

    let filename = "GA test 4 - 200game";
    run_ga_4(filename);
    plot_data(filename);

    let filename = "GA test 5 - 0.95mut0.01cross";
    run_ga_5(filename);
    plot_data(filename);

    let filename = "GA test 6 - 0.01mut0.95cross";
    run_ga_6(filename);
    plot_data(filename);

    let filename = "GA test 7 - 10elites";
    run_ga_7(filename);
    plot_data(filename);


    let filename = "GA test 8 - final";
    run_ga_8(filename);
    plot_data(filename);

}

fn plot_data(filename: &str) {
    let output = Command::new("python")
                         .arg("./src/plot.py")
                         .arg(filename)
                         .output()
                         .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}