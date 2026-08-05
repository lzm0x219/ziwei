# 农历出生输入契约（from_birth）

历法换算（公历↔农历、真太阳时、时区、闰月归属、晚子时/日界）放在引擎外。引擎权威入口是历法层归一化后、打平的 `ZiweiBirth`（`gender`、`year`、`month`、`day`、`hour`）：`year` 是农历年序号，`month` 为 `0..=11`（正月 = 0），`day` 为 `1..=30`（初一 = 1），`hour` 为 `0..=11`（子时 = 0）；不接收闰月标志与年干支字段，也不嵌套 `date` / `NormalizedDate`。年干支由 `(year - 4).rem_euclid(10|12)` 在引擎内推导，以便 `from_birth` 可测且不依赖历法库。`ZiweiInput` / `from_input` 仅作预处理与测例捷径，注入边界见 ADR-0002。

校验：`ZiweiBirth::try_new` 是唯一公开构造入口，负责校验 `year + 124` 可由 `i32` 表示，并校验月/日/时范围；字段私有并通过只读 getter 暴露。`from_birth` 只接收已验证的 `ZiweiBirth`，因此直接返回命盘，不保留不可能发生的错误分支。

## Considered Options

- 传入年干支、引擎不推 — 否决：调用方重复历法职责，且与「只传农历年月日时」心智不一致。
- `ZiweiBirth { gender, date: NormalizedDate }` 嵌套日期 — 否决：多一层无增益，与 `ZiweiInput` 扁平字段风格不一致；字段直接落在 `ZiweiBirth` 上。
- 引擎内处理闰月/晚子时 — 否决：地图已将历法换算划出引擎；消解规则派别多，v1 不背。
