mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // 插入一个 `ClearColor` 资源，设置清除颜色为 RGB(0.1, 0.0, 0.15)。这是每次清除帧缓冲区时使用的颜色。
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        // 插入一个 `AmbientLight` 资源，设置环境光的颜色为默认颜色，亮度为 750.0。环境光是在整个场景中均匀分布的光源，它影响到场景中所有物体的颜色和亮度。
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
