// 引入 bevy 库的所有预定义类型
use bevy::prelude::*;

// 定义相机距离的常量，值为 80.0
const CAMERA_DISTANCE: f32 = 80.0;

// 定义一个名为 `CameraPlugin` 的结构体
pub struct CameraPlugin;

// 为 `CameraPlugin` 结构体实现 `Plugin` trait
impl Plugin for CameraPlugin {
    // 在 `build` 方法中，我们将 `spawn_camera` 系统添加到启动阶段
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

// 定义一个名为 `spawn_camera` 的函数，它接受一个可变的 `Commands` 类型参数
fn spawn_camera(mut commands: Commands) {
    // 使用 `commands` 的 `spawn` 方法来创建一个新的实体。
    // 这个新实体拥有 `Camera3dBundle` 组件，其中包含 `transform` 和其他默认组件。
    // `transform` 组件的值被设置为一个新的 `Transform` 对象，它的位置被设置为 (0.0, CAMERA_DISTANCE, 0.0)，并且它正在看向 (0.0, 0.0, 0.0) 的位置，上方向为 Z 轴。
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
