use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

// 定义一个名为 `spawn_camera` 的函数，它接受一个可变的 `Commands` 类型参数
// 这个函数用于生成一个新的摄像机实体
fn spawn_camera(mut commands: Commands) {
    // 使用 `commands` 的 `spawn` 方法来创建一个新的实体
    // 这个新实体拥有 `Camera3dBundle` 组件和 `MainCamera` 组件
    // `Camera3dBundle` 组件包含一个 `transform`，它的位置被设置为 (0.0, CAMERA_DISTANCE, 0.0)，并且朝向原点，上方向为 Z 轴
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        MainCamera,
    ));
}
