MathArts 开源生态的标准驱动紫微斗数排盘引擎

## Status

> [!WARNING]
> 目前正在积极开发中...

## 公开构造接口

- `Ziwei::from_birth(ZiweiBirth) -> Ziwei`：权威路径，保留真实农历出生年。
- `Ziwei::from_input(ZiweiInput) -> Ziwei`：原始量捷径；接收生年干支，但不虚构真实出生年。

`Ziwei::birth_year() -> Option<i32>` 明确表示是否具备真实出生年；
`Ziwei::years_in_decade(...) -> Result<_, DecadeYearsError>` 区分出生年缺失与年份溢出。

完整接口与示例见 [Ziwei Core Reference](docs/guides/ziwei-core-reference.html)。
