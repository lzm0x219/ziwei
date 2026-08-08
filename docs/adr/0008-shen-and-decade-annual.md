# 身宫与大限/流年查询细节

> 状态：**部分由 [ADR-0011](0011-immutable-natal-model.md)、[ADR-0012](0012-query-layers-by-chart-scope.md) 与 [ADR-0013](0013-use-reframe-for-relative-palace-scopes.md) 取代。** 身宫、大限顺序与农历流年公式仍有效；旧 `DecadeStep`、按需 `years_in_decade`、`ZiweiView` 公开面和流年宫位 scope 计划均已失效。

## 身宫

v1 公开身宫。身宫为叠支（落在十二支之一，非第十三宫职）：寅起正月顺月，再顺时至生时（与 research 安命身口诀一致）。引擎必算，禁止注入。

```text
shen_branch() -> Branch
// 叠在哪一本命宫职：对该支做本命宫职反查（或 shen_natal_role()）
```

## 大限序列

```text
// DecadeStep 字段私有；只读 getter（ADR-0010）
step.step()           // DecadeIndex；0 = 第一限
step.ming_branch()    // 大限命所在支
step.age_start()      // 虚岁起 = 局数 + 10*step
step.age_end()        // age_start + 9

decade_steps() -> [DecadeStep; 12]  // 或等价序列
decade_step_for_age(virtual_age: u8) -> Option<DecadeIndex>
// 虚岁由调用方提供（农历年 − 生年 + 1）；core 不收公历 Date
```

顺逆见 ADR-0006；第一限在命宫。

## 选定大限下的十年流年

用户选中某步大限时，必须能列出该限覆盖的**十个流年**（按需生成，不强制预存进 `Ziwei`）：

```text
years_in_decade(index: DecadeIndex) -> Result<[DecadeYear; 10], DecadeYearsError>
// DecadeYear：lunar_year() / virtual_age()；字段私有（ADR-0010）
// virtual_age 从 age_start 到 age_end
// lunar_year = birth_lunar_year + virtual_age - 1
// BirthYearUnavailable：命盘来自 create_from_input，没有真实出生年
// LunarYearOutOfRange：lunar_year 超出 i32
```

绝对流年序号能力只属于 `create_from_birth` 路径（ADR-0009）。成功列表中的 `lunar_year` 可直接作为历史草图中的 `ZiweiView::Annual { year }` 参数。

## 流年宫职

以下方案只保留为历史记录，当前不进入查询 interface：

`ZiweiView::Annual { year }`（`year` 为农历年序号）：

1. 年支 = `(year - 4).rem_euclid(12)` 映射到 `Branch`（与 ADR-0001 年干锚点一致，子=0 序）。
2. **流年命坐该年支（太岁）**。
3. 以流年命支为命，逆布十二流年宫职（与本命/大限同一 `branch_of_role` 机制）。
4. 流年只叠加宫职；查询宫干飞化时，使用该宫职所落地支的本命宫干，不生成额外四化。

## 不做

- core 内公历 `Date` / 时区 / 真太阳时
- 小限
- 把 12 步×10 年流年对象强制物化为 `Ziwei` 必存字段（允许按需 API）
