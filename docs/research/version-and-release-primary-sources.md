# 版本与发布规范：一手资料研究

> 研究日期：2026-08-14
> 范围：Zig 0.16.0、Semantic Versioning 2.0.0、GitHub Releases。本文只记录上游语义与能力，不替本仓库作最终决策。

## 1. `build.zig.zon` 的版本字段

### `.version`

- Zig 0.16.0 将 `.version` 解析为 `std.SemanticVersion`；该字段缺失或不是合法 SemVer 时，manifest 会产生错误。[Zig 0.16.0 `Manifest.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/Package/Manifest.zig)
- 官方初始化模板称它为包的 Semantic Version，并注明“未来版本将用于 package deduplication”。因此，它表达的是**包版本**，不是 Zig 编译器版本。[Zig 0.16.0 `build.zig.zon` 模板](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/init/build.zig.zon)
- Zig 0.16.0 计算 package hash 时，会把 manifest 中的 `name`、`version`、`fingerprint` 派生 id、筛选后文件内容摘要及大小纳入结果；`.version` 因而已经参与包身份/hash，但不能据此推断 0.16.0 已具备版本范围求解。[Zig 0.16.0 `Fetch.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/Package/Fetch.zig)
- 0.16.0 release notes 说明：`name` 与 `fingerprint` 用于判断两个不同版本是否属于同一项目；同一 `fingerprint`、同一 `version` 却出现不同 `hash`，代表忘记升级版本或发生 hostile fork，计划成为错误。[Zig 0.16.0 Release Notes](https://ziglang.org/download/0.16.0/release-notes.html#Fetch-Packages-Into-Project-Local-Directory)

### `.minimum_zig_version`

- 字段可选，值同样解析为 `std.SemanticVersion`。[Zig 0.16.0 `Manifest.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/Package/Manifest.zig)
- 官方模板给出的精确定义是：记录“该包认为属于受支持使用场景的最早 Zig 版本”。所以它是**最低支持版本声明**，不表示精确锁定某个 Zig 版本，也不表达上界。[Zig 0.16.0 `build.zig.zon` 模板](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/init/build.zig.zon)

## 2. Zig 依赖的 URL、Git tag、commit 与 hash

### Manifest 实际记录什么

- 远程依赖使用 `.url` + `.hash`；本地依赖使用 `.path`，且 `.url` 与 `.path` 互斥。依赖项没有版本范围字段。[Zig 0.16.0 `build.zig.zon` 模板](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/init/build.zig.zon)、[Zig 0.16.0 `Manifest.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/Package/Manifest.zig)
- 官方模板明确把 `.hash` 称为 source of truth：包来自匹配的 hash，`.url` 只是取得该包的一个镜像位置。hash 根据获取并应用包自身 `.paths` 过滤后的文件计算，使用 multihash 格式；URL 内容不匹配声明 hash 时，Zig 拒绝使用该包。[Zig 0.16.0 `build.zig.zon` 模板](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/init/build.zig.zon)、[Zig 0.16.0 `Fetch.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/Package/Fetch.zig)

### Git 与归档 URL

- `zig fetch` 官方 CLI 接受 `git+http` / `git+https` 仓库、tarball 或 git bundle；也可用 `--save` / `--save=<name>` 写入 `build.zig.zon`。[Zig 0.16.0 `main.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/main.zig)
- 对 `git+http[s]` URL，fragment 可写 ref、branch、tag 或 commit；没有 fragment 时取 `HEAD`。实现会尝试原 ref、`refs/heads/<ref>` 与 `refs/tags/<ref>`，然后解析到 commit SHA。[Zig 0.16.0 `Fetch.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/Package/Fetch.zig)
- 默认 `zig fetch --save` 会把 fragment 替换为已解析的 commit SHA；若原来使用 tag/branch，会把原 ref 保存到 query 参数 `ref=`，便于后续检查更新。`--save-exact` 则原样保留输入 URL。[Zig 0.16.0 `main.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/src/main.zig)
- 若 URL 是托管平台的 tag tarball，tag 只是 URL 的组成部分；Zig 仍以下载结果是否匹配 `.hash` 为准。[Zig 0.16.0 `build.zig.zon` 模板](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/init/build.zig.zon)

### 版本关系的边界

从上述 manifest schema 与 fetch 实现可以推得：Zig 0.16.0 的依赖声明是“位置/镜像 + 精确 package hash”，Git tag 主要承担人类可读的上游版本定位；它不会像带版本范围的 registry resolver 那样，根据依赖包 `.version` 自动挑选满足范围的 tag。依赖升级需要取得新的 ref/URL 与相应的新 hash。这个结论是对官方 schema 和实现的归纳，不是 Zig 对未来包管理器行为的承诺。

## 3. SemVer 2.0.0 与 `0.x`

- 使用 SemVer 的软件必须声明 public API；已发布版本的内容不得修改，任何修改必须发布为新版本。[Semantic Versioning 2.0.0，第 1、3 条](https://semver.org/spec/v2.0.0.html#semantic-versioning-specification-semver)
- `0.y.z` 用于初始开发；任何内容都可能随时变化，public API 不应被视为稳定。[Semantic Versioning 2.0.0，第 4 条](https://semver.org/spec/v2.0.0.html#semantic-versioning-specification-semver)
- PATCH、MINOR、MAJOR 的兼容性强制规则显式限定为 `x > 0`。因此，SemVer 2.0.0 **没有规定** `0.x` 的破坏性变更必须递增 MINOR，也没有保证 `0.1.z` 内向后兼容。[Semantic Versioning 2.0.0，第 6–8 条](https://semver.org/spec/v2.0.0.html#semantic-versioning-specification-semver)
- FAQ 只给出简化建议：初始开发可从 `0.1.0` 开始，之后每次发布递增 MINOR；这不是 `0.x` 兼容性保证。[SemVer FAQ](https://semver.org/spec/v2.0.0.html#how-should-i-deal-with-revisions-in-the-0yz-initial-development-phase)
- `1.0.0` 定义 public API。此后，向后兼容 bug fix 递增 PATCH；向后兼容功能或弃用递增 MINOR；向后不兼容 public API 变更递增 MAJOR。[Semantic Versioning 2.0.0，第 5–8 条](https://semver.org/spec/v2.0.0.html#semantic-versioning-specification-semver)
- `v1.2.3` 不是 SemVer 字符串，但可作为常见 Git tag 名；其中的 SemVer 是 `1.2.3`。[SemVer FAQ](https://semver.org/spec/v2.0.0.html#is-v123-a-semantic-version)

## 4. GitHub tag 与 Release 的最小可靠链路

### Tag 与 Release

- GitHub Release 基于 Git tag；tag 指向仓库历史中的具体位置，而 Release 为该 tag 添加说明与可下载资产。[About releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases)
- 创建 Release 时可以选择已有 tag，也可以创建新 tag；CLI 的最小入口是 `gh release create TAG`。[Managing releases in a repository](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)
- 若需要明确控制 tag 指向和签名状态，官方文档能直接支持的链路是：在已验证提交上创建 tag、推送该单个 tag，再让 Release 选择已有 tag。Release 页面直接创建新 tag 的文档没有承诺生成 signed tag。[Signing tags](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-tags)、[Managing releases in a repository](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)

### Signed / Verified tag

GitHub 支持用 GPG、SSH 或 S/MIME 在本地签名 tag；签名能被 GitHub 验证时，tag 显示 `Verified`（vigilant mode 下也可能是 `Partially verified`）。官方基本命令是：

```sh
git tag -s TAG
git tag -v TAG
git push origin TAG
```

来源：[Signing tags](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-tags)、[About commit signature verification](https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification)、[Pushing commits to a remote repository](https://docs.github.com/en/get-started/using-git/pushing-commits-to-a-remote-repository)

### Immutable releases

- 当前 GitHub Docs 将 immutable releases 列为适用于任意 repository type；它默认不开启，可在 repository 或 organization 层启用，且只影响启用后的未来 Release。[Feature availability](https://docs.github.com/en/code-security/concepts/supply-chain-security/supply-chain-security#feature-availability)、[Preventing changes to your releases](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/prevent-release-changes)
- immutable Release 发布后，关联 tag 被锁定到具体 commit 且 Release 存在期间不能删除；assets 不能修改或删除；标题和 release notes 仍可编辑。删除 Release 后可以删除 tag，但该 tag 名不能再次用于新 Release。[Immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- GitHub 建议先创建 draft、上传全部 assets，再发布 immutable Release。发布时会自动生成 release attestation，记录 tag、commit SHA 与 release assets。[Managing releases in a repository](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)、[Immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- 可用 `gh release verify RELEASE-TAG` 验证 Release，用 `gh release verify-asset RELEASE-TAG ARTIFACT-PATH` 验证资产；GitHub 自动生成的源码 ZIP/tarball 无法用 `verify-asset` 验证，因为它们在下载时才生成。[Verifying the integrity of a release](https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/verify-release-integrity)

### 两种完整性保证不能混同

- `Verified` 表示 commit/tag 的签名验证状态。[About commit signature verification](https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification)
- immutable release 的 attestation 覆盖 release tag、commit SHA 与上传的 release assets。[Immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)
- 因此，signed/Verified tag 能证明 tag 身份与指向，但不能单独证明 Release 附件未被替换；附件完整性属于 immutable release / attestation 的范围。

## 5. 仍需由仓库自行决定的事项

这些上游资料没有替本仓库决定：

- 当前公开 API 的定义，以及何时从 `0.x` 进入 `1.0.0`；
- `0.x` 阶段是否额外承诺某种兼容性；
- Git tag 是否使用 `v` 前缀；
- 是否要求 signed/Verified tag；
- 是否启用 immutable releases，以及 Release 是否包含自建资产；
- 由人工、GitHub CLI 还是自动化 workflow 执行发布。
