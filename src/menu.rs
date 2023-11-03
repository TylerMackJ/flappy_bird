use crate::{despawn_screen, GameState};
use bevy::prelude::*;

#[derive(Component)]
struct FromMenuState;

fn show_menu(mut commands: Commands) {
    commands.spawn((
        FromMenuState,
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "You Died\u{1F480}",
                    TextStyle {
                        font_size: 60.0,
                        ..Default::default()
                    },
                )],
                ..Default::default()
            },
            text_anchor: bevy::sprite::Anchor::Center,
            ..Default::default()
        },
    ));
}

fn wait_for_input(mut game_state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>) {
    if input.any_just_pressed(vec![KeyCode::Space]) {
        game_state.set(GameState::Game);
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn name(&self) -> &str {
        "Menu Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(Update, wait_for_input.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<FromMenuState>);
    }
}
