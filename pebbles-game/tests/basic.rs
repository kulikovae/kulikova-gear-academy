use crate::pebbles_game::io::{PebblesInit, PebblesAction, DifficultyLevel};
use crate::pebbles_game::io::{GameState, PebblesEvent, Player};

#[test]
fn test_init() {
    // Test valid initialization
    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
    };
    let game_state = pebbles_game::io::init(init_data.into());
    assert_eq!(game_state.pebbles_count, 15);
    assert_eq!(game_state.max_pebbles_per_turn, 2);
}

#[test]
fn test_handle_user_win() {
    // Test User wins scenario
    let game_state = GameState {
        pebbles_count: 5,
        max_pebbles_per_turn: 2,
        pebbles_remaining: 2,
        difficulty: DifficultyLevel::Easy,
        first_player: Player::User,
        winner: None,
    };
    let event = pebbles_game::io::handle(PebblesAction::Turn(2).into());
    assert_eq!(event, PebblesEvent::Won(Player::User));
}

// Add more tests as needed to cover other scenarios
