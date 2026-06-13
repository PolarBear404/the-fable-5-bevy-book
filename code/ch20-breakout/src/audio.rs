//! audio：武场——听 Knock 敲家伙，管 BGM、胜负定音与总闸。
//! 它对玩法一无所知：碰撞长什么样、分数怎么算，一概不问。

use bevy::audio::Volume;
use bevy::prelude::*;

use crate::game::{Knock, Outcome};
use crate::{GameState, IsPaused};

/// BGM 自己的基准音量（总闸另算）
const BGM_LEVEL: f32 = 0.45;

/// 武场的家伙什：全部音效的提货单
#[derive(Resource)]
struct SoundBank {
    clack: Handle<AudioSource>,
    shatter: Handle<AudioSource>,
    drum: Handle<AudioSource>,
    win: Handle<AudioSource>,
    lose: Handle<AudioSource>,
}

/// 循环 BGM 的标记——总闸调音量时要找到它
#[derive(Component)]
struct Bgm;

// ANCHOR: plugin
pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_band)
            .add_systems(OnEnter(GameState::GameOver), verdict_sting)
            .add_systems(OnEnter(IsPaused::Paused), hold_sinks)
            .add_systems(OnExit(IsPaused::Paused), release_sinks)
            .add_systems(
                Update,
                (
                    play_knocks,
                    master_dial,
                    apply_master.run_if(resource_changed::<GlobalVolume>),
                ),
            );
    }
}
// ANCHOR_END: plugin

/// 开张就位：备好提货单，序曲循环起播
fn setup_band(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundBank {
        clack: asset_server.load("sfx/clack.wav"),
        shatter: asset_server.load("sfx/shatter.wav"),
        drum: asset_server.load("sfx/drum.wav"),
        win: asset_server.load("sfx/win.wav"),
        lose: asset_server.load("sfx/lose.wav"),
    });
    commands.spawn((
        Bgm,
        AudioPlayer::new(asset_server.load("music/changfeng-overture.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(BGM_LEVEL)),
    ));
}

/// 武场只听 Knock：一声动静一个 DESPAWN 实体（19.2 的标准答案）。
/// clack 一份素材三种用场——速度即音高，凳高墙平瓦裂闷
fn play_knocks(mut knocks: MessageReader<Knock>, bank: Res<SoundBank>, mut commands: Commands) {
    for knock in knocks.read() {
        let (source, speed) = match knock {
            Knock::Wall => (&bank.clack, 1.0),
            Knock::Paddle => (&bank.clack, 1.3),
            Knock::Crack => (&bank.clack, 0.6),
            Knock::Shatter => (&bank.shatter, 1.0),
            Knock::Gutter => (&bank.drum, 1.0),
        };
        commands.spawn((
            AudioPlayer::new(source.clone()),
            PlaybackSettings::DESPAWN.with_speed(speed),
        ));
    }
}

/// 闭幕一声定音：满堂彩上行，绣球散尽下行
fn verdict_sting(outcome: Res<Outcome>, bank: Res<SoundBank>, mut commands: Commands) {
    let source = match *outcome {
        Outcome::Cleared => &bank.win,
        Outcome::Spilled => &bank.lose,
    };
    commands.spawn((AudioPlayer::new(source.clone()), PlaybackSettings::DESPAWN));
}

/// 中场协议的声卡侧（19.3 的教训）：戏台钟管不到声卡，sink 的闸自己拧
fn hold_sinks(sinks: Query<&AudioSink>) {
    sinks.iter().for_each(AudioSink::pause);
}

fn release_sinks(sinks: Query<&AudioSink>) {
    sinks.iter().for_each(AudioSink::play);
}

/// 总闸：-/= 拧 GlobalVolume——新开播的声音自动吃到新值
fn master_dial(keyboard: Res<ButtonInput<KeyCode>>, mut master: ResMut<GlobalVolume>) {
    let step = i32::from(keyboard.just_pressed(KeyCode::Equal))
        - i32::from(keyboard.just_pressed(KeyCode::Minus));
    if step != 0 {
        let turned = (master.volume.to_linear() + step as f32 * 0.1).clamp(0.0, 1.0);
        master.volume = Volume::Linear(turned);
        println!("老雷：总闸拧到 {turned:.1}。");
    }
}

/// 19.4 的坑：总闸不管已经在播的——BGM 这种长寿声音得自己补一遍
fn apply_master(master: Res<GlobalVolume>, mut bgm: Query<&mut AudioSink, With<Bgm>>) {
    for mut sink in &mut bgm {
        sink.set_volume(Volume::Linear(BGM_LEVEL) * master.volume);
    }
}
