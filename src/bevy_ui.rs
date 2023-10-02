use bevy::prelude::*;

pub struct ConsoleBevyPlugin;

impl Plugin for ConsoleBevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct Console {
    pub is_visible: bool,
}

fn setup(
    mut commands: Commands,
) {
    commands
        .insert_resource(Console { is_visible: false });
}
