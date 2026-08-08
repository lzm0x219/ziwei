# 本命 / 大限 / 流年可切换查询 API

> 状态：**由 [ADR-0011](0011-immutable-natal-model.md)、[ADR-0012](0012-query-layers-by-chart-scope.md) 与 [ADR-0013](0013-use-reframe-for-relative-palace-scopes.md) 部分取代。** 本文的 `Ziwei` / `ZiweiView` 查询公开面和流年 scope 计划不再有效；只保留宫位坐标 scope 不复制命盘、不重算星曜、宫干与四化事实的约束。

调用面是一份固定的 `Ziwei` 本命盘，加上轻量 `ZiweiView`（本命 / 第几步大限 / 某农历年流年）。视图只改宫职→地支的叠落；不复制整盘，不重算宫干飞化边、星位、生年四化、来因宫。`Palace::role()` 始终是本命宫职；大限/流年宫职只通过带 `view` 的查询得到，宫干飞化查询则复用叠落地支的本命宫干。结果类型字段私有（ADR-0010）。v1 **不**提供 `with_view` / `ZiweiHandle`：视图相关方法一律在 `Ziwei` 上显式传 `ZiweiView`，避免半成品句柄与第二门面。

## 选择器

```text
ZiweiView::Natal
ZiweiView::Decade(DecadeIndex)    // 0 = 第一大限；构造时校验 0..=11
ZiweiView::Annual { year: i32 }   // 农历年序号，语义同 ZiweiBirth.year
```

## 查询草图（Rust）

```text
// 宫
palace_at(branch) -> &Palace
branch_of_role(role, view) -> Branch
palace_for_role(role, view) -> &Palace

// 星与生年固定信息
stars_at(branch) -> …
laiyin_branch() -> Branch
year_transformations() -> …          // 生年四化，固定

// 大限序列
decade_steps() -> …
decade_step(index: DecadeIndex) -> &DecadeStep

// 宫干飞化（边集固定；view 只影响按宫职索引）
palace_flies() -> &[ZiweiFly; 48]           // 布局：支序 × Transformation::ALL
flies_from_branch(branch) -> &[ZiweiFly; 4] // O(1) 切片
flies_from_role(role, view) -> &[ZiweiFly; 4]
```

## 不变式

1. 切换 `view` 不重建 `Ziwei`，不重算飞边 / 星位 / 生年四化 / 来因。
2. 生年四化与来因宫固定挂在本命盘上；大限/流年不产生额外四化。
3. 构造仍走 `create_from_birth` / `create_from_input`（ADR-0001、0002）；结果一律引擎算。

## 必过场景

1. 本命：来因 + 生年四化 + `palace_at` + 命宫飞边。
2. `Decade(DecadeIndex::try_new(1)?)`：大限「命」叠落地支变化；生年四化不变；按新宫职查询时复用该支本命宫干飞化。
3. `Annual { year }`：流年宫职叠落；不产生额外四化；`palace_flies` 条数仍 ≤48。

## 明确不做

小限、批命文案、每步物化整盘、多跳宫干飞化、流月/流日/流时、`ZiweiHandle` / `with_view` 薄句柄。

## 否决过的做法

- 每步大限/流年生成完整 `Ziwei` 快照 — 否决：与 #249 及「宫干/飞边固定」冲突。
- 用视图改写 `Palace::role()` — 否决：本命宫职与大限/流年宫职会搅在一起。
- 可选 `ZiweiHandle` 只代理部分视图方法 — 否决：零使用、半门面；需要会话式 API 时另开 ADR 做满，不恢复中间态。
