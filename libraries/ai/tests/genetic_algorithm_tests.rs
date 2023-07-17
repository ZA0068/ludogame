use ai::GeneticAlgorithm;
use game::Game;
use iplayers::{IPlayer, Playstyle, ACTIONS, SELECTIONS};

#[cfg(test)]
mod genetic_algorithm_test {
    use std::borrow::BorrowMut;

    use super::*;

    #[test]
    fn init_test() {
        let mut ga = GeneticAlgorithm::new();

        ga.set_population_size(10);
        ga.set_mutation_rate(0.01);
        ga.set_crossover_rate(0.95);
        ga.set_elitism_count(2);
        ga.set_tournament_size(5);

        assert_eq!(ga.population_size(), 10);
        assert_eq!(ga.mutation_rate(), 0.01);
        assert_eq!(ga.crossover_rate(), 0.95);
        assert_eq!(ga.elitism_count(), 2);
        assert_eq!(ga.tournament_size(), 5);
    }

    #[test]
    fn init_test_2() {
        let ga = GeneticAlgorithm::default();
        assert_eq!(ga.population_size(), 10);
        assert_eq!(ga.mutation_rate(), 0.01);
        assert_eq!(ga.crossover_rate(), 0.95);
        assert_eq!(ga.elitism_count(), 2);
        assert_eq!(ga.tournament_size(), 5);
    }

    #[test]
    fn init_test_3() {
        let mut ga = GeneticAlgorithm::default();
        ga.set_populations();

        assert_eq!(ga.population_size(), 10);
        assert_eq!(ga.population().len(), 10);
        for iplayer in ga.population() {
            assert_eq!(iplayer.get_actions().len(), 10);
        }
        for iplayer in ga.population() {
            for action in &ACTIONS {
                assert!(iplayer.get_actions().contains(action));
            }
        }
        for iplayer in ga.population() {
            assert!(SELECTIONS.contains(iplayer.get_piece_selector()));
        }

        ga.set_population_size(100);
        ga.set_populations();

        assert_eq!(ga.population_size(), 100);
        assert_eq!(ga.population().len(), 100);
        for iplayer in ga.population() {
            assert_eq!(iplayer.get_actions().len(), 10);
        }
        for iplayer in ga.population() {
            for action in &ACTIONS {
                assert!(iplayer.get_actions().contains(action));
            }
        }
        for iplayer in ga.population() {
            assert!(SELECTIONS.contains(iplayer.get_piece_selector()));
        }
    }

    #[test]
    fn evaluation_test() {
        let mut ga = GeneticAlgorithm::new();
        let mut game = Game::new();
        game.setup_game();
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Fast);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        ga.set_evaluator(game);
        ga.set_population_size(10);
        ga.set_populations();
        ga.evaluate_fitness_for_all_populations();

    }
}
