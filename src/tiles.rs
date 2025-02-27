use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use avian2d::prelude::*;
use tiled::Map;

fn setup_tilesets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TiledMapHandle(
            asset_server.load("Tilemap/game.tmx"),
        ),
        Transform::from_xyz(-180.0, -160.0, 0.0),
    )).insert(TiledMapSettings {
        layer_z_offset: 0.1,
        ..default()
    });
}

#[derive(Default)]
struct MyCustomAvianPhysicsBackend(TiledPhysicsAvianBackend);

impl TiledPhysicsBackend for MyCustomAvianPhysicsBackend {
    fn spawn_collider(
        &self,
        commands: &mut Commands,
        map: &Map,
        collider_source: &TiledColliderSource,
    ) -> Option<TiledColliderSpawnInfos> {
        let collider = self.0.spawn_collider(commands, map, collider_source);
        if let Some(c) = &collider {
            commands.entity(c.entity).insert(RigidBody::Static);
        }
        collider
    }
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
        app.add_plugins(TiledMapPlugin::default());
        app.add_plugins(TiledPhysicsPlugin::<MyCustomAvianPhysicsBackend>::default());
        app.add_systems(Startup, (
            setup_tilesets,
        ));
    }
}