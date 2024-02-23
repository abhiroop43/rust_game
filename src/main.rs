use rand::prelude::*;
use rusty_engine::prelude::*;

const MOVEMENT_SPEED: f32 = 100.0;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_index: u32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_index: 0,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        // resolution: WindowResolution::new(1400.0, 500.0),
        title: "Catch the cones".to_string(),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.15);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    let current_score = game.add_text("current_score", "Score: 0");
    current_score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    // let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    // car1.translation = Vec2::new(300.0, 0.0);
    // car1.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // engine.show_colliders = true;

    // quit if Q is pressed
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }

    // keep the text near the edges of the screen
    let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;

    let current_score = engine.texts.get_mut("current_score").unwrap();
    current_score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    current_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    // handle collisions
    for event in engine.collision_events.drain(..) {
        // println!("{:#?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // remove the sprite the player has collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }

            // increase the score
            game_state.current_score += 1;
            let current_score = engine.texts.get_mut("current_score").unwrap();
            current_score.value = format!("Score: {}", game_state.current_score);

            // update high score if current score is more
            if game_state.current_score > game_state.high_score {
                game_state.high_score = game_state.current_score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
        }
    }

    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    // player.translation.x += 100.0 * engine.delta_f32;

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("enemy{}", game_state.enemy_index);
        game_state.enemy_index += 1;
        let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingConeStraight);
        enemy.translation.x = thread_rng().gen_range(-550.0..550.0);
        enemy.translation.y = thread_rng().gen_range(-325.0..325.0);
        enemy.collision = true;
    }

    // Reset the score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.current_score = 0;
        let current_score = engine.texts.get_mut("current_score").unwrap();
        current_score.value = String::from("Current score: 0");
    }
}
