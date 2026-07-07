//! Listing 27-5：会写字的粉线——描线字体给记号配说明。
//! 道具箱滑到哪儿，坐标牌跟到哪儿；再看一眼它不认识中文的下场。

use bevy::color::palettes::css::*;
use bevy::prelude::*;

const CRATE_SIZE: Vec2 = Vec2::new(120.0, 90.0);
const TRACK_HALF: f32 = 320.0;

#[derive(Component)]
struct PropCrate {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (slide_crate, chalk_labels).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        PropCrate { speed: 240.0 },
        Sprite::from_color(Color::srgb(0.52, 0.42, 0.30), CRATE_SIZE),
        Transform::from_xyz(-TRACK_HALF, 0.0, 0.0),
    ));
    println!("检场：粉笔削尖了——这回写字。");
}

fn slide_crate(mut crates: Query<(&mut PropCrate, &mut Transform)>, time: Res<Time>) {
    for (mut prop, mut transform) in &mut crates {
        transform.translation.x += prop.speed * time.delta_secs();
        if transform.translation.x.abs() > TRACK_HALF {
            transform.translation.x = transform.translation.x.clamp(-TRACK_HALF, TRACK_HALF);
            prop.speed = -prop.speed;
        }
    }
}

// ANCHOR: labels
fn chalk_labels(mut gizmos: Gizmos, crates: Query<(&PropCrate, &Transform)>) {
    for (prop, transform) in &crates {
        let center = transform.translation.truncate();
        gizmos.rect_2d(center, CRATE_SIZE, GOLD);

        // 跟班坐标牌：锚点 (0, -0.5) = 下沿居中，牌子悬在箱顶上方
        gizmos.text_2d(
            center + Vec2::Y * (CRATE_SIZE.y / 2.0 + 12.0),
            &format!("x = {:+.0}", center.x),
            28.0,
            Vec2::new(0.0, -0.5),
            GOLD,
        );

        // 速度牌分两段上色：标签白、数值随方向换色
        let speed_color = if prop.speed > 0.0 { LIME } else { ORANGE_RED };
        gizmos.text_sections_2d(
            center - Vec2::Y * (CRATE_SIZE.y / 2.0 + 12.0),
            &[
                ("speed ", Color::WHITE),
                (&format!("{:+.0} px/s", prop.speed), speed_color.into()),
            ],
            28.0,
            Vec2::new(0.0, 0.5), // 锚点 (0, 0.5) = 上沿居中，牌子吊在箱底下方
        );
    }

    // 台口的固定招牌：多行文字，锚点 (-0.5, 0.5) = 左上角钉在给定点
    gizmos.text_2d(
        Isometry2d::from_translation(Vec2::new(-620.0, 340.0)),
        "STAGE CHECK\nprops: 1",
        32.0,
        Vec2::new(-0.5, 0.5),
        AQUA,
    );

    // 检场逞能写中文——描线字体只认 ASCII 32~126，"检场"二字被当成两个空格跳过
    gizmos.text_2d(
        Isometry2d::from_translation(Vec2::new(-620.0, -340.0)),
        "检场 on duty",
        32.0,
        Vec2::new(-0.5, -0.5),
        PLUM,
    );
}
// ANCHOR_END: labels
