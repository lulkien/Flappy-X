use rusty_engine::prelude::*;

pub struct GameState {
    pub started: bool,
    pub lost: bool,
    pub pipe_spawn_index: u32,
    pub pipe_spawn_timer: Timer,
    pub last_pipe_label: String,
    pub start_scroring: bool,
    pub current_score: u32,
    pub p_state: PlayerState,
}

pub struct PlayerState {
    pub vert_velocity: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self { vert_velocity: 0.0 }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            started: false,
            lost: false,
            pipe_spawn_index: 0,
            pipe_spawn_timer: Timer::from_seconds(2.5, true),
            last_pipe_label: String::new(),
            start_scroring: false,
            current_score: 0,
            p_state: PlayerState::default(),
        }
    }
}
