//! 飞行眼睛敌人

use avian2d::prelude::*;
use bevy::prelude::*;
use big_brain::prelude::*;
use game_derive::enter;
use game_derive::exit;
use rand::Rng;

use crate::animator::Condition;
use crate::animator::*;
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
struct FlyingEyes;

fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture =
        asset_server.load("Art/Monster_Creatures_Fantasy(Version 1.3)/flying_eyes_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(150, 150), 8, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    spawn_enemy(
        &mut commands,
        Vec2::new(640.0, 573.1),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(820.0, 573.1),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(2886.0, 174.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1400.0, 509.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1600.0, 509.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1840.0, 509.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1960.0, 509.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(2160.0, 509.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(2360.0, 509.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1787.0, 685.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1987.0, 685.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(2345.0, 685.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(977.0, 637.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1177.0, 637.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(1528.0, 637.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(41., 594.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(241., 594.),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_enemy(
        &mut commands,
        Vec2::new(541., 584.),
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

    let entity = commands.spawn((
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
        FlyingEyes,
        ControllerBundle::new(14.),
        PhysicsBundle {
            collider: Collider::capsule_endpoints(
                15.0, 
                Vec2::Y * 3.0, 
                Vec2::NEG_Y * 3.0
            ),
            layer: collider_layer,
            ..default()
        },
        Damagable::new(100.),
        animator,
        Notice::new(0.0, 50.0, 10.0),
        thinker, 
    )).id();
    commands.spawn((
            Collider::rectangle(30., 30.),
            Transform::from_xyz(0., 0., 0.),
            Sensor,
            HitBox { damage: 20. },
            CollisionLayers::new(GameLayer::EnemyHitBox, [GameLayer::Player]),
            CollisionEventsEnabled,
            ChildOf(entity),
            HitboxOf(entity),
        )).observe(check_hitbox);
}

#[derive(Component, Reflect)]
enum AnimationType {
    Attack,
    Attack2,
    Attack3,
    Death,
    Flight,
    Hurt,
}

impl AnimationType {
    fn config_index(&self) -> (usize, usize) {
        match self {
            Self::Attack => (16, 23),
            Self::Attack2 => (24, 31),
            Self::Attack3 => (8, 13),
            Self::Death => (0, 3),
            Self::Hurt => (4, 7),
            Self::Flight => (32, 39),
        }
    }
}

fn setup_animator() -> Animator {
    let flight_state = AnimationState {
        name: "Flight".to_string(),
        first_index: AnimationType::Flight.config_index().0,
        last_index: AnimationType::Flight.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack2".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack2".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack3".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack3".to_string(),
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

    let attack_state = AnimationState {
        name: "Attack".to_string(),
        first_index: AnimationType::Attack.config_index().0,
        last_index: AnimationType::Attack.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Flight".to_string(),
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
        ..default()
    };

    let attack2_state = AnimationState {
        name: "Attack2".to_string(),
        first_index: AnimationType::Attack2.config_index().0,
        last_index: AnimationType::Attack2.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Flight".to_string(),
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
        ..default()
    };


    let attack3_state = AnimationState {
        name: "Attack3".to_string(),
        first_index: AnimationType::Attack3.config_index().0,
        last_index: AnimationType::Attack3.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Flight".to_string(),
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
        ..default()
    };


    let death_state = AnimationState {
        name: "Death".to_string(),
        first_index: AnimationType::Death.config_index().0,
        last_index: AnimationType::Death.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Flight".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        on_enter: Some(__death_enter_handler),
        on_exit: Some(__death_exit_handler),
        ..default()
    };

    let hurt_state = AnimationState {
        name: "Hurt".to_string(),
        first_index: AnimationType::Hurt.config_index().0,
        last_index: AnimationType::Hurt.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Flight".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        ..default()
    };

    let mut animator = Animator::new();
    animator.add_parameter("attack", AnimatorParam::Trigger(false));
    animator.add_parameter("attack2", AnimatorParam::Trigger(false));
    animator.add_parameter("attack3", AnimatorParam::Trigger(false));
    animator.add_parameter("is_alive", AnimatorParam::Bool(true));
    animator.add_parameter("hit", AnimatorParam::Trigger(false));
    animator.add_parameter("is_on_wall", AnimatorParam::Bool(false));
    animator.add_parameter("facing_direction", AnimatorParam::Float(1.0));
    animator.add_parameter("is_noticing", AnimatorParam::Bool(false));
    animator.add_parameter("noticed", AnimatorParam::Bool(false));
    animator.add_parameter("can_move", AnimatorParam::Bool(true));

    animator.add_state(flight_state);
    animator.add_state(attack_state);
    animator.add_state(attack2_state);
    animator.add_state(attack3_state);
    animator.add_state(death_state);
    animator.add_state(hurt_state);

    animator.set_initial_state(
        "Flight",
        AnimationType::Flight.config_index().0,
        AnimationType::Flight.config_index().1,
        8,
    );

    animator
}

#[enter("death")]
fn on_death_enter(
    mut commands: Commands,
    mut player: Query<(&mut Animator, &HasHitbox), With<FlyingEyes>>,
) {
    let entity = trigger.entity;
    let (mut animator, hitboxes) = player.get_mut(entity).unwrap();
    animator.set_bool("can_move", false);
    for hitbox in (**hitboxes).clone() {
        commands.entity(hitbox).despawn();
    }
}

#[exit("death")]
fn on_death_exit(
    mut commands: Commands, 
    items: Res<ItemList>, 
    transform: Query<&Transform, With<FlyingEyes>>,
) {
    let entity = trigger.entity;
    let x = transform.get(entity).unwrap().translation.x;
    let y = transform.get(entity).unwrap().translation.y;
    commands.entity(entity).despawn();
    let rand :f32 = rand::rng().random();
    if rand < 0.2 {
        commands.spawn((
            Sprite {
                image: items.infos.get(&String::from("HealthPotion")).unwrap().icon.clone(),
                ..default()
            },
            Collider::rectangle(20.0, 20.0),
            Transform::from_xyz(x, y, 0.0),
            ItemHint,
            NotpickedItems { id: "HealthPotion".to_string(), num: 1 },
            Sensor,
            CollisionEventsEnabled,
            CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player])
        )).observe(item_cantpick_observer).observe(item_canpick_observer);
    }
}

fn check_contact(
    spatial_query: SpatialQuery,
    mut query: Query<(&Transform, &mut Animator, &Collider), With<FlyingEyes>>,
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

fn on_flip_direction(mut query: Query<(&mut Transform, &Animator), With<FlyingEyes>>) {
    for (mut transform, animator) in &mut query {
        let facing_direction = animator.get_float("facing_direction");
        if transform.scale.x * facing_direction < 0. {
            transform.scale.x *= -1.;
        }
    }
}

pub struct FlyingEyesPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for FlyingEyesPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlyingEyesBehaviourPlugin { state: self.state.clone() });
        app.add_systems(OnEnter(self.state.clone()), setup_enemy.run_if(in_state(self.state.clone())));
        app.add_systems(
            FixedUpdate,
            (
                check_contact.run_if(in_state(self.state.clone())),
                on_flip_direction.run_if(in_state(self.state.clone())),
            ),
        );
        app.add_observer(on_death_enter);
        app.add_observer(on_death_exit);
    }
}
