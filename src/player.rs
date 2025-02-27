use bevy::{
    prelude::*
};
use avian2d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

const WALK_SPEED: f32 = 80.0;
const RUN_SPEED: f32 = 120.0;
const CROUCH_SPEED: f32 = 50.0;
const JUMP_IMPULSE: f32 = 160.0;

fn get_speed(animation_parameters: &AnimationParameters) -> f32 {
    if animation_parameters.is_crouching && animation_parameters.is_moving {
        return CROUCH_SPEED;
    }
    if animation_parameters.is_running {
        return RUN_SPEED;
    }
    if animation_parameters.is_moving {
        return WALK_SPEED;
    }
    0.0
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Art/Adventurer/adventurer-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 20, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    let animation_state = AnimationStates::new();
    let (first_index, last_index) = animation_state.config_index();
    let animation_config = AnimationConfig::new(first_index, last_index, 10);
    let animation_parameters = AnimationParameters::new();

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        PlayerInput {
            player: Player,
            input_manager: InputManagerBundle::with_map(PlayerInput::default_input_map()),
        },
        animation_config,
        animation_parameters,
        animation_state,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        SweptCcd::default(),
        Mass(1.0),
        LinearVelocity::default(),
        GravityScale(30.0),
        Collider::capsule_endpoints(6.0, Vec2::Y * 7.0, Vec2::NEG_Y * 11.0),
    ));
}

fn check_contact(
    spatial_query: SpatialQuery,
    query: Query<(&Transform, Entity), With<Player>>,
    mut animation_parameters: Single<&mut AnimationParameters, With<Player>>
) {
    for (transform, entity) in &query {
        let collider_y = Collider::capsule_endpoints(5.0, Vec2::Y * 8.0, Vec2::NEG_Y * 12.0);
        let collider_x = Collider::capsule_endpoints(6.0, Vec2::Y * 6.0, Vec2::NEG_Y * 10.0);
        let origin = Vec2::new(transform.translation.x, transform.translation.y);
        let rotation = transform.rotation.z;
        let direction_x = if animation_parameters.is_facing_right {Dir2::X} else {Dir2::NEG_X};
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
        
        animation_parameters.set_grounded(hits_ground.len() > 0);
        animation_parameters.set_on_wall(hits_wall.len() > 0);
        animation_parameters.set_on_ceiling(hits_ceiling.len() > 0)
    }
}

#[derive(Component)]
pub struct PlayerCollider;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerInput {
    player: Player,
    input_manager: InputManagerBundle<Action>,
}

impl PlayerInput {
    fn default_input_map() -> InputMap<Action> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        let mut input_map = InputMap::default();

        input_map.insert(Action::Up, KeyCode::KeyW);
        input_map.insert(Action::Down, KeyCode::KeyS);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Jump, KeyCode::Space);
        input_map.insert(Action::Run, KeyCode::ShiftLeft);
        input_map.insert(Action::Crouch, KeyCode::KeyC);

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

fn input_crouch(
    query: Query<&ActionState<Action>, With<Player>>,
    mut animation_parameters: Single<&mut AnimationParameters, With<Player>>
) {
    let action_state = query.single();
    if action_state.just_pressed(&Action::Crouch) {
        if animation_parameters.is_crouching {
            animation_parameters.set_crouching(false);
        } else {
            animation_parameters.set_crouching(true);
        }
    }
}

fn input_jump(
    query: Query<&ActionState<Action>, With<Player>>,
    mut velocities: Query<&mut LinearVelocity, With<Player>>,
    mut animation_parameters: Single<&mut AnimationParameters, With<Player>>
) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&Action::Jump) && animation_parameters.is_grounded {
        animation_parameters.set_jumping(true);
        for mut vel in velocities.iter_mut() {
            vel.y = JUMP_IMPULSE;
        }
    }
}

fn input_run(
    query: Query<&ActionState<Action>, With<Player>>,
    mut animation_parameters: Single<&mut AnimationParameters, With<Player>>
) {
    let action_state = query.single();

    animation_parameters.set_running(action_state.pressed(&Action::Run));
}

fn input_move(
    query: Query<&ActionState<Action>, With<Player>>,
    mut velocities: Query<&mut LinearVelocity, With<Player>>,
    mut transform: Single<&mut Transform, With<Player>>,
    mut animation_parameters: Single<&mut AnimationParameters, With<Player>>
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
    if direction_vector.x > 0.0 && !animation_parameters.is_facing_right {
        animation_parameters.set_facing_right(true);
        transform.scale.x *= -1.0;
    }
    if direction_vector.x < 0.0 && animation_parameters.is_facing_right {
        animation_parameters.set_facing_right(false);
        transform.scale.x *= -1.0;
    }
    animation_parameters.set_moving(is_moving);
    if !animation_parameters.is_on_wall {
        for mut vel in velocities.iter_mut() {
            vel.x = direction_vector.x * get_speed(&animation_parameters);
            animation_parameters.set_velocity_y(vel.y);
        }
    }
}


#[derive(Component)]
struct AnimationParameters {
    pub is_moving: bool,
    pub is_running: bool,
    pub is_crouching: bool,
    pub is_facing_right: bool,
    pub is_grounded: bool,
    pub velocity_y: f32,
    pub is_jumping: bool,
    pub is_on_wall: bool,
    pub is_on_ceiling: bool,
}

impl AnimationParameters {
    fn new() -> Self {
        Self {
            is_moving: false,
            is_running: false,
            is_facing_right: true,
            is_crouching: false,
            is_grounded: true,
            velocity_y: 0.0,
            is_jumping: false,
            is_on_wall: false,
            is_on_ceiling: false,
        }
    }

    fn set_moving(&mut self, moving: bool) {
        self.is_moving = moving;
    }
    
    fn set_running(&mut self, running: bool) {
        self.is_running = running;
    }

    fn set_crouching(&mut self, crouching: bool) {
        self.is_crouching = crouching;
    }

    fn set_facing_right(&mut self, facing_right: bool) {
        self.is_facing_right = facing_right;
    }

    fn set_grounded(&mut self, grounded: bool) {
        self.is_grounded = grounded;
    }

    fn set_velocity_y(&mut self, velocity_y: f32) {
        self.velocity_y = velocity_y;
    }

    fn set_jumping(&mut self, jumping: bool) {
        self.is_jumping = jumping;
    }

    fn set_on_wall(&mut self, on_wall: bool) {
        self.is_on_wall = on_wall;
    }

    fn set_on_ceiling(&mut self, on_ceiling: bool) {
        self.is_on_ceiling = on_ceiling;
    }
}

#[derive(Component)]
enum AnimationStates {
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

impl AnimationStates {
    fn new() -> Self {
        Self::Idle
    }
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

    fn is_in_states(&self, state: AnimationStates) -> bool {
        self.config_index() == state.config_index()
    }

    fn check_states(&self, animation_parameters: &AnimationParameters) -> bool{
        match self {
            Self::AirAttack3End => false,
            Self::AirAttack1 => false,
            Self::AirAttack2 => false,
            Self::AirAttack3Loop => false,
            Self::AirAttack3Rdy => false,
            Self::Attack1 => false,
            Self::Attack2 => false,
            Self::Attack3 => false,
            Self::Bow => false,
            Self::BowJump => false,
            Self::Cast => false,
            Self::CastLoop => false,
            Self::CrnrClimb => false,
            Self::CrnrGrb => false,
            Self::CrnrJump => false,
            Self::Crouch => animation_parameters.is_grounded
                            && animation_parameters.is_crouching 
                            && !animation_parameters.is_moving
                            && !animation_parameters.is_jumping,
            Self::CrouchWalk => animation_parameters.is_crouching 
                            && animation_parameters.is_moving
                            && !animation_parameters.is_jumping,
            Self::Die => false,
            Self::DropKick => false,
            Self::Fall => !animation_parameters.is_grounded 
                            && animation_parameters.velocity_y <= 0.0
                            && !animation_parameters.is_jumping,
            Self::GetUp => false,
            Self::Hurt => false,
            Self::Idle => !animation_parameters.is_crouching 
                            && !animation_parameters.is_moving
                            && animation_parameters.is_grounded
                            && !animation_parameters.is_jumping,
            Self::IdleWithSword => false,
            Self::Items => false,
            Self::Jump => animation_parameters.is_jumping,
            Self::Rise => !animation_parameters.is_grounded 
                            && animation_parameters.velocity_y > 0.0
                            && !animation_parameters.is_jumping,
            Self::Kick => false,
            Self::KnockDown => false,
            Self::LaderClimb => false,
            Self::Punch => false,
            Self::Run => !animation_parameters.is_crouching 
                            && animation_parameters.is_moving 
                            && animation_parameters.is_running
                            && animation_parameters.is_grounded
                            && !animation_parameters.is_jumping,
            Self::RunPunch => false,
            Self::FastRun => false,
            Self::Slide => false,
            Self::SmrSlt => false,
            Self::Stand => false,
            Self::SwordDraw => false,
            Self::SwordShte => false,
            Self::Walk => !animation_parameters.is_crouching 
                            && animation_parameters.is_moving 
                            && !animation_parameters.is_running
                            && animation_parameters.is_grounded
                            && !animation_parameters.is_jumping,
            Self::WallRun => false,
            Self::WallSlide => false,
        }
    }
}

fn animation_states_machine(
    animation_parameters: Single<&AnimationParameters, With<Player>>,
    mut query: Query<(&mut AnimationConfig, &mut AnimationStates, &mut Sprite)>
) {
    for (mut config, mut state, mut sprite) in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if AnimationStates::Idle.check_states(&animation_parameters)
            && !state.is_in_states(AnimationStates::Idle) {
                *state = AnimationStates::Idle;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::Walk.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::Walk) {
                *state = AnimationStates::Walk;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::Run.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::Run) {
                *state = AnimationStates::Run;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::Crouch.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::Crouch) {
                *state = AnimationStates::Crouch;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::CrouchWalk.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::CrouchWalk) {
                *state = AnimationStates::CrouchWalk;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::Rise.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::Rise) {
                *state = AnimationStates::Rise;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::Fall.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::Fall) {
                *state = AnimationStates::Fall;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
            if  AnimationStates::Jump.check_states(&animation_parameters)
            &&  !state.is_in_states(AnimationStates::Jump) {
                *state = AnimationStates::Jump;
                let (first_index, last_index) = state.config_index();
                config.first_sprite_index = first_index;
                config.last_sprite_index = last_index;
                atlas.index = first_index;
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
        app.add_systems(Startup, (
            setup_player,
            trigger_animation::<Player>,
        ).chain());
        app.add_systems(FixedUpdate, (
            input_move,
            input_run,
            input_jump,
            input_crouch,
            check_contact
        ).chain());
        app.add_systems(Update, (
            animation_states_machine,
            execute_animations
        ).chain());
    }
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut AnimationParameters, &mut Sprite)>) {
    for (mut config, mut parameter, mut sprite) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                    
                    if parameter.is_jumping {
                        parameter.set_jumping(false);
                    } else {
                        atlas.index = config.first_sprite_index;
                    }
                    
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    // we create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}