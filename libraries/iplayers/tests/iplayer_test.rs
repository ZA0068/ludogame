use iplayers::{IPlayer, Playstyle, Behavior};
use players::{Act, Player};
use dice::Dice;

static AGGRO_ACTIONS: [Act; 10] = [
    Act::Kill,
    Act::Move,
    Act::Join,
    Act::Free,
    Act::Goal,
    Act::Starjump,
    Act::Leave,
    Act::Safe,
    Act::Nothing,
    Act::Die,
];

static FAST_AGGRO_ACTIONS: [Act; 10] = [
    Act::Kill,
    Act::Goal,
    Act::Starjump,
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
    Act::Starjump,
    Act::Free,
    Act::Leave,
    Act::Nothing,
    Act::Die,
];

static FAST_ACTIONS: [Act; 10] = [
    Act::Goal,
    Act::Starjump,
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
    Act::Starjump,
    Act::Nothing,
];

#[cfg(test)]
mod iplayer_tests {
    use super::*;

    #[test]
    fn instantiation_test() {
        let mut player = IPlayer::new(0);
        assert_eq!(player.player().id(), 0);
        
        let actions = DEFAULT_ACTIONS.to_vec();
        player.add_actions(actions);
        assert_eq!(player.get_actions(), &DEFAULT_ACTIONS.to_vec());

        let playstyle = Playstyle::Random;
        player.set_playstyle(playstyle);
        assert_eq!(player.get_playstyle(), &Playstyle::Random);
    }

    #[test]
    fn play_test() {
        let mut player = IPlayer::new(0);
        let playstyle = Playstyle::Random;
        player.set_playstyle(playstyle);
        player.play(true);


}
}