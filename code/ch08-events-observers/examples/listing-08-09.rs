//! Listing 8-9：一个组件、同种钩子只能有一个

use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

/// 组件定义里已经声明了 on_add 钩子
#[derive(Component)]
#[component(on_add = register_weapon)]
struct Weapon;

fn register_weapon(_world: DeferredWorld, _ctx: HookContext) {
    println!("账房：登记入册。");
}

fn second_hook(_world: DeferredWorld, _ctx: HookContext) {
    println!("二账房：我也想记一笔。");
}

fn main() {
    let mut app = App::new();
    // 试图给 Weapon 再挂第二个 on_add 钩子
    app.world_mut()
        .register_component_hooks::<Weapon>()
        .on_add(second_hook);
}
