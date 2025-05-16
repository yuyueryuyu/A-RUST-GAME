use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
#[derive(Component, Reflect)]
pub struct Background {
    pub starting_position: Vec2,
    pub starting_z: f32,
}

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
        Transform::from_xyz(0.0, 0.0, -2.5).with_scale(Vec3::new(1.8, 1.8, 1.)),
        Background {
            starting_position: Vec2::ZERO,
            starting_z: -1.5
        }
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
        Transform::from_xyz(0.0, 0.0, -1.5).with_scale(Vec3::new(1.8, 1.8, 1.)),
        Background {
            starting_position: Vec2::ZERO,
            starting_z: -0.9
        }
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
        Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::new(1.8, 1.8, 1.)),
        Background {
            starting_position: Vec2::ZERO,
            starting_z: -0.3
        }
    ));
}

fn play_audio(
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>) {
    audio.play(asset_server.load("Audio/Music/mp3/Dark Ambient 3.mp3")).looped();
}
pub struct BackgroundPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for BackgroundPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), (
            setup_bg.run_if(in_state(self.state.clone())),
            play_audio.run_if(in_state(self.state.clone()))
        ));
    }
}