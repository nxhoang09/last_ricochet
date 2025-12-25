use bevy::prelude::*;
use crate::states::AppState;
use crate::resources::sound::SoundAssets;

const FONT_PATH: &str = "fonts/pixel_3.ttf";

// --- MÀU SẮC MỚI ---
const TITLE_COLOR: Color = Color::srgb(1.0, 0.84, 0.0); // Vàng Gold
const NORMAL_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9); // Trắng hơi xám
const HOVERED_TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 0.0); // Vàng tươi khi hover
const PRESSED_TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 0.0); // Vàng đậm khi ấn

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct MainMenuNode;

#[derive(Component)]
pub struct TutorialNode;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Tutorial,
    Exit,
    BackToMenu,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);
    let bg_image = asset_server.load("sprites/ui/menu_bg.png"); 

    // 1. Background
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            image: UiImage::new(bg_image),
            ..default()
        },
        MenuUI,
    ));

    // 2. Container Chính
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row, 
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Percent(10.0)),
                ..default()
            },
            ..default()
        },
        MenuUI,
    )).with_children(|parent| {
        
        // --- CỘT TRÁI: TITLE ---
        parent.spawn(TextBundle::from_section(
            "LAST\nRICOCHET",
            TextStyle {
                font: font.clone(),
                font_size: 70.0,
                color: TITLE_COLOR,
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(100.0)),
            ..default()
        }));

        // --- CỘT PHẢI: BẢNG ĐIỀU KHIỂN ---
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // Căn chỉnh khung bảng (khớp với hình ảnh)
                width: Val::Px(350.0), 
                height: Val::Px(500.0), 
                margin: UiRect::right(Val::Px(50.0)), 
                ..default()
            },
            ..default()
        }).with_children(|board| {
            
            // === LỚP 1: MENU CHÍNH ===
            board.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(25.0), // Tăng khoảng cách giữa các nút
                        display: Display::Flex,
                        ..default()
                    },
                    ..default()
                },
                MainMenuNode,
            )).with_children(|menu| {
                // Tăng font size lên 35.0 (lớn hơn cũ)
                spawn_button(menu, &font, "PLAY GAME", MenuButtonAction::Play, 35.0);
                spawn_button(menu, &font, "TUTORIAL", MenuButtonAction::Tutorial, 35.0);
                spawn_button(menu, &font, "EXIT", MenuButtonAction::Exit, 35.0);
            });

            // === LỚP 2: TUTORIAL ===
            board.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.0),
                        display: Display::None,
                        ..default()
                    },
                    ..default()
                },
                TutorialNode,
            )).with_children(|tutorial| {
                tutorial.spawn(TextBundle::from_section(
                    "HOW TO PLAY",
                    TextStyle { font: font.clone(), font_size: 25.0, color: TITLE_COLOR },
                ));

                let instructions = "WASD: Move\nMouse: Aim\nClick: Shoot\n\nKill enemies\nto survive!";
                tutorial.spawn(TextBundle::from_section(
                    instructions,
                    TextStyle { font: font.clone(), font_size: 18.0, color: Color::WHITE },
                ).with_text_justify(JustifyText::Center));

                spawn_button(tutorial, &font, "BACK", MenuButtonAction::BackToMenu, 30.0);
            });
        });
    });
}

// HÀM SPAWN BUTTON ĐÃ CẢI TIẾN
fn spawn_button(
    parent: &mut ChildBuilder, 
    font: &Handle<Font>, 
    text: &str, 
    action: MenuButtonAction,
    font_size: f32, // Thêm tham số size để linh hoạt
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                // Không set width cố định để nút tự co giãn theo chữ
                padding: UiRect::all(Val::Px(10.0)), 
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            // QUAN TRỌNG: Màu nền trong suốt
            background_color: BackgroundColor(Color::NONE), 
            ..default()
        },
        action,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: font_size, // Dùng size to hơn
                color: NORMAL_TEXT_COLOR,
            },
        ));
    });
}

pub fn menu_action(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,

    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &Children, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    mut menu_node_query: Query<&mut Style, (With<MainMenuNode>, Without<TutorialNode>)>,
    mut tutorial_node_query: Query<&mut Style, (With<TutorialNode>, Without<MainMenuNode>)>,
) {
    for (interaction, action, children, mut transform) in &mut interaction_query {
        
        // 1. Xác định trạng thái để chọn màu và tỷ lệ (scale)
        let (text_color, scale) = match *interaction {
            Interaction::Pressed => (PRESSED_TEXT_COLOR, 1.05),
            Interaction::Hovered => (HOVERED_TEXT_COLOR, 1.2), // Phóng to 1.2 lần khi hover
            Interaction::None => (NORMAL_TEXT_COLOR, 1.0),     // Về bình thường
        };

        // 2. Áp dụng hiệu ứng phóng to cho nút
        transform.scale = Vec3::splat(scale);

        // 3. Áp dụng đổi màu cho Text con
        for &child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.sections[0].style.color = text_color;
            }
        }

        // 4. Xử lý logic bấm nút (giữ nguyên logic cũ)
        if *interaction == Interaction::Pressed {
            commands.spawn(AudioBundle {
                source: sound_assets.select.clone(),
                settings: PlaybackSettings::DESPAWN, 
            });

            match action {
                MenuButtonAction::Play => {
                    next_state.set(AppState::Playing);
                }
                MenuButtonAction::Tutorial => {
                    if let Ok(mut style) = menu_node_query.get_single_mut() {
                        style.display = Display::None;
                    }
                    if let Ok(mut style) = tutorial_node_query.get_single_mut() {
                        style.display = Display::Flex;
                    }
                }
                MenuButtonAction::BackToMenu => {
                    if let Ok(mut style) = menu_node_query.get_single_mut() {
                        style.display = Display::Flex;
                    }
                    if let Ok(mut style) = tutorial_node_query.get_single_mut() {
                        style.display = Display::None;
                    }
                }
                MenuButtonAction::Exit => {
                    app_exit_events.send(bevy::app::AppExit::Success);
                }
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}