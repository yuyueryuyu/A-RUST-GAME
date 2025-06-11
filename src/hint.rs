//! 生成游戏提示

use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{game_layer::GameLayer, healthbar::Hint, player::Player};
/// 上下左右提示
#[derive(Component)]
pub struct WASDHint;
/// 战斗提示
#[derive(Component)]
pub struct BattleHint;
/// 滑行提示
#[derive(Component)]
pub struct SlideHint;
/// 跳跃提示
#[derive(Component)]
pub struct JumpHint;
/// 使用物品提示
#[derive(Component)]
pub struct UseHint;
/// 拾取物品提示
#[derive(Component)]
pub struct ItemHint;
/// 提示标识组件
#[derive(Component)]
pub struct HintEntity;
/// 生成提示Sensor
fn spawn_hint_colliders(
    mut commands: Commands
) {
    commands.spawn((
        Collider::rectangle(20.0, 20.0),
        Sensor,
        WASDHint,
        Transform::from_xyz(125.0, 45.1, 0.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        CollisionEventsEnabled,
        HintEntity
    ));

    commands.spawn((
        Collider::rectangle(20.0, 20.0),
        Sensor,
        BattleHint,
        Transform::from_xyz(185.0, 45.1, 0.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        CollisionEventsEnabled,
        HintEntity
    ));

    commands.spawn((
        Collider::rectangle(20.0, 20.0),
        Sensor,
        JumpHint,
        Transform::from_xyz(545.0, 45.1, 0.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        CollisionEventsEnabled,
        HintEntity
    ));

    commands.spawn((
        Collider::rectangle(20.0, 20.0),
        Sensor,
        SlideHint,
        Transform::from_xyz(171.0, 428.0, 0.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        CollisionEventsEnabled,
        HintEntity
    ));

    commands.spawn((
        Collider::rectangle(20.0, 20.0),
        Sensor,
        UseHint,
        Transform::from_xyz(898.0, 45.0, 0.),
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        CollisionEventsEnabled,
        HintEntity
    ));
}

/// 玩家进入Sensor-> 设置提示
fn print_started_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    player: Query<&Player>, hint: Query<&WASDHint>, bhint: Query<&BattleHint>,
    jhint: Query<&JumpHint>, shint: Query<&SlideHint>, ihint: Query<&ItemHint>, uhint: Query<&UseHint>,
    mut text: Single<&mut Text, With<Hint>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if (player.contains(*entity1) && hint.contains(*entity2)) ||
           (player.contains(*entity2) && hint.contains(*entity1))  {
            text.0 = "Move: [W] [A] [S] [D] \nOpen Item Bag: [Esc][Open Bag]".to_string();
        }
        if (player.contains(*entity1) && bhint.contains(*entity2)) ||
           (player.contains(*entity2) && bhint.contains(*entity1))  {
            text.0 = "Attack: [Left Mouse] \nDefense: [Right Mouse]".to_string();
        }

        if (player.contains(*entity1) && jhint.contains(*entity2)) ||
           (player.contains(*entity2) && jhint.contains(*entity1))  {
            text.0 = "Jump: [Space]".to_string();
        }

        if (player.contains(*entity1) && shint.contains(*entity2)) ||
           (player.contains(*entity2) && shint.contains(*entity1))  {
            text.0 = "Run: long [Shift] \nSlide: short [Shift]".to_string();
        }

        if (player.contains(*entity1) && ihint.contains(*entity2)) ||
           (player.contains(*entity2) && ihint.contains(*entity1))  {
            text.0 = "Pick up Item: [E]".to_string();
        }

        if (player.contains(*entity1) && uhint.contains(*entity2)) ||
           (player.contains(*entity2) && uhint.contains(*entity1))  {
            text.0 = "Use Items for healing: [R]".to_string();
        }
    }
}

/// 玩家退出Sensor-> 关闭提示
fn print_ended_collisions(
    mut collision_event_reader: EventReader<CollisionEnded>,
    player: Query<&Player>, hint: Query<&WASDHint>, bhint: Query<&BattleHint>,
    jhint: Query<&JumpHint>, shint: Query<&SlideHint>, ihint: Query<&ItemHint>, uhint: Query<&UseHint>,
    mut text: Single<&mut Text, With<Hint>>,
) {
    for CollisionEnded(entity1, entity2) in collision_event_reader.read() {
        if (player.contains(*entity1) && hint.contains(*entity2)) ||
           (player.contains(*entity2) && hint.contains(*entity1))  {
            text.0 = "".to_string();
        }
        if (player.contains(*entity1) && bhint.contains(*entity2)) ||
           (player.contains(*entity2) && bhint.contains(*entity1))  {
            text.0 = "".to_string();
        }
        if (player.contains(*entity1) && jhint.contains(*entity2)) ||
           (player.contains(*entity2) && jhint.contains(*entity1))  {
            text.0 = "".to_string();
        }
        if (player.contains(*entity1) && shint.contains(*entity2)) ||
           (player.contains(*entity2) && shint.contains(*entity1))  {
            text.0 = "".to_string();
        }
        if (player.contains(*entity1) && ihint.contains(*entity2)) ||
           (player.contains(*entity2) && ihint.contains(*entity1))  {
            text.0 = "".to_string();
        }
        if (player.contains(*entity1) && uhint.contains(*entity2)) ||
           (player.contains(*entity2) && uhint.contains(*entity1))  {
            text.0 = "".to_string();
        }
    }
}

pub struct HintPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for HintPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), spawn_hint_colliders.run_if(in_state(self.state.clone())));
        app.add_systems(Update, print_started_collisions.run_if(in_state(self.state.clone())));
        app.add_systems(Update, print_ended_collisions.run_if(in_state(self.state.clone())));
    }
}

