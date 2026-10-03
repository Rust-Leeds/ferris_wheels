use bevy::prelude::*;

#[derive(Component)]
pub struct Vehicle;

// Marking velocity as dead code until movement is added
#[expect(dead_code)]
#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn spawn_vehicle(commands: &mut Commands) {
    commands.spawn((
        Vehicle,
        Velocity(Vec2::ZERO),
        Transform::default(),
        // using a red rectangle as a placeholder for now.
        Sprite::from_color(Color::srgb(0.9, 0.2, 0.2), Vec2::new(60.0, 30.0)),
    ));
}
