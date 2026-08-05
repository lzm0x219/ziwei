# 宫干飞化单跳模型与查询面

> 状态：**部分由 [ADR-0011](0011-immutable-natal-model.md) 取代。** 宫位四化关系与自化判定规则仍有效；旧 `ZiweiFly` 存储形态和查询草图不再是当前公开 API。

宫干飞化只算一套：十二宫的本命宫干各自查四化表，最多 48 条单跳边。大限、流年不重排宫干，只改「哪个宫职贴在哪一支」；查的时候先按视图把宫职落到地支，再读那一支宫干已经算好的边。

自化由 `ziwei_core` 在排盘时判定并随宫干飞化边保存：目标落在本宫为出，落在对宫为入，其余为无自化。查询层直接读取标注，不重新判断几何关系。生年四化固定挂在本命盘上；大限、流年只叠加宫职，查询时复用叠落地支的本命宫干飞化，不产生额外四化或边集。多跳串链、三爻变、断象文案都不做。

## 接口草图

```text
// ZiweiFly 字段私有；只读 getter（ADR-0010）
fly.source_branch() / transformation() / target_branch() / star() / self_transformation()
// Out | In | None 在排盘时判定并随边保存

palace_flies() -> &[ZiweiFly; 48]              // 全量，只来自本命宫干
// 布局：Branch::index 升序，每支 4 条，四化序 = Transformation::ALL
flies_from_branch(branch) -> &[ZiweiFly; 4]    // O(1) 切片
flies_from_role(role, view) -> &[ZiweiFly; 4]  // 先贴标再切片
```

## 否决过的做法

- 大限只取大限命所在地支的四条宫干飞化边：不对。大限视图仍使用十二宫的宫干飞化，换的是宫职索引。
- 大限/流年重布十二宫干再飞：和「宫干固定」冲突，地图也排除了。
- 为大限或流年另算四化或飞化边集：和「只有本命宫干作用」冲突。
