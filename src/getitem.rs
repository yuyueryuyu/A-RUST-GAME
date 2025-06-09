use bevy::prelude::*;
use moonshine_save::load;

use crate::animator::Animator;
use crate::damagable::Damagable;
use crate::items::{ItemList, NearingItem, NotpickedItems};
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
    player: Single<&NearingItem, With<Player>>,
    items: Query<&NotpickedItems>,
    item_list: Res<ItemList>,
    asset_server: Res<AssetServer>,
) {
    let nearing_items = player.into_inner();
    let item = (**nearing_items).get(0).unwrap();
    let item_info = items.get(*item).unwrap();
    let id = &item_info.id;
    let info = item_list.infos.get(id).unwrap();
    
    let ui_container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let title_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(50.),
        top: Val::Percent(15.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let title = ImageNode::new(info.icon.clone());
    let color = Color::srgb(0., 0., 0.).with_alpha(0.7);
    let name_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(20.),
        top: Val::Percent(63.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    
    let desc_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(20.),
        top: Val::Percent(73.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    
    let start_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        top: Val::Percent(83.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    
    let start_text = Text::new("[ Continue ]");

    let name_text = Text::new(info.name.clone());
    let desc_text = Text::new(info.description.clone());
   
    let font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 80.0,
        ..default()
    };

    let sfont = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 60.0,
        ..default()
    };
    
    let ui_entity = commands.spawn((ui_container, BackgroundColor(color), UI)).id();
    
    let title_entity = commands.spawn((
        title_node,
    )).with_children(|parent| {
        parent.spawn(
            title
        );
    }).id();

    let name_node_entity = commands.spawn((
        name_node,
    )).with_children(|parent| {
        parent.spawn((
            name_text, sfont.clone(), Label
        ));
    }).id();
    
    let desc_node_entity = commands.spawn((
        desc_node,
    )).with_children(|parent| {
        parent.spawn((
            desc_text, sfont.clone(), Label
        ));
    }).id();

    let start_node_entity = commands.spawn((
        start_node,
    )).with_children(|parent| {
        parent.spawn((
            start_text, font.clone(), Label, MenuItem { id: 0, is_selected : true }
        ));
    }).id();


    commands
        .entity(ui_entity)
        .add_children(&[title_entity, name_node_entity, desc_node_entity, start_node_entity]);

    commands.entity(*item).despawn();
}

fn handle_enter(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    items: Query<&MenuItem>,
    mut commands: Commands,
    ui: Query<Entity, With<UI>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        for item in &items {
            if item.is_selected {
                if item.id == 0 {
                    if let Ok(entity) = ui.single() {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(PausedState::Running);
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

pub struct GetItemPlugin;

impl Plugin for GetItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PausedState::GetItem), (
            spawn_box.run_if(in_state(PausedState::GetItem)),
        ));
        app.add_systems(Update, (
            handle_enter.run_if(in_state(PausedState::GetItem)),
        ));
    }
}
