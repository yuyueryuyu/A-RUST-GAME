use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::Length;
use bevy_inspector_egui::egui_utils::easymark::parser::Item;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use bevy_kira_audio::AudioInstance;
use bevy_kira_audio::AudioTween;
use bevy_tnua::prelude::*;
use moonshine_save::save::Save;
use my_bevy_game::enter;
use my_bevy_game::exit;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process::CommandArgs;

use crate::animator::Condition;
use crate::animator::*;
use crate::damagable::*;
use crate::game_layer::GameLayer;
use crate::healthbar::ItemImg;
use crate::hint::ItemHint;
use crate::input;
use crate::input::*;
use crate::controller::*;
use crate::items::item_canpick_observer;
use crate::items::item_cantpick_observer;
use crate::items::ActiveItems;
use crate::items::HasItem;
use crate::items::ItemBag;
use crate::items::ItemList;
use crate::items::ItemOf;
use crate::items::ItemType;
use crate::items::NotpickedItems;
use crate::items::UseItemTrigger;
use crate::physics::*;
use crate::save::TransformData;

#[derive(Component, Reflect)]
#[require(Save)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut transform_data: ResMut<TransformData>,
    items: Res<ItemList>,
) {
    let texture = asset_server.load("Art/Adventurer/adventurer-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 20, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animator = setup_animator(transform_data.params.clone());
    let collider_layer = CollisionLayers::new(
        GameLayer::Player,
        [
            GameLayer::Default,
            GameLayer::Ground,
            GameLayer::EnemyHitBox,
            GameLayer::Sensor,
        ],
    );
    let translation = Vec3::new(transform_data.translation[0], transform_data.translation[1], transform_data.translation[2]);
    let scale = Vec3::new(transform_data.scale[0], transform_data.scale[1], transform_data.scale[2]);
    commands.spawn((
        Player,
        Save,
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animator.first_index,
            }),
            ..default()
        },
        Transform::from_translation(translation).with_scale(scale),
        PlayerInputBundle::default(),
        animator,
        ControllerBundle::new(11.8),
        PhysicsBundle {
            collider: Collider::capsule_endpoints(6.0, Vec2::Y * 7.0, Vec2::NEG_Y * 11.0),
            layer: collider_layer,
            friction: Friction::new(-0.1),
            ..default()
        },
        ItemBag { slots: HashMap::new() },
        ActiveItems { items: vec![], current: 0 },
        transform_data.damagable.clone(),
    ));

    commands.spawn((
        Sprite {
            image: items.infos.get(&String::from("HealthPotion")).unwrap().icon.clone(),
            ..default()
        },
        Collider::rectangle(20.0, 20.0),
        Transform::from_xyz(translation.x, translation.y, 0.0),
        ItemHint,
        NotpickedItems { id: "HealthPotion".to_string(), num: 5 },
        Sensor,
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player])
    )).observe(item_cantpick_observer).observe(item_canpick_observer);
}

#[derive(Reflect)]
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

fn setup_animator(params: HashMap<String, AnimatorParam>) -> Animator {
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
        audio_path: Some("Audio/SFX/12_Player_Movement_SFX/03_Step_grass_03.wav".to_string()),
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
        audio_path: Some("Audio/SFX/12_Player_Movement_SFX/03_Step_grass_03.wav".to_string()),
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
        audio_path: Some("Audio/SFX/12_Player_Movement_SFX/03_Step_grass_03.wav".to_string()),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "is_on_wall".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "WallSlide".to_string(),
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
            Transition {
                conditions: vec![Condition {
                    param_name: "is_on_wall".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(true),
                }],
                target_state: "WallSlide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        on_exit: Some(__fall_exit_handler),
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
        on_enter: Some(__jump_enter_handler),
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
        on_enter: Some(__slide_enter_handler),
        on_exit: Some(__slide_exit_handler),
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
                target_state: "Attack1End".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(__attack_enter_handler),
        on_exit: Some(__attack_exit_handler),
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
        on_exit: Some(__stun_exit_handler),
        audio_path: Some("Audio/SFX/10_Battle_SFX/39_Block_03.wav".to_string()),
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
                target_state: "Attack2End".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            },
        ],
        loop_animation: false,
        on_enter: Some(__attack_enter_handler),
        on_exit: Some(__attack_exit_handler),
        audio_path: Some("Audio/SFX/10_Battle_SFX/39_Block_03.wav".to_string()),
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
        on_exit: Some(__stun_exit_handler),
        ..default()
    };

    let hit_state = AnimationState {
        name: "Hit".to_string(),
        first_index: AnimationType::Hit.config_index().0,
        last_index: AnimationType::Hit.config_index().1,
        transitions: vec![Transition {
            conditions: vec![],
            target_state: "Idle".to_string(),
            has_exit_time: false,
            exit_time: 3.0,
        }],
        loop_animation: false,
        on_enter: Some(__stun_enter_handler),
        on_exit: Some(__stun_exit_handler),
        ..default()
    };

    let die_state = AnimationState {
        name: "Die".to_string(),
        first_index: AnimationType::Die.config_index().0,
        last_index: AnimationType::Die.config_index().1,
        transitions: vec![Transition {
            conditions: vec![Condition {
                param_name: "revival".to_string(),
                operator: ConditionOperator::Equals,
                value: AnimatorParam::Trigger(true),
            }],
            target_state: "Idle".to_string(),
            has_exit_time: false,
            exit_time: 0.0,
        }],
        loop_animation: false,
        on_enter: Some(__stun_enter_handler),
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
        on_enter: Some(__stun_enter_handler),
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
        on_exit: Some(__stun_exit_handler),
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
        on_enter: Some(__stun_enter_handler),
        on_exit: Some(__stun_exit_handler),
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
        on_enter: Some(__item_enter_handler),
        on_exit: Some(__item_exit_handler),
        ..default()
    };

    let wall_slide_state = AnimationState {
        name: "WallSlide".to_string(),
        first_index: AnimationType::WallSlide.config_index().0,
        last_index: AnimationType::WallSlide.config_index().1,
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
                    param_name: "is_on_wall".to_string(),
                    operator: ConditionOperator::Equals,
                    value: AnimatorParam::Bool(false),
                }],
                target_state: "Rise".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            }
        ],
        loop_animation: true,
        ..default()
    };

    let mut animator = Animator::new().with_params(params.clone());
    if params.is_empty() {
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
        animator.add_parameter("revival", AnimatorParam::Trigger(false));
        animator.add_parameter("is_on_wall", AnimatorParam::Bool(false));
        animator.add_parameter("can_wall_jump", AnimatorParam::Bool(false));
    }

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
    animator.add_state(wall_slide_state);
    animator.set_initial_state(
        "Lie",
        AnimationType::Lie.config_index().0,
        AnimationType::Lie.config_index().1,
        10,
    );

    animator
}

#[enter("attack")]
fn on_attack_enter(
    mut commands: Commands
) {
    let entity = trigger.entity;
    commands.spawn((
            Collider::rectangle(30., 10.),
            Transform::from_xyz(10., 0., 0.),
            Sensor,
            HitBox { damage: 1000. },
            CollisionLayers::new(GameLayer::PlayerHitBox, [GameLayer::Enemy]),
            CollisionEventsEnabled,
            ChildOf(entity),
            HitboxOf(entity),
        )).observe(check_hitbox);
}

#[exit("attack")]
fn on_attack_exit(
    mut commands: Commands,
    player: Query<&HasHitbox, With<Player>>,
) {
    let entity = trigger.entity;
    let hitboxes = player.get(entity).unwrap();
    let vec = (**hitboxes).clone();
    for hitbox in vec {
        commands.entity(hitbox).despawn();
    }
}

#[enter("stun")]
fn on_stun_enter(
    mut player: Query<&mut Animator, With<Player>>,
) {
    let entity = trigger.entity;
    let mut animator = player.get_mut(entity).unwrap();
    animator.set_bool("can_move", false);
}

#[exit("stun")]
fn on_stun_exit(
    mut player: Query<&mut Animator, With<Player>>,
) {
    let entity = trigger.entity;
    let mut animator = player.get_mut(entity).unwrap();
    animator.set_bool("can_move", true);
}

#[enter("item")]
fn on_item_enter(
    mut commands: Commands,
    player: Query<&ActiveItems, With<Player>>,
    item_list: Res<ItemList>,
) {
    let entity = trigger.entity;
    let items = player.get(entity).unwrap();
    if items.items.is_empty() { return; }
    let item_name = items.get_current_item();
    let item_info = item_list.infos.get(&item_name).unwrap();
    commands.trigger(UseItemTrigger { user: entity, item: items.get_current_item().clone() });
    commands.spawn((
        Sprite {
            image: item_info.icon.clone(),
            ..default()
        },
        Transform::from_xyz(10.,10.,0.).with_scale(Vec3::new(0.8, 0.8, 0.8)),
        ChildOf(entity),
        ItemOf(entity),
    ));
} 

#[exit("item")]
fn on_item_exit(
    mut commands: Commands,
    player: Query<&HasItem, With<Player>>,
) {
    let entity = trigger.entity;
    let items = player.get(entity).unwrap();
    let vec = (**items).clone();
    for item in vec {
        commands.entity(item).despawn();
    }
} 

#[enter("jump")]
fn on_jump_enter(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load(
        "Audio/SFX/12_Player_Movement_SFX/30_Jump_03.wav"));
}

#[exit("fall")]
fn on_fall_exit(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load(
        "Audio/SFX/12_Player_Movement_SFX/45_Landing_01.wav"));
}

#[enter("slide")]
fn on_slide_enter() {

}

#[exit("slide")]
fn on_slide_exit() {

}

fn check_contact(
    mut player: Single<(&mut Animator, &Transform, &Collider), With<Player>>,
    controller: Single<&TnuaController, With<Player>>,
    spatial_query: SpatialQuery,
) {
    let (mut animator, transform, collider) = player.into_inner();
    let is_grounded = !controller.is_airborne().unwrap();
    animator.set_bool("is_grounded", is_grounded);
    let origin = Vec2::new(transform.translation.x, transform.translation.y);
    let rotation = transform.rotation.z;
    let direction_x = if animator.get_bool("is_facing_right") {
        Dir2::X
    } else {
        Dir2::NEG_X
    };
    let max_distance_x = 0.5;
    let max_hits = 1;
    let config_x = ShapeCastConfig::from_max_distance(max_distance_x);
    let filter = SpatialQueryFilter::default().with_mask(GameLayer::Ground);
    let hits_wall = spatial_query.shape_hits(
        &collider,
        origin,
        rotation,
        direction_x,
        max_hits,
        &config_x,
        &filter,
    );
    animator.set_bool("is_on_wall", hits_wall.length() > 0 && !is_grounded);
}

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::PlayerInputPlugin { state: self.state.clone() });
        app.add_systems(OnEnter(self.state.clone()), setup_player.run_if(in_state(self.state.clone())));
        app.add_systems(FixedUpdate, check_contact.run_if(in_state(self.state.clone())));
        app.add_observer(on_attack_enter);
        app.add_observer(on_attack_exit);
        app.add_observer(on_stun_enter);
        app.add_observer(on_stun_exit);
        app.add_observer(on_item_enter);
        app.add_observer(on_item_exit);
        app.add_observer(on_jump_enter);
        app.add_observer(on_fall_exit);
        app.add_observer(on_slide_enter);
        app.add_observer(on_slide_exit);
    }
}
