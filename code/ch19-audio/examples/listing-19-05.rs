//! Listing 19-5：AudioSink 哪一帧上岗——递谱、到货、开嗓的三步时序

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, watch_backstage)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        PlaybackSettings::LOOP,
    ));
    println!("琴师：曲谱递上去了。场记，盯着后台，看引擎哪一帧把缰绳递过来。");
}

// ANCHOR: watch
/// 场记的时序台账：递谱之后，曲谱哪帧到货、AudioSink 哪帧上岗
fn watch_backstage(
    mut frame: Local<u64>,
    mut broadcasts: MessageReader<AssetEvent<AudioSource>>,
    fresh_sinks: Query<&AudioSink, Added<AudioSink>>,
) {
    *frame += 1;
    if *frame == 1 {
        println!("场记：第 1 帧——查 AudioSink：还没有。");
    }
    // 库房广播（第 14 章）：曲谱字节进架了
    for event in broadcasts.read() {
        if let AssetEvent::LoadedWithDependencies { .. } = event {
            println!("场记：第 {} 帧——曲谱到货。", *frame);
        }
    }
    // 引擎真正开播的那一刻：AudioSink 被插上实体
    for sink in &fresh_sinks {
        println!(
            "场记：第 {} 帧——AudioSink 上岗。音量 {:?}，速度 ×{}，暂停 {}，进度 {:.2} 秒。",
            *frame,
            sink.volume(),
            sink.speed(),
            sink.is_paused(),
            sink.position().as_secs_f32(),
        );
    }
}
// ANCHOR_END: watch
