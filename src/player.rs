use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::collections::HashSet;

mod parameters;
use crate::animator::Condition;
use crate::animator::*;
use crate::damagable::*;
use crate::game_layer::GameLayer;
use crate::input;
use crate::input::*;
use parameters::Parameters;

const WALK_SPEED: f32 = 80.0;
const RUN_SPEED: f32 = 120.0;
const CROUCH_SPEED: f32 = 50.0;
const SLIDE_SPEED: f32 = 120.0;
const JUMP_IMPULSE: f32 = 200.0;

#[derive(Component)]
pub struct Player;

pub fn get_speed(animator: &Animator) -> f32 {
    if animator.get_bool("can_move") {
        if animator.get_bool("is_moving") {
            if animator.get_bool("is_crouching") {
                return CROUCH_SPEED;
            }
            if animator.get_bool("is_running") {
                return RUN_SPEED;
            }
            return WALK_SPEED;
        }
    }
    0.0
}


fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Art/Adventurer/adventurer-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 20, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let param = Parameters::new();
    let animator = setup_animator();
    let collider_layer =
        CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Ground, GameLayer::EnemyHitBox]);
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animator.first_index,
            }),
            ..default()
        },
        Transform::from_xyz(120., 44.1, 0.0),
        Player,
        PlayerInputBundle::default(),
        param,
        animator,
        (RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        SweptCcd::default(),
        Mass(1.0),
        LinearVelocity::default(),
        GravityScale(30.0),
        Collider::capsule_endpoints(6.0, Vec2::Y * 7.0, Vec2::NEG_Y * 11.0),
        CollisionMargin(0.1),
        collider_layer),
        Damagable::new(100.),
    ));
}

#[derive(Component)]
enum AnimationType {
    AirAttack1,
    AirAttack2,
    AirAttack3Loop,
    AirAttack3Rdy,
    AirAttack3End,
    Attack1,
    Attack2,
    Attack3,
    Bow,
    BowJump,
    Cast,
    CastLoop,
    CrnrClimb,
    CrnrGrb,
    CrnrJump,
    Crouch,
    CrouchWalk,
    Die,
    DropKick,
    Fall,
    GetUp,
    Hurt,
    Idle,
    IdleWithSword,
    Items,
    Jump,
    Rise,
    Kick,
    KnockDown,
    LaderClimb,
    Punch,
    Run,
    FastRun,
    RunPunch,
    Slide,
    SmrSlt,
    Lie,
    Stand,
    SwordDraw,
    SwordShte,
    Walk,
    WallRun,
    WallSlide,
    Hit,
}

impl AnimationType {
    fn config_index(&self) -> (usize, usize) {
        match self {
            Self::AirAttack3End => (0, 2),
            Self::AirAttack1 => (3, 6),
            Self::AirAttack2 => (7, 9),
            Self::AirAttack3Loop => (10, 11),
            Self::AirAttack3Rdy => (12, 12),
            Self::Attack1 => (13, 17),
            Self::Attack2 => (18, 23),
            Self::Attack3 => (24, 29),
            Self::Bow => (30, 38),
            Self::BowJump => (39, 44),
            Self::Cast => (45, 48),
            Self::CastLoop => (49, 52),
            Self::CrnrClimb => (53, 57),
            Self::CrnrGrb => (58, 61),
            Self::CrnrJump => (62, 63),
            Self::Crouch => (64, 67),
            Self::CrouchWalk => (68, 73),
            Self::Die => (74, 80),
            Self::DropKick => (81, 84),
            Self::Fall => (85, 86),
            Self::Lie => (87, 87),
            Self::GetUp => (87, 93),
            Self::Hurt => (94, 96),
            Self::Idle => (97, 100),
            Self::IdleWithSword => (101, 104),
            Self::Items => (105, 107),
            Self::Jump => (108, 111),
            Self::Rise => (111, 111),
            Self::Kick => (112, 119),
            Self::KnockDown => (120, 126),
            Self::Hit => (121, 121),
            Self::LaderClimb => (127, 130),
            Self::Punch => (131, 143),
            Self::Run => (144, 149),
            Self::RunPunch => (150, 156),
            Self::FastRun => (157, 162),
            Self::Slide => (163, 164),
            Self::SmrSlt => (165, 168),
            Self::Stand => (169, 171),
            Self::SwordDraw => (172, 175),
            Self::SwordShte => (176, 179),
            Self::Walk => (180, 185),
            Self::WallRun => (186, 191),
            Self::WallSlide => (192, 193),
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
                    param_name: "is_crouching".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "jump".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack1".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "slide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
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
                    param_name: "is_running".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Run".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_crouching".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "jump".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack1".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "slide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
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
                    param_name: "is_running".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Walk".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
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
                    param_name: "is_crouching".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "jump".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack1".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "slide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        ..default()
    };

    let crouch_state = AnimationState {
        name: "Crouch".to_string(),
        first_index: AnimationType::Crouch.config_index().0,
        last_index: AnimationType::Crouch.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "is_moving".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "CrouchWalk".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_crouching".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "jump".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "slide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        ..default()
    };

    let crouch_walk_state = AnimationState {
        name: "CrouchWalk".to_string(),
        first_index: AnimationType::CrouchWalk.config_index().0,
        last_index: AnimationType::CrouchWalk.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "is_moving".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_crouching".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_grounded".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "jump".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "slide".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "is_alive".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
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
                    param_name: "velocity_y".to_string(),
                    operator: ConditionOperator::LessOrEqual,
                    value: AnimatorParam::Float(0.0),
                }],
                target_state: "Fall".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
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
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
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
                target_state: "Die".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "hit".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Hit".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        ..default()
    };

    let jump_state = AnimationState {
        name: "Jump".to_string(),
        first_index: AnimationType::Jump.config_index().0,
        last_index: AnimationType::Jump.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Rise".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        ..default()
    };

    let slide_state = AnimationState {
        name: "Slide".to_string(),
        first_index: AnimationType::Slide.config_index().0,
        last_index: AnimationType::Slide.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: true,
            exit_time: 2.0,
        }],
        loop_animation: false,
        on_enter: Some(set_sliding),
        on_exit: Some(set_not_sliding),
        ..default()
    };

    let attack1_state = AnimationState {
        name: "Attack1".to_string(),
        first_index: AnimationType::Attack1.config_index().0,
        last_index: AnimationType::Attack1.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack2".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            }, 
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            }
        ],
        loop_animation: false,
        on_enter: Some(set_attack),
        on_exit: Some(set_not_attack),
        ..default()
    };

    let attack2_state = AnimationState {
        name: "Attack2".to_string(),
        first_index: AnimationType::Attack2.config_index().0,
        last_index: AnimationType::Attack2.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        on_enter: Some(set_attack),
        on_exit: Some(set_not_attack),
        ..default()
    };

    let hit_state = AnimationState {
        name: "Hit".to_string(),
        first_index: AnimationType::Hit.config_index().0,
        last_index: AnimationType::Hit.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: true,
            exit_time: 2.0,
        }],
        loop_animation: false,
        ..default()
    };

    let die_state = AnimationState {
        name: "Die".to_string(),
        first_index: AnimationType::Die.config_index().0,
        last_index: AnimationType::Die.config_index().1,
        transitions: vec![],
        loop_animation: false,
        on_enter: Some(set_cant_move),
        on_exit: None,
        ..default()
    };

    let lie_state = AnimationState {
        name: "Lie".to_string(),
        first_index: AnimationType::Lie.config_index().0,
        last_index: AnimationType::Lie.config_index().1,
        transitions: vec![Transition {
            conditions: vec![Condition {
                param_name: "is_moving".to_string(),
                operator: ConditionOperator::Equals,
                value: AnimatorParam::Bool(true),
            }],
            target_state: "Stand".to_string(),
            has_exit_time: false,
            exit_time: 0.0,
        }],
        loop_animation: false,
        on_enter: Some(set_cant_move),
        on_exit: None,
        ..default()
    };

    let stand_state = AnimationState {
        name: "Stand".to_string(),
        first_index: AnimationType::GetUp.config_index().0,
        last_index: AnimationType::GetUp.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: true,
            exit_time: 1.0,
        }],
        loop_animation: false,
        on_enter: None,
        on_exit: Some(set_can_move),
        ..default()
    };

    let mut animator = Animator::new();
    animator.add_parameter("is_moving", AnimatorParam::Bool(false));
    animator.add_parameter("is_running", AnimatorParam::Bool(false));
    animator.add_parameter("is_crouching", AnimatorParam::Bool(false));
    animator.add_parameter("is_grounded", AnimatorParam::Bool(true));
    animator.add_parameter("can_move", AnimatorParam::Bool(true));
    animator.add_parameter("velocity_y", AnimatorParam::Float(0.0));
    animator.add_parameter("jump", AnimatorParam::Trigger(false));
    animator.add_parameter("slide", AnimatorParam::Trigger(false));
    animator.add_parameter("is_sliding", AnimatorParam::Bool(false));
    animator.add_parameter("attack", AnimatorParam::Trigger(false));
    animator.add_parameter("hit", AnimatorParam::Trigger(false));
    animator.add_parameter("is_alive", AnimatorParam::Bool(true));

    animator.add_parameter("is_facing_right", AnimatorParam::Bool(true));
    animator.add_parameter("is_on_wall", AnimatorParam::Bool(false));
    animator.add_parameter("is_on_ceiling", AnimatorParam::Bool(false));
    animator.add_parameter("shift_press_time", AnimatorParam::Float(0.0));
    animator.add_parameter("impulse_x", AnimatorParam::Float(0.0));

    animator.add_state(idle_state);
    animator.add_state(walk_state);
    animator.add_state(run_state);
    animator.add_state(crouch_state);
    animator.add_state(crouch_walk_state);
    animator.add_state(rise_state);
    animator.add_state(fall_state);
    animator.add_state(jump_state);
    animator.add_state(slide_state);
    animator.add_state(attack1_state);
    animator.add_state(attack2_state);
    animator.add_state(die_state);
    animator.add_state(hit_state);
    animator.add_state(lie_state);
    animator.add_state(stand_state);
    

    animator.set_initial_state(
        "Lie",
        AnimationType::Lie.config_index().0,
        AnimationType::Lie.config_index().1,
        10,
    );

    animator
}

fn set_not_attack(mut commands: &mut Commands, entity: Entity, animator: &mut Animator) {
    animator.set_bool("can_move", true);
    for child in animator.active_children.iter() {
        commands.entity(*child).despawn_recursive();
    }

    animator.active_children = HashSet::new();
}

fn set_attack(mut commands: &mut Commands, entity: Entity, animator: &mut Animator) {
    animator.set_bool("can_move", false);
    for child in animator.active_children.iter() {
        commands.entity(*child).despawn_recursive();
    }
    let collider_layer =
        CollisionLayers::new(GameLayer::PlayerHitBox, [GameLayer::Enemy]);
    let id = commands.spawn((
        Collider::rectangle(30., 10.),
        Transform::from_xyz(10., 0., 0.),
        Sensor,
        collider_layer,
    )).set_parent(entity).id();
    animator.push_active_child(id);
}

fn set_can_move(mut commands: &mut Commands, entity: Entity, animator: &mut Animator) {
    animator.set_bool("can_move", true);
}

fn set_cant_move(mut commands: &mut Commands, entity: Entity, animator: &mut Animator) {
    animator.set_bool("can_move", false);
}

fn set_sliding(mut commands: &mut Commands, entity: Entity, animator: &mut Animator) {
    animator.set_bool("can_move", false);
    animator.set_bool("is_sliding", true);
}

fn set_not_sliding(mut commands: &mut Commands, entity: Entity, animator: &mut Animator) {
    animator.set_bool("can_move", true);
    animator.set_bool("is_sliding", false);
}

fn check_contact(
    spatial_query: SpatialQuery,
    query: Query<(&Transform, &Collider), With<Player>>,
    mut param: Single<&mut Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    for (transform, collider) in &query {
        let origin = Vec2::new(transform.translation.x, transform.translation.y);
        let rotation = transform.rotation.z;
        let direction_x = if param.get_bool("is_facing_right") {
            Dir2::X
        } else {
            Dir2::NEG_X
        };
        let max_distance_y = 0.2;
        let max_distance_x = 0.5;
        let max_hits = 1;

        let config_y = ShapeCastConfig::from_max_distance(max_distance_y);
        let config_x = ShapeCastConfig::from_max_distance(max_distance_x);
        let filter = SpatialQueryFilter::default().with_mask(GameLayer::Ground);

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

        param.set_bool("is_grounded", hits_ground.len() > 0);
        animator.set_bool("is_grounded", hits_ground.len() > 0);

        param.set_bool("is_on_wall", hits_wall.len() > 0);
        param.set_bool("is_on_ceiling", hits_ceiling.len() > 0);
    }
}

fn check_hitbox(
    collisions: Res<Collisions>,
    mut query: Query<(Entity, &Transform), With<Sensor>>,
    mut actor_query: Query<(Entity, &mut Damagable, &mut Animator, &mut ExternalImpulse, &Transform), With<Player>>,
) {
    for (entity, transform) in &query {
        for (aentity, mut damagable, mut animator, mut impulse, atransform) in &mut actor_query {
            if collisions.contains(aentity, entity) || collisions.contains(entity, aentity){
                let dir = if (atransform.translation - transform.translation).x >= 0. {
                    1.
                } else {
                    -1.
                };
                if !damagable.is_invincible {
                    //impulse.apply_impulse(Vec2::new(dir * 300., 10.));
                    animator.set_trigger("hit");
                }
                damagable.take_hit(10.);
                println!("PlayerHealth:{}",damagable.health);
                
            }
        }

    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::PlayerInputPlugin);
        app.add_systems(Startup, (setup_player,).chain());
        app.add_systems(
            FixedUpdate,
            (
                check_contact,
                check_hitbox,
            )
                .chain(),
        );
    }
}
