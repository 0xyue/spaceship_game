use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
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
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

// 为 `SpaceshipPlugin` 实现 `Plugin` trait
impl Plugin for SpaceshipPlugin {
    // 在 `build` 方法中，配置 `PostStartup` 阶段的系统集，包括 `spawn_spaceship` 系统
    // 并在更新阶段添加 `spaceship_movement_controls`、`spaceship_weapon_controls` 和 `spaceship_shield_controls` 系统，这些系统在 `InGameSet::UserInput` 集合中运行
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (
                spaceship_movement_controls,
                spaceship_weapon_controls,
                spaceship_shield_controls,
            )
                .chain()
                .in_set(InGameSet::UserInput),
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
    // 从查询中获取单个飞船的变换和速度
    // 如果没有找到飞船或者发生其他错误，则直接返回并不执行后续代码
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
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
    let Ok(transform) = query.get_single() else {
        return;
    };
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

// 定义一个名为 `spaceship_shield_controls` 的函数，它接受三个参数：一个可变的 `Commands` 类型参数、一个 `Query` 类型参数和一个 `ButtonInput<KeyCode>` 资源引用
// 这个函数用于处理飞船的护盾控制
fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // 从查询结果中获取单个飞船实体，如果获取失败（例如没有飞船实体），那么直接返回，不进行后续操作
    let spaceship = match query.get_single() {
        Ok(spaceship) => spaceship,
        Err(_) => return,
    };

    // 如果用户按下了 Tab 键，那么给飞船实体添加 `SpaceshipShield` 组件
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}
