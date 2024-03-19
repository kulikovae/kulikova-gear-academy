use gstd::prelude::*;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    User,
    Program,
}

impl PebblesInit {
    fn validate(&self) -> bool {
        // Validate pebbles count and max pebbles per turn
        // You can add more validation as needed
        self.pebbles_count > 0 && self.max_pebbles_per_turn > 0
    }
}

fn choose_first_player() -> Player {
    // Choose the first player randomly
    if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    }
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn init(_: In<PebblesInit>) -> Out<GameState> {
    let init_data = msg::load::<PebblesInit>();

    // Validate input data
    if !init_data.validate() {
        panic!("Invalid input data for game initialization");
    }

    // Choose the first player
    let first_player = choose_first_player();

    // Process the first turn if the first player is Program
    let mut pebbles_remaining = init_data.pebbles_count;
    if first_player == Player::Program {
        let remove_count = if init_data.max_pebbles_per_turn >= pebbles_remaining {
            pebbles_remaining
        } else {
            get_random_u32() % init_data.max_pebbles_per_turn + 1
        };
        pebbles_remaining -= remove_count;
    }

    // Fill the GameState structure
    let game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining,
        difficulty: init_data.difficulty,
        first_player,
        winner: None,
    };

    Out(game_state)
}

pub fn handle(action: In<PebblesAction>) -> Out<PebblesEvent> {
    let game_state = msg::load::<GameState>();
    let action = action.into_inner();

    match action {
        PebblesAction::Turn(remove_count) => {
            // Check if user wins
            if game_state.pebbles_remaining - remove_count == 0 {
                Out(PebblesEvent::Won(Player::User))
            } else {
                // Process Program's turn
                let program_remove_count = if game_state.max_pebbles_per_turn >= game_state.pebbles_remaining {
                    game_state.pebbles_remaining
                } else {
                    get_random_u32() % game_state.max_pebbles_per_turn + 1
                };
                let remaining = game_state.pebbles_remaining - program_remove_count;
                if remaining == 0 {
                    Out(PebblesEvent::Won(Player::Program))
                } else {
                    Out(PebblesEvent::CounterTurn(program_remove_count))
                }
            }
        }
        PebblesAction::GiveUp => Out(PebblesEvent::Won(Player::Program)),
        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            // Restart the game with new settings
            let first_player = choose_first_player();
            let pebbles_remaining = pebbles_count;
            let game_state = GameState {
                pebbles_count,
                max_pebbles_per_turn,
                pebbles_remaining,
                difficulty,
                first_player,
                winner: None,
            };
            Out(game_state)
        }
    }
}

pub fn state(_: ()) -> Out<GameState> {
    let game_state = msg::load::<GameState>();
    Out(game_state)
}
