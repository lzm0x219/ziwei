import type { ChangelogFunctions } from "@changesets/types";

/**
 * Commit 类型定义
 */
type CommitType = "feat" | "fix" | "perf" | "refactor" | "docs" | "chore" | "test" | "breaking";

/**
 * GitHub 风格中文映射
 */
const TYPE_MAP: Record<CommitType, { title: string; emoji: string }> = {
  feat: { title: "新功能", emoji: "✨" },
  fix: { title: "修复", emoji: "🐛" },
  perf: { title: "性能优化", emoji: "🚀" },
  refactor: { title: "重构", emoji: "♻️" },
  docs: { title: "文档", emoji: "📚" },
  chore: { title: "杂项", emoji: "🔧" },
  test: { title: "测试", emoji: "🧪" },
  breaking: { title: "破坏性变更", emoji: "⚠️" }
};

/**
 * 解析 commit message
 * 支持：
 * feat: xxx (#123)
 */
function parseCommit(message: string) {
  const match = message.match(
    /^(feat|fix|perf|refactor|docs|chore|test|breaking):\s(.+?)(?:\s\(#(\d+)\))?$/
  );

  if (!match) return null;

  return {
    type: match[1] as CommitType,
    text: match[2],
    pr: match[3] ? Number(match[3]) : undefined
  };
}

/**
 * GitHub PR 链接
 */
function getPRLink(repo: string, pr?: number) {
  if (!pr) return "";
  return `([#${pr}](https://github.com/${repo}/pull/${pr}))`;
}

/**
 * Commit 级别 Release Line
 */
export const getReleaseLine: ChangelogFunctions["getReleaseLine"] = async (
  changeset,
  _type,
  options
) => {
  const repo = options?.repo;

  // changeset.summary 是用户在 changeset 里写的内容
  const lines = changeset.summary
    .split("\n")
    .map((l) => l.trim())
    .filter(Boolean);

  // 按 type 分组
  const grouped: Partial<Record<CommitType, string[]>> = {};

  for (const line of lines) {
    const parsed = parseCommit(line);

    const type = parsed?.type ?? "chore";

    if (!grouped[type]) grouped[type] = [];
    grouped[type]!.push(line);
  }

  let output = "";

  // 按固定顺序输出
  const order: CommitType[] = [
    "feat",
    "fix",
    "perf",
    "refactor",
    "breaking",
    "docs",
    "test",
    "chore"
  ];

  for (const type of order) {
    const items = grouped[type];
    if (!items?.length) continue;

    const meta = TYPE_MAP[type];

    output += `### ${meta.emoji} ${meta.title}\n`;

    for (const item of items) {
      const parsed = parseCommit(item);

      if (!parsed) {
        output += `- ${item}\n`;
        continue;
      }

      const prLink = getPRLink(repo, parsed.pr);

      output += `- ${parsed.text} ${prLink}\n`;
    }

    output += `\n`;
  }

  return output.trim();
};

/**
 * 依赖更新 Release Line（Monorepo）
 */
export const getDependencyReleaseLine: ChangelogFunctions["getDependencyReleaseLine"] = async (
  changesets,
  dependenciesUpdated
) => {
  if (!dependenciesUpdated.length) return "";

  let output = `### 📦 依赖更新\n`;

  for (const dep of dependenciesUpdated) {
    output += `- ${dep.name} 更新到 ${dep.newVersion}\n`;
  }

  return output;
};

/**
 * 导出给 Changesets 使用
 */
const changelogFunctions: ChangelogFunctions = {
  getReleaseLine,
  getDependencyReleaseLine
};

export default changelogFunctions;
