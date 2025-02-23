use bevy::{
    prelude::*
};

fn setup_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_size_x = 320.0;
    let background_size_y = 192.0;
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0 * background_size_x, background_size_y)),
            image: asset_server.load("Art/FreeCuteTileset/BG1.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0
            },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.5)
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0 * background_size_x, background_size_y)),
            image: asset_server.load("Art/FreeCuteTileset/BG2.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0
            },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.9)
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0 * background_size_x, background_size_y)),
            image: asset_server.load("Art/FreeCuteTileset/BG3.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0
            },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.3)
    ));
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_bg
        ).chain());
    }
}