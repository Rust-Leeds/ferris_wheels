//! An empty application (does nothing)

use bevy::prelude::*;

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
        .add_systems(Update, system)
        .run();
    println!("Bevy App has exited. We are back in our main function.");
}

fn system() {
    info!("Logging from Bevy App");
}