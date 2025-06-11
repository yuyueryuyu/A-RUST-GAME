//! 结局滚动字幕

use bevy::prelude::*;

/// 字幕文字
#[derive(Component)]
struct CreditText;

/// 滚动字幕
#[derive(Component)]
struct ScrollingCredits {
    /// 滚动速度
    speed: f32,
}

/// 初始化字幕
fn setup_credits(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Node {  
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart, 
            align_items: AlignItems::Center,
            overflow: Overflow::clip(),
            ..default()
        },BackgroundColor(Color::BLACK.into())))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        top: Val::Percent(100.0),
                        ..default()
                    },
                    ScrollingCredits { speed: 50.0 },
                ))
                .with_children(|parent| {
                    let credits = vec![
                        "The Hero found the throne of the lost kingdom",
                        "He became the new king...",
                        " ",
                        "But, at what cost?",
                        " ",
                        " ",
                        " ",
                        "The Lost Kingdom",
                        "Whispers of Dark",
                        " ",
                        " ",
                        " ",
                        " ",
                        "Developed By",
                        "Team Ruast",
                        " ",
                        " ",
                        " ",
                        "Art Director",
                        "Programming Assistant",
                        "Tianyi Wang",
                        " ",
                        " ",
                        " ",
                        "Map Designer",
                        "Promotion Director",
                        "Moshi Zhou",
                        " ",
                        " ",
                        " ",
                        "Director",
                        "Game Designer",
                        "Technical Director",
                        "Kaiyue Yu",
                        " ",
                        " ",
                        " ",
                        "Thanks for Playing!",
                        " ",
                        " ",
                        " ",
                        "Press Enter to Exit",
                    ];

                    for credit in credits {
                        parent.spawn((
                            Text::new(credit),
                            TextFont {
                                font: asset_server.load("UI/Fonts/m5x7.ttf"),
                                font_size: 40.0,
                                ..Default::default()
                            },
                            TextColor::WHITE,
                            CreditText,
                        ));
                    }
                });
        });
}

/// 滚动字幕系统
fn scroll_credits_system(
    time: Res<Time>,
    mut query: Query<(&mut Node, &ScrollingCredits)>,
    window_query: Query<&Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit_events: EventWriter<AppExit>,
) {
    let window = window_query.single().unwrap();
    let window_height = window.height();

    for (mut style, credits) in query.iter_mut() {
        // 获取当前位置
        let current_top = match style.top {
            Val::Px(px) => px,
            _ => window_height,
        };

        // 计算新位置
        let new_top = (current_top - credits.speed * time.delta_secs()).clamp(-1500.0, 50000.);

        // 更新位置
        style.top = Val::Px(new_top);
    }
    if keyboard_input.just_pressed(KeyCode::Enter) {
        exit_events.write(AppExit::Success);
    }
}

pub struct EndingPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for EndingPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), setup_credits);
        app.add_systems(Update, (
            scroll_credits_system.run_if(in_state(self.state.clone())),
        ));
    }
}
