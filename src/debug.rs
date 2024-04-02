// 引入 bevy 库的所有预定义类型
use bevy::prelude::*;

use crate::schedule::InGameSet;

// 定义一个名为 `DebugPlugin` 的结构体
pub struct DebugPlugin;

// 为 `DebugPlugin` 结构体实现 `Plugin` trait
impl Plugin for DebugPlugin {
    // 在 `build` 方法中，我们将 `print_position` 系统添加到更新阶段
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    }
}

// 定义一个名为 `print_position` 的函数，它接受一个 `Query` 类型参数
// 这个查询获取所有拥有 `Entity` 和 `Transform` 组件的实体。
fn print_position(query: Query<(Entity, &Transform)>) {
    // 对查询结果进行迭代。每次迭代都会返回一个元组，其中包含一个实体的 `Entity` 和 `Transform` 组件。
    // 打印实体的 ID 和位置信息。
    for (entity, transform) in query.iter() {
        info!("实体 {:?} 的位置是 {:?},", entity, transform.translation);
    }
}
