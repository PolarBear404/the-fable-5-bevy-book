//! Listing 16-4：一副字模的三种叫法——Handle、家族名、语义类别

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
/// 字模库：把提货单收好——Family 找的是库房里的字体，提货单丢光就清库（第 14 章）
#[derive(Resource)]
struct FontStash(#[allow(dead_code)] Vec<Handle<Font>>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // 两副面都上架：Regular 与 Bold 同属 "Book Sans SC" 家族
    let regular = asset_server.load("fonts/book-sans-sc-regular.otf");
    let bold: Handle<Font> = asset_server.load("fonts/book-sans-sc-bold.otf");
    commands.insert_resource(FontStash(vec![regular.clone(), bold]));

    let rows: [(&str, FontSource, FontWeight); 5] = [
        // ① 按提货单：钉死这一副面
        ("Handle 提货单", regular.clone().into(), FontWeight::NORMAL),
        // ② 按家族名：报出字体里写的名字
        ("Family 家族名", "Book Sans SC".into(), FontWeight::NORMAL),
        // ③ 家族名 + 字重：让引擎在家族里挑那副粗面
        ("Family 挑粗面", "Book Sans SC".into(), FontWeight::BOLD),
        // ④ 名字写错一个字母——会发生什么？
        ("写错家族名", "Book Sans CS".into(), FontWeight::NORMAL),
        // ⑤ 语义类别：不点名，只说"要一副等宽的"
        ("语义类别", FontSource::Monospace, FontWeight::NORMAL),
    ];
    for (i, (label, font, weight)) in rows.into_iter().enumerate() {
        let y = 220.0 - 110.0 * i as f32;
        // 旁注小签用提货单字体，保证坑挖出来时签子还在
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font: regular.clone().into(),
                font_size: FontSize::Px(20.0),
                ..default()
            },
            Transform::from_xyz(-440.0, y, 0.0),
        ));
        commands.spawn((
            Text2d::new("渡口夜话 AaGg 0123"),
            TextFont {
                font,
                font_size: FontSize::Px(40.0),
                weight,
                ..default()
            },
            Transform::from_xyz(120.0, y, 0.0),
        ));
    }
}
// ANCHOR_END: setup
