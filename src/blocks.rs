use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, OnCollisionStart, RigidBody, Sensor};
use bevy::prelude::*;

use crate::{game_layer::GameLayer, hint::ItemHint, items::{item_canpick_observer, item_cantpick_observer, ItemList, NotpickedItems}, player::Player, AppState};

pub struct BlockPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for BlockPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), setup_blocks);
    }
}

#[derive(Component)]
pub struct KingdomThrone;

#[derive(Component)]
pub struct FireDemonBlocks;

#[derive(Component)]
pub struct MartialBlocks;

#[derive(Component)]
pub struct InitialGate;

fn setup_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    items: Res<ItemList>,
) {
    let dbl_block: Handle<Image> = asset_server.load("Art/pixilart-drawing2.png");
    let qua_block: Handle<Image> = asset_server.load("Art/pixilart-drawing4.png");
    // 初始地点
    commands.spawn((
        Sprite {
            image: dbl_block.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(16., 32.),
        CollisionLayers::new(
            GameLayer::Ground,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
        ),
        Transform::from_xyz(155., 36.,0.),
        InitialGate,
    ));

    commands.spawn((
        Sprite {
            image: items.infos.get(&String::from("Key")).unwrap().icon.clone(),
            ..default()
        },
        Collider::rectangle(40.0, 20.0),
        Transform::from_xyz(165., 22.1, 0.0),
        ItemHint,
        NotpickedItems { id: "Key".to_string(), num: 1 },
        Sensor,
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player])
    )).observe(item_cantpick_observer).observe(item_canpick_observer);

    // martial back
    commands.spawn((
        Sprite {
            image: qua_block.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(16., 64.),
        CollisionLayers::new(
            GameLayer::Ground,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
        ),
        Transform::from_xyz(619.5, 250.,0.),
        MartialBlocks,
    ));

    // martial front
    commands.spawn((
        Collider::rectangle(16., 80.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(22., -140.,0.)
    )).observe(martial_observer);

    // demon front
    commands.spawn((
        Collider::rectangle(16., 100.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(1054.5, 46.,0.)
    )).observe(fire_demon_observer);

    // demon back
    commands.spawn((
        Sprite {
            image: qua_block.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(16., 64.),
        CollisionLayers::new(
            GameLayer::Ground,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
        ),
        FireDemonBlocks,
        Transform::from_xyz(1547., 68.,0.)
    ));

    // kingdom throne
    commands.spawn((
        Collider::rectangle(48., 64.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(470., 1652.,0.)
    )).observe(end_game);

}

fn fire_demon_observer(
    trigger: Trigger<OnCollisionStart>,
    asset_server: Res<AssetServer>, 
    mut commands: Commands,
    player: Single<Entity, With<Player>>
) {
    let entity = trigger.collider;
    let tpl_block: Handle<Image> = asset_server.load("Art/pixilart-drawing3.png");
    if entity.to_bits() != player.into_inner().to_bits() { return; }
     // demon front
    commands.spawn((
        Sprite {
            image: tpl_block.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(16., 48.),
        CollisionLayers::new(
            GameLayer::Ground,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
        ),
        Transform::from_xyz(954.5, 46.,0.),
        FireDemonBlocks,
    ));
    commands.entity(trigger.target()).despawn();
}


fn martial_observer(
    trigger: Trigger<OnCollisionStart>,
    asset_server: Res<AssetServer>, 
    mut commands: Commands,
    player: Single<Entity, With<Player>>
) {
    let entity = trigger.collider;
    let dbl_block: Handle<Image> = asset_server.load("Art/pixilart-drawing2.png");
    if entity.to_bits() != player.into_inner().to_bits() { return; }
     // demon front
    commands.spawn((
        Sprite {
            image: dbl_block.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(16., 32.),
        CollisionLayers::new(
            GameLayer::Ground,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
        ),
        Transform::from_xyz(-2., -140.,0.),
        MartialBlocks
    ));
    commands.entity(trigger.target()).despawn();
}

fn end_game(
    trigger: Trigger<OnCollisionStart>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Ending);
}