use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((Visibility::default(), MAIN_MENU_NODE.clone(), MainMenu {}))
        .with_children(|parent| {
            // === Title ===
            parent.spawn(TITLE_NODE.clone()).with_children(|parent| {
                // Image 1
                parent.spawn((
                    IMAGE_NODE.clone(),
                    ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                ));
                // Text
                parent.spawn((
                    Text("Ball & Stars".to_string()),
                    get_title_text_style(asset_server),
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
                // Image 2
                parent.spawn((
                    IMAGE_NODE.clone(),
                    ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                ));
            });
            // === Play Button ===
            parent
                .spawn((
                    Button,
                    BUTTON_NODE.clone(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Play".to_string()),
                        get_button_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // === Quit Button ===
            parent
                .spawn((
                    Button,
                    BUTTON_NODE.clone(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Quit".to_string()),
                        get_button_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        })
        .id();

    main_menu_entity
}
