use bevy::prelude::*;
use convars::bevy_ui::{Console, ConsoleBevyPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsoleBevyPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_console)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn toggle_console(keyboard_input: Res<Input<KeyCode>>, mut console: ResMut<Console>) {
    if keyboard_input.just_pressed(KeyCode::Grave) {
        console.is_visible = !console.is_visible;
        println!("Console is now {}", console.is_visible);
    }
}
