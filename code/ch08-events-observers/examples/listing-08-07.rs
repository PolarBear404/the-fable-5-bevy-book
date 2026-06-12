//! Listing 8-7：连锁联动——一次 insert，一串反应，全在同一帧

use bevy::prelude::*;

#[derive(Component)]
struct Flaming;

/// 火光：由 observer 自动补挂的"派生组件"
#[derive(Component)]
struct Glowing;

#[derive(Component)]
struct Weapon {
    name: &'static str,
}

fn main() {
    let mut app = App::new();
    app.add_observer(attach_glow)
        .add_observer(report_glow)
        .add_systems(Startup, forge)
        .add_systems(Update, (enchant, end_of_frame).chain());

    println!("—— 第 1 帧 ——");
    app.update();
}

fn forge(mut commands: Commands) {
    commands.spawn(Weapon { name: "小芙的长戟" });
}

fn enchant(weapons: Query<Entity, With<Weapon>>, mut commands: Commands, mut done: Local<bool>) {
    if *done {
        return;
    }
    *done = true;
    println!("附魔师：上火焰附魔——");
    commands.entity(weapons.single().unwrap()).insert(Flaming);
}

/// 第一环：火焰附魔上身，立刻补挂一个"火光"组件
fn attach_glow(add: On<Add, Flaming>, mut commands: Commands) {
    println!("  第一环：检测到火焰附魔，给它配上火光。");
    commands.entity(add.entity).insert(Glowing);
}

/// 第二环：火光出现，向全场报告
fn report_glow(add: On<Add, Glowing>, weapons: Query<&Weapon>) {
    let weapon = weapons.get(add.entity).unwrap();
    println!("  第二环：{} 亮起来了！", weapon.name);
}

/// 排在附魔师后面的普通系统：它上场时连锁已全部结束
fn end_of_frame(glowing: Query<&Weapon, With<Glowing>>) {
    if let Ok(weapon) = glowing.single() {
        println!("巡场员：本帧收工时，{} 已经在发光了。", weapon.name);
    }
}
