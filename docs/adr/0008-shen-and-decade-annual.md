# 身宫与大限/流年查询细节

## 身宫

v1 公开身宫。身宫为叠支（落在十二支之一，非第十三宫职）：寅起正月顺月，再顺时至生时（与 research 安命身口诀一致）。引擎必算，禁止注入。

```text
shen_branch() -> Branch
// 叠在哪一本命宫职：对该支做本命宫职反查（或 shen_natal_role()）
```

## 大限序列

```text
DecadeStep {
  step: u8,              // 0 = 第一限
  ming_branch: Branch,   // 大限命所在支
  age_start: u8,         // 虚岁起 = 局数 + 10*step
  age_end: u8,           // age_start + 9
  stem: Stem,            // 该支本命宫干
}

decade_steps() -> [DecadeStep; 12]  // 或等价序列
decade_step_for_age(virtual_age: u8) -> Option<u8>
// 虚岁由调用方提供（农历年 − 生年 + 1）；core 不收公历 Date
```

顺逆见 ADR-0006；第一限在命宫。

## 选定大限下的十年流年

用户选中某步大限时，必须能列出该限覆盖的**十个流年**（按需生成，不强制预存进 `Ziwei`）：

```text
years_in_decade(step) -> [(lunar_year, virtual_age); 10]
// virtual_age 从 age_start 到 age_end
// lunar_year = birth_lunar_year + virtual_age - 1
```

与 `ZiweiView::Annual { year }` 衔接：列表中的 `lunar_year` 可直接作为 Annual 视图参数。

## 流年宫职

`ZiweiView::Annual { year }`（`year` 为农历年序号）：

1. 年支 = `(year - 4).rem_euclid(12)` 映射到 `Branch`（与 ADR-0001 年干锚点一致，子=0 序）。
2. **流年命坐该年支（太岁）**。
3. 以流年命支为命，逆布十二流年宫职（与本命/大限同一 `branch_of_role` 机制）。
4. 流年四化 = 年干 overlay（ADR-0004），不覆盖生年四化。

## 不做

- core 内公历 `Date` / 时区 / 真太阳时
- 小限
- 把 12 步×10 年流年对象强制物化为 `Ziwei` 必存字段（允许按需 API）
