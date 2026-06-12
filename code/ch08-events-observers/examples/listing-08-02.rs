//! Listing 8-2：事件带数据，observer 带全套系统参数

use bevy::prelude::*;

/// 事件：任务板贴出了新任务
#[derive(Event)]
struct QuestPosted {
    title: &'static str,
    reward: u32,
}

/// 公会金库
#[derive(Resource)]
struct Treasury(u32);

fn main() {
    let mut app = App::new();
    app.insert_resource(Treasury(500))
        .add_observer(announce)
        .add_observer(reserve_reward)
        .add_systems(Update, post_quests);

    println!("—— 第 1 帧 ——");
    app.update();
}

/// 委托人：接连贴出两张任务
fn post_quests(mut commands: Commands) {
    commands.trigger(QuestPosted {
        title: "扫荡地窖鼠群",
        reward: 80,
    });
    commands.trigger(QuestPosted {
        title: "护送商队过山口",
        reward: 200,
    });
}

/// Observer 一：公告员念出任务——On 解引用直达事件字段
fn announce(quest: On<QuestPosted>) {
    println!("公告员：新任务「{}」，赏金 {} 金币！", quest.title, quest.reward);
}

/// Observer 二：账房从金库预扣赏金——Observer 就是系统，ResMut 照用
fn reserve_reward(quest: On<QuestPosted>, mut treasury: ResMut<Treasury>) {
    treasury.0 -= quest.reward;
    println!("账房：预扣 {} 金币，金库还剩 {}。", quest.reward, treasury.0);
}
