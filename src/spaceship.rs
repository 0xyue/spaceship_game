// 引入 bevy 库的所有预定义类型
use bevy::prelude::*;

// 引入我们自定义的 `Velocity` 结构体
use crate::movement::Velocity;

// 定义初始位置和初始速度的常量
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

// 定义一个名为 `SpaceshipBundle` 的结构体，并为其实现 `Bundle` trait。
// `SpaceshipBundle` 结构体包含两个字段：`velocity` 和 `model`。
#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

// 定义一个名为 `SpaceshipPlugin` 的结构体
pub struct SpaceshipPlugin;

// 为 `SpaceshipPlugin` 结构体实现 `Plugin` trait
impl Plugin for SpaceshipPlugin {
    // 在 `build` 方法中，我们将 `spawn_spaceship` 系统添加到启动阶段
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

// 定义一个名为 `spawn_spaceship` 的函数，它接受一个可变的 `Commands` 类型参数和一个 `Res<AssetServer>` 类型参数
fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 使用 `commands` 的 `spawn` 方法来创建一个新的实体。
    // 这个新实体拥有 `SpaceshipBundle` 组件，其中包含 `Velocity` 和 `SceneBundle`。
    // `Velocity` 组件的值被设置为 `STARTING_VELOCITY`，表示这个实体的初始速度。
    // `SceneBundle` 组件的 `scene` 字段被设置为从 `asset_server` 加载的模型，`transform` 字段被设置为 `STARTING_TRANSLATION`，表示这个实体的初始位置。
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
