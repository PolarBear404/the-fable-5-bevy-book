//! Listing 8-8：组件钩子——长在 Weapon 组件上的登记规矩

use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

/// 武器组件：钩子直接声明在组件定义上
#[derive(Component)]
#[component(on_add = register_weapon, on_remove = deregister_weapon)]
struct Weapon {
    name: &'static str,
}

/// 公会的武器登记簿
#[derive(Resource, Default)]
struct Ledger(Vec<&'static str>);

/// 钩子：Weapon 一上身就登记。签名是固定的 fn(DeferredWorld, HookContext)
fn register_weapon(mut world: DeferredWorld, ctx: HookContext) {
    let name = world.get::<Weapon>(ctx.entity).unwrap().name;
    world.resource_mut::<Ledger>().0.push(name);
    println!("  账房（钩子）：{name} 登记入册。");
}

/// 钩子：Weapon 移除（含销毁）时注销
fn deregister_weapon(mut world: DeferredWorld, ctx: HookContext) {
    let name = world.get::<Weapon>(ctx.entity).unwrap().name;
    world.resource_mut::<Ledger>().0.retain(|n| *n != name);
    println!("  账房（钩子）：{name} 销册。");
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Ledger>()
        // 同一对生命周期事件，再各挂一个 observer，看谁先谁后
        .add_observer(|add: On<Add, Weapon>, q: Query<&Weapon>| {
            println!("  巡查员（observer）：看到 {} 入库。", q.get(add.entity).unwrap().name);
        })
        .add_observer(|remove: On<Remove, Weapon>, q: Query<&Weapon>| {
            println!("  巡查员（observer）：看到 {} 出库。", q.get(remove.entity).unwrap().name);
        })
        .add_systems(Update, workshop_script);

    for frame in 1..=2 {
        println!("—— 第 {frame} 帧 ——");
        app.update();
    }
}

fn workshop_script(
    weapons: Query<Entity, With<Weapon>>,
    ledger: Res<Ledger>,
    mut commands: Commands,
    mut frame: Local<u32>,
) {
    *frame += 1;
    match *frame {
        1 => {
            println!("老锤：打好两把武器。");
            commands.spawn(Weapon { name: "铁剑" });
            commands.spawn(Weapon { name: "长戟" });
        }
        2 => {
            println!("老锤：两把都出货了。出货前在册：{:?}", ledger.0);
            for entity in &weapons {
                commands.entity(entity).despawn();
            }
        }
        _ => {}
    }
}
