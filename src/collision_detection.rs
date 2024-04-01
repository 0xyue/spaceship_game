use bevy::{prelude::*, utils::HashMap};

// `Collider` 结构体用于处理碰撞检测
#[derive(Component, Debug)]
pub struct Collider {
    // `radius` 字段表示碰撞器的半径
    pub radius: f32,
    // `colliding_entities` 字段是一个 `Entity` 类型的向量，用于存储与当前实体发生碰撞的其他实体
    pub colliding_entities: Vec<Entity>,
}

// 为 `Collider` 结构体实现方法
impl Collider {
    // 定义一个名为 `new` 的关联函数，它接受一个 `f32` 类型的参数 `radius`，并返回一个新的 `Collider` 实例
    // `new` 函数用于创建一个新的 `Collider` 实例
    pub fn new(radius: f32) -> Self {
        // 返回一个新的 `Collider` 实例，其中 `radius` 字段的值为传入的 `radius` 参数，`colliding_entities` 字段的值为一个空向量
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

// 这个函数用于检测碰撞
fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    // 创建一个空的 HashMap，用于存储发生碰撞的实体
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // 第一阶段：检测碰撞
    // 对查询结果进行迭代，每次迭代得到一个实体、全局变换和碰撞器的元组
    for (entity_a, transform_a, collider_a) in query.iter() {
        // 再次对查询结果进行迭代，每次迭代得到另一个实体、全局变换和碰撞器的元组
        for (entity_b, transform_b, collider_b) in query.iter() {
            // 如果两个实体不是同一个实体
            if entity_a != entity_b {
                // 计算两个实体的位置之间的距离
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                // 如果距离小于两个碰撞器的半径之和，那么这两个实体发生了碰撞
                if distance < collider_a.radius + collider_b.radius {
                    // 在 HashMap 中为 entity_a 添加一个与其发生碰撞的实体 entity_b
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // 第二阶段：更新碰撞器
    // 对查询结果进行迭代，每次迭代得到一个实体、全局变换和碰撞器的元组
    for (entity, _, mut collider) in query.iter_mut() {
        // 清空碰撞器的 `colliding_entities` 字段
        collider.colliding_entities.clear();
        // 如果在 HashMap 中找到了与当前实体发生碰撞的实体
        if let Some(collisions) = colliding_entities.get(&entity) {
            // 将这些实体添加到碰撞器的 `colliding_entities` 字段中
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}
