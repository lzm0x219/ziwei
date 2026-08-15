# 本命盘构建基准：一手资料研究与落地建议

> 研究日期：2026-08-15  
> 适用工具链：Zig 0.16.0。  
> 当前范围：只测 `createFromBirth`、`createFromInput` 完成本命盘构建的性能；不测基于 `Natal` 的只读查询，不测历法换算、闰月、晚子时、时区或真太阳时。  
> 本文提炼上游实践并给出本仓库的建议合同；其中阈值是待用本仓库噪声数据校准的初始策略，不是上游项目规定的通用标准。

## 1. 结论

本仓库适合采用“两层基准、一个报告合同”：

1. **Zig 进程内微基准是主测量层**：直接调用 `createFromBirth` / `createFromInput`，使用单调时钟计时，以 `std.mem.doNotOptimizeAway` 防止结果被优化掉；输入准备、JSON/SVG/Markdown 写出均在计时区外。
2. **`hyperfine` 只作为可选的进程级验收层**：它擅长完整命令的多次运行、预热、JSON/Markdown 导出和分布图，但进程启动成本会淹没一次本命盘构建，不应替代进程内微基准。[hyperfine 1.20.0 README](https://github.com/sharkdp/hyperfine/blob/v1.20.0/README.md)
3. **每次完整运行都产出同一目录下的原始数据、可读报告和 SVG 图表**。原始样本是事实来源，统计量和图表可重建；报告至少包含运行环境、采样配置、结果、噪声诊断、基线差异和门禁结论。
4. **回归判断同时要求统计显著和工程上有意义**，不能只比较两个均值。建议初始门禁为：相对基线变慢超过待校准的实际阈值，且变化的 95% 置信区间整体落在阈值之外；噪声过大时标为“不稳定”并重跑，不直接判回归。Criterion.rs 同时使用显著性水平和 noise threshold，正是为避免“统计显著但工程上无意义”的变化。[Criterion.rs analysis](https://bheisler.github.io/criterion.rs/book/analysis.html)、[`Criterion::noise_threshold`](https://bheisler.github.io/criterion.rs/criterion/struct.Criterion.html#method.noise_threshold)
5. **硬门禁只放在固定、安静、串行的基准机器上**。当前仓库没有 self-hosted runner，因此正式数据来自本地固定基准机，GitHub 共享 runner 只验证报告链路。Criterion.rs 官方 FAQ 明确提醒：云 CI 的虚拟化会引入大量噪声。[Criterion.rs FAQ](https://bheisler.github.io/criterion.rs/book/faq.html)

## 2. 一手实践提供了什么

### Zig 0.16.0：测量原语，而不是完整统计框架

- `std.Io.Clock.awake` 是不允许向后跳的单调时钟；Linux 对应 `CLOCK_MONOTONIC`，macOS 对应 `CLOCK_UPTIME_RAW`。本命盘构建的 wall time 应使用这一类单调时钟，而不是可被人工改时或 NTP 跳变影响的 real clock。[Zig 0.16.0 `Io.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/std/Io.zig)
- `std.mem.doNotOptimizeAway` 的官方注释就是强制表达式求值、尽量阻止结果在最终被丢弃时连同计算一起优化；实现使用 volatile inline assembly，C backend 则回退到 volatile store。[Zig 0.16.0 `mem.zig`](https://codeberg.org/ziglang/zig/src/tag/0.16.0/lib/std/mem.zig)
- Zig 的 Debug、ReleaseFast、ReleaseSafe、ReleaseSmall 有不同优化和运行时安全语义。基准结果必须记录 optimize mode；面向发布性能的主报告宜固定为 `ReleaseFast`，若项目还关心带安全检查的生产形态，可另外生成 `ReleaseSafe` 报告，但两者不能混在同一基线历史里。[Zig 0.16.0 Build Mode](https://ziglang.org/documentation/0.16.0/#Build-Mode)
- `@import("builtin")` 暴露 Zig 版本、target、optimize mode 等编译信息，适合直接写进结果元数据。[Zig 0.16.0 Compile Variables](https://ziglang.org/documentation/0.16.0/#Compile-Variables)
- Zig 项目自己的 [Performance Tracking](https://ziglang.org/perf/) 按 commit 长期展示 execution speed、memory、throughput 等资源指标，并同时给出“相对前一 commit / 相对首个 commit”的趋势图。[`benchmarks.json`](https://ziglang.org/perf/benchmarks.json) 是带 description、kind、directory、main path 的 case registry；[`records.csv`](https://ziglang.org/perf/records.csv) 保存 commit hash、Zig version、sample count，以及 wall/user/system time、CPU cycles、instructions、cache references/misses、branch misses 的 median/mean/min/max 和 max RSS。它直接支持本仓库采用“稳定 case identity + 原始记录 + 跨 commit 趋势”的设计；硬件计数器则应作为支持平台上的可选扩展，不能让首版跨平台 wall-time 报告依赖它们。

### Criterion.rs：完整微基准的统计参照

- Criterion.rs 的默认配置体现了一套成熟起点：3 秒 warmup、5 秒 measurement、100 个 samples、95% confidence level、5% significance level、1% noise threshold；这些值可作为本仓库初始采样合同的参照，而不是必须逐字复制的标准。[Criterion.rs source defaults](https://bheisler.github.io/criterion.rs/src/criterion/lib.rs.html)
- 每个 sample 应包含多次迭代，再用总耗时除以迭代数估计单次耗时；这样可把计时器开销摊薄。warmup 还用于让 CPU/OS/cache 适应负载，并估计后续迭代量。[Criterion.rs analysis](https://bheisler.github.io/criterion.rs/book/analysis.html)
- Criterion.rs 用修改后的 Tukey 方法标注异常值：`1.5 × IQR` 之外为 mild，`3 × IQR` 之外为 severe；**异常样本不会被删除**，而是连同其他样本继续分析，同时用警告和图表暴露可靠性风险。[Criterion.rs outlier classification](https://bheisler.github.io/criterion.rs/book/analysis.html#outlier-classification)
- 它报告 mean、median、standard deviation、MAD、线性回归斜率及置信区间；比较阶段用 bootstrap 和假设检验估计变化是否真实，再用 noise threshold 过滤过小变化。[Criterion.rs command-line output](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.html)、[Criterion.rs analysis](https://bheisler.github.io/criterion.rs/book/analysis.html)
- 它保留 raw CSV / sample JSON / estimates JSON，并生成 HTML 与 SVG，包括分布、回归、MAD、SD、基线对比等图。这说明“原始机器数据 + 可读报告 + 图表”应是一份报告包，而不是只有终端摘要。[Criterion.rs plots and graphs](https://bheisler.github.io/criterion.rs/book/user_guide/plots_and_graphs.html)
- 它支持命名 baseline，并能在不覆盖 baseline 的前提下比较当前结果。基线应有明确身份，不能用“本机上一次不知是什么环境的运行”充当权威基线。[Criterion.rs baselines](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html#baselines)

### Google Benchmark：重复、随机交错、上下文和机器格式

- Google Benchmark 允许配置 minimum run time、warmup time 和 repetitions；重复运行时报告 mean、median、standard deviation 和 coefficient of variation。[Google Benchmark user guide](https://google.github.io/benchmark/user_guide.html#statistics-reporting-the-mean-median-and-standard-deviation-coefficient-of-variation-of-repeated-benchmarks)
- random interleaving 会把不同 benchmark 的 repetitions 随机交错，降低随时间变化的系统状态对某一个 case 的偏置；官方页面报告其测试中平均降低了 40% 的 run-to-run variance。[Google Benchmark random interleaving](https://google.github.io/benchmark/random_interleaving.html)
- JSON 顶层区分 `context` 与 `benchmarks`；context 包含日期、主机、CPU、cache、load average、CPU scaling、库版本和 schema version，并允许加入自定义 context。其兼容性要求是消费者忽略未知字段、容忍可选字段缺失。[Google Benchmark output formats](https://google.github.io/benchmark/user_guide.html#output-formats)
- 官方 `compare.py` 可以比较两个可执行文件或两份 JSON；在重复数足够时可执行 Mann–Whitney U test。官方也强调：统计显著不自动等于实际有意义，反之亦然。[Google Benchmark tools](https://google.github.io/benchmark/tools.html)

### hyperfine：进程级补充与图表先例

- hyperfine 默认至少运行 10 次且至少测量 3 秒；`--warmup` 可建立 warm-cache 状态，`--prepare` 可在每个 timing run 前建立 cold-cache 等前置状态。[hyperfine 1.20.0 README](https://github.com/sharkdp/hyperfine/blob/v1.20.0/README.md#basic-benchmarks)
- JSON 保留 command、mean、stddev、median、user/system time、min/max、全部 times、退出码和参数；这说明汇总值不能替代 raw samples。[hyperfine 1.20.0 `BenchmarkResult`](https://github.com/sharkdp/hyperfine/blob/v1.20.0/src/benchmark/benchmark_result.rs)
- 官方仓库提供 histogram、whisker plot 和跨多份结果的 plotting scripts；Markdown 导出则提供 mean ± stddev、min、max 和 relative 表格。[hyperfine 1.20.0 README](https://github.com/sharkdp/hyperfine/blob/v1.20.0/README.md#exporting-results)、[hyperfine plotting scripts](https://github.com/sharkdp/hyperfine/tree/v1.20.0/scripts)

## 3. 本仓库建议的测量合同

### 3.1 基准 case

当前只建立以下 case，不把任何查询调用拼进计时区：

| Case | 输入在计时前准备 | 计时区内唯一目标 | 回答的问题 |
|---|---|---|---|
| `create_from_birth/single` | 一份来自现有已确认命例的有效 `ZiweiBirth` | `createFromBirth(birth)` | 含绝对农历年序号的公开入口构建一张完整 `Natal` 的延迟 |
| `create_from_input/single` | 与上例出生事实对应的有效 `ZiweiInput` | `createFromInput(input)` | 已预处理年柱入口构建一张完整 `Natal` 的延迟 |
| `create_from_birth/confirmed_corpus` | 固定顺序的多份已确认 `ZiweiBirth` | 为 corpus 中每项构建 `Natal` | 多种已确认出生事实下的批量吞吐及数据相关波动 |
| `create_from_input/confirmed_corpus` | 对应的固定 `ZiweiInput` corpus | 为 corpus 中每项构建 `Natal` | 预处理入口的批量吞吐及数据相关波动 |

fixture 应复用当前测试中已经确认的命例和术语；只有领域口径确认后才扩大 corpus。输入 `init`、fixture 复制、正确性断言、图表与文件 I/O 全部放在计时区外。每轮运行前可做一次轻量正确性检查，确保被测入口成功返回完整 `Natal`；性能报告不能替代领域测试。

### 3.2 编译与计时

- 主性能配置固定为仓库锁定的 Zig 0.16.0、host target、`ReleaseFast`；target CPU 特性、链接模式和是否启用额外安全检查都写入元数据。
- 使用 `Clock.awake` 取得开始/结束时间；结果统一存为整数 `ns`，避免机器格式在不同显示单位之间漂移。
- 每次迭代消费返回的 `Natal`，例如交给 `std.mem.doNotOptimizeAway`；不得只调用后丢弃。
- 自动校准 `iterations_per_sample`，使单个 sample 的总计时明显大于时钟读取开销；保存的是每个 sample 的总纳秒数、迭代数和换算后的 `ns/op`。
- warmup 与正式 samples 分开，warmup 数据不进入统计。建议初始采用 Criterion.rs 的量级：warmup 约 3 秒、100 samples、正式测量至少约 5 秒/case；若完整 suite 太慢，PR smoke 另用小配置，不能悄悄改变“full”合同。
- 多个 case 的 sample/repetition 随机交错，随机 seed 写入元数据，确保顺序偏置降低且运行可复查。
- 计时进程不并行运行其他 benchmark；同一本地固定基准机上的完整测量也应串行。

### 3.3 统计量与噪声

每个 case 至少保存和报告：

- `sample_count`、`iterations_per_sample`、原始 `ns/op` samples；
- median（主中心值）、mean、min、max；
- standard deviation、coefficient of variation；
- MAD、p90、p95、p99；
- median 与 mean 的 95% bootstrap confidence interval；
- Tukey mild/severe outlier 数量和占比，但不删除样本；
- 可选的 `ops/s`（由同一批原始样本换算），以及确定性的 `@sizeOf(Natal)`；若未来报告 allocation，必须来自实际 instrumentation，不能从“API 没有 allocator 参数”推断。

median/MAD 对偏斜和偶发干扰更稳健，mean/stddev/CV 便于和 Google Benchmark、hyperfine 输出对照，percentiles 与箱线图暴露尾部。p99 在样本较少时不稳定，因此图表和报告必须同时显示 sample count，不把 p99 当唯一门禁指标。

### 3.4 回归门禁

没有跨项目通用的“正确百分比”。建议先在本地固定基准机上对同一 commit 独立跑 20–30 次 full suite，按 case 观察自然波动，再确定 threshold。初始策略可为：

1. baseline 与 contender 使用同一 Zig、optimize mode、target、runner fingerprint 和采样配置；最好在同一 job 中分别构建 base/head 并交错测量，避免把日期或机器差异误判成代码差异。
2. 以 median change 为主，门禁候选阈值先设为 **5%**；只有 `p < 0.05` 且 95% change CI 的下界仍慢于 `+5%` 才判定回归。5% 是校准前的保守起点，不是行业标准。
3. 若 CV、outlier ratio、系统 load 或 CPU scaling 表明本轮异常，则结论为 `unstable`，自动重跑一次并保留两份报告；不能删除“难看”的样本后继续给出 pass。
4. 单个 case 可在有真实性能预算后使用更严格/更宽松的 per-case threshold；门禁配置本身要带版本并写进结果。
5. baseline 只在默认分支的完整基准成功后晋升；PR 结果不得覆盖权威 baseline。

在取得本机校准数据前，可把 `CV > 5%` 先作为噪声警告、`CV > 10%` 作为本轮无效并重跑的保守起点；它们同样属于项目策略，校准后应按 case 调整。

## 4. 每次运行的报告合同

建议每次 full run 生成独立目录：

```text
benchmark-results/<run-id>/
├── benchmark.json
├── report.md
├── report.html                 # 可选，自包含；方便直接打开
└── charts/
    ├── latency.svg             # 每个 case 的 median + 95% CI
    ├── distribution.svg        # 箱线/散点，显示 Tukey 异常值
    ├── change.svg              # 有 baseline 时：变化 CI + threshold 带
    └── trend.svg               # 有兼容历史时：按 commit 的长期趋势
```

即使没有 baseline，也必须生成 latency 与 distribution 图；有 baseline 才增加 change 图。SVG 便于 GitHub 和本地查看，也能像 Criterion.rs 报告一样与 Markdown/HTML 一起归档。

### `benchmark.json`

机器格式建议包含：

- `schema_version`、suite/config version、run id、UTC 时间；
- git commit/ref、dirty 状态；
- Zig version/backend、optimize mode、target triple/CPU features；
- OS/kernel、CPU model、logical CPU count、memory、load average、CPU scaling/governor（能取得时）、runner name/environment；
- GitHub 的 run id/attempt/SHA、`RUNNER_OS`、`RUNNER_ARCH`；这些是 GitHub 官方提供的运行上下文。[GitHub variables reference](https://docs.github.com/en/actions/reference/workflows-and-actions/variables)
- warmup、sample count、measurement minimum、iterations-per-sample、seed、计时 clock、case order；
- fixture/corpus id、版本、条目数与内容 digest；
- 每个 case 的 raw samples、统计量、outlier labels；
- baseline 的 commit、artifact/run id、runner fingerprint；
- gate config、逐 case verdict、整轮 verdict。

消费者应像 Google Benchmark JSON 的兼容性约定一样忽略未知字段，并允许非必需环境字段缺失；`schema_version` 发生不兼容变化时再升级。

### `report.md` / `report.html`

固定章节建议为：

1. 总结：pass / regression / unstable / no-baseline；
2. 本轮身份和环境 fingerprint；
3. 测量配置与范围声明（明确“不含查询”）；
4. 结果表：median、95% CI、p95、CV、outlier ratio、相对 baseline；
5. 三类图表；
6. 回归与噪声诊断；
7. artifact、commit、baseline 的可追溯链接。

图表不能只给一根没有误差范围的柱子：latency 图显示估计值与 95% CI；distribution 图显示原始点/箱体和异常值；change 图显示 0%、门禁阈值带与变化 CI。

## 5. 自动化与正式测量分层

| 层级 | 触发 | Runner | 内容 | 是否硬门禁 |
|---|---|---|---|---|
| smoke | Pull Request | 普通 GitHub-hosted | 构建 benchmark、列出 case、每 case 极少量迭代，验证报告管线可运行 | 只门禁可执行性，不门禁性能 |
| full compare | 本地手动 | 固定本地基准机 | base/head 同环境完整测量、统计、图表、报告，再显式发布到仓库文档 | 校准后决定 |
| release evidence | 发布候选时本地手动 | 同一本地基准机 | full suite，永久关联 tag/commit 的报告包 | 按项目发布政策决定 |

本地正式测量还应：

- 固定 runner ID、电源模式和后台负载，避免在完整测量期间运行其他重负载程序。Google Benchmark 官方列出的噪声来源包括不同 core 速度、boost、调度竞争、SMT、其他 CPU 的 cache 影响和 NUMA。[Google Benchmark: Reducing Variance](https://github.com/google/benchmark/blob/main/docs/reducing_variance.md)
- 将完整报告目录通过 `benchmark:publish` 写入版本化文档，并由 manifest 保存每个产物的 SHA-256；本地原始报告与已发布文档分开保存。
- baseline 与依赖 cache 分开。cache 是构建加速机制，benchmark 报告是带有 Git、环境和产物身份的证据。

如果以后增加 self-hosted runner，再考虑把 full compare 搬到 GitHub Actions，并同时配置串行队列、artifact 保留期和 job summary；在那之前不创建无法调度的正式基准工作流。

## 6. 明确不做的事

- 不把 `palaceAt`、`findStar`、query scope 或其他只读查询加入当前 suite。
- 不把历法换算或界面层成本混进 `createFromBirth`；该入口接收的是历法层已经归一化的 `ZiweiBirth`。
- 不把 correctness test 的通过等同于性能稳定，也不把性能报告等同于领域正确。
- 不在普通共享云 runner 的单次波动上阻止合并。
- 不只保存汇总数字，不静默丢弃异常样本，不让不同 optimize mode 或不同机器 fingerprint 共用同一趋势线。

## 7. 推荐的最小落地顺序

1. 先定义 versioned JSON schema 与两个 `single` case，验证 `ReleaseFast` 计时、防优化、raw samples。
2. 加入统计、Tukey 标注、Markdown 与两张无 baseline 也必出的 SVG。
3. 加入 confirmed corpus、命名 baseline 和 change 图；先 report-only 收集 20–30 个稳定 runner 样本。
4. 用实测噪声校准 per-case threshold 后再开启硬门禁。
5. 先稳定本地发布与历史趋势；只有确实配置 self-hosted runner 后再接 GitHub Actions artifact、job summary 和串行 full workflow。`hyperfine` 仅在需要验证完整 benchmark 命令启动成本或报告管线端到端耗时时补充。
