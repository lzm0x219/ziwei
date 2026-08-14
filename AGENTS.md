# 仓库指南

## 始终遵守

- **变更范围**：仅修改当前任务所需内容，保留工作树中的其他改动。

## 任务路由

- **Git 提交**：创建提交前，按 [约定式提交 1.0.0](https://www.conventionalcommits.org/zh-hans/v1.0.0/) 编写提交信息：`<type>[optional scope][!]: <description>`。
- **Zig 开发**：修改 Zig 源码或构建配置时，使用 `mise.toml` 固定的工具链；完成前运行 `mise run check`。
- **领域建模**：探索或变更领域概念、术语、历法规则、分析语义或架构前，完整读取 `docs/agents/domain.md`。
- **版本管理**：变更 public API、确定版本号、评估兼容性或更新 `CHANGELOG.md` 前，完整读取 `docs/VERSIONING.md`。
- **发布**：准备、创建、验证、撤回或恢复 GitHub Release 前，完整读取 `docs/RELEASING.md`。
- **工作项跟踪**：读取、创建、更新或关闭 GitHub Issue 前，完整读取 `docs/agents/issue-tracker.md`。
- **分诊**：分诊 Issue 或 Pull Request、维护分诊标签前，完整读取 `docs/agents/triage-labels.md`。
