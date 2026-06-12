//! 第 8 章综合示例：公会的一天
//! Weapon 的登记规矩长在组件钩子上；火焰附魔的装卸由生命周期 observer 即时联动；
//! 鉴定走 EntityEvent 指名道姓；收工锣是全局 Event——老板听到就写 AppExit

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use std::time::Duration;

/// 武器：一上身就登记、一离身就销册
#[derive(Component)]
#[component(on_add = register_weapon, on_remove = deregister_weapon)]
struct Weapon {
    name: &'static str,
}

/// 火焰附魔
#[derive(Component)]
struct Flaming;

/// 实体事件：鉴定这件武器
#[derive(EntityEvent)]
struct Identified {
    entity: Entity,
}

/// 全局事件：收工锣
#[derive(Event)]
struct GongStruck;

/// 武器登记簿
#[derive(Resource, Default)]
struct Ledger(Vec<&'static str>);

fn main() {
    App::new()
        // 真正的主循环：每 100 毫秒跑一帧，直到读到 AppExit
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .init_resource::<Ledger>()
        .add_observer(ignite)
        .add_observer(extinguish)
        .add_observer(appraise)
        .add_observer(close_guild)
        .add_systems(Update, (banner, guild_script).chain())
        .run();

    println!("（run() 返回，公会熄灯）");
}

/// 报幕：让"同一帧"在输出里看得见
fn banner(mut frame: Local<u32>) {
    *frame += 1;
    println!("—— 第 {} 帧 ——", *frame);
}

/// 公会一天的日程，每帧推进一格
fn guild_script(
    weapons: Query<(Entity, &Weapon)>,
    ledger: Res<Ledger>,
    mut commands: Commands,
    mut frame: Local<u32>,
) {
    *frame += 1;
    let find = |name: &str| weapons.iter().find(|(_, w)| w.name == name).map(|(e, _)| e);
    match *frame {
        1 => {
            println!("老锤：开炉！打一把长戟、一把铁剑。");
            commands.spawn(Weapon { name: "长戟" });
            commands.spawn(Weapon { name: "铁剑" });
        }
        2 => {
            println!("附魔师：给长戟上火焰附魔。");
            commands.entity(find("长戟").unwrap()).insert(Flaming);
        }
        3 => {
            println!("委托人：这把长戟，麻烦鉴定一下。");
            commands
                .entity(find("长戟").unwrap())
                .trigger(|entity| Identified { entity });
        }
        4 => {
            println!("附魔师：附魔到期，拆下来。");
            commands.entity(find("长戟").unwrap()).remove::<Flaming>();
        }
        5 => {
            println!("老锤：铁剑卖出去了。出货前在册：{:?}", ledger.0);
            commands.entity(find("铁剑").unwrap()).despawn();
        }
        6 => {
            println!("司仪：当——收工锣响。");
            commands.trigger(GongStruck);
        }
        _ => {}
    }
}

/// 钩子：Weapon 上身即登记
fn register_weapon(mut world: DeferredWorld, ctx: HookContext) {
    let name = world.get::<Weapon>(ctx.entity).unwrap().name;
    world.resource_mut::<Ledger>().0.push(name);
    println!("  账房（钩子）：{name} 登记入册。");
}

/// 钩子：Weapon 离身（含销毁）即销册
fn deregister_weapon(mut world: DeferredWorld, ctx: HookContext) {
    let name = world.get::<Weapon>(ctx.entity).unwrap().name;
    world.resource_mut::<Ledger>().0.retain(|n| *n != name);
    println!("  账房（钩子）：{name} 销册。");
}

/// 生命周期 observer：火焰附魔装上即点火
fn ignite(add: On<Add, Flaming>, weapons: Query<&Weapon>) {
    println!("  {} 轰地烧了起来！", weapons.get(add.entity).unwrap().name);
}

/// 生命周期 observer：火焰附魔卸下即熄灭
fn extinguish(remove: On<Remove, Flaming>, weapons: Query<&Weapon>) {
    println!("  {} 上的火光熄灭了。", weapons.get(remove.entity).unwrap().name);
}

/// EntityEvent observer：鉴定指名的那件武器
fn appraise(identified: On<Identified>, weapons: Query<&Weapon>, flames: Query<&Flaming>) {
    let Ok(weapon) = weapons.get(identified.entity) else {
        return;
    };
    let verdict = if flames.get(identified.entity).is_ok() {
        "火焰附魔货真价实"
    } else {
        "并无附魔"
    };
    println!("  鉴定师：{}——{}。", weapon.name, verdict);
}

/// 全局 Event observer：听到收工锣就写 AppExit，事件与消息在此交棒
fn close_guild(_gong: On<GongStruck>, mut exit: MessageWriter<AppExit>) {
    println!("  老板：听到锣了，今天到此为止！");
    exit.write(AppExit::Success);
}
