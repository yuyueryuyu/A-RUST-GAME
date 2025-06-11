//! 生成场景Sensor，当玩家进入特定场景时作出反应

use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, OnCollisionStart, RigidBody, Sensor};
use bevy::prelude::*;

use crate::{game_layer::GameLayer, healthbar::Hint, hint::ItemHint, items::{item_canpick_observer, item_cantpick_observer, ItemBag, ItemList, NotpickedItems}, player::Player, AppState};

pub struct BlockPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for BlockPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), setup_blocks);
        app.add_systems(Update, 
            throne_position_system
                .run_if(in_state(AppState::InGame))
                .run_if(throne_position_system_prequisite)
        );
    }
}

/// 最终王座
#[derive(Component)]
pub struct KingdomThrone;

/// 恶魔boss区域感应器
#[derive(Component)]
pub struct FireDemonBlocks;

/// 武师boss区域感应器
#[derive(Component)]
pub struct MartialBlocks;

/// 初始门组件
#[derive(Component)]
pub struct InitialGate;

/// 生成碰撞体和sensor
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

    // 生成初始门的钥匙
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

    // martial 后门
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

    // martial 感知器
    commands.spawn((
        Collider::rectangle(16., 80.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(22., -140.,0.)
    )).observe(martial_observer);

    // demon 感知器
    commands.spawn((
        Collider::rectangle(16., 100.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(1054.5, 46.,0.)
    )).observe(fire_demon_observer);

    // demon 后门
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

    // 最终王座感知器
    commands.spawn((
        Collider::rectangle(48., 64.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(470., 1652.,0.)
    )).observe(end_game);

}

/// 如果进入boss区域，关闭大门
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

/// 如果进入boss区域，关闭大门
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

/// 如果抵达王座，结束游戏播放字幕
fn end_game(
    _trigger: Trigger<OnCollisionStart>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Ending);
}

/// 如果获得卷轴道具，生成前往王座的提示
fn throne_position_system_prequisite(
    bag: Single<&ItemBag, With<Player>>,
) -> bool {
    let ret = bag.into_inner().slots.contains_key(&"MartialScroll".to_string());
    return ret;
}

/// 如果获得卷轴道具，生成前往王座的提示
fn throne_position_system(
    mut text: Single<&mut Text, With<Hint>>,
    player: Single<&Transform, With<Player>>,
    thorne: Single<&Transform, With<KingdomThrone>>,
) {
    let player_position = player.into_inner().translation;
    let throne_position = thorne.into_inner().translation;
    let rel_position = throne_position - player_position;
    if rel_position.y == 0. {
        text.0 = if rel_position.x > 0. {
            "Current Throne Position: Right".to_string()
        } else {
            "Current Throne Position: Left".to_string()
        };
        return;
    }
    if rel_position.x == 0. {
        text.0 = if rel_position.y > 0. {
            "Current Throne Position: Up".to_string()
        } else {
            "Current Throne Position: Down".to_string()
        };
        return;
    }
    text.0 = if rel_position.x > 0. && rel_position.y > 0. {
        "Current Throne Position: Right-Up".to_string()
    } else if rel_position.x > 0. && rel_position.y < 0. {
        "Current Throne Position: Right-Down".to_string()
    } else if rel_position.x < 0. && rel_position.y > 0. {
        "Current Throne Position: Left-Up".to_string()
    } else {
        "Current Throne Position: Left-Down".to_string()
    };
}