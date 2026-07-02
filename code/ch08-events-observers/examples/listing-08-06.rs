//! Listing 8-6：五个生命周期事件同台——一把长戟的五幕剧

use bevy::prelude::*;

/// 火焰附魔，这回带上了威力数值
#[derive(Component)]
struct Flaming {
    power: u32,
}

#[derive(Component)]
struct Weapon;

fn main() {
    let mut app = App::new();
    app.add_observer(|add: On<Add, Flaming>, q: Query<&Flaming>| {
        println!("  [Add]     附魔初次上身，威力 {}", q.get(add.entity).unwrap().power);
    })
    .add_observer(|insert: On<Insert, Flaming>, q: Query<&Flaming>| {
        println!("  [Insert]  新值写入完毕，当前威力 {}", q.get(insert.entity).unwrap().power);
    })
    .add_observer(|discard: On<Discard, Flaming>, q: Query<&Flaming>| {
        println!("  [Discard] 旧值即将清退，此刻还能读到威力 {}", q.get(discard.entity).unwrap().power);
    })
    .add_observer(|remove: On<Remove, Flaming>, q: Query<&Flaming>| {
        println!("  [Remove]  附魔彻底离身，临走前威力 {}", q.get(remove.entity).unwrap().power);
    })
    .add_observer(|_despawn: On<Despawn, Flaming>| {
        println!("  [Despawn] 整把武器进了熔炉");
    })
    .add_systems(Startup, forge)
    .add_systems(Update, enchant_script);

    for frame in 1..=5 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}

fn forge(mut commands: Commands) {
    commands.spawn(Weapon);
}

/// 附魔师的五天日程
fn enchant_script(
    weapons: Query<Entity, With<Weapon>>,
    mut commands: Commands,
    mut frame: Local<u32>,
) {
    *frame += 1;
    let Ok(weapon) = weapons.single() else {
        return;
    };
    match *frame {
        1 => {
            println!("附魔师：第一次附魔，威力 3。");
            commands.entity(weapon).insert(Flaming { power: 3 });
        }
        2 => {
            println!("附魔师：重新附魔，威力升到 9。");
            commands.entity(weapon).insert(Flaming { power: 9 });
        }
        3 => {
            println!("附魔师：把附魔拆下来。");
            commands.entity(weapon).remove::<Flaming>();
        }
        4 => {
            println!("附魔师：再附一次，威力 6。");
            commands.entity(weapon).insert(Flaming { power: 6 });
        }
        5 => {
            println!("附魔师：这把回炉重造！");
            commands.entity(weapon).despawn();
        }
        _ => {}
    }
}
