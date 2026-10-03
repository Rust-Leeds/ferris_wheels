use bevy::prelude::*;

mod vehicle;

use vehicle::spawn_vehicle;

fn main() {
    println!("Running Bevy App");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Close the window to return to the main function".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, system)
        .run();
    println!("Bevy App has exited. We are back in our main function.");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_vehicle(&mut commands);
}

fn system() {
    info!("Logging from Bevy App");
}
