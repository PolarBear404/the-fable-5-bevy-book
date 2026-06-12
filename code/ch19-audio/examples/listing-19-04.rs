//! Listing 19-4：DESPAWN 自动拆台 + 变速变调——一面锣敲出三个音

use bevy::prelude::*;

#[derive(Resource)]
struct GongSound(Handle<AudioSource>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (strike_gong, stage_count))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(GongSound(asset_server.load("sfx/gong.wav")));
    println!("鼓师：1 低锣、2 中锣、3 高锣——同一面锣，转速不同。");
}

// ANCHOR: strike
/// 数字键选音高：speed 既是快慢也是音高，0.8 倍速就是低八成的锣
fn strike_gong(
    keyboard: Res<ButtonInput<KeyCode>>,
    gong: Res<GongSound>,
    mut commands: Commands,
) {
    for (key, speed, name) in [
        (KeyCode::Digit1, 0.7, "低"),
        (KeyCode::Digit2, 1.0, "中"),
        (KeyCode::Digit3, 1.4, "高"),
    ] {
        if keyboard.just_pressed(key) {
            commands.spawn((
                AudioPlayer::new(gong.0.clone()),
                // 播完连实体一起拆掉，后台不留家伙
                PlaybackSettings::DESPAWN.with_speed(speed),
            ));
            println!("鼓师：{name}锣，哐——");
        }
    }
}
// ANCHOR_END: strike

fn stage_count(players: Query<Entity, With<AudioPlayer>>, mut last: Local<usize>) {
    let count = players.iter().count();
    if count != *last {
        println!("场记：后台躺着 {count} 个音频实体。");
        *last = count;
    }
}
