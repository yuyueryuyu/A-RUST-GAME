use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use bevy_kira_audio::AudioInstance;
use bevy_kira_audio::AudioTween;
use bevy_tnua::prelude::*;
use std::collections::HashSet;
use std::process::CommandArgs;

use crate::animator::Condition;
use crate::animator::*;
use crate::damagable::*;
use crate::game_layer::GameLayer;
use crate::input;
use crate::input::*;
use crate::controller::*;
use crate::items::Item;
use crate::items::ItemList;
use crate::items::ItemType;
use crate::physics::*;

const START_POSITION: Vec3 = Vec3::new(120., 44.1, 0.0);
#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Art/Adventurer/adventurer-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 20, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animator = setup_animator();
    let collider_layer = CollisionLayers::new(
        GameLayer::Player,
        [
            GameLayer::Default,
            GameLayer::Ground,
            GameLayer::EnemyHitBox,
        ],
    );
    let item_list = ItemList {
        items: vec![Item::new(
            ItemType::HealthPotion, 
            String::from("Art/Kyrise's 16x16 RPG Icon Pack - V1.3/icons/16x16/potion_02a.png"), 5
        )],
        item_now: 0,
    };
    commands.spawn((
        Player,
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animator.first_index,
            }),
            ..default()
        },
        Transform::from_translation(START_POSITION),
        PlayerInputBundle::default(),
        animator,
        ControllerBundle::new(11.8),
        PhysicsBundle {
            collider: Collider::capsule_endpoints(6.0, Vec2::Y * 7.0, Vec2::NEG_Y * 11.0),
            layer: collider_layer,
            friction: Friction::new(-0.1),
            ..default()
        },
        item_list,
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
    Attack1Prep,
    Attack1,
    Attack1End,
    Attack2Prep,
    Attack2,
    Attack2End,
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
            Self::Attack1Prep => (13, 14),
            Self::Attack1 => (15, 16),
            Self::Attack1End => (17, 17),
            Self::Attack2Prep => (18, 20),
            Self::Attack2 => (21, 22),
            Self::Attack2End => (23, 23),
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
                target_state: "Attack1Prep".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "defense".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Defense".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "items".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Items".to_string(),
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
                target_state: "Attack1Prep".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "defense".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Defense".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "items".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Items".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        on_enter: Some(set_move),
        on_exit: Some(set_not_move),
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
                target_state: "Attack1Prep".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "defense".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Defense".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![Condition {
                    param_name: "items".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Items".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        on_enter: Some(set_move),
        on_exit: Some(set_not_move),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "items".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Items".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "items".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Items".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "defense".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Defense".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "defense".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Defense".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        on_exit: Some(set_landing),
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
        },
        Transition {
            conditions: vec![Condition {
                param_name: "defense".to_string(),
                operator: ConditionOperator::Equals,
                value: AnimatorParam::Trigger(true),
            }],
            target_state: "Defense".to_string(),
            has_exit_time: false,
            exit_time: 0.0,
        },],
        on_enter: Some(set_jump),
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
        on_enter: Some(set_cant_move),
        on_exit: Some(set_can_move),
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
        ],
        loop_animation: false,
        on_enter: Some(set_cant_move),
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
                target_state: "Attack1End".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_attack),
        on_exit: Some(set_not_attack),
        ..default()
    };

    let attack1_end_state = AnimationState {
        name: "Attack1End".to_string(),
        first_index: AnimationType::Attack1End.config_index().0,
        last_index: AnimationType::Attack1End.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "attack".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Attack2Prep".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.1,
            },
        ],
        loop_animation: false,
        on_enter: None,
        on_exit: Some(set_can_move),
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
        ],
        loop_animation: false,
        on_enter: Some(set_cant_move),
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
                target_state: "Attack2End".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_attack),
        on_exit: Some(set_not_attack),
        ..default()
    };

    let attack2_end_state = AnimationState {
        name: "Attack2End".to_string(),
        first_index: AnimationType::Attack2End.config_index().0,
        last_index: AnimationType::Attack2End.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: None,
        on_exit: Some(set_can_move),
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

    let defense_state = AnimationState {
        name : "Defense".to_string(),
        first_index: AnimationType::Hurt.config_index().0,
        last_index: AnimationType::Hurt.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![Condition {
                    param_name: "defense".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Trigger(true),
                }],
                target_state: "Defense".to_string(),
                has_exit_time: false,
                exit_time: 1.0,
            },
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_cant_move),
        on_exit: Some(set_can_move),
        ..default()
    };

    let items_state = AnimationState {
        name : "Items".to_string(),
        first_index: AnimationType::Items.config_index().0,
        last_index: AnimationType::Items.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(set_cant_move),
        on_exit: Some(set_can_move),
        ..default()
    };

    let mut animator = Animator::new();
    animator.add_parameter("is_moving", AnimatorParam::Bool(false));
    animator.add_parameter("is_running", AnimatorParam::Bool(false));
    animator.add_parameter("is_crouching", AnimatorParam::Bool(false));
    animator.add_parameter("is_grounded", AnimatorParam::Bool(true));
    animator.add_parameter("can_move", AnimatorParam::Bool(false));
    animator.add_parameter("velocity_y", AnimatorParam::Float(0.0));
    animator.add_parameter("jump", AnimatorParam::Trigger(false));
    animator.add_parameter("slide", AnimatorParam::Trigger(false));
    animator.add_parameter("is_sliding", AnimatorParam::Bool(false));
    animator.add_parameter("attack", AnimatorParam::Trigger(false));
    animator.add_parameter("hit", AnimatorParam::Trigger(false));
    animator.add_parameter("defense", AnimatorParam::Trigger(false));
    animator.add_parameter("is_alive", AnimatorParam::Bool(true));
    animator.add_parameter("items", AnimatorParam::Trigger(false));
    animator.add_parameter("is_facing_right", AnimatorParam::Bool(true));
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
    animator.add_state(attack1_prep_state);
    animator.add_state(attack2_prep_state);
    animator.add_state(attack1_end_state);
    animator.add_state(attack2_end_state);
    animator.add_state(die_state);
    animator.add_state(hit_state);
    animator.add_state(lie_state);
    animator.add_state(stand_state);
    animator.add_state(defense_state);
    animator.add_state(items_state);

    animator.set_initial_state(
        "Lie",
        AnimationType::Lie.config_index().0,
        AnimationType::Lie.config_index().1,
        10,
    );

    animator
}

fn set_not_attack(mut commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    for child in animator.active_children.iter() {
        commands.entity(*child).despawn_recursive();
    }

    animator.active_children = HashSet::new();
}

fn set_attack(mut commands: &mut Commands, entity: Entity, animator: &mut Animator
    ,asset_server: &Res<AssetServer>, audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
        audio.play(asset_server.load(
            "Audio/SFX/10_Battle_SFX/39_Block_03.wav"));
    for child in animator.active_children.iter() {
        commands.entity(*child).despawn_recursive();
    }
    let collider_layer = CollisionLayers::new(GameLayer::PlayerHitBox, [GameLayer::Enemy]);
    let id = commands
        .spawn((
            Collider::rectangle(30., 10.),
            Transform::from_xyz(10., 0., 0.),
            Sensor,
            collider_layer,
        ))
        .set_parent(entity)
        .id();
    animator.push_active_child(id);
}

fn set_can_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", true);
}

fn set_cant_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    animator.set_bool("can_move", false);
}

fn set_jump(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,asset_server: &Res<AssetServer>, audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
        //println!("jump!");
        audio.play(asset_server.load(
            "Audio/SFX/12_Player_Movement_SFX/30_Jump_03.wav"));
}

fn set_landing(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,asset_server: &Res<AssetServer>, audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
        audio.play(asset_server.load(
            "Audio/SFX/12_Player_Movement_SFX/45_Landing_01.wav"));
}


fn set_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,asset_server: &Res<AssetServer>, audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    if let Some(haudio) = &animator.audio {
        if let Some(instance) = audio_instances.get_mut(haudio) {
            instance.stop(AudioTween::default());
        } else {
            audio.stop();
        }
    }    
    let handle = 
        audio.play(asset_server.load(
            "Audio/SFX/12_Player_Movement_SFX/03_Step_grass_03.wav"))
                .with_playback_rate(2.0).looped().handle();
    animator.audio = Some(handle);
}

fn set_not_move(mut _commands: &mut Commands, _entity: Entity, animator: &mut Animator
    ,_asset_server: &Res<AssetServer>, _audio: &Res<Audio>
    , audio_instances: &mut ResMut<Assets<AudioInstance>>) {
    if let Some(audio) = &animator.audio {
        if let Some(instance) = audio_instances.get_mut(audio) {
            instance.stop(AudioTween::default());
        } else {
            _audio.stop();
        }
    }
    animator.audio = None;
}


fn check_contact(
    mut animator: Single<&mut Animator, With<Player>>,
    controller: Single<&TnuaController, With<Player>>,
) {
    animator.set_bool("is_grounded", !controller.is_airborne().unwrap());
}

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::PlayerInputPlugin { state: self.state.clone() });
        app.add_systems(OnEnter(self.state.clone()), setup_player.run_if(in_state(self.state.clone())));
        app.add_systems(FixedUpdate, check_contact.run_if(in_state(self.state.clone())));
    }
}
