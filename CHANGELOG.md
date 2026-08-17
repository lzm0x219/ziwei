# 更新日志

本文件记录 Ziwei 面向使用者的显著变化。版本规则见 [`docs/VERSIONING.md`](docs/VERSIONING.md)。

## Unreleased

### Added

- 新增版本、自动发布、标签保护和变更日志规范。

### Changed

- `ZiweiBirth` 与 `ZiweiInput` 直接公开农历 `month`、`day`、`hour` 字段与对应构造参数。
- 本命盘由 `Natal.fromBirth` / `Natal.fromInput` 创建；包入口 `createFromBirth` / `createFromInput` 直接转发这两个方法。
- 只读查询从 `Natal.scope()` / `Natal.decadeScope()` 进入，不再提供独立的 `query()` 入口。
- 宫、星、大限的查询类型分别回到对应领域文件；`Natal` 只保留本命盘装配与立极入口。
- `DecadeScope` 只保留大限事实与年份导航，相对十二宫通过 `chart()` 进入立极坐标。
- 只读查询实现并入 `Natal`、`Palace`、`Star`、`Decade` 所在领域文件。

### Deprecated

### Removed

- 包入口不再导出 `NatalContext`；它只作为本命盘上的只读出生事实。
- `ReframeScope` 不再提供 `palaceLines`、`trineGroups`、`fourCardinalGroups`、`essenceRelations`、`sixHarmonies` 全盘聚合方法；同一关系由 `ScopedPalace` 上的单宫查询按需遍历获得。
- `DecadeYearOrdinal.fromZeroBased` 不再公开；限内年份统一由 `init` 校验构造。

### Fixed

### Security
