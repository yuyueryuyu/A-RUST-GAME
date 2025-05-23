use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioInstance};
use big_brain::prelude::*;
use std::collections::HashSet;

use crate::animator::Condition;
use crate::animator::*;
use crate::controller::ControllerBundle;
use crate::damagable::{Damagable, HitBox};
use crate::game_layer::GameLayer;
use crate::hint::ItemHint;
use crate::physics::PhysicsBundle;
use crate::player::Player;
mod behaviour;
use behaviour::*;

#[derive(Component, Reflect)]
struct FireDemon;

fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture =
        asset_server.load("Art/boss_demon_slime_FREE_v1.0/spritesheets/demon_slime_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(288, 160), 7, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    spawn_enemy(
        &mut commands,
        Vec2::new(1430.0, 144.1),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
}

fn spawn_enemy(
    commands: &mut Commands,
    position: Vec2,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    let animator = setup_animator();
    let collider_layer = CollisionLayers::new(
        GameLayer::Enemy,
        [
            GameLayer::Default,
            GameLayer::Ground,
            GameLayer::PlayerHitBox,
        ],
    );
    let move_and_attack = Steps::build()
        .label("MoveAndAttack")
        .step(MoveToPlayer)
        .step(Attack);

    //let patrol = Steps::build().label("Patrol").step(Patrol);

    let thinker = Thinker::build()
        .label("Thinker")
        .picker(Highest)
        .when(NoticeScorer, move_and_attack);

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animator.first_index,
            }),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        FireDemon,
        ControllerBundle::new(29.8),
        PhysicsBundle {
            collider: Collider::capsule_endpoints(
                15.0, 
                Vec2::Y * 1.0, 
                Vec2::NEG_Y * 60.0
            ),
            layer: collider_layer,
            ..default()
        },
        Damagable::new(600.),
        animator,
        Notice::new(0.0, 50.0, 10.0),
        thinker,
    ));
}

#[derive(Component, Reflect)]
enum AnimationType {
    AttackPrep,
    Attack,
    AttackEnd,
    Boom,
    Death,
    Idle,
    Hurt,
    Walk,
}

impl AnimationType {
    fn config_index(&self) -> (usize, usize) {
        match self {
            Self::AttackPrep => (0, 8),
            Self::Attack => (9, 11),
            Self::AttackEnd => (12, 14),
            Self::Boom => (15, 24),
            Self::Death => (15, 36),
            Self::Idle => (37, 42),
            Self::Hurt => (43, 47),
            Self::Walk => (48, 59),
        }
    }
}

fn setup_animator() -> Animator {
    let idle_state = AnimationState {
        name: "Idle".to_string(),
        first_index: AnimationType::Idle.config_index().0,
        last_index: AnimationType::Idle.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "is_moving".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Walk".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "AttackPrep".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "boom".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Boom".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Death".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },

        ],
        loop_animation: true,
        ..default()
    };

    let walk_state = AnimationState {
        name: "Walk".to_string(),
        first_index: AnimationType::Walk.config_index().0,
        last_index: AnimationType::Walk.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "is_moving".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "AttackPrep".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "boom".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Boom".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Death".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        ..default()
    };

    let attack_prep_state = AnimationState {
        name: "AttackPrep".to_string(),
        first_index: AnimationType::AttackPrep.config_index().0,
        last_index: AnimationType::AttackPrep.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Attack".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Death".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_cant_move),
        on_exit: None,
        ..default()
    };

    let attack_state = AnimationState {
        name: "Attack".to_string(),
        first_index: AnimationType::Attack.config_index().0,
        last_index: AnimationType::Attack.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "AttackEnd".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Death".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_attack),
        on_exit: Some(set_not_attack),
        ..default()
    };

    let boom_state = AnimationState {
        name: "Boom".to_string(),
        first_index: AnimationType::Boom.config_index().0,
        last_index: AnimationType::Boom.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_boom),
        on_exit: Some(set_not_boom),
        ..default()
    };

    let attack_end_state = AnimationState {
        name: "AttackEnd".to_string(),
        first_index: AnimationType::AttackEnd.config_index().0,
        last_index: AnimationType::AttackEnd.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Death".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: false,
        on_exit: Some(set_can_move),
        ..default()
    };

    let death_state = AnimationState {
        name: "Death".to_string(),
        first_index: AnimationType::Death.config_index().0,
        last_index: AnimationType::Death.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        on_enter: Some(set_cant_move),
        on_exit: Some(set_death),
        ..default()
    };

    let hurt_state = AnimationState {
        name: "Hurt".to_string(),
        first_index: AnimationType::Hurt.config_index().0,
        last_index: AnimationType::Hurt.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        on_enter: None,
        on_exit: None,
        ..default()
    };

    let mut animator = Animator::new();
    animator.add_parameter("is_moving", AnimatorParam::Bool(false));
    animator.add_parameter("can_move", AnimatorParam::Bool(true));
    animator.add_parameter("attack", AnimatorParam::Trigger(false));
    animator.add_parameter("boom", AnimatorParam::Trigger(false));
    animator.add_parameter("is_alive", AnimatorParam::Bool(true));
    animator.add_parameter("hit", AnimatorParam::Trigger(false));
    animator.add_parameter("is_grounded", AnimatorParam::Bool(true));
    animator.add_parameter("is_on_wall", AnimatorParam::Bool(false));
    animator.add_parameter("is_on_ceiling", AnimatorParam::Bool(false));
    animator.add_parameter("facing_direction", AnimatorParam::Float(-1.0));
    animator.add_parameter("is_noticing", AnimatorParam::Bool(false));
    animator.add_parameter("noticed", AnimatorParam::Bool(false));

    animator.add_state(idle_state);
    animator.add_state(walk_state);
    animator.add_state(attack_state);
    animator.add_state(boom_state);
    animator.add_state(attack_prep_state);
    animator.add_state(attack_end_state);
    animator.add_state(death_state);
    animator.add_state(hurt_state);

    animator.set_initial_state(
        "Idle",
        AnimationType::Idle.config_index().0,
        AnimationType::Idle.config_index().1,
        8,
    );

    animator
}

#[derive(Event)]
pub struct FireDemonDeath;

fn on_fire_demon_death(
    _trigger: Trigger<FireDemonDeath>,
    mut commands: Commands,
    demon: Single<(Entity, &Transform), With<FireDemon>>,
    asset_server: Res<AssetServer>,
) {
    let (entity, transform) = demon.into_inner();
    commands.entity(entity).despawn_recursive();
    commands.spawn((
        Sprite {
            image: asset_server.load("Art/Kyrise's 16x16 RPG Icon Pack - V1.3/icons/16x16/gloves_01e.png"),
            ..default()
        },
        Collider::rectangle(20.0, 20.0),
        Transform::from_xyz(transform.translation.x, 27.1, 0.0),
        FireGlove,
        ItemHint,
        Sensor,
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player])
    )
    );
} 

#[derive(Component, Reflect)]
pub struct FireGlove;

fn set_death(commands: &mut Commands, entity: Entity, animator: &mut Animator
    ,asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    commands.trigger(FireDemonDeath);
}

fn set_not_attack(commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    for child in animator.active_children.iter() {
        commands.entity(*child).despawn_recursive();
    }

    animator.active_children = HashSet::new();
}

fn set_attack(commands: &mut Commands, entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    let collider_layer = CollisionLayers::new(GameLayer::EnemyHitBox, [GameLayer::Player]);
    let id = commands
        .spawn((
            Collider::rectangle(50., 20.),
            Transform::from_xyz(-80., -70., 0.),
            Sensor,
            HitBox,
            collider_layer,
        ))
        .set_parent(entity)
        .id();
    animator.push_active_child(id);
}

fn set_not_boom(commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", true);
    for child in animator.active_children.iter() {
        commands.entity(*child).despawn_recursive();
    }

    animator.active_children = HashSet::new();
}

fn set_boom(commands: &mut Commands, entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", false);
    let collider_layer = CollisionLayers::new(GameLayer::EnemyHitBox, [GameLayer::Player]);
    let id = commands
        .spawn((
            Collider::rectangle(80., 100.),
            Transform::from_xyz(0., -20., 0.),
            Sensor,
            HitBox,
            collider_layer,
        ))
        .set_parent(entity)
        .id();
    animator.push_active_child(id);
}

fn _set_can_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", true);
}

fn set_cant_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", false);
}

fn set_can_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", true);
}


fn check_contact(
    spatial_query: SpatialQuery,
    mut query: Query<(&Transform, &mut Animator, &Collider), With<FireDemon>>,
    entity: Single<Entity, With<Player>>,
) {
    let collider_notice = Collider::rectangle(1., 100.);

    for (transform, mut animator, collider) in &mut query {
        let origin = Vec2::new(transform.translation.x, transform.translation.y);
        let rotation = transform.rotation.z;
        let direction_x = if animator.get_float("facing_direction") > 0.0 {
            Dir2::X
        } else {
            Dir2::NEG_X
        };
        let max_distance_y = 0.2;
        let max_distance_x = 10.;
        let max_distance_notice = 100.;
        let max_hits = 1;

        let config_y = ShapeCastConfig::from_max_distance(max_distance_y);
        let config_x = ShapeCastConfig::from_max_distance(max_distance_x);
        let notice_config = ShapeCastConfig::from_max_distance(max_distance_notice);
        let filter = SpatialQueryFilter::default().with_mask(GameLayer::Ground);
        let notice_filter =
            SpatialQueryFilter::default().with_mask([GameLayer::Player, GameLayer::Ground]);

        let hits_ground = spatial_query.shape_hits(
            &collider,
            origin,
            rotation,
            Dir2::NEG_Y,
            max_hits,
            &config_y,
            &filter,
        );
        let hits_wall = spatial_query.shape_hits(
            &collider,
            origin,
            rotation,
            direction_x,
            max_hits,
            &config_x,
            &filter,
        );
        let hits_ceiling = spatial_query.shape_hits(
            &collider,
            origin,
            rotation,
            Dir2::Y,
            max_hits,
            &config_y,
            &filter,
        );
        let hits_notice = spatial_query.shape_hits(
            &collider_notice,
            origin,
            rotation,
            direction_x,
            max_hits,
            &notice_config,
            &notice_filter,
        );

        animator.set_bool("is_grounded", hits_ground.len() > 0);
        animator.set_bool("is_on_wall", hits_wall.len() > 0);
        animator.set_bool("is_on_ceiling", hits_ceiling.len() > 0);
        if let Some(noticed_entity) = hits_notice.get(0) {
            animator.set_bool(
                "is_noticing",
                noticed_entity.entity.to_bits() == entity.to_bits(),
            );
        } else {
            animator.set_bool("is_noticing", false);
        }
    }
}

fn on_flip_direction(mut query: Query<(&mut Transform, &Animator), With<FireDemon>>) {
    for (mut transform, animator) in &mut query {
        let facing_direction = animator.get_float("facing_direction");
        if transform.scale.x * facing_direction > 0. {
            transform.scale.x *= -1.;
        }
    }
}

fn on_move(mut query: Query<(&LinearVelocity, &mut Animator), With<FireDemon>>) {
    for (vel, mut animator) in query.iter_mut() {
        let is_moving = vel.x != 0.;
        animator.set_bool("is_moving", is_moving);
    }
}

pub struct FireDemonPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for FireDemonPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(FireDemonBehaviourPlugin { state: self.state.clone() });
        app.add_systems(OnEnter(self.state.clone()), setup_enemy.run_if(in_state(self.state.clone())));
        app.add_systems(
            FixedUpdate,
            (
                check_contact.run_if(in_state(self.state.clone())),
                on_move.run_if(in_state(self.state.clone())),
                on_flip_direction.run_if(in_state(self.state.clone())),
            ),
        );
        app.add_observer(on_fire_demon_death);
    }
}
