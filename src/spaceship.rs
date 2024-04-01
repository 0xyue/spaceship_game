use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

// 定义飞船的初始位置，这是一个三维向量，初始值为 (0.0, 0.0, -20.0)
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);

// 定义飞船的半径，初始值为 5.0
const SPACESHIP_RADIUS: f32 = 5.0;

// 定义飞船的速度，初始值为 25.0
const SPACESHIP_SPEED: f32 = 25.0;

// 定义飞船的旋转速度，初始值为 2.5
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;

// 定义飞船的翻滚速度，初始值为 2.5
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

// 定义导弹的速度，初始值为 50.0
const MISSILE_SPEED: f32 = 50.0;

// 定义导弹前向生成标量，初始值为 7.5
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;

// 定义导弹的半径，初始值为 1.0
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
    }
}

// 定义一个名为 `spawn_spaceship` 的函数，它接受一个可变的 `Commands` 类型参数和一个 `SceneAssets` 资源引用参数
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // 使用 `commands` 的 `spawn` 方法来创建一个新的实体。
    // 这个新实体拥有 `MovingObjectBundle` 组件和 `Spaceship` 组件。
    // `MovingObjectBundle` 组件包含 `velocity`、`acceleration`、`collider` 和 `model`。
    // `velocity` 和 `acceleration` 被设置为零向量，表示飞船的初始速度和加速度都是零。
    // `collider` 被设置为飞船的半径，用于碰撞检测。
    // `model` 被设置为 `SceneBundle`，其中 `scene` 是从 `scene_assets` 中克隆的飞船模型，`transform` 是飞船的初始位置。
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

// 定义一个名为 `spaceship_movement_controls` 的函数，它接受一个可变的 `Query` 类型参数，一个 `ButtonInput<KeyCode>` 资源引用参数和一个 `Time` 资源引用参数
fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // 从查询中获取飞船的变换和速度
    let (mut transform, mut velocity) = query.single_mut();
    // 初始化旋转、翻滚和移动的变量
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // 如果按下了 "D" 键，那么飞船将向右旋转
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    // 如果按下了 "A" 键，那么飞船将向左旋转
    else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    // 如果按下了 "S" 键，那么飞船将向后移动
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    }
    // 如果按下了 "W" 键，那么飞船将向前移动
    else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    // 如果按下了左 Shift 键，那么飞船将向左翻滚
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    // 如果按下了左 Control 键，那么飞船将向右翻滚
    else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // 根据旋转值，使飞船绕 Y 轴旋转
    transform.rotate_y(rotation);

    // 根据翻滚值，使飞船绕本地 Z 轴旋转
    transform.rotate_local_z(roll);

    // 根据新的方向，更新飞船的速度
    velocity.value = -transform.forward() * movement;
}

// 定义一个名为 `spaceship_weapon_controls` 的函数，它接受一个可变的 `Commands` 类型参数，一个 `Query` 类型参数，一个 `ButtonInput<KeyCode>` 资源引用参数和一个 `SceneAssets` 资源引用参数
fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    // 从查询中获取飞船的变换
    let transform = query.single();
    // 如果按下了空格键
    if keyboard_input.pressed(KeyCode::Space) {
        // 使用 `commands` 的 `spawn` 方法来创建一个新的实体。
        // 这个新实体拥有 `MovingObjectBundle` 组件和 `SpaceshipMissile` 组件。
        // `MovingObjectBundle` 组件包含 `velocity`、`acceleration`、`collider` 和 `model`。
        // `velocity` 被设置为飞船前方的反方向乘以导弹的速度，表示导弹的初始速度。
        // `acceleration` 被设置为零向量，表示导弹的初始加速度是零。
        // `collider` 被设置为导弹的半径，用于碰撞检测。
        // `model` 被设置为 `SceneBundle`，其中 `scene` 是从 `scene_assets` 中克隆的导弹模型，`transform` 是导弹的初始位置，初始位置是在飞船前方一定距离的位置。
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
