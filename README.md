<p align="center">
  <img src=".github/splash.png" alt="Ziwei banner" width="100%" />
</p>

<h1 align="center">Ziwei</h1>

<p align="center">
  为紫微斗数计算、解盘分析与命盘可视化提供清晰分层、可长期维护的开源基础设施。
</p>

<p align="center">
  <img alt="Node.js 22.12+" src="https://img.shields.io/badge/Node.js-22.12%2B-339933?logo=nodedotjs&logoColor=white" />
  <img alt="pnpm 11.7.0" src="https://img.shields.io/badge/pnpm-11.7.0-F69220?logo=pnpm&logoColor=white" />
  <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/github/license/lzm0x219/ziwei" /></a>
</p>

<p align="center">
  <a href="#why-ziwei">Why Ziwei</a> ·
  <a href="#current-status">Current Status</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#commands">Commands</a> ·
  <a href="#documentation">Documentation</a>
</p>

> [!WARNING]
> 仓库目前仍处于 alpha 阶段。当前 `calendar`、`core` 已有基础代码，`chart`、`analysis`、`render` 已补齐骨架，但真实排盘、分析与渲染实现仍未完成，请不要把当前仓库视为完整可用产品。

## Why ziwei

`ziwei` 的目标不是先做一个功能堆叠的应用，而是先把紫微斗数软件最容易分叉、最难回头的基础层做稳：

- 把历法输入、边界规则和时间归一化统一起来。
- 把宫位、星曜、四化等规则沉淀为纯计算模块。
- 用唯一的排盘入口承接后续分析和渲染，避免上层各自拼装命盘。

## Current Status

仓库采用 `pnpm workspace + moonrepo` 的 monorepo 结构，但当前实现仍以底层打底为主。

| Package                                   | 当前状态        | 当前实现                                                   |
| ----------------------------------------- | --------------- | ---------------------------------------------------------- |
| [`@ziweijs/calendar`](packages/calendar/) | ✅ 已建模       | 已定义历法边界配置类型；`normalizeDateTime()` 仍是占位实现 |
| [`@ziweijs/core`](packages/core/)         | ✅ 已有核心代码 | 已有常量表、宫位/天干规则、循环索引工具和对应测试          |
| [`@ziweijs/chart`](packages/chart/)       | 🔲 骨架         | 已有 `BirthInput`、`Chart`、`createChart()` 占位入口       |
| [`@ziweijs/analysis`](packages/analysis/) | 🔲 骨架         | 已有分析结果类型与 `analyzeChart()` 占位入口               |
| [`@ziweijs/render`](packages/render/)     | 🔲 骨架         | 仅有渲染参数接口和 `createZiweiRender()` 占位入口          |
| `apps/*`                                  | 🔲 未创建       | 未来 Web / Desktop UI 层                                   |

当前最重要的事实有两个：

- 真实落地的算法代码主要在 `packages/core/src/`。
- 分层命名和包边界已经补齐，但 `Chart` 的真实内容与上下游实现还没完成。

## Architecture

仓库的目标分层保持严格单向依赖：

```text
calendar → core → chart → [analysis, render] → apps
```

其中：

- `calendar` 负责把不同历法输入归一化为统一时间数据。
- `core` 只做纯计算，不承载渲染、解释或应用逻辑。
- `chart` 是唯一命盘生成入口。
- `analysis` 只解释命盘，不改写命盘。
- `render` 只渲染命盘，不重新计算命盘。
- `apps` 只消费上层公开 API，不直接碰底层内部实现。

当前代码与这套分层的对应关系详见 [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)。

## Commands

```bash
pnpm install
pnpm test
pnpm coverage
pnpm build:chart
pnpm build:analysis
pnpm exec oxlint .
pnpm exec oxfmt --check .
pnpm exec lefthook run pre-commit
```

构建任务当前由 `moon` 编排，入口脚本保留在根 `package.json`：

```bash
pnpm build:chart
pnpm build:analysis
pnpm build:calendar
pnpm build:core
pnpm build:render
```

在具备正常 Git 历史的仓库环境中，也可以直接使用 `pnpm moon run <project>:build`。

## Documentation

- 架构与实现现状：[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- 版本策略与发布约束：[`docs/VERSIONING.md`](docs/VERSIONING.md)
- 协作约束与编码规范：[`AGENTS.md`](AGENTS.md)

## License

MIT
