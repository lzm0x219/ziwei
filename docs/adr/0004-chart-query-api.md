# 本命 / 大限 / 流年可切换查询 API

调用面是一份固定的 `Ziwei` 本命盘，加上轻量 `ZiweiView`（本命 / 第几步大限 / 某农历年流年）。视图只改宫职→地支的贴标，以及大限/流年四化叠加；不复制整盘，不重算飞边、星位、生年四化、来因宫。`Palace.role` 始终是本命宫职；大限/流年宫职只通过带 `view` 的查询得到。v1 **不**提供 `with_view` / `ZiweiHandle`：视图相关方法一律在 `Ziwei` 上显式传 `ZiweiView`，避免半成品句柄与第二门面。

## 选择器

```text
ZiweiView::Natal
ZiweiView::Decade { step: u8 }    // 0 = 第一大限
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

// 大限序列与叠层四化
decade_steps() -> …
overlay_transformations(view) -> …   // Natal 空；Decade/Annual 为该层四化

// 飞宫（边集固定；view 只影响按宫职索引）
palace_flies() -> &[ZiweiFly; 48]           // 布局：支序 × Transformation::ALL
flies_from_branch(branch) -> &[ZiweiFly; 4] // O(1) 切片
flies_from_role(role, view) -> Option<&[ZiweiFly; 4]>  // 大限 step 越界 → None
```

## 不变式

1. 切换 `view` 不重建 `Ziwei`，不重算飞边 / 星位 / 生年四化 / 来因。
2. 生年四化与来因挂在本命上；大限/流年四化只作 overlay，不覆盖生年。
3. 构造仍走 `from_birth` / `from_input`（ADR-0001、0002）；结果一律引擎算。

## 必过场景

1. 本命：来因 + 生年四化 + `palace_at` + 命宫飞边。
2. `Decade { step: 1 }`：大限「命」落支变化；生年四化不变；overlay 为大限四化。
3. `Annual { year }`：流年宫职 + 流年四化叠加；`palace_flies` 条数仍 ≤48。

## 明确不做

小限、批命文案、每步物化整盘、多跳飞宫、流月/流日/流时、`ZiweiHandle` / `with_view` 薄句柄。

## 否决过的做法

- 每步大限/流年生成完整 `Ziwei` 快照 — 否决：与 #249 及「宫干/飞边固定」冲突。
- 用视图改写 `Palace.role` — 否决：本命宫职与大限/流年宫职会搅在一起。
- 可选 `ZiweiHandle` 只代理部分视图方法 — 否决：零使用、半门面；需要会话式 API 时另开 ADR 做满，不恢复中间态。
