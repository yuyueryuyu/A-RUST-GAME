use bevy::prelude::*;
use avian2d::prelude::*;

use crate::animator::*;
use crate::animator::Condition;
use crate::game_layer::GameLayer;

mod parameters;
use parameters::Parameters;

const WALK_SPEED: f32 = 30.0;

fn get_speed(param: &Parameters) -> f32 {
    if param.get_bool("is_moving") {
        return WALK_SPEED;
    }

    0.0
}

#[derive(Component)]
struct Skeleton;

fn setup_enemy(mut commands: Commands, asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Art/Monster_Creatures_Fantasy(Version 1.3)/Skeleton_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(150, 150), 7, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    spawn_enemy(&mut commands, Vec2::new(50.0, 0.0), texture.clone(), texture_atlas_layout.clone());
    spawn_enemy(&mut commands, Vec2::new(30.0, -80.0), texture.clone(), texture_atlas_layout.clone());
}

fn spawn_enemy(
    commands: &mut Commands,
    position: Vec2, 
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>
) {
    let animator = setup_animator();
    let param = Parameters::new();
    let collider_layer = CollisionLayers::new(
        GameLayer::Enemy,
        [GameLayer::Default, GameLayer::Player, GameLayer::Ground],
    );
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animator.first_index,
            }),
            ..default()
        },
        Skeleton,
        Transform::from_xyz(position.x, position.y, 0.0)
            .with_scale(Vec3::new(0.7, 0.7, 0.7)),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        SweptCcd::default(),
        Mass(1.0),
        LinearVelocity::default(),
        GravityScale(30.0),
        Collider::capsule_endpoints(12.0, Vec2::Y * 14.0, Vec2::NEG_Y * 14.0),
        collider_layer,
        animator,
        param,
    ));
}

#[derive(Component)]
enum AnimationType {
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
            Self::Attack => (0,7),
            Self::Attack2 => (8,15),
            Self::Attack3 => (16,21),
            Self::Death => (22,25),
            Self::Idle => (26,29),
            Self::Hurt => (30,33),
            Self::Sheild => (34,37),
            Self::Walk => (38,41),
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
        ],
        loop_animation:true,
        on_enter: None,
        on_exit: None,
    };

    let mut animator = Animator::new();
    animator.add_parameter("is_moving", AnimatorParam::Bool(false));

    animator.add_state(idle_state);
    animator.add_state(walk_state);

    animator.set_initial_state(
        "Idle", 
        AnimationType::Idle.config_index().0, 
        AnimationType::Idle.config_index().1,
        8
    );

    animator
}

fn check_contact(
    spatial_query: SpatialQuery,
    mut query: Query<(&Transform, &mut Parameters), With<Skeleton>>,
    entity_query: Query<Entity, With<Sprite>>,
) {
    let collider_y = Collider::capsule_endpoints(7.0, Vec2::Y * 11.0, Vec2::NEG_Y * 11.0);
    let collider_x = Collider::capsule_endpoints(8.0, Vec2::Y * 10.0, Vec2::NEG_Y * 10.0);
    for (transform, mut param) in &mut query {
        let origin = Vec2::new(transform.translation.x, transform.translation.y);
        let rotation = transform.rotation.z;
        let direction_x = if param.get_float("facing_direction") > 0.0 {Dir2::X} else {Dir2::NEG_X};
        let max_distance_y = 0.2;
        let max_distance_x = 0.5;
        let max_hits = 1;

        let config_y = ShapeCastConfig::from_max_distance(max_distance_y);
        let config_x = ShapeCastConfig::from_max_distance(max_distance_x);
        let filter = SpatialQueryFilter::default().with_excluded_entities(entity_query.iter());

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
        param.set_bool("is_on_wall", hits_wall.len() > 0);
        param.set_bool("is_on_ceiling", hits_ceiling.len() > 0);
    }
}

fn flip_direction_on_wall(mut query: Query<(&mut Transform, &mut Parameters), With<Skeleton>>,) {
    for (mut transform, mut param) in &mut query {
        if param.get_bool("is_grounded") && param.get_bool("is_on_wall") {
            transform.scale.x *= -1.0;
            let facing_direction = param.get_float("facing_direction");
            param.set_float("facing_direction", -facing_direction);
        }
    }
}

fn on_move(
    mut query: Query<(&mut LinearVelocity, &mut Parameters, &mut Animator), With<Skeleton>>,
) {

    // Then reconvert at the end, normalizing the magnitude
    for (mut vel, mut param, mut animator) in query.iter_mut() {
        let is_moving  = true;

        param.set_bool("is_moving", is_moving);
        animator.set_bool("is_moving", is_moving);
        if !param.get_bool("is_on_wall") {
            vel.x = param.get_float("facing_direction") * get_speed(&param);
        }
    }
}

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy);
        app.add_systems(FixedUpdate, (
            check_contact,
            on_move,
            flip_direction_on_wall
        ).chain());
    }
}