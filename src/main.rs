// 引入 bevy 库的所有预定义类型
use bevy::prelude::*;

// 定义一个名为 `Position` 的结构体，并为其实现 `Component` 和 `Debug` trait。
// `Position` 结构体包含两个字段：`x` 和 `y`，它们都是 f32 类型，表示在游戏世界中的位置。
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

// 定义一个名为 `Velocity` 的结构体，并为其实现 `Component` 和 `Debug` trait。
// `Velocity` 结构体也包含两个字段：`x` 和 `y`，它们都是 f32 类型，表示在游戏世界中的速度。
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    // 创建一个新的 Bevy 应用
    App::new()
        // 在启动阶段，添加 `spawn_spaceship` 系统。这个系统会在应用启动时运行一次。
        .add_systems(Startup, spawn_spaceship)
        // 在更新阶段，添加 `update_positions` 和 `print_positions` 系统。这些系统会在每个游戏帧中运行。
        .add_systems(Update, (update_positions, print_positions))
        // 添加默认插件。这些插件提供了一些基本的游戏功能，如时间管理和输入处理。
        .add_plugins(DefaultPlugins)
        // 运行应用。这将启动游戏的主循环。
        .run();
}

// 定义一个名为 `spawn_spaceship` 的函数，它接受一个可变的 `Commands` 类型参数。
fn spawn_spaceship(mut commands: Commands) {
    // 使用 `commands` 的 `spawn` 方法来创建一个新的实体。
    // 这个新实体拥有两个组件：`Position` 和 `Velocity`。
    // `Position` 组件的 `x` 和 `y` 值都被初始化为 0.0，表示这个实体在游戏世界中的位置是 (0.0, 0.0)。
    // `Velocity` 组件的 `x` 和 `y` 值被初始化为 1.0，表示这个实体的初始速度是 (1.0, 1.0)（即，它在 x 和 y 方向上都有速度）。
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

// 定义一个名为 `update_positions` 的函数，它接受一个可变的 `Query` 类型参数。
// 这个查询获取所有拥有 `Velocity` 和 `Position` 组件的实体。
fn update_positions(mut query: Query<(&Velocity, &mut Position)>) {
    // 对查询结果进行迭代。每次迭代都会返回一个元组，其中包含一个实体的 `Velocity` 和 `Position` 组件。
    for (velocity, mut position) in query.iter_mut() {
        // 更新实体的位置。新的位置是旧的位置加上实体的速度。
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

// 定义一个名为 `print_positions` 的函数，它接受一个 `Query` 类型参数。
// 这个查询获取所有拥有 `Entity` 和 `Position` 组件的实体。
fn print_positions(query: Query<(Entity, &Position)>) {
    // 对查询结果进行迭代。每次迭代都会返回一个元组，其中包含一个实体的 `Entity` 和 `Position` 组件。
    for (entity, position) in query.iter() {
        // 打印实体的位置信息。
        info!("Entity {:?} is at position {:?}", entity, position);
    }
}
