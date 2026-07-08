//! Listing 28-12：一张皮四种绷法——ImageNode 的 Auto、Stretch、Sliced、Tiled。
//! 同一张 96×96 看板皮：Auto 保持原大，Stretch 硬拉变形，
//! Sliced 九宫格四角不动，Tiled 平铺重复。下排是图集帧与染色。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let board_skin = asset_server.load("ui/panel-board.png");

    // 四种绷法，一字排开。前三块框子一律 240×150，Auto 那块不给尺寸
    let modes: [(&str, NodeImageMode); 3] = [
        ("Stretch", NodeImageMode::Stretch),
        (
            "Sliced",
            // 皮子四边各留 28 像素不许拉伸——跟画皮时的木框宽度对齐
            NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(28.0),
                ..default()
            }),
        ),
        (
            "Tiled",
            // 两个方向都平铺；画幅超过原图 1 倍就开始铺第二张
            NodeImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
        ),
    ];

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(24),
            ..default()
        })
        .with_children(|stage| {
            // 上排：Auto + 三种指定绷法
            stage
                .spawn(Node {
                    column_gap: px(20),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    // Auto：一个尺寸都不写——图多大，节点便多大
                    row.spawn(ImageNode::new(board_skin.clone()));
                    for (_label, mode) in modes {
                        row.spawn((
                            ImageNode::new(board_skin.clone()).with_mode(mode),
                            Node {
                                width: px(240),
                                height: px(150),
                                ..default()
                            },
                        ));
                    }
                });

            // 下排：同一条图集里点帧——球、上釉瓦、素瓦、金瓦
            let layout = layouts.add(TextureAtlasLayout::from_grid(
                UVec2::splat(48),
                4,
                1,
                None,
                None,
            ));
            stage
                .spawn(Node {
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    for index in 0..4 {
                        row.spawn(ImageNode::from_atlas_image(
                            asset_server.load("ui/icons-sheet.png"),
                            TextureAtlas {
                                layout: layout.clone(),
                                index,
                            },
                        ));
                    }
                    // 第五枚：还是金瓦那帧，染成半透明——「还没挣到」的画法
                    row.spawn(
                        ImageNode::from_atlas_image(
                            asset_server.load("ui/icons-sheet.png"),
                            TextureAtlas {
                                layout: layout.clone(),
                                index: 3,
                            },
                        )
                        .with_color(Color::srgba(1.0, 1.0, 1.0, 0.25)),
                    );
                });
        });

    println!("水牌师傅：一张皮，四种绷法；一条图集，五枚小签。");
}
// ANCHOR_END: setup
