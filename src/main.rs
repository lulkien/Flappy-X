use std::f32::consts::PI;

use game_define::GameState;
use rand::{thread_rng, Rng};
use rusty_engine::prelude::*;

mod game_define;

fn main() {
    // Create game
    let mut game = Game::new();

    // Create player
    let player = game.add_sprite("player", "Bird.png");
    player.translation = Vec2::new(0.0, 200.0);
    player.scale = 0.125;
    player.layer = 1.0;
    player.collision = true;

    // Text
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(-500.0, 320.0);
    score.layer = 1.0;

    // Apply game logic
    game.add_logic(gravity_logic);
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost {
        return;
    }

    if !game_state.started {
        if engine.keyboard_state.just_pressed(KeyCode::Return) {
            game_state.started = true;
            spawn_pipe(engine, &mut game_state.pipe_spawn_index);
        }
        return;
    }

    // Handle collision
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            println!("DEAD");
            game_state.lost = true;
            return;
        }
    }

    // Handle keyboard
    let p_state = &mut game_state.p_state;
    if engine.keyboard_state.just_pressed(KeyCode::Space) {
        if p_state.vert_velocity < 0.0 {
            p_state.vert_velocity = 6.0;
        } else {
            p_state.vert_velocity += 6.0;
        }

        if p_state.vert_velocity > 11.0 {
            p_state.vert_velocity = 11.0;
        }
    }

    if game_state
        .pipe_spawn_timer
        .tick(engine.delta)
        .just_finished()
    {
        spawn_pipe(engine, &mut game_state.pipe_spawn_index);
        if game_state.start_scroring {
            game_state.current_score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Scrore: {}", game_state.current_score);
        }
    }

    // Move pipe
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("pipe") {
            sprite.translation.x -= 2.0;
            if !game_state.start_scroring {
                if sprite.translation.x < 0.0 {
                    game_state.start_scroring = true;
                }
            }
        }
    }
}

fn gravity_logic(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.started {
        return;
    }
    // Implement gravity
    const GRAVITY_FACTOR: f32 = -9.81 * 2.0;
    let p_state = &mut game_state.p_state;
    p_state.vert_velocity += GRAVITY_FACTOR * engine.delta_f32;

    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.y += p_state.vert_velocity;

    if player.translation.y >= 360.0 {
        p_state.vert_velocity = 0.0;
        player.translation.y = 359.0;
    }

    player.rotation = match p_state.vert_velocity {
        v if v > 0.0 => 0.0,
        _ => -0.5,
    };

    if player.translation.y <= -360.0 {
        game_state.lost = true;
        player.translation.y = -370.0;
    }
}

fn spawn_pipe(engine: &mut Engine, spawn_index: &mut u32) {
    let spawn_position = thread_rng().gen_range(-210..=210);
    // pipe down
    let label_down = format!("pipedown-{}", *spawn_index);
    let pipe_down = engine.add_sprite(label_down, "Pipe.png");
    pipe_down.translation = Vec2::new(680.0, (spawn_position as f32) - 60.0 - 256.0);
    pipe_down.collision = true;
    pipe_down.layer = 0.0;

    // pipe up
    let label_up = format!("pipeup-{}", *spawn_index);
    let pipe_up = engine.add_sprite(label_up, "Pipe.png");
    pipe_up.translation = Vec2::new(680.0, (spawn_position as f32) + 60.0 + 256.0);
    pipe_up.rotation = PI; // 180 deg = PI rad
    pipe_up.collision = true;
    pipe_up.layer = 0.0;

    // Increase spawn index
    *spawn_index = (*spawn_index + 1) % 5;
}
