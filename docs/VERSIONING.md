# 版本规范

本文定义 Ziwei Zig 包的版本线、公开应用程序编程接口（public API）边界和兼容性承诺。确定版本号或判断变更是否兼容时，以本文为准。上游语义和平台能力的核对记录见[一手资料研究](research/version-and-release-primary-sources.md)。

## 区分历史版本线

仓库中已有的 `v0.1.*`、`ziwei.js@*` 和 `@ziweijs/*` 标签属于旧 JavaScript 版本线。保留这些标签和 Release，不移动、不删除、不复用，也不将其 API 承诺延续到 Zig 包。

Zig 版本线从 `v1.0.0-alpha.1` 开始。`v1.0.0` 留给 public API 稳定后的正式版本。

## 使用唯一版本来源

`build.zig.zon` 的 `.version` 是唯一版本来源。发布标签在该值前添加 `v`：

```text
build.zig.zon: 1.0.0-alpha.1
Git tag:       v1.0.0-alpha.1
```

仓库不维护 `VERSION` 等重复版本文件。只在发布准备变更中修改 `.version`，发布流程必须拒绝标签与 `.version` 不一致的提交。

保持 `build.zig.zon` 中的 `.name` 和 `.fingerprint` 稳定。包身份确需变更时，先单独评审迁移方案。

## 界定 public API

public API 包括从 `src/root.zig` 可访问的全部 `pub` 声明，以及这些声明已记录的行为。兼容性评审必须覆盖以下内容：

- 导出的类型、函数签名、错误集合和字段。
- 调用结果、失败条件和其他已记录语义。
- `src/root.zig` 导出的子模块及其可访问 `pub` 声明。

未由 `src/root.zig` 导出的文件和声明不作兼容性承诺。源文件路径本身也不属于 public API。

## 推进预发布阶段

预发布阶段表达 API 的成熟度：

| 阶段 | 允许的变更 | 进入下一阶段的条件 |
| --- | --- | --- |
| `alpha.N` | API 和领域模型可以发生破坏性调整 | 核心能力齐备，API 进入冻结期 |
| `beta.N` | 只接受兼容性新增与修复 | 不再有已知兼容性风险 |
| `rc.N` | 只接受发布阻塞问题的修复 | 所有稳定版门禁通过 |
| `1.0.0` | public API 正式稳定 | 后续严格遵循 SemVer |

同一稳定目标按 `alpha.1 → alpha.2 → … → beta.1 → … → rc.1 → … → 1.0.0` 推进。阶段升级时从 `.1` 开始，不回退阶段，不复用已经出现的版本号。每次公开发布都递增编号。

## 遵循稳定版 SemVer

从 `v1.0.0` 起，按 [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) 判断版本增量：

- PATCH 用于向后兼容的缺陷修复。
- MINOR 用于向后兼容的新增能力和弃用声明。
- MAJOR 用于任何破坏 public API 的变更，包括删除已弃用 API。

提高最低 Zig 版本至少发布 MINOR。若工具链升级同时破坏 public API，则发布 MAJOR。

## 声明 Zig 工具链支持

`mise.toml` 固定的 Zig 版本是每个 Release 唯一保证通过的编译器版本。`build.zig.zon.minimum_zig_version` 必须与最低受测版本一致，发布持续集成（CI）也必须使用该固定版本。

Zig 仍处于 `1.0` 之前时，不默认承诺后续编译器版本兼容。支持新版 Zig 前，先在 CI 中验证并更新工具链声明。

## 维护变更日志

`CHANGELOG.md` 使用中文记录面向使用者的变化，代码标识符保留原文。每个影响使用者的拉取请求都应更新 `Unreleased`；纯内部改动可以省略。

按 `Added`、`Changed`、`Deprecated`、`Removed`、`Fixed` 和 `Security` 分类。发布准备变更将条目移入带版本号和日期的章节，GitHub Release Notes 直接复用该章节，不生成提交记录列表。
