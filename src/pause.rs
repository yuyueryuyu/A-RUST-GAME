use bevy::prelude::*;

use crate::{AppState, PausedState};
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
    let exit_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(65.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let start_text = Text::new("[ Game Start ]");
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

    let exit_node_entity = commands.spawn((
        exit_node,
    )).with_children(|parent| {
        parent.spawn((
            exit_text, font.clone(), Label, MenuItem { id: 1, is_selected : false }
        ));
    }).id();

    commands
        .entity(ui_entity)
        .add_children(&[title_entity, choice_entity]);
    commands
        .entity(choice_entity)
        .add_children(&[start_node_entity, exit_node_entity]);
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
    mut exit_events: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for item in &items {
            if item.is_selected {
                if item.id == 0 {
                    if let Ok(entity) = ui.get_single() {
                        commands.entity(entity).despawn_recursive();
                    }
                    next_state.set(PausedState::Running);
                } else if item.id == 1 {
                    exit_events.send(AppExit::Success);
                }
                break;
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(entity) = ui.get_single() {
            commands.entity(entity).despawn_recursive();
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
