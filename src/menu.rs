use bevy::prelude::*;

use crate::{save::{load, trigger_load, LoadRequest, TransformData}, AppState};
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
    let color = Color::srgb(0., 0., 0.);

    let start_text = Text::new("[ Game Start ]");
    let load_text = Text::new("Game Load");
    let exit_text = Text::new("Game Exit");
    let font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 80.0,
        ..default()
    };
    
    let ui_entity = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        }, BackgroundColor(color), UI
    )).id();
    
    let title_entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(50.),
            top: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
            ..default()
        },
        children![ImageNode::new(asset_server.load("UI/title.png"))]
    )).id();

    let start_node_entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            top: Val::Percent(63.),
            justify_content: JustifyContent::Center,
            //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
            ..default()
        },
        children![(
            start_text, 
            font.clone(), 
            Label, 
            MenuItem { id: 0, is_selected : true }
        )]
    )).id();

    let load_node_entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            top: Val::Percent(73.),
            justify_content: JustifyContent::Center,
            //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
            ..default()
        },
        children![(
            load_text, font.clone(), Label, MenuItem { id: 1, is_selected : false }
        )]
    )).id();

    let exit_node_entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            top: Val::Percent(83.),
            justify_content: JustifyContent::Center,
            //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
            ..default()
        },
        children![
            (exit_text, font.clone(), Label, MenuItem { id: 2, is_selected : false })
        ]
    )).id();

    commands
        .entity(ui_entity)
        .add_children(&[title_entity, start_node_entity, load_node_entity, exit_node_entity]);
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
    ui: Single<Entity, With<UI>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: EventWriter<AppExit>,
    mut transform: ResMut<TransformData>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for item in &items {
            if item.is_selected {
                if item.id == 0 {
                    let entity = ui.into_inner();
                    commands.entity(entity).despawn_recursive();
                    next_state.set(AppState::InGame);
                } else if item.id == 1 {
                    let entity = ui.into_inner();
                    let trans = load().unwrap();
                    transform.translation = trans.translation;
                    transform.rotation = trans.rotation;
                    transform.scale = trans.scale;
                    transform.damagable = trans.damagable;
                    transform.params = trans.params;
                    commands.entity(entity).despawn_recursive();
                    next_state.set(AppState::InGame)
                } else if item.id == 2 {
                    exit_events.send(AppExit::Success);
                }
                break;
            }
        }
    }
}

pub struct MenuPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MenuPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_box.run_if(in_state(self.state.clone())));
        app.add_systems(Update, (
            handle_choice.run_if(in_state(self.state.clone())),
            handle_enter.run_if(in_state(self.state.clone())),
        ));
    }
}
