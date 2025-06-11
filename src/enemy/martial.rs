use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioInstance};
use big_brain::prelude::*;
use my_bevy_game::{enter, exit};
use std::collections::HashSet;

use crate::animator::Condition;
use crate::animator::*;
use crate::blocks::MartialBlocks;
use crate::controller::ControllerBundle;
use crate::damagable::{check_hitbox, Damagable, HasHitbox, HitBox, HitboxOf};
use crate::game_layer::GameLayer;
use crate::hint::ItemHint;
use crate::items::{item_canpick_observer, item_cantpick_observer, ItemList, NotpickedItems};
use crate::physics::PhysicsBundle;
use crate::player::Player;
mod behaviour;
use behaviour::*;

#[derive(Component, Reflect)]
struct Martial;

fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture =
        asset_server.load("Art/Martial Hero/martial_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(200, 200), 8, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    spawn_enemy(
        &mut commands,
        Vec2::new(392., -140.),
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

    // 第一阶段行为树
    let phase1_combo = Steps::build()
        .label("Phase1Combo")
        .step(MoveToPlayer)
        .step(Attack1)
        .step(Attack2);

    let phase1_jump_attack = Steps::build()
        .label("Phase1JumpAttack")
        .step(JumpAttack);

    // 第二阶段行为树
    let phase2_transition = Steps::build()
        .label("Phase2Transition")
        .step(PhaseTransition);

    let phase2_teleport = Steps::build()
        .label("Phase2Teleport")
        .step(TeleportAttack);

    // 主行为树
    let thinker = Thinker::build()
        .label("NewEnemyThinker")
        .picker(Highest)
        .when(HealthScorer, phase2_transition)
        .when(PhaseTwoScorer, phase2_teleport)
        .when(NoticeScorer, phase1_combo);

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
        Martial,
        ControllerBundle::new(19.8),
        PhysicsBundle {
            collider: Collider::capsule_endpoints(
                10.0, 
                Vec2::Y * 15.0, 
                Vec2::NEG_Y * 15.0
            ),
            layer: collider_layer,
            ..default()
        },
        Damagable::new(1500.), // 较高血量
        animator,
        Notice::new(0.0, 60.0, 0.0),
        HealthState::new(1500.0),
        PhaseTwoTimer::new(),
        thinker,
    ));
}

#[derive(Component, Reflect)]
enum AnimationType {
    Attack1Prep,
    Attack1,
    Attack2Prep,
    Attack2,
    Death,
    Idle,
    Hurt,
    Run,
    Jump,
    Rise,
    Fall,
    Hidden,
}

impl AnimationType {
    fn config_index(&self) -> (usize, usize) {
        match self {
            Self::Attack1Prep => (8, 11),
            Self::Attack1 => (12, 13),
            Self::Attack2Prep => (16, 19),
            Self::Attack2 => (20, 21),
            Self::Jump => (2, 3),
            Self::Rise => (3, 3),
            Self::Fall => (0, 1),
            Self::Death => (24, 29),
            Self::Idle => (32, 39),
            Self::Hurt => (4, 7),
            Self::Run => (40, 47),
            Self::Hidden => (22, 22),
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
                target_state: "Run".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack1".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack1Prep".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack2".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack2Prep".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hurt".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hidden".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        ..default()
    };

    let run_state = AnimationState {
        name: "Run".to_string(),
        first_index: AnimationType::Run.config_index().0,
        last_index: AnimationType::Run.config_index().1,
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
                    param_name: "attack1".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack1Prep".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack2".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack2Prep".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hurt".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hidden".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        ..default()
    };

    let attack1_prep_state = AnimationState {
        name: "Attack1Prep".to_string(),
        first_index: AnimationType::Attack1Prep.config_index().0,
        last_index: AnimationType::Attack1Prep.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Attack1".to_string(),
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
        on_enter: Some(__stun_enter_handler),
        on_exit: None,
        ..default()
    };

    let attack1_state = AnimationState {
        name: "Attack1".to_string(),
        first_index: AnimationType::Attack1.config_index().0,
        last_index: AnimationType::Attack1.config_index().1,
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
        on_enter: Some(__attack1_enter_handler),
        on_exit: Some(__attack1_exit_handler),
        ..default()
    };

    let attack2_prep_state = AnimationState {
        name: "Attack2Prep".to_string(),
        first_index: AnimationType::Attack2Prep.config_index().0,
        last_index: AnimationType::Attack2Prep.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Attack2".to_string(),
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
        on_enter: Some(__stun_enter_handler),
        on_exit: None,
        ..default()
    };

    let attack2_state = AnimationState {
        name: "Attack2".to_string(),
        first_index: AnimationType::Attack2.config_index().0,
        last_index: AnimationType::Attack2.config_index().1,
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
        on_enter: Some(__attack2_enter_handler),
        on_exit: Some(__attack2_exit_handler),
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
        on_enter: Some(__stun_enter_handler),
        on_exit: Some(__death_exit_handler),
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
        on_enter: Some(__stun_enter_handler),
        on_exit: Some(__stun_exit_handler),
        ..default()
    };

    let jump_state = AnimationState {
        name: "Jump".to_string(),
        first_index: AnimationType::Jump.config_index().0,
        last_index: AnimationType::Jump.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Rise".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Idle".to_string(),
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

    let rise_state = AnimationState {
        name: "Rise".to_string(),
        first_index: AnimationType::Rise.config_index().0,
        last_index: AnimationType::Rise.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "velocity_y".to_string(),
                    operator: ConditionOperator::Less,
                    value: AnimatorParam::Float(0.0),
                }],
                target_state: "Fall".to_string(),
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

    let fall_state = AnimationState {
        name: "Fall".to_string(),
        first_index: AnimationType::Fall.config_index().0,
        last_index: AnimationType::Fall.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "velocity_y".to_string(),
                    operator: ConditionOperator::Greater,
                    value: AnimatorParam::Float(0.0),
                }],
                target_state: "Rise".to_string(),
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

    let hidden_state = AnimationState {
        name: "Hidden".to_string(),
        first_index: AnimationType::Hidden.config_index().0,
        last_index: AnimationType::Hidden.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "showup".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack1Prep".to_string(),
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

    let mut animator = Animator::new();
    animator.add_parameter("is_moving", AnimatorParam::Bool(false));
    animator.add_parameter("can_move", AnimatorParam::Bool(true));
    animator.add_parameter("attack1", AnimatorParam::Trigger(false));
    animator.add_parameter("attack2", AnimatorParam::Trigger(false));
    animator.add_parameter("is_alive", AnimatorParam::Bool(true));
    animator.add_parameter("hit", AnimatorParam::Trigger(false));
    animator.add_parameter("is_grounded", AnimatorParam::Bool(true));
    animator.add_parameter("is_on_wall", AnimatorParam::Bool(false));
    animator.add_parameter("is_on_ceiling", AnimatorParam::Bool(false));
    animator.add_parameter("facing_direction", AnimatorParam::Float(1.0));
    animator.add_parameter("is_noticing", AnimatorParam::Bool(false));
    animator.add_parameter("noticed", AnimatorParam::Bool(false));
    animator.add_parameter("velocity_y", AnimatorParam::Float(0.0));
    animator.add_parameter("hide", AnimatorParam::Trigger(false));
    animator.add_parameter("showup", AnimatorParam::Trigger(false));

    animator.add_state(idle_state);
    animator.add_state(run_state);
    animator.add_state(attack1_prep_state);
    animator.add_state(attack1_state);
    animator.add_state(attack2_prep_state);
    animator.add_state(attack2_state);
    animator.add_state(death_state);
    animator.add_state(hurt_state);
    animator.add_state(jump_state);
    animator.add_state(rise_state);
    animator.add_state(fall_state);
    animator.add_state(hidden_state);

    animator.set_initial_state(
        "Idle",
        AnimationType::Idle.config_index().0,
        AnimationType::Idle.config_index().1,
        8,
    );

    animator
}

#[exit("death")]
fn on_martial_death(
    mut commands: Commands,
    martial: Single<(Entity, &Transform), With<Martial>>,
    blocks: Query<Entity, With<MartialBlocks>>,
    items: Res<ItemList>, 
) {
    let (entity, transform) = martial.into_inner();
    commands.entity(entity).despawn();
    commands.spawn((
        Sprite {
            image: items.infos.get(&String::from("MartialScroll")).unwrap().icon.clone(),
            ..default()
        },
        Collider::rectangle(20.0, 20.0),
        Transform::from_xyz(120., 39.1, 0.0),
        ItemHint,
        NotpickedItems { id: "MartialScroll".to_string(), num: 1 },
        Sensor,
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player])
    )).observe(item_cantpick_observer).observe(item_canpick_observer);
    for block in blocks {
        commands.entity(block).despawn();
    }
} 

#[enter("attack1")]
fn on_attack1_enter(
    mut commands: Commands
) {
    let entity = trigger.entity;
    commands.spawn((
            Collider::rectangle(60., 40.),
            Transform::from_xyz(40., 0., 0.),
            Sensor,
            HitBox { damage: 35. },
            CollisionLayers::new(GameLayer::EnemyHitBox, [GameLayer::Player]),
            CollisionEventsEnabled,
            ChildOf(entity),
            HitboxOf(entity),
        )).observe(check_hitbox);
}

#[exit("attack1")]
fn on_attack1_exit(
    mut commands: Commands,
    mut enemy: Query<(&HasHitbox, &mut Animator), With<Martial>>,
) {
    let entity = trigger.entity;
    let (hitboxes, mut animator) = enemy.get_mut(entity).unwrap();
    let vec = (**hitboxes).clone();
    for hitbox in vec {
        commands.entity(hitbox).despawn();
    }
    animator.set_bool("can_move", true);
}

#[enter("attack2")]
fn on_attack2_enter(
    mut commands: Commands
) {
    let entity = trigger.entity;
    commands.spawn((
            Collider::rectangle(80., 40.),
            Transform::from_xyz(40., 0., 0.),
            Sensor,
            HitBox { damage: 50. },
            CollisionLayers::new(GameLayer::EnemyHitBox, [GameLayer::Player]),
            CollisionEventsEnabled,
            ChildOf(entity),
            HitboxOf(entity),
        )).observe(check_hitbox);
}

#[exit("attack2")]
fn on_attack2_exit(
    mut commands: Commands,
    mut enemy: Query<(&HasHitbox, &mut Animator), With<Martial>>,
) {
    let entity = trigger.entity;
    let (hitboxes, mut animator) = enemy.get_mut(entity).unwrap();
    let vec = (**hitboxes).clone();
    for hitbox in vec {
        commands.entity(hitbox).despawn();
    }
    animator.set_bool("can_move", true);
}

#[enter("stun")]
fn on_stun_enter(
    mut martial: Query<&mut Animator, With<Martial>>,
) {
    let entity = trigger.entity;
    let mut animator = martial.get_mut(entity).unwrap();
    animator.set_bool("can_move", false);
}

#[exit("stun")]
fn on_stun_exit(
    mut martial: Query<&mut Animator, With<Martial>>,
) {
    let entity = trigger.entity;
    let mut animator = martial.get_mut(entity).unwrap();
    animator.set_bool("can_move", true);
}

fn check_contact(
    spatial_query: SpatialQuery,
    mut query: Query<(&Transform, &mut Animator, &Collider), With<Martial>>,
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
        let max_distance_notice = 550.;
        let max_hits = 1;

        let config_y = ShapeCastConfig::from_max_distance(max_distance_y);
        let config_x = ShapeCastConfig::from_max_distance(max_distance_x);
        let notice_config = ShapeCastConfig::from_max_distance(max_distance_notice);
        let filter = SpatialQueryFilter::default().with_mask(GameLayer::Ground);
        let notice_filter =
            SpatialQueryFilter::default().with_mask(GameLayer::Player);

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
            -direction_x,
            max_hits,
            &notice_config,
            &notice_filter,
        );

        animator.set_bool("is_grounded", hits_ground.len() > 0);
        animator.set_bool("is_on_wall", hits_wall.len() > 0);
        animator.set_bool("is_on_ceiling", hits_ceiling.len() > 0);
        if let Some(_) = hits_notice.get(0) {
            animator.set_bool(
                "is_noticing",
                true,
            );
        } else {
            animator.set_bool("is_noticing", false);
        }
    }
}

fn on_flip_direction(mut query: Query<(&mut Transform, &Animator), With<Martial>>) {
    for (mut transform, animator) in &mut query {
        let facing_direction = animator.get_float("facing_direction");
        if transform.scale.x * facing_direction < 0. {
            transform.scale.x *= -1.;
        }
    }
}

fn on_move(mut query: Query<(&LinearVelocity, &mut Animator), With<Martial>>) {
    for (vel, mut animator) in query.iter_mut() {
        let is_moving = vel.x != 0.;
        animator.set_bool("is_moving", is_moving);
    }
}

pub struct MartialPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for MartialPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(MartialBehaviourPlugin { state: self.state.clone() });
        app.add_systems(OnEnter(self.state.clone()), setup_enemy.run_if(in_state(self.state.clone())));
        app.add_systems(
            FixedUpdate,
            (
                check_contact.run_if(in_state(self.state.clone())),
                on_move.run_if(in_state(self.state.clone())),
                on_flip_direction.run_if(in_state(self.state.clone())),
            ),
        );
        app.add_observer(on_martial_death);
        app.add_observer(on_attack1_enter);
        app.add_observer(on_attack1_exit);
        app.add_observer(on_attack2_enter);
        app.add_observer(on_attack2_exit);
        app.add_observer(on_stun_enter);
        app.add_observer(on_stun_exit);
    }
}