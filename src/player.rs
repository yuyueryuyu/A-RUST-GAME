use bevy::{
    prelude::*
};
use bevy_rapier2d::prelude::*;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(
            asset_server.load("Art/Adventurer/Individual Sprites/adventurer-idle-00.png"),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        Velocity::default(),
        GravityScale(0.0),
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_player
        ).chain());
    }
}