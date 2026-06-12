//! Listing 8-5：生命周期事件——附魔台上，装上点火、卸下熄灭

use bevy::prelude::*;

/// 火焰附魔。它只是个普通组件——没有为它写任何 trigger
#[derive(Component)]
struct Flaming;

#[derive(Component)]
struct Weapon {
    name: &'static str,
}

fn main() {
    let mut app = App::new();
    app.add_observer(ignite)
        .add_observer(extinguish)
        .add_systems(Startup, forge)
        .add_systems(Update, enchant_script);

    for frame in 1..=2 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}

fn forge(mut commands: Commands) {
    commands.spawn(Weapon { name: "小芙的长戟" });
}

/// 附魔师的工作日程：第 1 帧装上火焰附魔，第 2 帧卸下来
fn enchant_script(
    weapons: Query<Entity, With<Weapon>>,
    mut commands: Commands,
    mut frame: Local<u32>,
) {
    *frame += 1;
    let weapon = weapons.single().unwrap();
    match *frame {
        1 => {
            println!("附魔师：上火焰附魔——");
            commands.entity(weapon).insert(Flaming);
        }
        2 => {
            println!("附魔师：拆掉附魔——");
            commands.entity(weapon).remove::<Flaming>();
        }
        _ => {}
    }
}

/// Flaming 组件装上的瞬间触发
fn ignite(add: On<Add, Flaming>, weapons: Query<&Weapon>) {
    let weapon = weapons.get(add.entity).unwrap();
    println!("{} 轰地烧了起来！", weapon.name);
}

/// Flaming 组件卸下的瞬间触发
fn extinguish(remove: On<Remove, Flaming>, weapons: Query<&Weapon>) {
    let weapon = weapons.get(remove.entity).unwrap();
    println!("{} 上的火光熄灭了。", weapon.name);
}
