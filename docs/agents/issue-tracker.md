# 工作项跟踪：GitHub

本仓库的工作项与规格说明均记录在 GitHub Issues 中。所有相关操作都使用 `gh` CLI。

## 操作约定

- **创建 Issue**：`gh issue create --title "..." --body "..."`。多行正文使用 heredoc。
- **读取 Issue**：`gh issue view <number> --comments`；同时获取标签，并按需使用 `jq` 筛选评论。
- **列出 Issue**：`gh issue list --state open --json number,title,body,labels,comments --jq '[.[] | {number, title, body, labels: [.labels[].name], comments: [.comments[].body]}]'`，并根据任务添加合适的 `--label` 和 `--state` 过滤条件。
- **评论 Issue**：`gh issue comment <number> --body "..."`
- **添加或移除标签**：`gh issue edit <number> --add-label "..."` / `--remove-label "..."`
- **关闭 Issue**：`gh issue close <number> --comment "..."`

通过 `git remote -v` 确定仓库；在仓库克隆目录中运行时，`gh` 会自动完成此解析。

## 将 Pull Request 作为分诊入口

**PRs as a request surface: no.**（如需把外部 PR 视为功能请求，可改为 `yes`；`/triage` 会读取此标记。）

设置为 `yes` 后，PR 使用与 Issue 相同的标签和状态流转，并改用对应的 `gh pr` 命令：

- **读取 PR**：使用 `gh pr view <number> --comments`，并用 `gh pr diff <number>` 查看 diff。
- **列出待分诊的外部 PR**：运行 `gh pr list --state open --json number,title,body,labels,author,authorAssociation,comments`，只保留 `authorAssociation` 为 `CONTRIBUTOR`、`FIRST_TIME_CONTRIBUTOR` 或 `NONE` 的项目，排除 `OWNER`、`MEMBER` 和 `COLLABORATOR`。
- **评论、标记或关闭 PR**：使用 `gh pr comment`、`gh pr edit --add-label`/`--remove-label`、`gh pr close`。

GitHub 的 Issue 与 PR 共用编号空间，因此单独的 `#42` 可能指向任意一种对象。先运行 `gh pr view 42`；若失败，再运行 `gh issue view 42`。

## 技能要求“发布到工作项跟踪系统”时

创建一个 GitHub Issue。

## 技能要求“获取相关工作项”时

运行 `gh issue view <number> --comments`。

## Wayfinding 操作

供 `/wayfinder` 使用。**Map** 是一个独立 Issue，工作项以其**子 Issue** 的形式存在。

- **Map**：使用标签 `wayfinder:map` 的单个 Issue，其正文包含 Notes / Decisions-so-far / Fog。创建命令为 `gh issue create --label wayfinder:map`。
- **子工作项**：通过子 Issue API（使用 `gh api`）把工作项关联为 Map 的 GitHub 子 Issue。未启用子 Issue 时，将子工作项加入 Map 正文的任务列表，并在子工作项正文开头写入 `Part of #<map>`。标签使用 `wayfinder:<type>`，其中类型为 `research`、`prototype`、`grilling` 或 `task`。工作项被认领后，将其分配给负责推进的开发者。
- **阻塞关系**：以 GitHub 原生 Issue 依赖关系作为规范且可在 UI 中查看的表示。使用 `gh api --method POST repos/<owner>/<repo>/issues/<child>/dependencies/blocked_by -F issue_id=<blocker-db-id>` 添加依赖边；`<blocker-db-id>` 必须是阻塞项的数字 **database id**，通过 `gh api repos/<owner>/<repo>/issues/<n> --jq .id` 获取，而不是 `#number` 或 `node_id`。GitHub 的 `issue_dependencies_summary.blocked_by` 只统计尚未关闭的阻塞项，是实时放行条件。依赖关系不可用时，在子工作项正文开头使用 `Blocked by: #<n>, #<n>`。所有阻塞项关闭后，工作项才解除阻塞。
- **前沿查询**：按 Map 中的子 Issue 或任务列表范围，使用 `gh issue list --state open` 列出未关闭的子工作项；排除仍有开放阻塞项（`issue_dependencies_summary.blocked_by > 0`，或 `Blocked by` 行中存在未关闭 Issue）以及已有负责人者，选择 Map 顺序中的第一项。
- **认领**：运行 `gh issue edit <n> --add-assignee @me`，这是会话中的首次写操作。
- **解决**：先运行 `gh issue comment <n> --body "<answer>"`，再运行 `gh issue close <n>`，最后在 Map 的 Decisions-so-far 中追加上下文指针（gist 与链接）。
