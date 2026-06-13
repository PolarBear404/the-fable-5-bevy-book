//! Listing 25-1：最小 3D mesh 拾取。鼠标移入、按下、拖拽，都是挂在实体上的 Observer。

use bevy::{picking::pointer::PointerButton, prelude::*};

fn main() {
    App::new()
        // ANCHOR: plugins
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        // ANCHOR_END: plugins
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.07)))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Swatch {
    idle: Handle<StandardMaterial>,
    hover: Handle<StandardMaterial>,
    pressed: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let idle = materials.add(Color::srgb(0.45, 0.58, 0.78));
    let hover = materials.add(Color::srgb(0.35, 0.88, 0.88));
    let pressed = materials.add(Color::srgb(1.0, 0.78, 0.30));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.16, 0.17, 0.18))),
        Pickable::IGNORE,
    ));

    // ANCHOR: mesh_observers
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.4, 1.4, 1.4))),
            MeshMaterial3d(idle.clone()),
            Transform::from_xyz(0.0, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(0.45)),
            Swatch {
                idle,
                hover,
                pressed,
            },
        ))
        .observe(on_over)
        .observe(on_out)
        .observe(on_press)
        .observe(on_release)
        .observe(on_drag_rotate);
    // ANCHOR_END: mesh_observers

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 5.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.2, 6.0).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));

    let font = asset_server.load("fonts/book-sans-sc-regular.otf");
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(16),
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Text::new("移入变青，按下变黄，左键拖拽旋转方块"),
            TextFont {
                font,
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        )],
    ));
}

fn set_material(
    entity: Entity,
    choose: impl FnOnce(&Swatch) -> Handle<StandardMaterial>,
    query: &mut Query<(&Swatch, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if let Ok((swatch, mut material)) = query.get_mut(entity) {
        material.0 = choose(swatch);
    }
}

fn on_over(
    event: On<Pointer<Over>>,
    mut query: Query<(&Swatch, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    set_material(event.entity, |s| s.hover.clone(), &mut query);
}

fn on_out(
    event: On<Pointer<Out>>,
    mut query: Query<(&Swatch, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    set_material(event.entity, |s| s.idle.clone(), &mut query);
}

fn on_press(
    event: On<Pointer<Press>>,
    mut query: Query<(&Swatch, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if event.button == PointerButton::Primary {
        set_material(event.entity, |s| s.pressed.clone(), &mut query);
    }
}

fn on_release(
    event: On<Pointer<Release>>,
    mut query: Query<(&Swatch, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if event.button == PointerButton::Primary {
        set_material(event.entity, |s| s.hover.clone(), &mut query);
    }
}

// ANCHOR: drag
fn on_drag_rotate(
    event: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    let Ok(mut transform) = transforms.get_mut(event.entity) else {
        return;
    };
    transform.rotate_y(event.delta.x * 0.015);
    transform.rotate_local_x(event.delta.y * 0.015);
}
// ANCHOR_END: drag
