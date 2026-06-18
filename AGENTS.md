# Repository Guidelines

## 项目简介

**ziwei** — 专业级紫微斗数排盘与分析系统。

目标：为紫微斗数计算、解盘分析、命盘可视化提供开源基础设施。

Monorepo 架构，pnpm workspace + moonrepo 任务编排。

---

## 包结构（单向依赖，禁止反向引用）

```
calendar → core → engine → [analysis, canvas] → apps
```

| 包         | 名称                | 当前状态                           | 说明                                                           |
| ---------- | ------------------- | ---------------------------------- | -------------------------------------------------------------- |
| `calendar` | `@ziweijs/calendar` | ✅ 公共 API 类型已定义             | 历法输入归一化：三类日期输入（公历/农历/干支历）→ 统一时间数据 |
| `core`     | `@ziweijs/core`     | ✅ 常量 + 部分计算逻辑 + 10 个测试 | 星曜、宫位纯计算，零外部依赖                                   |
| `engine`   | `@ziweijs/engine`   | 🔲 仅 spec                         | 唯一 Chart 生成入口，编排 calendar + core                      |
| `analysis` | `@ziweijs/analysis` | 🔲 仅 spec                         | 解盘分析与格局识别                                             |
| `canvas`   | `@ziweijs/canvas`   | 🔲 骨架                            | Canvas 命盘渲染，纯渲染层                                      |
| `apps/`    | —                   | 🔲 未创建                          | Web / Desktop 应用                                             |

### 架构约束

- ❌ 不做过早抽象（无 `shared` / `utils` 层——`core/src/utils/` 是 core 的内部工具，不是全仓库的共享层）
- ❌ `core` 必须纯计算、无外部 npm 依赖
- ❌ `engine` 是唯一排盘入口
- ❌ `analysis` 只负责解释，不修改数据
- ❌ `canvas` 只负责渲染，不包含计算/解释逻辑
- ❌ `apps` 只负责 UI 层，不直接调用 `core` 内部函数

---

## 开发命令

所有命令在仓库根目录执行，使用 pnpm：

```bash
# 安装依赖
pnpm install

# 运行测试（所有包聚合）
pnpm test

# 类型检查
CI=true pnpm run typecheck

# 构建所有包
CI=true pnpm build

# Lint（Oxlint）
pnpm exec oxlint .

# 格式检查（Oxfmt）
pnpm exec oxfmt --check .

# 自动格式化
pnpm exec oxfmt .

# Pre-commit 手动触发
pnpm exec lefthook run pre-commit
```

> 非交互环境建议设置 `CI=true`，避免 pnpm 触发交互式确认。

### Moonrepo 命令

```bash
# 构建特定包
pnpm moon run core:build

# 查看可用任务
pnpm moon project --list

# 查看任务依赖图
pnpm moon run :build --graph
```

---

## TypeScript 配置

当前根 `tsconfig.json` 使用：

- **target**: ES2023
- **lib**: DOM, ES2023
- **module**: ESNext
- **moduleResolution**: Bundler
- **strict**: true（含 `noUncheckedIndexedAccess`、`exactOptionalPropertyTypes`、`isolatedDeclarations`）
- **jsx**: react-jsx（为 canvas 包预留）
- 支持 `.ts` 扩展名导入（`allowImportingTsExtensions`）

包级 `tsconfig.json` 继承根配置，独立设置 `compilerOptions.outDir` 和 `include`。

---

## 编码规范

### 风格

- 2 空格缩进，LF 换行符，UTF-8 编码
- 行尾无空格，文件末尾保留空行
- 使用 Oxlint + Oxfmt 替代 ESLint + Prettier
- Markdown 文件同样受 Oxfmt 格式化

### 命名

- 类型名：`PascalCase`
- 函数/变量：`camelCase`
- 常量：`UPPER_SNAKE_CASE`（枚举映射表/配置常量）
- 文件：`kebab-case.ts`

### 常量 vs 枚举

使用 `const` 对象 + `as const` 断言替代 TypeScript `enum`。原因：与 `isolatedModules` 兼容、支持 tree-shaking、类型推断更安全。

```typescript
// ✅ 正确
export const STEM = {
  Jia: "甲",
  Yi: "乙"
  // ...
} as const;
export type StemKey = keyof typeof STEM;

// ❌ 避免
// export const enum Stem { ... }
```

### 中文 TSDoc

所有公共 API 必须包含中文 TSDoc：

```typescript
/**
 * 计算命宫的索引
 *
 * 寅起正月，顺月逆时为命宫。
 *
 * @param monthIndex - 出生月数的索引（0-11）
 * @param hourIndex - 出生时数的索引（0-11）
 * @returns 命宫的地支索引（0-11）
 */
export function calculateMainPalaceIndex(monthIndex: number, hourIndex: number): number;
```

内部算法在关键公式、边界处理和非显然转换处写中文注释。

### Indexed access

项目使用 `noUncheckedIndexedAccess`，访问数组元素后 TypeScript 会推断为 `T | undefined`。

优先使用数组迭代方法（`reduceRight`、`forEach`、`for...of`）避免手动索引循环。若必须用索引，通过 `$index()` 工具函数做循环取模，用类型守卫（如 `isNumber`）缩小类型。

```typescript
// ✅ 推荐：数组迭代
values.forEach((v, i) => {
  /* i 是 number，v 是 T */
});

// ❌ 避免非空断言
// const val = arr[i]!;
```

### 导出边界

每个包的 `src/index.ts` 是唯一的公共 API 入口。内部实现放入 `src/internal/`，不得从主入口导出，也不得被其他包引用。

---

## 测试指南

- 框架：Vitest 4（根配置 `projects: ["packages/*"]`）
- 测试文件：`packages/<name>/tests/` 目录下，按源文件结构对应
- 导入路径：从 `../../src/<file>.ts` 导入（测试不经过 `index.ts`，直接测模块）
- 禁止在测试文件里使用 `xdescribe`/`xtest`——用 `.skip` 或 `.todo` 替代
- 数值断言使用 `toBe` 或 `toEqual`，不依赖近似匹配（当前测试场景均为精确值）

### 测试文件示例

```typescript
import { describe, expect, test } from "vitest";
import { calculateStems } from "../../src/rules/stem.ts";

describe("calculateStems()", () => {
  test("传入无效的参数应该返回空数组", () => {
    expect(calculateStems(undefined)).toEqual([]);
  });
});
```

---

## 依赖与工具链

### 版本管理

- `pnpm-workspace.yaml` 的 `catalog` 统一管理依赖版本
- 包 `package.json` 中使用 `"catalog:"` 占位引用
- Renovate 自动更新，使用 `dependencies` 标签

### Changesets

所有影响发布的变更必须创建 changeset：

```bash
pnpm changeset
# 选择包 + bump 类型 + 填写摘要
```

### 包构建

- 构建工具：tsdown
- 入口：`src/index.ts`
- 输出：ESM（`.mjs`）+ CJS（`.cjs`）+ TypeScript 声明（`.d.mts`）
- 开发期 `exports` 指向源码，发布期指向 `dist/`

---

## Git 工作流

- 分支策略：GitHub Flow（main + PR）
- 提交信息：Conventional Commits

```
feat(core): 新增紫微星定位算法
fix(calendar): 修正夏令时偏移方向
docs: 更新 API 文档
chore: 升级依赖版本
refactor(core): 拆分安星函数
```

- 初始 commit 已完成（`8992cd5`），后续变更基于此
- 每个 PR 创建一个或多个 changeset

---

## 安全约束

- 核心包不隐式读取文件系统、不发起网络请求、不读系统时区
- 不修改全局可变状态
- 不包含敏感/隐私数据处理逻辑
- 天文数据（节气表/历法表）应该为编译时常量，非运行时下载
