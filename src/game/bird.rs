use crate::game::FromGameState;
use crate::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component, Clone, Copy)]
struct Bird {
    gravity: f32,
    initial_height: f32,
    jump_velocity: f32,
    min_velocity: f32,
    size: f32,
    velocity: f32,
}

impl Default for Bird {
    fn default() -> Self {
        let max_velocity = 300.0;
        Self {
            gravity: 100.0,
            initial_height: 0.0,
            jump_velocity: max_velocity * 0.5,
            min_velocity: 0.0 - max_velocity,
            size: 50.0,
            velocity: 0.0,
        }
    }
}

fn spawn_bird(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();

    let bird = Bird::default();
    commands.spawn((
        FromGameState,
        bird,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(bird.size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform::from_xyz(
                -(window.width() / 2.0) * 0.75,
                bird.initial_height,
                0.0,
            ),
            ..default()
        },
    ));
}

fn move_bird(
    mut query: Query<(&mut Transform, &mut Bird)>,
    windows: Query<&Window>,
    mut game_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut bird) = query.single_mut();
    let window = windows.single();

    if input.just_pressed(KeyCode::Space) {
        bird.velocity = bird.jump_velocity;
    } else {
        bird.velocity -= bird.gravity * time.delta_seconds();
        if bird.velocity < bird.min_velocity {
            bird.velocity = bird.min_velocity;
        }
    }

    transform.translation.y += bird.velocity * time.delta_seconds();

    if transform.translation.y - bird.size <= -(window.height() / 2.0) {
        game_state.set(GameState::Menu);
    }
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn name(&self) -> &str {
        "Bird Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_bird)
            .add_systems(Update, move_bird.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<FromGameState>);
    }
}
