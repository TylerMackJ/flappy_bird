use crate::game::FromGameState;
use bevy::prelude::*;

pub enum PipeType {
    Top,
    Bottom,
}

#[derive(Component)]
struct Pipe {
    pipe_type: PipeType,
    length: f32,
}

#[derive(Resource)]
struct PipeTimer(Timer);

fn spawn_pipes(
    mut commands: Commands,
    windows: Query<&Window>,
    time: Res<Time>,
    mut pipe_timer: ResMut<PipeTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();

    if pipe_timer.0.tick(time.delta()).just_finished() {
        commands.spawn((FromGameState,));
    }
}
