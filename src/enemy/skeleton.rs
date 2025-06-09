use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioInstance};
use big_brain::prelude::*;
use my_bevy_game::{enter, exit};
use std::collections::HashSet;

use crate::animator::Condition;
use crate::animator::*;
use crate::controller::ControllerBundle;
use crate::damagable::{check_hitbox, Damagable, HasHitbox, HitBox, HitboxOf};
use crate::game_layer::GameLayer;
use crate::physics::PhysicsBundle;
use crate::player::Player;
mod behaviour;
use behaviour::*;

#[derive(Component, Reflect)]
struct Skeleton;

fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture =
        asset_server.load("Art/Monster_Creatures_Fantasy(Version 1.3)/Skeleton_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(150, 150), 6, 7, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    spawn_enemy(
        &mut commands,
        Vec2::new(250.0, 45.1),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(400.0, 45.2),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(250.0, 157.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(400.0, 157.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(400.0, 230.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(500.0, 230.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(726.0, 413.),
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

    let patrol = Steps::build().label("Patrol").step(Patrol);

    let thinker = Thinker::build()
        .label("Thinker")
        .picker(Highest)
        .when(NoticeScorer, move_and_attack)
        .when(PatrolScorer, patrol);

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animator.first_index,
            }),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0).with_scale(
            Vec3::new(0.7, 0.7, 0.7)
        ),
        Skeleton,
        ControllerBundle::new(14.),
        PhysicsBundle {
            collider: Collider::capsule_endpoints(
                10.0, 
                Vec2::Y * 16.0, 
                Vec2::NEG_Y * 16.0
            ),
            layer: collider_layer,
            ..default()
        },
        Damagable::new(100.),
        animator,
        Notice::new(0.0, 50.0, 10.0),
        thinker,
    ));
}

#[derive(Component, Reflect)]
enum AnimationType {
    AttackPrep,
    Attack,
    Attack2,
    Attack3,
    Death,
    Idle,
    Hurt,
    Sheild,
    Walk,
}

impl AnimationType {
    fn config_index(&self) -> (usize, usize) {
        match self {
            Self::AttackPrep => (0, 5),
            Self::Attack => (6, 7),
            Self::Attack2 => (8, 15),
            Self::Attack3 => (16, 21),
            Self::Death => (22, 25),
            Self::Idle => (26, 29),
            Self::Sheild => (30, 33),
            Self::Hurt => (34, 37),
            Self::Walk => (38, 41),
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
        ],
        loop_animation: false,
        on_enter: Some(__stun_enter_handler),
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
        ],
        loop_animation: false,
        on_enter: Some(__attack_enter_handler),
        on_exit: Some(__attack_exit_handler),
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

    let mut animator = Animator::new();
    animator.add_parameter("is_moving", AnimatorParam::Bool(false));
    animator.add_parameter("can_move", AnimatorParam::Bool(true));
    animator.add_parameter("attack", AnimatorParam::Trigger(false));
    animator.add_parameter("is_alive", AnimatorParam::Bool(true));
    animator.add_parameter("hit", AnimatorParam::Trigger(false));
    animator.add_parameter("is_grounded", AnimatorParam::Bool(true));
    animator.add_parameter("is_on_wall", AnimatorParam::Bool(false));
    animator.add_parameter("is_on_ceiling", AnimatorParam::Bool(false));
    animator.add_parameter("facing_direction", AnimatorParam::Float(1.0));
    animator.add_parameter("is_noticing", AnimatorParam::Bool(false));
    animator.add_parameter("noticed", AnimatorParam::Bool(false));

    animator.add_state(idle_state);
    animator.add_state(walk_state);
    animator.add_state(attack_state);
    animator.add_state(attack_prep_state);
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

#[exit("death")]
fn on_death_exit(mut commands: Commands) {
    let entity = trigger.entity;
    commands.entity(entity).despawn();
}

#[enter("attack")]
fn on_attack_enter(
    mut commands: Commands
) {
    let entity = trigger.entity;
    commands.spawn((
            Collider::rectangle(30., 10.),
            Transform::from_xyz(30., 0., 0.),
            Sensor,
            HitBox { damage: 20. },
            CollisionLayers::new(GameLayer::EnemyHitBox, [GameLayer::Player]),
            CollisionEventsEnabled,
            ChildOf(entity),
            HitboxOf(entity),
        )).observe(check_hitbox);
}

#[exit("attack")]
fn on_attack_exit(
    mut commands: Commands,
    enemy: Query<&HasHitbox, With<Skeleton>>,
) {
    let entity = trigger.entity;
    let hitboxes = enemy.get(entity).unwrap();
    let vec = (**hitboxes).clone();
    for hitbox in vec {
        commands.entity(hitbox).despawn();
    }
}


#[enter("stun")]
fn on_stun_enter(
    mut player: Query<&mut Animator, With<Skeleton>>,
) {
    let entity = trigger.entity;
    let mut animator = player.get_mut(entity).unwrap();
    animator.set_bool("can_move", false);
}

#[exit("stun")]
fn on_stun_exit(
    mut player: Query<&mut Animator, With<Skeleton>>,
) {
    let entity = trigger.entity;
    let mut animator = player.get_mut(entity).unwrap();
    animator.set_bool("can_move", true);
}

fn check_contact(
    spatial_query: SpatialQuery,
    mut query: Query<(&Transform, &mut Animator, &Collider), With<Skeleton>>,
    entity: Single<Entity, With<Player>>,
) {
    let collider_notice = Collider::rectangle(1., 20.);

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

fn on_flip_direction(mut query: Query<(&mut Transform, &Animator), With<Skeleton>>) {
    for (mut transform, animator) in &mut query {
        let facing_direction = animator.get_float("facing_direction");
        if transform.scale.x * facing_direction < 0. {
            transform.scale.x *= -1.;
        }
    }
}

fn on_move(mut query: Query<(&LinearVelocity, &mut Animator), With<Skeleton>>) {
    for (vel, mut animator) in query.iter_mut() {
        let is_moving = vel.x != 0.;
        animator.set_bool("is_moving", is_moving);
    }
}

pub struct SkeletonPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for SkeletonPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkeletonBehaviourPlugin { state: self.state.clone() });
        app.add_systems(OnEnter(self.state.clone()), setup_enemy.run_if(in_state(self.state.clone())));
        app.add_systems(
            FixedUpdate,
            (
                check_contact.run_if(in_state(self.state.clone())),
                on_move.run_if(in_state(self.state.clone())),
                on_flip_direction.run_if(in_state(self.state.clone())),
            ),
        );
        app.add_observer(on_death_exit);
        app.add_observer(on_attack_enter);
        app.add_observer(on_attack_exit);
        app.add_observer(on_stun_enter);
        app.add_observer(on_stun_exit);
    }
}
