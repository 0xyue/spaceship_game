use bevy::prelude::*;

use crate::collision_detection::Collider;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
// 定义一个名为 `MovingObjectBundle` 的公共结构体，它包含四个公共字段：
// `velocity`：一个 `Velocity` 类型的字段，表示移动对象的速度。
// `acceleration`：一个 `Acceleration` 类型的字段，表示移动对象的加速度。
// `collider`：一个 `Collider` 类型的字段，用于处理移动对象的碰撞检测。
// `model`：一个 `SceneBundle` 类型的字段，用于存储移动对象的模型数据。
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position));
    }
}

// 定义一个名为 `update_velocity` 的函数，它接受一个可变的 `Query` 类型参数和一个 `Time` 资源引用参数
fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    // 对查询结果进行迭代，每次迭代得到一个加速度和速度的元组
    for (acceleration, mut velocity) in query.iter_mut() {
        // 更新速度值，新的速度值等于原速度值加上加速度值乘以时间的增量
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

// 定义一个名为 `update_position` 的函数，它接受一个可变的 `Query` 类型参数和一个 `Time` 资源引用参数
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    // 对查询结果进行迭代，每次迭代得到一个速度和变换的元组
    for (velocity, mut transform) in query.iter_mut() {
        // 更新变换的平移部分，新的平移值等于原平移值加上速度值乘以时间的增量
        transform.translation += velocity.value * time.delta_seconds();
    }
}
