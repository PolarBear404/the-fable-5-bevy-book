//! Listing 13-5：场记的两本账——投影的 area 与视口反算，丈量实拍范围

use bevy::prelude::*;

/// 标记：侠客阿燕
#[derive(Component)]
struct Hero;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(Update, ((walk_hero, follow_hero).chain(), report_frame))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.13, 0.11), Vec2::new(1400.0, 900.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
    for i in -3..=3 {
        for y in [-350.0, 350.0] {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.95, 0.75, 0.25), Vec2::splat(22.0)),
                Transform::from_xyz(i as f32 * 200.0, y, -5.0),
            ));
        }
    }
    commands.spawn((
        Hero,
        Sprite::from_color(Color::srgb(0.85, 0.2, 0.2), Vec2::splat(30.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    println!("老雷：小鹿，每个镜头给我报实拍范围！");
}

fn walk_hero(time: Res<Time>, mut hero: Single<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs() * 0.5;
    hero.translation.x = 500.0 * t.sin();
    hero.translation.y = 250.0 * (2.0 * t).sin();
}

fn follow_hero(
    time: Res<Time>,
    mut lens: Single<&mut Transform, (With<Camera2d>, Without<Hero>)>,
    hero: Single<&Transform, With<Hero>>,
) {
    let target = hero.translation.with_z(lens.translation.z);
    lens.translation.smooth_nudge(&target, 2.0, time.delta_secs());
}

// ANCHOR: report
/// 场记小鹿：每两秒丈量一次镜头实拍范围，两种量法相互核对
fn report_frame(
    time: Res<Time>,
    lens: Single<(&Camera, &GlobalTransform, &Projection)>,
    mut clock: Local<f32>,
) {
    *clock += time.delta_secs();
    if *clock < 2.0 {
        return;
    }
    *clock -= 2.0;
    let (camera, lens_pos, projection) = *lens;

    // 量法一：正交投影的 area 是“以相机为原点”的取景框，平移到相机位置就是世界范围
    let Projection::Orthographic(ortho) = projection else {
        return;
    };
    let center = lens_pos.translation().truncate();
    let frame = Rect {
        min: ortho.area.min + center,
        max: ortho.area.max + center,
    };
    println!(
        "场记：实拍范围 x [{:.0}, {:.0}]，y [{:.0}, {:.0}]",
        frame.min.x, frame.max.x, frame.min.y, frame.max.y
    );

    // 量法二：把视口的左上角与右下角反算回世界坐标
    let Some(size) = camera.logical_viewport_size() else {
        return;
    };
    let (Ok(top_left), Ok(bottom_right)) = (
        camera.viewport_to_world_2d(lens_pos, Vec2::ZERO),
        camera.viewport_to_world_2d(lens_pos, size),
    ) else {
        return;
    };
    println!(
        "场记：视口反算 左上→[{:.0}, {:.0}]，右下→[{:.0}, {:.0}]，两本账对上了。",
        top_left.x, top_left.y, bottom_right.x, bottom_right.y
    );
}
// ANCHOR_END: report
