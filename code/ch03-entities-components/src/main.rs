use bevy::prelude::*;

// —— 组件定义 ——

/// 生命值；Monster 的 required component，默认 30 点
#[derive(Component)]
struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(30)
    }
}

/// 怪物标记：凡有 Monster，必有 Health
#[derive(Component, Default)]
#[require(Health)]
struct Monster;

/// 石巨人：天生是怪物，且自带 120 点生命
#[derive(Component)]
#[require(Monster, Health(120))]
struct Golem;

/// 玩家标记
#[derive(Component)]
struct Player;

/// 负伤标记
#[derive(Component)]
struct Wounded;

fn main() {
    App::new()
        .add_systems(Startup, spawn_dungeon)
        .add_systems(
            Update,
            (print_roster, sweep, triage, print_survivors).chain(),
        )
        .run();
}

// —— Startup：搭建地下城 ——

fn spawn_dungeon(mut commands: Commands) {
    commands.spawn((Name::new("罗兰"), Player, Health(100)));
    commands.spawn((Name::new("史莱姆"), Monster));
    commands.spawn((Name::new("史莱姆王"), Monster, Health(99)));
    commands.spawn((Name::new("石巨人"), Golem));
    commands.spawn_batch(
        (1..=3).map(|i| (Name::new(format!("蝙蝠 {i} 号")), Monster, Health(10))),
    );
}

// —— Update：一个回合 ——

fn print_roster(creatures: Query<(Entity, &Name, &Health)>) {
    println!("=== 开场清单 ===");
    for (entity, name, health) in &creatures {
        println!("{entity}  {name}  HP {}", health.0);
    }
}

/// 罗兰横扫：每只怪物受 50 点伤害；死者销毁，伤者插上 Wounded
fn sweep(
    mut commands: Commands,
    mut monsters: Query<(Entity, &Name, &mut Health), With<Monster>>,
) {
    for (entity, name, mut health) in &mut monsters {
        health.0 -= 50;
        if health.0 <= 0 {
            println!("{name} 倒下了");
            commands.entity(entity).despawn();
        } else {
            println!("{name} 负伤，剩 {} 点生命", health.0);
            commands.entity(entity).insert(Wounded);
        }
    }
}

/// 牧师治疗：摘除 Wounded 标记
fn triage(mut commands: Commands, wounded: Query<(Entity, &Name), With<Wounded>>) {
    for (entity, name) in &wounded {
        println!("牧师治疗了 {name}");
        commands.entity(entity).remove::<Wounded>();
    }
}

fn print_survivors(survivors: Query<(Entity, &Name, &Health)>) {
    println!("=== 战后清点 ===");
    for (entity, name, health) in &survivors {
        println!("{entity}  {name}  HP {}", health.0);
    }
}
