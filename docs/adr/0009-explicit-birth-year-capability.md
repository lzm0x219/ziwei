# 显式表达真实出生年能力

> 状态：**部分由 [ADR-0011](0011-immutable-natal-model.md) 取代。** 不从生年干支伪造绝对农历年的能力边界仍有效；旧 `Ziwei` getter、按需大限查询和错误类型已由 `NatalContext` 与预存的 `DecadeYear` 取代。

`Natal` 只在 `from_birth` 路径保存绝对农历出生年序号；`from_input` 保留生年干支，但不再用六十甲子合成代表年。两条路径都保存生年干支，只有前者的 `natal.context().year() -> Option<i32>` 与各 `DecadeYear::year()` 为 `Some`；这里的 `year` 均指历法层归一化后的农历年序号。这样调用方不会把推造值当成历史事实。

## Considered Options

- 给 `ZiweiInput` 增加真实年 — 否决：会把原始量捷径退化为 `from_birth` 的重复入口。
- 将 `ZiweiInput` 收窄为仅生年天干 — 否决：生年地支仍是年柱原始事实与后续规则输入；保留它不等于拥有真实历史年。
- 用泛型 typestate 或第二个命盘包装类型区分入口 — 否决：只为两个绝对年份查询引入额外门面，增加常规查询与 binding 的适配成本。
- 保留同一 `Ziwei`，以 `Option` + 专用错误表达能力 — 采用：不扩大命盘查询面，同时让缺失与溢出在接口中可见。
