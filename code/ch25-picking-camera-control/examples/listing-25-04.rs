//! Listing 25-4：sprite 与 UI 的拾取后端。这里不加 `MeshPickingPlugin`。

use bevy::{picking::pointer::PointerButton, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct StatusLine;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let font = asset_server.load("fonts/book-sans-sc-regular.otf");

    // ANCHOR: sprite_pick
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.84, 0.28, 0.24), Vec2::new(180.0, 120.0)),
            Transform::from_xyz(-140.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(0.12)),
            Pickable::default(),
        ))
        .observe(recolor_sprite_on_over)
        .observe(recolor_sprite_on_out)
        .observe(report_sprite_click);
    // ANCHOR_END: sprite_pick

    commands.spawn((
        Sprite::from_color(Color::srgb(0.22, 0.55, 0.86), Vec2::new(160.0, 110.0)),
        Transform::from_xyz(120.0, -20.0, -1.0).with_rotation(Quat::from_rotation_z(-0.18)),
        Pickable::default(),
    ));

    // ANCHOR: ui_pick
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: px(22),
                bottom: px(22),
                padding: UiRect::axes(px(16), px(10)),
                border: UiRect::all(px(1)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.12, 0.15)),
            BorderColor::all(Color::srgb(0.34, 0.75, 0.95)),
        ))
        .observe(report_ui_click)
        .with_child((
            Text::new("UI 也能收 Pointer<Click>"),
            TextFont {
                font: font.clone(),
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        ));
    // ANCHOR_END: ui_pick

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(22),
            top: px(22),
            ..default()
        },
        Pickable::IGNORE,
        children![(
            StatusLine,
            Text::new("点红色 sprite，或点右下角 UI"),
            TextFont {
                font,
                font_size: 23.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        )],
    ));
}

fn recolor_sprite_on_over(event: On<Pointer<Over>>, mut sprites: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
        sprite.color = Color::srgb(0.32, 0.88, 0.78);
    }
}

fn recolor_sprite_on_out(event: On<Pointer<Out>>, mut sprites: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
        sprite.color = Color::srgb(0.84, 0.28, 0.24);
    }
}

fn report_sprite_click(
    event: On<Pointer<Click>>,
    mut status: Single<&mut Text, With<StatusLine>>,
) {
    if event.button == PointerButton::Primary {
        status.0 = "红色 sprite 收到了 Pointer<Click>".into();
    }
}

fn report_ui_click(
    event: On<Pointer<Click>>,
    mut status: Single<&mut Text, With<StatusLine>>,
) {
    if event.button == PointerButton::Primary {
        status.0 = "右下角 UI 节点收到了 Pointer<Click>".into();
    }
}
