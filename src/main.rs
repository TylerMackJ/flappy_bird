mod game;
mod menu;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash, States)]
enum GameState {
    Menu,
    #[default]
    Game,
    Paused,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy Bird".to_string(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            game::bird::BirdPlugin,
            menu::MenuPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
