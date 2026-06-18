# ziwei 版本策略

这份文档只描述仓库当前真实采用的版本规则，不描述尚未落地的“产品版本体系”。

## 1. 当前策略概览

`ziwei` 现在使用的是：

- Changesets 维护版本变更
- SemVer 表达包版本
- `@ziweijs/*` 作为一个 linked group 统一版本线
- alpha 预发布阶段优先，先保证边界稳定，再谈长期兼容承诺

当前配置见 [`.changeset/config.json`](../.changeset/config.json)。

## 2. 为什么是 linked versioning

仓库虽然是多包结构，但目前还处在底层架构收敛阶段：

- `calendar`、`core`、`chart`、`analysis`、`render` 的边界仍在形成中。
- 很多公开 API 还没真正冻结。
- 一处底层结构变动，通常会连带影响整个调用链。

因此，当前没有采用“完全独立版本线”，而是把 `@ziweijs/*` 放在同一个 linked group 中统一演进。这样做更适合早期阶段，也更符合当前仓库真实状态。

## 3. 当前配置事实

根据 [`.changeset/config.json`](../.changeset/config.json)，仓库当前具有以下发布约束：

- `baseBranch` 是 `main`
- 包发布权限是 `public`
- `@ziweijs/*` 被声明为 linked group
- 内部依赖更新策略为 `patch`
- `changeset` 不自动创建版本提交

这意味着版本推进仍然以人工审查和合并流程为主，而不是把版本管理完全交给自动化。

## 4. 版本号规则

包版本遵循 SemVer：

```text
MAJOR.MINOR.PATCH[-PRERELEASE]
```

在当前阶段，最常见的是带预发布标签的版本，例如：

```text
1.0.0-alpha.0
1.0.0-alpha.1
1.0.0-beta.0
```

## 5. bump 规则

### 5.1 PATCH

用于不改变公开行为的修复，例如：

- 修正规则实现错误
- 修复类型声明错误
- 修复构建、测试、发布配置
- 不改变 API 含义的内部重构

### 5.2 MINOR

用于向后兼容的新能力，例如：

- 新增公开函数
- 新增稳定导出的类型
- 新增渲染能力或分析能力
- 在不破坏旧调用方式的前提下扩展 `Chart` 字段

### 5.3 MAJOR

用于破坏性变更，例如：

- 修改公开函数签名
- 删除公开导出
- 改变 `Chart` 的核心结构或字段语义
- 改变调用方必须遵守的输入 / 输出协议

## 6. 预发布阶段规则

仓库当前仍处于 alpha 阶段，因此版本解读要更保守：

- 即使版本号在增长，也不代表 API 已冻结。
- 文档必须明确区分“已实现”和“规划中”。
- 只有真正进入公共 API 的内容，才值得做兼容性承诺。

简化来说，alpha 阶段的重点不是“快发版”，而是“先把边界写对”。

## 7. Changeset 使用约定

凡是会影响发布包的变更，都应该创建 changeset：

```bash
pnpm changeset
```

changeset 摘要建议直接写成可读的变更说明，而不是重复文件列表。仓库已经为自定义 changelog 生成逻辑预留了文件位置，后续发布记录应尽量保持对人友好。

## 8. 典型发布流程

本仓库当前最稳妥的发布流程如下：

1. 完成功能或修复。
2. 运行测试、构建和格式检查。
3. 为影响发布的改动创建 changeset。
4. 合并到 `main`。
5. 执行 `pnpm changeset version` 更新版本与 changelog。
6. 再执行发布流程。

建议在发布前至少运行：

```bash
pnpm test
pnpm coverage
pnpm exec oxlint .
pnpm exec oxfmt --check .
```

如需构建发布包，使用根 `package.json` 中的 `build:*` 脚本，或在具备正常 Git 历史的环境里执行对应的 `moon` 构建任务。

## 9. 现在不做什么

为了让版本文档保持可信，当前明确不写下面这些内容：

- 不额外引入“产品版本”和“包版本”两套体系。
- 不假定 `chart`、`analysis`、`apps` 已经具备独立发布节奏。
- 不提前定义远期 `v2`、`v3` 的发布承诺。

等 `Chart` 模型、`chart` 包和应用层真正成形后，再决定是否需要引入仓库级 release 版本概念。
