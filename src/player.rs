use bevy::prelude::*;
use avian2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod parameters;
use parameters::Parameters;
use crate::game_layer::GameLayer;
use crate::animator::*;
use crate::animator::Condition;

const WALK_SPEED: f32 = 80.0;
const RUN_SPEED: f32 = 120.0;
const CROUCH_SPEED: f32 = 50.0;
const SLIDE_SPEED: f32 = 200.0;
const JUMP_IMPULSE: f32 = 160.0;

fn get_speed(param: &Parameters, animator: &Animator) -> f32 {
    if animator.get_bool("can_move") {
        if param.get_bool("is_moving") {
            if param.get_bool("is_crouching") {
                return CROUCH_SPEED;
            }
            if param.get_bool("is_running") {
                return RUN_SPEED;
            }
            return WALK_SPEED;
        }
    }
    0.0
}

#[derive(Component)]
pub struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Art/Adventurer/adventurer-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 20, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let param = Parameters::new();
    let animator = setup_animator();
    let collider_layer = CollisionLayers::new(
        GameLayer::Player,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Ground],
    );
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animator.first_index,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        PlayerInput {
            player: Player,
            input_manager: InputManagerBundle::with_map(PlayerInput::default_input_map()),
        },
        param,
        animator,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        SweptCcd::default(),
        Mass(1.0),
        LinearVelocity::default(),
        GravityScale(30.0),
        Collider::capsule_endpoints(6.0, Vec2::Y * 7.0, Vec2::NEG_Y * 11.0),
        collider_layer,
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
    Stand,
    SwordDraw,
    SwordShte,
    Walk,
    WallRun,
    WallSlide,
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
            Self::GetUp => (87, 93),
            Self::Hurt => (94, 96),
            Self::Idle => (97, 100),
            Self::IdleWithSword => (101, 104),
            Self::Items => (105, 107),
            Self::Jump => (108, 111),
            Self::Rise => (111, 111),
            Self::Kick => (112, 119),
            Self::KnockDown => (120, 126),
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
                conditions: vec![
                    Condition {
                        param_name: "is_moving".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    }
                ],
                target_state: "Walk".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_crouching".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    }
                ],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "jump".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "attack".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Attack1".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "slide".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation: true,
        on_enter: None,
        on_exit: None,
    };
    
    let walk_state = AnimationState {
        name: "Walk".to_string(),
        first_index: AnimationType::Walk.config_index().0,
        last_index: AnimationType::Walk.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_moving".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    }
                ],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_running".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    }
                ],
                target_state: "Run".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_crouching".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    }
                ],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "jump".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "attack".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Attack1".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "slide".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
    };

    let run_state = AnimationState {
        name: "Run".to_string(),
        first_index: AnimationType::Run.config_index().0,
        last_index: AnimationType::Run.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_running".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Walk".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_moving".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_crouching".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    }
                ],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "jump".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "attack".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Attack1".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "slide".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
    };

    let crouch_state = AnimationState {
        name: "Crouch".to_string(),
        first_index: AnimationType::Crouch.config_index().0,
        last_index: AnimationType::Crouch.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_moving".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    },
                ],
                target_state: "CrouchWalk".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_crouching".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "jump".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "slide".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
    };

    let crouch_walk_state = AnimationState {
        name: "CrouchWalk".to_string(),
        first_index: AnimationType::CrouchWalk.config_index().0,
        last_index: AnimationType::CrouchWalk.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_moving".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Crouch".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_crouching".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(false),
                    },
                ],
                target_state: "Fall".to_string(),
                has_exit_time: true,
                exit_time: 0.5,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "jump".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Jump".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "slide".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Trigger(true),
                    },
                ],
                target_state: "Slide".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
    };

    let rise_state = AnimationState {
        name: "Rise".to_string(),
        first_index: AnimationType::Rise.config_index().0,
        last_index: AnimationType::Rise.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "velocity_y".to_string(),
                        operator: ConditionOperator::LessOrEqual,
                        value: AnimatorParam::Float(0.0),
                    },
                ],
                target_state: "Fall".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    },
                ],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            }
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
    };

    let fall_state = AnimationState {
        name: "Fall".to_string(),
        first_index: AnimationType::Fall.config_index().0,
        last_index: AnimationType::Fall.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "velocity_y".to_string(),
                        operator: ConditionOperator::Greater,
                        value: AnimatorParam::Float(0.0),
                    },
                ],
                target_state: "Rise".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            },
            Transition {
                conditions: vec![
                    Condition {
                        param_name: "is_grounded".to_string(),
                        operator: ConditionOperator::Equals,
                        value: AnimatorParam::Bool(true),
                    },
                ],
                target_state: "Idle".to_string(),
                has_exit_time: false,
                exit_time: 0.0,
            }
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
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
            }
        ],
        loop_animation:false,
        on_enter: None,
        on_exit: None,
    };

    let slide_state = AnimationState {
        name: "Slide".to_string(),
        first_index: AnimationType::Slide.config_index().0,
        last_index: AnimationType::Slide.config_index().1,
        transitions: vec![
            Transition {
                conditions: vec![],
                target_state: "Idle".to_string(),
                has_exit_time: true,
                exit_time: 1.0,
            }
        ],
        loop_animation:false,
        on_enter: Some(set_sliding),
        on_exit: Some(set_not_sliding),
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
            }
        ],
        loop_animation:false,
        on_enter: Some(set_cant_move),
        on_exit: Some(set_can_move),
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

    animator.set_initial_state(
        "Idle", 
        AnimationType::Idle.config_index().0, 
        AnimationType::Idle.config_index().1,
        10
    );

    animator
}

fn set_can_move(animator: &mut Animator) {
    animator.set_bool("can_move", true);
}

fn set_cant_move(animator: &mut Animator) {
    animator.set_bool("can_move", false);
}

fn set_sliding(animator: &mut Animator) {
    animator.set_bool("can_move", false);
    animator.set_bool("is_sliding", true);
}

fn set_not_sliding(animator: &mut Animator) {
    animator.set_bool("can_move", true);
    animator.set_bool("is_sliding", false);
}

fn check_contact(
    spatial_query: SpatialQuery,
    query: Query<(&Transform, Entity), With<Player>>,
    mut param: Single<&mut Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    for (transform, entity) in &query {
        let collider_y = Collider::capsule_endpoints(5.0, Vec2::Y * 8.0, Vec2::NEG_Y * 12.0);
        let collider_x = Collider::capsule_endpoints(6.0, Vec2::Y * 6.0, Vec2::NEG_Y * 10.0);
        let origin = Vec2::new(transform.translation.x, transform.translation.y);
        let rotation = transform.rotation.z;
        let direction_x = if param.get_bool("is_facing_right") {Dir2::X} else {Dir2::NEG_X};
        let max_distance_y = 0.2;
        let max_distance_x = 0.5;
        let max_hits = 1;

        let config_y = ShapeCastConfig::from_max_distance(max_distance_y);
        let config_x = ShapeCastConfig::from_max_distance(max_distance_x);
        let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

        let hits_ground = spatial_query.shape_hits(
            &collider_y, origin, rotation, Dir2::NEG_Y, max_hits, &config_y, &filter
        );
        let hits_wall = spatial_query.shape_hits(
            &collider_x, origin, rotation, direction_x, max_hits, &config_x, &filter
        );
        let hits_ceiling = spatial_query.shape_hits(
            &collider_y, origin, rotation, Dir2::Y, max_hits, &config_y, &filter
        );
        
        param.set_bool("is_grounded", hits_ground.len() > 0);
        animator.set_bool("is_grounded", hits_ground.len() > 0);

        param.set_bool("is_on_wall", hits_wall.len() > 0);
        param.set_bool("is_on_ceiling", hits_ceiling.len() > 0);
    }
}

#[derive(Bundle)]
struct PlayerInput {
    player: Player,
    input_manager: InputManagerBundle<Action>,
}

impl PlayerInput {
    fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        input_map.insert(Action::Up, KeyCode::KeyW);
        input_map.insert(Action::Down, KeyCode::KeyS);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Jump, KeyCode::Space);
        input_map.insert(Action::Run, KeyCode::ShiftLeft);
        input_map.insert(Action::Crouch, KeyCode::KeyC);
        input_map.insert(Action::Attack, MouseButton::Left);

        input_map
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Run,
    Jump,
    Crouch,
    Attack,
}

impl Action {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        Action::Up,
        Action::Down,
        Action::Left,
        Action::Right,
    ];

    fn direction(self) -> Option<Dir2> {
        match self {
            Action::Up => Some(Dir2::Y),
            Action::Down => Some(Dir2::NEG_Y),
            Action::Left => Some(Dir2::NEG_X),
            Action::Right => Some(Dir2::X),
            _ => None,
        }
    }
}

fn on_move(
    query: Query<&ActionState<Action>, With<Player>>,
    mut velocities: Query<&mut LinearVelocity, With<Player>>,
    mut transform: Single<&mut Transform, With<Player>>,
    mut param: Single<&mut Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    let action_state = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in Action::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += *direction;
            }
        }
    }

    // Then reconvert at the end, normalizing the magnitude
    let is_moving  = direction_vector != Vec2::ZERO;
    if is_moving {
        direction_vector = direction_vector.normalize();
    }

    // Set Face Direction
    if direction_vector.x > 0.0 && !param.get_bool("is_facing_right") {
        param.set_bool("is_facing_right", true);
        transform.scale.x *= -1.0;
    }
    if direction_vector.x < 0.0 && param.get_bool("is_facing_right") {
        param.set_bool("is_facing_right", false);
        transform.scale.x *= -1.0;
    }

    param.set_bool("is_moving", is_moving);
    animator.set_bool("is_moving", is_moving);
    if !param.get_bool("is_on_wall") {
        for mut vel in velocities.iter_mut() {
            vel.x = direction_vector.x * get_speed(&param, &*animator);
            param.set_float("velocity_y", vel.y);
            animator.set_float("velocity_y", vel.y);
        }
    }
}

fn on_crouch(
    query: Query<&ActionState<Action>, With<Player>>,
    mut param: Single<&mut Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    let action_state = query.single();
    if action_state.just_pressed(&Action::Crouch) {
        let crouching = param.get_bool("is_crouching");
        param.set_bool("is_crouching", !crouching);
        animator.set_bool("is_crouching", !crouching);
    }
}

fn on_jump(
    query: Query<&ActionState<Action>, With<Player>>,
    mut velocities: Query<&mut LinearVelocity, With<Player>>,
    param: Single<&Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&Action::Jump) 
        && param.get_bool("is_grounded") && animator.get_bool("can_move") {
        animator.set_trigger("jump");
        for mut vel in velocities.iter_mut() {
            vel.y = JUMP_IMPULSE;
        }
    }
}

fn on_run(
    query: Query<&ActionState<Action>, With<Player>>,
    mut param: Single<&mut Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    let action_state = query.single();
    let is_running = action_state.pressed(&Action::Run);
    param.set_bool("is_running", is_running);
    animator.set_bool("is_running", is_running);
}

fn on_attack(
    query: Query<&ActionState<Action>, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&Action::Attack) {
        animator.set_trigger("attack");
    }
}

fn on_slide(
    time: Res<Time>,
    query: Query<&ActionState<Action>, With<Player>>,
    mut param: Single<&mut Parameters, With<Player>>,
    mut animator: Single<&mut Animator, With<Player>>,
    mut velocities: Query<&mut LinearVelocity, With<Player>>,
    mut layers: Single<&mut CollisionLayers, With<Player>>,
) {
    if animator.get_bool("is_sliding") {
        for mut vel in velocities.iter_mut() {
            let dir = if param.get_bool("is_facing_right") {1.0} else {-1.0}; 
            vel.x = dir * SLIDE_SPEED;
        }
        if layers.filters.has_all(GameLayer::Enemy) {
            layers.filters.remove(GameLayer::Enemy);
        }
    } else {
        if !layers.filters.has_all(GameLayer::Enemy) {
            layers.filters.add(GameLayer::Enemy);
        }
    }

    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.pressed(&Action::Run) {
        let shift_press_time = param.get_float("shift_press_time");
        param.set_float("shift_press_time", shift_press_time + time.delta_secs());
    }
    if action_state.just_released(&Action::Run) {
        let shift_press_time = param.get_float("shift_press_time");
        if shift_press_time <= 0.4 {
            animator.set_trigger("slide");
        }
        param.set_float("shift_press_time", 0.0);
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
        app.add_systems(Startup, (
            setup_player,
        ).chain());
        app.add_systems(FixedUpdate, (
            on_move,
            on_run,
            on_jump,
            on_crouch,
            on_attack,
            on_slide,
            check_contact
        ).chain());
    }
}