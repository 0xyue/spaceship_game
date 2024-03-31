// 引入 bevy 库的所有预定义类型
use bevy::prelude::*;

// 定义一个名为 `Velocity` 的结构体，并为其实现 `Component` 和 `Debug` trait。
// `Velocity` 结构体包含一个字段：`value`，它是 Vec3 类型，表示在游戏世界中的速度。
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

// 定义一个名为 `MovementPlugin` 的结构体
pub struct MovementPlugin;

// 为 `MovementPlugin` 结构体实现 `Plugin` trait
impl Plugin for MovementPlugin {
    // 在 `build` 方法中，我们将 `update_position` 系统添加到更新阶段
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

// 定义一个名为 `update_position` 的函数，它接受一个可变的 `Query` 类型参数和一个 `Res<Time>` 类型参数
// 这个查询获取所有拥有 `Velocity` 和 `Transform` 组件的实体。
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    // 对查询结果进行迭代。每次迭代都会返回一个元组，其中包含一个实体的 `Velocity` 和 `Transform` 组件。
    for (velocity, mut transform) in query.iter_mut() {
        // 更新实体的位置。新的位置是旧的位置加上实体的速度乘以时间的增量。
        // 这样可以确保实体的移动速度不会因为帧率的变化而变化。
        transform.translation += velocity.value * time.delta_seconds();
    }
}
