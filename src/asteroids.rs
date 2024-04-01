use std::ops::Range;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

// 定义一个常量 `VELOCITY_SCALAR`，表示速度的缩放因子，初始值为 5.0
const VELOCITY_SCALAR: f32 = 5.0;

// 定义一个常量 `ACCELERATION_SCALAR`，表示加速度的缩放因子，初始值为 1.0
const ACCELERATION_SCALAR: f32 = 1.0;

// 定义一个常量 `SPAWN_RANGE_X`，表示在 X 轴上生成实体的范围，范围为 -25.0 到 25.0
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;

// 定义一个常量 `SPAWN_RANGE_Z`，表示在 Z 轴上生成实体的范围，范围为 0.0 到 25.0
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;

// 定义一个常量 `SPAWN_TIME_SECONDS`，表示生成新实体的时间间隔，单位为秒，初始值为 1.0
const SPAWN_TIME_SECONDS: f32 = 1.0;

// 定义一个常量 `ROTATE_SPEED`，表示旋转速度，初始值为 2.5
const ROTATE_SPEED: f32 = 2.5;

// 定义一个常量 `RADIUS`，表示半径，初始值为 2.5
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

// 为 `AsteroidPlugin` 实现 `Plugin` trait
impl Plugin for AsteroidPlugin {
    // 在 `build` 方法中，将 `SpawnTimer` 资源插入到应用中，设置其计时器为每 `SPAWN_TIME_SECONDS` 秒重复一次
    // 并在更新阶段添加 `spawn_asteroid`、`rotate_asteroids` 和 `handle_asteroid_collisions` 系统
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids, handle_asteroid_collisions),
        );
    }
}

// 定义一个名为 `spawn_asteroid` 的函数，它接受四个参数：一个可变的 `Commands` 类型参数、一个可变的 `SpawnTimer` 资源引用、一个 `Time` 资源引用和一个 `SceneAssets` 资源引用
// 这个函数用于生成新的小行星实体
fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    // 更新 `spawn_timer` 的计时器
    spawn_timer.timer.tick(time.delta());
    // 如果计时器没有刚刚结束，那么直接返回，不生成新的小行星
    if !spawn_timer.timer.just_finished() {
        return;
    }

    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();

    // 生成一个随机的位置，位置的 X 和 Z 坐标在 `SPAWN_RANGE_X` 和 `SPAWN_RANGE_Z` 范围内，Y 坐标为 0
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    // 定义一个函数，用于生成一个随机的单位向量
    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    // 生成一个随机的速度，速度的方向为一个随机的单位向量，大小为 `VELOCITY_SCALAR`
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    // 生成一个随机的加速度，加速度的方向为一个随机的单位向量，大小为 `ACCELERATION_SCALAR`
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    // 使用 `commands` 的 `spawn` 方法来创建一个新的小行星实体
    // 这个新实体拥有 `MovingObjectBundle` 组件和 `Asteroid` 组件
    // `MovingObjectBundle` 组件包含一个 `Acceleration`，其值为生成的随机加速度，一个 `Velocity`，其值为生成的随机速度，一个 `Collider`，其半径为 `RADIUS`，和一个 `SceneBundle`，其场景为 `scene_assets.asteroid`，位置为生成的随机位置
    commands.spawn((
        MovingObjectBundle {
            acceleration: Acceleration::new(acceleration),
            velocity: Velocity::new(velocity),
            collider: Collider::new(RADIUS),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

// 这个函数用于旋转所有的小行星
fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    // 对查询结果进行迭代，每次迭代得到一个小行星的变换
    for mut transform in query.iter_mut() {
        // 使用 `Transform` 的 `rotate_local_z` 方法来旋转小行星，旋转的速度为 `ROTATE_SPEED`，旋转的时间为 `time.delta_seconds()`
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}

// 这个函数用于处理小行星的碰撞事件
fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
) {
    // 对查询结果进行迭代，每次迭代得到一个小行星实体和它的碰撞器
    for (entity, collider) in query.iter() {
        // 对碰撞器的 `colliding_entities` 字段进行迭代，每次迭代得到一个与小行星发生碰撞的实体
        for &collided_entity in collider.colliding_entities.iter() {
            // 如果发生碰撞的实体也是一个小行星，那么跳过这次迭代，不处理这次碰撞
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // 如果发生碰撞的实体不是一个小行星，那么销毁这个小行星
            commands.entity(entity).despawn_recursive();
        }
    }
}
