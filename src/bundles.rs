use bevy::prelude::*;

pub fn text_bundle(text: &str, asset_server: &Res<AssetServer>, font_size: f32) -> TextBundle {
    return TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: font_size,
                    color: Color::WHITE,
                },
            )],
            alignment: TextAlignment::Center,
            ..Text::default()
        },
        ..TextBundle::default()
    };
}

pub fn button_bundle(with: f32, height: f32, button_color: Color) -> ButtonBundle {
    return ButtonBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(with), Val::Px(height)),
            ..default()
        },
        background_color: button_color.into(),
        ..ButtonBundle::default()
    };
}
