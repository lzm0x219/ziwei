# 基准测试规范

本文定义 Ziwei Zig 项目的本命盘构建基准、统计口径与报告产物。基准当前只覆盖本命盘构建，不包含只读查询。

## 运行

使用仓库固定的 Zig `0.16.0`：

```bash
mise run benchmark
```

也可以直接传递基准参数：

```bash
zig build benchmark -- --warmups 5 --samples 50 --target-ms 50
```

快速验证基准程序及报告链路：

```bash
zig build benchmark -- --quick
```

`--quick` 生成的报告标记为 `smoke-only`，只证明基准程序和报告产物可用，不参与性能比较。

使用一份完整运行的 `results.json` 作为命名 baseline：

```bash
zig build benchmark
zig build benchmark -- \
  --baseline main benchmark-results/run-<base-run-id>/results.json
```

程序会从 Git 自动记录完整提交哈希、工作树是否干净，并生成 `<commit 前 12 位>` 或 `<commit 前 12 位>-dirty` 版本标识；不接受手工填写 revision。baseline 比较要求两轮均为 `full`，baseline 结论为稳定的 `no-baseline` 或 `pass`，并具有完全相同的环境指纹、schema、采样配置、fixture 和 case 合同。环境指纹包括 runner、CPU 型号、OS、target、Zig、backend、优化模式和逻辑核心数。比较结果只写入报告，不改变进程退出码；在本地固定基准机完成噪声校准前不作为硬门禁。

`benchmark` 构建步骤默认独立使用 `ReleaseFast`，不会改变普通构建和测试的优化模式。需要验证其他模式时显式传入：

```bash
zig build benchmark -Dbenchmark-optimize=ReleaseSafe
```

## 测量范围

| 基准 | 每次迭代的操作数 | 目的 |
| --- | ---: | --- |
| `natal/create_from_input/single` | 1 | 测量预处理输入公开入口构建单张命盘的直接延迟 |
| `natal/create_from_birth/single` | 1 | 测量含绝对农历年序号入口构建单张命盘的直接延迟 |
| `natal/create_from_input/sexagenary_cycle` | 60 | 覆盖完整干支周期，并轮换性别、月、日、时辰 |
| `natal/create_from_birth/sexagenary_cycle` | 60 | 覆盖含绝对农历年序号的公开构建入口 |
| `natal/create_from_input/exhaustive_valid_space` | 518,400 | 穷举两种性别、60 个合法年柱、12 月、30 日、12 时辰 |

夹具准备、统计和文件写出不计入测量。每个构建结果都传给 `std.mem.doNotOptimizeAway`，避免编译器消除被测工作。全量合法输入负载单次迭代已经足够长，因此最多收集 10 个样本，避免一次常规基准占用过长时间。

## 统计口径

每个普通基准先校准单个样本的迭代数，再执行 60 次 warm-up 和 100 个正式样本；默认目标样本时长为 50 ms，对应约 3 秒预热和至少约 5 秒正式测量。不同 case 的重复测量按报告中记录的 seed 随机交错，降低温度、调度和后台负载随时间变化造成的顺序偏差。报告同时保留：

- 最小值、Q1、P50、Q3、P90、P95、P99、最大值；
- 均值、样本标准差和相对标准差（RSD）；
- 中位绝对偏差（MAD）；
- `mean ± 1.96 × standard error` 的 95% 均值区间；
- 与 baseline 比较时，使用 1,000 次确定性 bootstrap 计算 P50 相对变化的 95% 区间；
- 按 `1.5 × IQR` 识别的异常值数量；
- 由 P50 换算的每秒操作数；
- `@sizeOf(Natal)`，用于跟踪固定值类型的数据体积。

不要用单次最小值宣称性能提升。比较前应固定 Zig 版本、优化模式、目标架构、机器、电源模式和后台负载；跨机器结果只适合观察数量级，不适合作为回归结论。

在尚未配置权威 baseline 时，报告结论只用于诊断本轮噪声：RSD 超过 5% 或异常值占比超过 10% 标为 `no-baseline-noise-warning`；RSD 超过 10% 或严重异常值占比超过 10% 标为 `unstable`；其余为 `no-baseline`。这些是校准前的保守起点，不是性能回归门禁，也不替代在本地固定基准机上收集 20–30 次运行后确定的 per-case 阈值。

配置 baseline 后，以 P50 相对变化为主：95% bootstrap 区间下界仍超过 `+5%` 的 case 标为 `regression`，否则标为 `pass`。本轮噪声超过阈值时，整体结论优先标为 `noise-warning` 或 `unstable`，不让不稳定测量产生性能结论。`5%` 只是校准前的报告阈值，不是跨项目行业标准。

每份 `report.md` 顶部固定生成“大白话总结”，直接回答一张命盘大约要算多久、单线程每秒大约能算多少张、哪个场景最快或最慢、两个单张入口相差多少，以及全量合法输入折算后的速度。存在旧结果时会指出变化最大的一项；测量不稳定时会指出具体是哪一项、测了多少次、波动多大、出现几次明显跑偏。`P50`、`RSD`、bootstrap 等统计术语仍保留在后面的技术明细中，不要求只看总结的读者先理解它们。

## 每次运行的报告

每次运行都会创建新的 `benchmark-results/run-<unix-nanoseconds>/`，其中包含：

| 文件 | 内容 |
| --- | --- |
| `report.md` | 大白话总结、运行元数据、统计表、baseline 比较与图表入口 |
| `latency.svg` | P50、P95 与均值 95% 区间图 |
| `distribution.svg` | 四分位箱体、P95、范围与异常值图 |
| `variability.svg` | RSD 波动图 |
| `change.svg` | 有 baseline 时生成的 P50 相对变化、95% bootstrap 区间与 `+5%` 阈值图 |
| `results.json` | versioned schema、运行身份、baseline 身份、比较结果、统计值与全部原始样本 |
| `summary.csv` | 每个基准的一行统计摘要 |
| `samples.csv` | 每个正式样本的一行原始数据 |
| `manifest.json` | 上述固定产物的 SHA-256 清单，用于发现缺失、替换或意外修改 |

报告目录已被 `.gitignore` 排除。CI 或发布流程可以把单次运行目录整体作为构建产物上传，但不应把共享 runner 的噪声数据直接设为硬性回归门禁。

## 发布长期记录

本地报告不会自动进入仓库。确认一轮完整运行值得长期保留后，显式发布它：

```bash
zig build benchmark-publish -- \
  benchmark-results/run-<unix-nanoseconds> \
  <YYYY-MM-DD-revision>
```

使用 mise 的等价命令是：

```bash
mise run benchmark:publish -- \
  benchmark-results/run-<unix-nanoseconds> \
  <YYYY-MM-DD-revision>
```

发布命令会校验 `results.json`、Git 身份、环境指纹、完整 case 集合、样本数和 `manifest.json` 中的产物哈希，拒绝 `smoke`、未知结论、缺失或被篡改的文件、目录穿越以及覆盖已有记录。通过后，它会：

1. 把报告、图表、CSV、原始 JSON 和清单复制到 `docs/benchmarks/runs/<record-id>/`；
2. 根据全部已发布记录重新生成 [`docs/benchmarks/README.md`](benchmarks/README.md) 和 `trend.svg`；
3. 在首页用大白话展示最新速度、环境、可比较基线、关键图表和历史报告索引；
4. 趋势图只连接环境指纹与最新记录完全相同的数据，不把不同机器拼成一条趋势。

索引和趋势先在内存中完整生成，再通过临时文件替换；主索引最后更新，避免生成中断留下“看似已经发布”的首页。CI 中的 `benchmark-docs-check` 会重新校验所有已发布产物哈希，并检查索引和趋势是否由现有记录准确生成。

测量有波动的完整报告可以存档，但首页会明确标为“建议重跑”或“不能下结论”。发布不会删除 `benchmark-results/` 中的本地原件，也不会自动提交 Git 变更。工作树不干净时，程序会自动在 revision 中添加 `dirty`；record ID 也应保留 `dirty`，避免把开发快照误认为干净提交的权威基线。

## 本地固定基准机的正式测量

当前没有 self-hosted runner，因此日常 CI 只运行 `--quick`，用于证明程序和报告链路没有坏；GitHub 共享 runner 的数据不用于性能比较。正式测量统一在一台本地固定机器上执行，并给它设置长期不变、不会与其他机器复用的 runner ID。

先确认工作树干净。没有输出表示可以建立干净基线：

```bash
git status --short
```

然后执行全量测量。下面的 ID 只是示例；一旦确定就不要随意更换：

```bash
ZIWEI_BENCHMARK_RUNNER_ID=ziwei-local-m4max-01 mise run benchmark
```

与同一环境的已发布记录比较时，两轮必须使用相同的 runner ID：

```bash
ZIWEI_BENCHMARK_RUNNER_ID=ziwei-local-m4max-01 \
  mise run benchmark -- \
  --baseline <record-id> docs/benchmarks/runs/<record-id>/results.json
```

审核报告后，用上面的 `benchmark:publish` 命令把值得长期保留的记录写入文档。正式测量时还应保持 CPU、操作系统、电源模式和后台负载稳定，并避免同时执行其他重负载任务；更换硬件、系统或 runner ID 后，环境指纹会变化，历史趋势会自动断开。

## 后续完善计划

以下三项尚未实现，必须按顺序完成，避免先生成的数据被后续合同变更作废。

1. **固定基准合同**：增加独立的 `suite_version`、`suite_id` 和 `contract_fingerprint`；把负载与夹具拆到独立 suite 模块。指纹覆盖 fixture、case、操作数、采样上限、计时边界和统计方法。报告、baseline、发布器与趋势图均要求合同指纹一致。验收标准是修改负载、夹具或测量方法后，旧 baseline 会被自动拒绝。
2. **建立噪声校准工具**：增加 `benchmark:calibrate -- --runs 20`，在干净工作树、固定 runner ID 和同一提交上串行运行完整基准。每轮保留独立报告，并生成跨轮次的 `calibration.json`、大白话总结和噪声图。按 case 给出阈值建议，审核后再写入版本化配置，不自动启用硬门禁。
3. **建立干净权威基线**：前两项完成并提交后，先在本地固定基准机完成 20–30 轮校准，再单独运行一次完整测量并发布。发布器应确认工作树干净、环境与合同指纹匹配、所有 case 稳定，并自动生成与报告身份一致的 record ID。验收标准是基准首页出现一条非 `dirty`、可追溯、可比较的正式基线。

## 调整参数

```text
--warmups <count>       每个普通基准的预热次数，默认 60，最大 200
--samples <count>       每个普通基准的正式样本数，默认 100，范围 3-100
--target-ms <ms>        校准后的目标样本时长，默认 50 ms
--output-root <path>    报告父目录，默认 benchmark-results
--seed <integer>        控制 case 交错顺序，默认 1592598566
--baseline <name> <results.json>
                        使用命名 full-run baseline 生成 report-only 比较
--quick                 使用小样本快速验证完整链路
```

基准结果不替代正确性测试。提交基准相关变更前仍需运行：

```bash
mise run check
```

只检查已发布文档及其产物是否一致，也可以运行：

```bash
mise run benchmark:docs-check
```

调整索引或趋势图的渲染代码后，可根据现有已发布记录重新生成这两个文件：

```bash
mise run benchmark:docs-generate
```
