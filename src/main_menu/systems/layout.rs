use bevy::prelude::*;

use crate::{
    bundles::{button_bundle, text_bundle},
    main_menu::{
        components::{MainMenu, PlayButton, QuitButton},
        styles::{MAIN_MENU_IMAGE_STYLE, NORMAL_BUTTON_COLOR, TITLE_STYLE},
    },
};

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
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                // background_color: Color::RED.into(),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: MAIN_MENU_IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });
                    parent.spawn(text_bundle("Bevy Ball Game", &asset_server, 32.0));
                    parent.spawn(ImageBundle {
                        style: MAIN_MENU_IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });
            // Play button
            parent
                .spawn((
                    button_bundle(200.0, 80.0, NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(text_bundle("Play", &asset_server, 32.0));
                });
            // Quit button
            parent
                .spawn((
                    button_bundle(200.0, 80.0, NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(text_bundle("Quit", &asset_server, 32.0));
                });
        })
        .id();
    return main_menu_entity;
}
