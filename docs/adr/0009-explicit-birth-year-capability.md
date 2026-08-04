# 显式表达真实出生年能力

`Ziwei` 只在 `from_birth` 路径保存真实农历出生年；`from_input` 保留生年干支，但不再用六十甲子合成代表年。两条路径都可查询生年干支，只有前者的 `birth_year() -> Option<i32>` 为 `Some`；`years_in_decade(...) -> Result<_, DecadeYearsError>` 区分出生年缺失与年份溢出。这样 Rust 调用方与 JavaScript 语言绑定（首个纵向切片只暴露 `fromBirth`；见 ADR-0011）都不会把推造值当成历史事实。

## Considered Options

- 给 `ZiweiInput` 增加真实年 — 否决：会把原始量捷径退化为 `from_birth` 的重复入口。
- 将 `ZiweiInput` 收窄为仅生年天干 — 否决：生年地支仍是年柱原始事实与后续规则输入；保留它不等于拥有真实历史年。
- 用泛型 typestate 或第二个命盘包装类型区分入口 — 否决：只为两个绝对年份查询引入额外门面，增加常规查询与 binding 的适配成本。
- 保留同一 `Ziwei`，以 `Option` + 专用错误表达能力 — 采用：不扩大命盘查询面，同时让缺失与溢出在接口中可见。
