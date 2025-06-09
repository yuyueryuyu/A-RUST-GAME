use bevy::prelude::*;

use crate::damagable::Damagable;
use crate::items::{ActiveItems, ItemBag, ItemList};
use crate::player::Player;

const MAX_HEALTH_LEN: f32 = 300.;
const MAX_HEALTH_WID: f32 = 30.;

#[derive(Component)]
pub struct HealthBarBg {
    pub length: f32,
}

#[derive(Component)]
pub struct HealthBar {
    pub length: f32,
}

#[derive(Component)]
pub struct ItemImg {
    pub index: Option<usize>,   
}

#[derive(Component)]
pub struct ItemNum {
    pub nums: u32,
}


fn update_item(
    item_list: Single<(&ActiveItems, &ItemBag), With<Player>>,
    item_img: Single<(&mut ImageNode, &mut ItemImg)>,
    item_num: Single<(&mut Text, &mut ItemNum)>,
    items: Res<ItemList>,
    asset_server: Res<AssetServer>,
) {
    let (mut item, mut item_img) = item_img.into_inner();
    let (mut text, mut item_num) = item_num.into_inner();
    let (active, bag) = item_list.into_inner();
    if active.items.is_empty() { 
        item.image = asset_server.load("Art/empty.png");
        text.0 = "".to_string();
        return;
    }
    let item_now = active.get_current_item();
    let item_stack = bag.slots.get(&item_now).unwrap();
    if item_img.index == None {
        item_img.index = Some(active.current);
        item.image = items.infos.get(&item_now).unwrap().icon.clone();
        item_num.nums = 0;
    }
    if let Some(_) = item_img.index {
        item_img.index = Some(active.current);
        item.image = items.infos.get(&item_now).unwrap().icon.clone()
    }
    if item_num.nums != *item_stack {
        item_num.nums = *item_stack;
        text.0 = (*item_stack).to_string();
    }
}

#[derive(Component)]
pub struct Hint;

fn spawn_box(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let ui_container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Start,
        ..default()
    };

    let hint_container = Node {
        width: Val::Percent(40.),
        height: Val::Px(100.),
        justify_content: JustifyContent::Center,
        left: Val::Percent(30.),
        top: Val::Percent(80.),
        position_type: PositionType::Absolute,
        ..Default::default()
    };

    let hint = Text::new("");

    let health_bar_container = Node {
        width: Val::Percent(80.),
        height: Val::Px(80.),
        justify_content: JustifyContent::Start,
        left: Val::Percent(5.),
        top: Val::Percent(7.),
        position_type: PositionType::Relative,
        ..Default::default()
    };

    let left_box = Node {
        width: Val::Px(80.),
        height: Val::Px(80.),
        position_type: PositionType::Relative,
        justify_content: JustifyContent::Center,
        padding: UiRect::all(Val::Px(10.)),
        ..default()
    };

    let red_bar = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        border: UiRect::all(Val::Px(2.)),
        position_type: PositionType::Relative,
        ..default()
    };

    let bar_bg = Node {
        width: Val::Px(MAX_HEALTH_LEN),
        height: Val::Px(MAX_HEALTH_WID),
        position_type: PositionType::Relative,
        justify_content: JustifyContent::Start,
        ..default()
    };
    let left = ImageNode::new(asset_server.load("Art/empty.png"));
    let text_node = Node {
        position_type: PositionType::Absolute,
        width: Val::Px(30.),
        height: Val::Px(30.),
        right: Val::Px(0.),
        top: Val::Percent(0.),
        justify_content: JustifyContent::Center,
        //padding: UiRect::left(Val::Px(5.)).with_bottom(Val::Px(5.)),
        ..default()
    };
    let text = Text::new("0");
    let font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 30.0,
        ..default()
    };

    let big_font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 40.0,
        ..default()
    };

    let bar_color = Color::srgb(0.67, 0., 0.);
    let barbg_color = Color::srgb(0.3, 0.3, 0.3);
    let left_color = Color::srgb(0.7, 0.6, 0.3);
    
    let ui_entity = commands.spawn(ui_container).id();
    let healthbar_entity = commands.spawn(health_bar_container).id();
    let left_entity = commands.spawn((
        left_box, BackgroundColor(left_color)
    )).id();
    let bar_entity = commands
        .spawn((red_bar, BackgroundColor(bar_color), HealthBar { length: 100. }))
        .id();
    let barbg_entity = commands
        .spawn((
            bar_bg,
            BackgroundColor(barbg_color),
            HealthBarBg {
                length: MAX_HEALTH_LEN,
            },
        ))
        .id();
    let item_entity = commands.spawn((
        left, ItemImg { index: None }
    )).id();
    let text_node_entity = commands.spawn((
        text_node,
    )).with_children(|parent| {
        parent.spawn((
            text, font.clone(), Label, ItemNum { nums : 0 }
        ));
    }).id();

    let hint_node_entity = commands.spawn((
        hint_container,
    )).with_children(|parent| {
        parent.spawn((
            hint, big_font.clone(), Label, Hint
        ));
    }).id();

    commands
        .entity(ui_entity)
        .add_children(&[healthbar_entity, hint_node_entity]);
    commands.entity(healthbar_entity).add_children(&[left_entity, barbg_entity]);
    commands.entity(barbg_entity).add_children(&[bar_entity]);
    commands.entity(left_entity).add_children(&[item_entity, text_node_entity]);
    //commands.entity(text_node_entity).add_children(&[text_entity]);
}

fn update_health(
    time: Res<Time>,
    damagable: Single<&Damagable, With<Player>>,
    health_bar_bg: Single<(&mut Node, &mut HealthBarBg)>,
    health_bar: Single<(&mut Node, &mut HealthBar), Without<HealthBarBg>>,
) {
    let (mut bar_bg_node, mut bar_bg) = health_bar_bg.into_inner();
    let (mut bar_node, mut bar) = health_bar.into_inner();
    let target_max = damagable.max_health / 100. * MAX_HEALTH_LEN;
    if bar_bg.length > target_max {
        bar_bg.length -= time.delta_secs() * MAX_HEALTH_LEN;
        if bar_bg.length < target_max {
            bar_bg.length = target_max;
        }
        bar_bg_node.width = Val::Px(bar_bg.length);
    } else if bar_bg.length < target_max {
        bar_bg.length += time.delta_secs() * MAX_HEALTH_LEN;
        if bar_bg.length > target_max {
            bar_bg.length = target_max;
        }
        bar_bg_node.width = Val::Px(bar_bg.length);
    }

    let target_len = damagable.health / damagable.max_health * 100.;
    if bar.length > target_len {
        bar.length -= time.delta_secs() * 100.;
        if bar.length < target_len {
            bar.length = target_len;
        }
        bar_node.width = Val::Percent(bar.length);
    } else if bar.length < target_len {
        bar.length += time.delta_secs() * 100.;
        if bar.length > target_len {
            bar.length = target_len;
        }
        bar_node.width = Val::Percent(bar.length);
    }
}

pub struct HealthBarPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for HealthBarPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), spawn_box.run_if(in_state(self.state.clone())));
        app.add_systems(Update, update_health.run_if(in_state(self.state.clone())));
        app.add_systems(PostUpdate, update_item.run_if(in_state(self.state.clone())));
    }
}
