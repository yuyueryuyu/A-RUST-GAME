use bevy::{ecs::system::command, prelude::*, render::view::visibility};
use std::{collections::HashMap, process::Child, thread::current};

use crate::{items::{ActiveItems, ItemBag, ItemList, ItemType}, player::Player, PausedState};

#[derive(Component)]
#[relationship(relationship_target = HasItemSlot)]
pub struct ItemSlotOf(pub Entity);

/// 使用道具时生成的道具子实体与角色的关系
#[derive(Component, Deref)]
#[relationship_target(relationship = ItemSlotOf)]
pub struct HasItemSlot(Vec<Entity>);

/// UI组件标记
#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct ItemInfoPanel;

#[derive(Component)]
pub struct ItemInfoName;

#[derive(Component)]
pub struct ItemInfoType;

#[derive(Component)]
pub struct ItemInfoIcon;

#[derive(Component)]
pub struct ItemInfoDesc;

#[derive(Component)]
pub struct ItemInfoNum;

#[derive(Component)]
pub struct ItemGridPanel{
    pub current: usize, 
    pub page: usize,
    pub max: Option<usize>,
}

#[derive(Component)]
pub struct ItemPage(pub usize);

#[derive(Component)]
pub struct ItemSlot;

// 常量定义
const GRID_WIDTH: usize = 6;
const GRID_HEIGHT: usize = 6;

fn up(current: usize) -> usize {
    let row = current / GRID_WIDTH;
    let col = current % GRID_WIDTH;
    if row == 0 { return row; }
    (row-1) * GRID_WIDTH + col
}

fn down(current: usize, max: usize) -> usize {
    let row = current / GRID_WIDTH;
    let col = current % GRID_WIDTH;
    let max_row = max / GRID_WIDTH;
    if row == max_row { return row; }
    (row+1) * GRID_WIDTH + col
}

fn left(current: usize) -> usize {
    if current > 0 { current-1 } else { current } 
}

fn right(current: usize, max: usize) -> usize {
    if current < max { current+1 } else { current } 
}

fn get_page(current: usize) -> usize {
    let row = current / GRID_WIDTH;
    row / GRID_HEIGHT
}

fn spawn_inventory_ui(
    mut commands: Commands,
    item_bag: Single<&ItemBag, With<Player>>,
    asset_server: Res<AssetServer>,
    items: Res<ItemList>,
) {
    let bag = item_bag.into_inner();
    // 主容器
    let main_container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    // 左侧信息面板 (1/3)
    let info_panel_node = Node {
        width: Val::Percent(33.33),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    // 右侧物品栏面板 (2/3)
    let grid_panel_node = Node {
        width: Val::Percent(66.67),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    let bg_color = Color::srgb(0.1, 0.1, 0.1).with_alpha(0.9);
    let panel_color = Color::srgb(0.2, 0.2, 0.2).with_alpha(0.8);

    // 字体样式
    let font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 30.0,
        ..default()
    };

    let title_font = TextFont {
        font: asset_server.load("UI/Fonts/m5x7.ttf"),
        font_size: 45.0,
        ..default()
    };

    // 创建主容器
    let main_entity = commands.spawn((
        main_container,
        BackgroundColor(bg_color),
        InventoryUI,
    )).id();

    // 创建左侧信息面板
    let info_panel_entity = commands.spawn((
        info_panel_node,
        BackgroundColor(panel_color),
        ItemInfoPanel,
    )).with_children(|parent| {
        // 物品名称
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        )).with_children(|parent| {
            parent.spawn((
                Text::new("No items"),
                title_font.clone(),
                TextColor(Color::WHITE),
                ItemInfoName,
            ));
        });

        // 物品图标
        parent.spawn((
            Node {
                width: Val::Px(80.0),
                height: Val::Px(80.0),
                margin: UiRect::all(Val::Px(10.0)),
                align_self: AlignSelf::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            ItemInfoIcon,
            children![],
        ));

        // 物品数量
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Amount: 0/0"),
                font.clone(),
                TextColor(Color::WHITE),
                ItemInfoNum,
            ));
        });

        // 物品类型
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Type: Unknown"),
                font.clone(),
                TextColor(Color::WHITE),
                ItemInfoType,
            ));
        });

        // 物品描述
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(40.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        )).with_children(|parent| {
            parent.spawn((
                Text::new(""),
                font.clone(),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ItemInfoDesc,
            ));
        });

        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Use Arrow Button to Move \n Use Enter for Selecting"),
                font.clone(),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
    }).id();

    // 创建右侧物品栏面板
    let grid_panel_entity = commands.spawn((
        grid_panel_node,
        BackgroundColor(panel_color),
        ItemGridPanel {
            current: 0,
            page: 0,
            max: if bag.slots.len() == 0 { None } else { Some(bag.slots.len()-1) },
        },
    )).id();

    // 物品网格容器
    let mut grid_entity = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(80.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ItemPage(0),
        Visibility::Visible,
        ChildOf(grid_panel_entity),
    )).id();

    let mut current_row: Option<Entity> = None;
    for (i, key) in bag.slots.keys().enumerate() {
        let info = items.infos.get(key).unwrap();
        let num = bag.get(key);
        if i != 0 && i % (GRID_HEIGHT * GRID_WIDTH) == 0 {
            grid_entity = commands.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ItemPage(i / (GRID_HEIGHT * GRID_WIDTH)),
                Visibility::Hidden,
                ChildOf(grid_panel_entity),
            )).id();
        }
        if i % GRID_WIDTH == 0 {
            current_row = Some(commands.spawn(
                (Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0 / GRID_HEIGHT as f32),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                Visibility::Inherited,
                ChildOf(grid_entity),
            )).id()); 
        }
        let slot_color = if i == 0 {
            Color::srgb(1.0, 1.0, 0.0).with_alpha(0.3) // 选中状态
        } else {
            Color::srgb(0.3, 0.3, 0.3).with_alpha(0.8)
        };
        commands.spawn((
            Node {
                width: Val::Percent(100.0 / GRID_WIDTH as f32),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            Visibility::Inherited,
            BackgroundColor(slot_color),
            ItemSlot,
            ItemSlotOf(grid_entity),
            ChildOf(current_row.unwrap()),
        )).with_children(|parent| {
            // 物品图标
            parent.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(70.0),
                    ..default()
                },
                children![
                    ImageNode::new(info.icon.clone())
                ],
                BackgroundColor(Color::srgb(0., 0., 0.).with_alpha(0.)),
            ));

            // 物品数量
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                    width: Val::Px(20.0),
                    height: Val::Px(15.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            )).with_children(|parent| {
                parent.spawn((
                    Text::new( num.to_string()),
                    TextFont {
                        font: asset_server.load("UI/Fonts/m5x7.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
    }

    // 将面板添加到主容器
    commands.entity(main_entity)
        .add_children(&[info_panel_entity, grid_panel_entity]);
}

// 处理键盘输入的系统
fn handle_inventory_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    actives: Single<(&ItemBag, &mut ActiveItems), With<Player>>,
    scroll_query: Single<(&mut ItemGridPanel, &Children)>,
    mut slot_query: Query<(&mut Visibility, &HasItemSlot), With<ItemPage>>,
    mut next_state: ResMut<NextState<PausedState>>,
    mut slots: Query<&mut BackgroundColor, With<ItemSlot>>,
) {
    let (mut grid, childs) = scroll_query.into_inner();
    if let Some(gridmax) = grid.max {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {  
            let updated = up(grid.current);
            let cur_page = get_page(grid.current);
            let updated_page = get_page(updated);
            let cur = (**childs).get(cur_page).unwrap();
            let (mut cur_visibility, has_slot) = slot_query.get_mut(*cur).unwrap();
            
            let cur_entity = (**has_slot).get(grid.current % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut cur_slot = slots.get_mut(*cur_entity).unwrap();
            cur_slot.0 = Color::srgb(0.3, 0.3, 0.3).with_alpha(0.8);
            let next_entity = (**has_slot).get(updated % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut next_slot = slots.get_mut(*next_entity).unwrap();
            next_slot.0 = Color::srgb(1.0, 1.0, 0.0).with_alpha(0.3);
            grid.current = updated;
            if cur_page != updated_page {
                let next = (**childs).get(updated_page).unwrap();
                *cur_visibility = Visibility::Hidden;
                let (mut next_visibility, _) = slot_query.get_mut(*next).unwrap();
                *next_visibility = Visibility::Visible;
            }
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let updated = down(grid.current, gridmax);
            let cur_page = get_page(grid.current);
            let updated_page = get_page(updated);
            let cur = (**childs).get(cur_page).unwrap();
            let (mut cur_visibility, has_slot) = slot_query.get_mut(*cur).unwrap();
            
            let cur_entity = (**has_slot).get(grid.current % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut cur_slot = slots.get_mut(*cur_entity).unwrap();
            cur_slot.0 = Color::srgb(0.3, 0.3, 0.3).with_alpha(0.8);
            let next_entity = (**has_slot).get(updated % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut next_slot = slots.get_mut(*next_entity).unwrap();
            next_slot.0 = Color::srgb(1.0, 1.0, 0.0).with_alpha(0.3);
            grid.current = updated;
            if cur_page != updated_page {
                let next = (**childs).get(updated_page).unwrap();
                *cur_visibility = Visibility::Hidden;
                let (mut next_visibility, _) = slot_query.get_mut(*next).unwrap();
                *next_visibility = Visibility::Visible;
            }
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let updated = left(grid.current);
            let cur_page = get_page(grid.current);
            let updated_page = get_page(updated);
            let cur = (**childs).get(cur_page).unwrap();
            let (mut cur_visibility, has_slot) = slot_query.get_mut(*cur).unwrap();
            
            let cur_entity = (**has_slot).get(grid.current % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut cur_slot = slots.get_mut(*cur_entity).unwrap();
            cur_slot.0 = Color::srgb(0.3, 0.3, 0.3).with_alpha(0.8);
            let next_entity = (**has_slot).get(updated % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut next_slot = slots.get_mut(*next_entity).unwrap();
            next_slot.0 = Color::srgb(1.0, 1.0, 0.0).with_alpha(0.3);
            grid.current = updated;
            if cur_page != updated_page {
                let next = (**childs).get(updated_page).unwrap();
                *cur_visibility = Visibility::Hidden;
                let (mut next_visibility, _) = slot_query.get_mut(*next).unwrap();
                *next_visibility = Visibility::Visible;
            }
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let updated = right(grid.current, gridmax);
            let cur_page = get_page(grid.current);
            let updated_page = get_page(updated);
            let cur = (**childs).get(cur_page).unwrap();
            let (mut cur_visibility, has_slot) = slot_query.get_mut(*cur).unwrap();
            
            let cur_entity = (**has_slot).get(grid.current % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut cur_slot = slots.get_mut(*cur_entity).unwrap();
            cur_slot.0 = Color::srgb(0.3, 0.3, 0.3).with_alpha(0.8);
            let next_entity = (**has_slot).get(updated % (GRID_HEIGHT * GRID_WIDTH)).unwrap();
            let mut next_slot = slots.get_mut(*next_entity).unwrap();
            next_slot.0 = Color::srgb(1.0, 1.0, 0.0).with_alpha(0.3);
            grid.current = updated;
            if cur_page != updated_page {
                let next = (**childs).get(updated_page).unwrap();
                *cur_visibility = Visibility::Hidden;
                let (mut next_visibility, _) = slot_query.get_mut(*next).unwrap();
                *next_visibility = Visibility::Visible;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Enter) {
            let (bag, mut acts) = actives.into_inner();
            let key = bag.slots.keys().nth(grid.current).unwrap();
            if acts.items.contains(key) {
                if let Some(pos) = acts.items.iter().position(|x| *x == *key) {
                    acts.items.remove(pos);
                }
            } else {
                acts.items.push(key.clone());
            }
            
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(PausedState::Running);
    }
}

// 更新物品信息显示的系统
fn update_item_info_display(
    mut commands: Commands,
    grid_panel: Single<&ItemGridPanel>,
    item_bag: Single<&ItemBag, With<Player>>,
    item_list: Res<ItemList>,
    mut name_query: Query<&mut Text, With<ItemInfoName>>,
    mut desc_query: Query<&mut Text, (With<ItemInfoDesc>, Without<ItemInfoName>)>,
    mut type_query: Query<&mut Text, (With<ItemInfoType>, Without<ItemInfoDesc>, Without<ItemInfoName>)>,
    icon_query: Query<(Entity, &Children), With<ItemInfoIcon>>,
    mut num_query: Query<&mut Text, (With<ItemInfoNum>, Without<ItemInfoType>, Without<ItemInfoDesc>, Without<ItemInfoName>)>,
) {
    let panel = grid_panel.into_inner();
    let bag = item_bag.into_inner();
    if bag.slots.is_empty() { return; }
    let current = panel.current;
    let key = bag.slots.keys().nth(current).unwrap();
    let info = item_list.infos.get(key).unwrap();
    let num = bag.get(key);

    let mut name = name_query.single_mut().unwrap();
    let mut desc = desc_query.single_mut().unwrap();
    let mut itype = type_query.single_mut().unwrap();
    let (ientity, icon) = icon_query.single().unwrap();
    let mut ntext = num_query.single_mut().unwrap();

    name.0 = info.name.clone();
    desc.0 = info.description.clone();
    itype.0 = match info.item_type {
        ItemType::Consumable(_) => "Consumable".to_string(),
        ItemType::Ability(_) => "Ability".to_string(),
    };
    ntext.0 = format!("{}/{}", num, info.max_stack); 
    for entity in (**icon).iter() {
        commands.entity(*entity).despawn();
    }
    commands.spawn(
        (
            ImageNode::new(info.icon.clone()),
            ChildOf(ientity),
        )
    );
}

fn despawn_inventory_ui(
    mut commands: Commands,
    ui: Single<Entity, With<InventoryUI>>
) {
    let entity = ui.into_inner();
    commands.entity(entity).despawn();
}

pub struct BagUIPlugin;

impl Plugin for BagUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PausedState::BagUI), spawn_inventory_ui);
        app.add_systems(OnExit(PausedState::BagUI), despawn_inventory_ui);
        app.add_systems(Update, (
            handle_inventory_input.run_if(in_state(PausedState::BagUI)),
            update_item_info_display.run_if(in_state(PausedState::BagUI)),
        ));
    }
}
