use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

// 为 `SchedulePlugin` 实现 `Plugin` trait
impl Plugin for SchedulePlugin {
    // 在 `build` 方法中，配置更新阶段的系统集，包括 `DespawnEntities`、`UserInput`、`EntityUpdates` 和 `CollisionDetection`
    // 并在更新阶段添加 `apply_deferred` 系统，该系统在 `DespawnEntities` 之后、`UserInput` 之前运行
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                // 执行命令（即运行 `apply_deferred`）
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInput),
        );
    }
}
