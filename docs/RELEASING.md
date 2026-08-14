# 发布规范

本文规定 Ziwei Zig 包从发布准备到公开验证的完整流程。执行发布、恢复失败发布或撤回版本时，按顺序完成每个门禁。

## 发布前提

开始发布前，确认以下条件：

- 目标提交已进入 `main`，且该提交的 `Zig CI` 成功。
- 发布工作在干净的检出中进行；当前工作树有其他改动时，使用独立 worktree。
- 本地 Git 已配置可被 GitHub 验证的 GPG、SSH 或 S/MIME 签名。
- 仓库已启用 Immutable Releases，并为 `v*` 启用两条标签保护规则。
- 发布者具有推送受保护标签和创建 Release 的权限。

## 准备发布变更

发布准备通过单独的拉取请求完成：

1. 按 [`VERSIONING.md`](VERSIONING.md) 选择下一个未使用的版本。
2. 更新 `build.zig.zon` 的 `.version`。
3. 将 `CHANGELOG.md` 的待发布条目移入对应版本章节，并填写发布日期。
4. 保留新的空 `Unreleased` 分类，供后续变更使用。
5. 运行所有发布质量门禁。
6. 合并拉取请求，等待合并提交的 `Zig CI` 成功。

发布质量门禁包括：

```sh
mise exec -- zig fmt --check build.zig build.zig.zon src
mise exec -- zig build
mise exec -- zig build test
```

## 创建签名标签

标签必须是 signed annotated tag，并指向已通过 CI 的 `main` 提交。先同步一个干净的检出，再从 `build.zig.zon` 读取本次发布版本：

```sh
git switch main
git pull --ff-only
test -z "$(git status --short)"

release_version="$(awk -F '"' '/^[[:space:]]*\.version = / { print $2; exit }' build.zig.zon)"
release_tag="v${release_version}"
release_commit="$(git rev-parse HEAD)"
test -n "$release_version"
test "$release_commit" = "$(git rev-parse origin/main)"
test -z "$(git ls-remote --tags origin "refs/tags/$release_tag")"
gh run list --workflow ci.yml --commit "$release_commit" --status success --limit 1
```

`gh run list` 必须列出目标提交的成功记录。在推送前核对 `.version`、变更日志和提交 SHA；检查全部通过后，创建、验证并单独推送标签：

```sh
git tag -s "$release_tag" -m "Release $release_tag" "$release_commit"
git tag -v "$release_tag"
git push origin "$release_tag"
```

标签一旦推送，版本号立即占用。发布者不得移动、删除或复用该标签。

## 自动发布 Release

推送 `v*` 标签后，`.github/workflows/release.yml` 负责发布。Workflow 必须在创建 Release 前完成以下检查：

- 标签符合 SemVer，并与 `build.zig.zon.version` 完全一致。
- 标签指向 `main` 上的提交，且签名在 GitHub 显示 `Verified`。
- `CHANGELOG.md` 包含对应版本和发布日期。
- `mise.toml` 与 `minimum_zig_version` 一致。
- 格式检查、构建和测试全部通过。
- 发布源码可以计算并复验 Zig package hash。

Workflow 先创建 draft，再写入该版本的变更日志和依赖示例，最后发布。带预发布标识的版本必须标记为 Pre-release；只有稳定版可以标记为 Latest。

当前项目只发布源码，不上传二进制或重复的源码附件。Release Notes 使用 GitHub 源码归档 URL，并提供包含精确 `.hash` 的 `build.zig.zon` 依赖示例。

任一检查失败时，Workflow 必须停在公开 Release 之前。

## 验证公开结果

Workflow 成功后，发布者仍需验证以下结果：

1. 标签指向预期提交，并在 GitHub 显示 `Verified`。
2. Release 不是 draft，预发布和 Latest 状态符合版本类型。
3. Release Notes 与 `CHANGELOG.md` 对应章节一致。
4. 依赖示例能通过固定版本的 Zig 获取并校验源码。
5. GitHub 将 Release 标记为 `Immutable`。

使用 GitHub CLI 验证 Release 完整性：

```sh
gh release view "$release_tag"
gh release verify "$release_tag"
```

GitHub 自动生成的源码归档不能使用 `gh release verify-asset`。Zig 通过 Release Notes 中的 package hash 校验解包后的包内容。

## 恢复失败发布

根据失败发生的时间处理：

- 标签推送前发现问题：修正发布准备变更，版本号不算占用。
- 标签推送后发生临时 Workflow 故障：修复流程后，对同一标签重新运行 Workflow。
- 标签内容或版本有误：保留原标签，将对应 Release 标记为“已撤回”，再发布下一个版本。
- Release 已公开：保留原内容，通过新版本发布修复。

只有法律、安全事件或凭据泄露等紧急情况允许删除 Release。即使删除，原标签名也不得复用。

## 保护发布标签

GitHub 使用两条匹配 `v*` 的 tag ruleset。`release-tag-creation` 只允许发布负责人创建标签；`immutable-release-tags` 对所有人禁止更新和删除，不设置 bypass。拆分规则可以让发布负责人创建新标签，同时保留标签不可改写的硬边界。

Workflow 仍需独立验证标签签名，ruleset 不能替代 `Verified` 检查。

现有旧 JavaScript `v0.1.*` 标签也受该规则保护，但不属于 Zig 版本线。
