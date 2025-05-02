use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use avian2d::prelude::*;
use tiled::Map;

use crate::game_layer::GameLayer;

fn setup_tilesets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TiledMapHandle(
            asset_server.load("Tilemap/game.tmx"),
        ),
        Transform::from_xyz(-180.0, -160.0, 0.0),
    )).insert(TiledMapLayerZOffset(0.1));
}

#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
struct MyCustomAvianPhysicsBackend(TiledPhysicsAvianBackend);

impl TiledPhysicsBackend for MyCustomAvianPhysicsBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        tiled_map: &TiledMap,
        filter: &TiledNameFilter,
        collider: &TiledCollider,
    ) -> Vec<TiledColliderSpawnInfos> {
        let colliders = self
            .0
            .spawn_colliders(commands, tiled_map, filter, collider);
        let collider_layer = CollisionLayers::new(
            GameLayer::Ground,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
        );
        for c in &colliders {
            commands.entity(c.entity).insert(RigidBody::Static).insert(collider_layer);
        }
        colliders
    }
}

pub struct TilesPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for TilesPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
        app.add_plugins(TiledMapPlugin::default());
        app.add_plugins(TiledPhysicsPlugin::<MyCustomAvianPhysicsBackend>::default());
        app.add_systems(OnEnter(self.state.clone()), (
            setup_tilesets.run_if(in_state(self.state.clone())),
        ));
    }
}