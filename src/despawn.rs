use bevy::prelude::*;

use crate::schedule::InGameSet;

// 定义一个常量 `DESPAWN_DISTANCE`，表示实体从原点距离超过这个值时将被销毁，初始值为 100.0
const DESPAWN_DISTANCE: f32 = 100.0;

// 定义一个公共结构体 `DespawnPlugin`
pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_far_away_entities.in_set(InGameSet::DespawnEntities),
        );
    }
}

// 定义一个名为 `despawn_far_away_entities` 的函数，它接受一个可变的 `Commands` 类型参数和一个 `Query` 类型参数
fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    // 对查询结果进行迭代，每次迭代得到一个实体和全局变换的元组
    for (entity, transform) in query.iter() {
        // 计算实体的位置与原点的距离
        let distance = transform.translation().distance(Vec3::ZERO);

        // 如果实体的位置距离原点的距离大于 `DESPAWN_DISTANCE`，则销毁该实体
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}
