use bevy::prelude::*;
use moonshine_save::load;

use crate::animator::Animator;
use crate::damagable::Damagable;
use crate::player::Player;
use crate::{AppState, PausedState};
use crate::save::load;
use crate::save::{trigger_save, LoadRequest, SaveRequest};
#[derive(Component)]
pub struct MenuItem {
    pub id: i32,
    pub is_selected: bool,
}

#[derive(Component)]
pub struct UI;

fn spawn_box(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let ui_container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let title_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(40.),
        height: Val::Percent(100.),
        left: Val::Percent(0.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let title_container_node = Node {
        position_type: PositionType::Relative,
        width: Val::Percent(100.),
        height: Val::Percent(30.),
        top: Val::Percent(35.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let choice_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(60.),
        height: Val::Percent(100.),
        left: Val::Percent(40.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let title = ImageNode::new(asset_server.load("UI/title.png"));
    let color = Color::srgb(0., 0., 0.);
    let color_alp = Color::srgb(0., 0., 0.).with_alpha(0.5);

    let start_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(35.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };

    let bag_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(45.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };

    let save_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(55.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };

    let load_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(65.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };

    let exit_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(75.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let start_text = Text::new("[ Continue ]");
    let bag_text = Text::new("Open Bag");
    let save_text = Text::new("Save Game");
    let load_text = Text::new("Load Game");
    let exit_text = Text::new("Game Exit");
    let font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 80.0,
        ..default()
    };
    
    let ui_entity = commands.spawn((ui_container, UI)).id();
    
    let title_entity = commands.spawn((
        title_node, BackgroundColor(color),
    )).with_children(|parent| {
        parent.spawn(
            title_container_node
        ).with_children(|parent2| {
            parent2.spawn(
                title
            );
        });
    }).id();

    let choice_entity = commands.spawn((choice_node, BackgroundColor(color_alp))).id();

    let start_node_entity = commands.spawn((
        start_node,
    )).with_children(|parent| {
        parent.spawn((
            start_text, font.clone(), Label, MenuItem { id: 0, is_selected : true }
        ));
    }).id();

    let bag_node_entity = commands.spawn((
        bag_node,
    )).with_children(|parent| {
        parent.spawn((
            bag_text, font.clone(), Label, MenuItem { id: 1, is_selected : true }
        ));
    }).id();

    let save_node_entity = commands.spawn((
        save_node,
    )).with_children(|parent| {
        parent.spawn((
            save_text, font.clone(), Label, MenuItem { id: 2, is_selected : false }
        ));
    }).id();

    let load_node_entity = commands.spawn((
        load_node,
    )).with_children(|parent| {
        parent.spawn((
            load_text, font.clone(), Label, MenuItem { id: 3, is_selected : false }
        ));
    }).id();

    let exit_node_entity = commands.spawn((
        exit_node,
    )).with_children(|parent| {
        parent.spawn((
            exit_text, font.clone(), Label, MenuItem { id: 4, is_selected : false }
        ));
    }).id();

    commands
        .entity(ui_entity)
        .add_children(&[title_entity, choice_entity]);
    commands
        .entity(choice_entity)
        .add_children(&[start_node_entity, bag_node_entity, save_node_entity, load_node_entity, exit_node_entity]);
    //commands.entity(text_node_entity).add_children(&[text_entity]);
}

fn handle_choice(
    mut items: Query<(&mut Text, &mut MenuItem)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        let mut selected  = -1;
        let mut switched = false;
        for (_, item) in &mut items {
            if item.is_selected {
                selected = item.id;
                break;
            }
        } 
        for (mut text, mut item) in &mut items {
            if item.id == selected + 1 {
                text.0 = "[ ".to_string() + &text.0 + &" ]".to_string();
                item.is_selected = true;
                switched = true;
                break;
            }
        }
        if switched {
            for (mut text, mut item) in &mut items {
                if item.id == selected {
                    item.is_selected = false;
                    text.0 = text.0[2..text.0.len()-2].to_string();
                }
            }
        }
    }        
    else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let mut selected  = -1;
        let mut switched = false;
        for (_, item) in &mut items {
            if item.is_selected {
                selected = item.id;
                break;
            }
        } 
        for (mut text, mut item) in &mut items {
            if item.id == selected - 1 {
                text.0 = "[ ".to_string() + &text.0 + &" ]".to_string();
                item.is_selected = true;
                switched = true;
                break;
            }
        }
        if switched {
            for (mut text, mut item) in &mut items {
                if item.id == selected {
                    item.is_selected = false;
                    text.0 = text.0[2..text.0.len()-2].to_string();
                }
            }
        }
    }
}

fn handle_enter(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    items: Query<&MenuItem>,
    mut commands: Commands,
    ui: Query<Entity, With<UI>>,
    mut next_state: ResMut<NextState<PausedState>>,
    mut exit_events: EventWriter<AppExit>,
    save_events: EventWriter<SaveRequest>,
    mut player: Single<(&mut Transform, &mut Animator, &mut Damagable), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for item in &items {
            if item.is_selected {
                if item.id == 0 {
                    if let Ok(entity) = ui.single() {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(PausedState::Running);
                } else if item.id == 1 {
                    if let Ok(entity) = ui.single() {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(PausedState::BagUI);
                }  if item.id == 2 {
                    trigger_save(save_events);
                    //trigger_load(load_events)
                    if let Ok(entity) = ui.single() {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(PausedState::Running);
                }  else if item.id == 3 {
                    let transform = load().unwrap();
                    let (
                        mut trans, 
                        mut animator,
                        mut dam,
                    ) = player.into_inner();
                    trans.translation.x = transform.translation[0];
                    trans.translation.y = transform.translation[1];
                    trans.translation.z = transform.translation[2];
                    trans.scale.x = transform.scale[0];
                    trans.scale.y = transform.scale[1];
                    trans.scale.z = transform.scale[2];
                    animator.parameters = transform.params;
                    dam.copy(transform.damagable);
                    if let Ok(entity) = ui.single() {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(PausedState::Running);
                } else if item.id == 4 {
                    exit_events.write(AppExit::Success);
                }
                break;
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(entity) = ui.single() {
            commands.entity(entity).despawn();
        }
        next_state.set(PausedState::Running);
    }
}

fn handle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(PausedState::Paused);
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_pause.run_if(in_state(PausedState::Running)));
        app.add_systems(OnEnter(PausedState::Paused), 
        spawn_box.run_if(in_state(PausedState::Paused)));
        app.add_systems(Update, (
            handle_choice.run_if(in_state(PausedState::Paused)),
            handle_enter.run_if(in_state(PausedState::Paused)),
        ));
    }
}
