# Rust 包与适配层架构

## 状态与目的

本文记录紫微斗数排盘引擎的 Rust 包设计。它是当前 workspace 的架构约束，不替代领域术语表 [`CONTEXT.md`](../../CONTEXT.md)，也不规定尚未确认的公开类型签名。

目标是让 Rust 使用者得到一个小而深的核心模块：调用方只需提供已经归一化的出生资料，并从 crate 根调用 `Ziwei::from_birth` 或 `Ziwei::from_input`，即可得到不可变 `Natal`。`Ziwei` 是只承载这两条关联构造方法的公开入口；`Natal` 是命盘结果对象。调用方再在同一个内核中取得本命、大限、流年的只读结果。Node.js/TypeScript 与 WebAssembly 只在各自运行时把这一能力适配出去，不能复制或改变排盘规则。

## 决策摘要

1. 当前 workspace 只有一个包：`ziwei_core`。
2. 本命构建、按需大限/流年、只读查询同属 `ziwei_core`；它们不是独立 Cargo 包。
3. `PalaceScope` 统一表达本命、大限与流年宫职，并提供简、繁宫职名称；`Star` 保留固有名称，不创建显示语言或标签包。
4. Node.js/TypeScript 与 WebAssembly 在接口稳定后各自成为一个 adapter 包，单向依赖 `ziwei_core`。
5. 历法换算和解释/断语不属于 V1，不创建对应包。
6. 不创建仅重导出的 Rust 门面包；它会制造浅模块而不提供额外能力。
7. 只有出现真实、独立的边界时才拆包。目录预留不是拆包理由。

## 设计原则

### 一个深核心模块

`ziwei_core` 的 interface 面向三类调用者：项目自身的上层应用、其他 Rust 开发者，以及未来 adapter。它要隐藏五行局、安星、宫位四化、自化、连续飞化和期间计算的具体实现，让调用者通过少量稳定入口取得结果。

因此，领域计算不能分散到绑定层、调用方或多个相互转发的包中。删除 `ziwei_core` 后，排盘复杂度应该重新出现在所有调用方；这证明它承担了应有的深度与 leverage。

### 只在真实 seam 处拆包

Node-API 与 `wasm-bindgen` 的编译目标、错误模型、对象生命周期和序列化方式不同，因此是两个真实 adapter，分别拆包有价值。

相反，当前的本命计算、查询和期间计算共享同一个不可变 `Natal`，没有第二个实现，也没有独立运行时；把它们拆为 `core`、`query`、门面三层只会扩大 interface，降低 locality。

### 依赖只能向内

所有排盘规则、领域事实、`PalaceScope` 宫职名称与 `Star` 星曜名称都只依赖 Rust 标准库及 `ziwei_core` 内部实现。核心不定义运行时本地化、全局语言状态、adapter、JavaScript、WebAssembly、时区、历法或解释模块。

```text
上层 Rust 应用 ───────────────────────────► ziwei_core
ziwei-napi ────────────────────────────────► ziwei_core
ziwei-wasm ────────────────────────────────► ziwei_core
```

`ziwei-napi` 与 `ziwei-wasm` 之间也没有依赖关系。

## Workspace 形状

### 当前形状

```text
.
├── Cargo.toml
├── mise.toml
├── crates/
│   ├── ziwei_core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── domain.rs
│   │       ├── domain/
│   │       ├── error.rs
│   │       └── rules.rs
├── docs/
│   └── architecture/
│       └── rust-package-design.md
└── tests/                         # 按需创建
```

根 `Cargo.toml` 是 workspace 配置，不是一个虚假的业务包。它统一 edition、MSRV、许可证、仓库地址与 lint；所有排盘领域实现和固有双字名称均位于 `crates/ziwei_core`。

`mise.toml` 是唯一的工具链入口。当前固定 Rust `1.98.0`，以避免本机默认 Rust 版本改变构建结果。

### 未来形状

只有达到后文的拆分门槛后，workspace 才演进为：

```text
.
├── crates/
│   └── ziwei_core/
├── bindings/
│   ├── ziwei-napi/
│   └── ziwei-wasm/
├── docs/
├── fixtures/
│   └── natal/                      # 经人工核验的命例输入与事实
└── tests/
```

`bindings/` 在当前不存在。不能为了目录完整性创建没有 interface、测试和交付目标的空 crate。

## 包职责

### `ziwei_core`

这是唯一的领域实现包，也是唯一可以执行排盘规则的包。

它负责：

- 接收 `ZiweiBirth` 与 `ZiweiInput`，并完成已确认的结构和范围校验。
- 将两类输入归一化为私有构建事实，构建不可变 `Natal`；不引入或导出 `ZiweiSeed` 一类中间领域对象。
- 保存十二宫、十八星、生肖、五行局、命/身/来因宫、生年四化、宫位四化与自化等本命事实。
- 按需计算大限与流年；不在构建 `Natal` 时预存完整期间序列，也不在核心中缓存。
- 提供已确认的只读宫位、星曜与四化查询；连续飞化暂缓，不纳入当前核心。
- 由 `PalaceScope` 统一提供本命、大限与流年宫职的 `name_hans()`、`name_hant()`，并由 `Star` 提供固有名称；`Palace`、键与基础领域值不提供名称 API。`Stem` 与 `Branch` 的固定简体 `Display` 仅用于组合 `ZiweiError::Display` 中文诊断。
- 返回可匹配的领域错误，且不依赖绑定层错误类型。

它不负责：

- 公历/农历换算、闰月、时区、晚子时、真太阳时或日期有效性。
- Node-API、JavaScript 对象、Wasm ABI、JSON、浏览器能力或运行时初始化。
- 解释、断语、评分、建议或其他分析能力。

Cargo 包名与 Rust import 名均为 `ziwei_core`。不能通过新增纯重导出门面包来回避这一决策。

### `ziwei-napi`（后续）

该 adapter 面向 Node.js/TypeScript。它依赖 `ziwei_core`，并且只做以下转换：

- 将 JavaScript 传入的数字、字符串或对象解析为核心确认的输入类型。
- 持有或封装 `Natal`，以适合 Node 对象生命周期的方式暴露构建和查询。
- 将核心事实、对象自带的双字名称字段和领域错误转换为约定的 TypeScript 输出与错误代码。
- 提供 Node 端端到端测试、打包和平台构建配置。

它不能内置规则表、重算宫位、修正核心结果，或创建第二套 `Natal` 结构。

### `ziwei-wasm`（后续）

该 adapter 面向浏览器和其他 Wasm host。职责与 `ziwei-napi` 相同，但实现可针对 `wasm-bindgen`、Wasm 对象生命周期和 Web 测试 runtime 调整。

它同样不能包含领域规则。Wasm 不是 `ziwei_core` 的 feature：两者是不同 adapter，拥有不同的编译与测试约束。

## `ziwei_core` 内部模块图

当前按“领域事实、排盘规则、公开入口”分离内部职责。`domain` 只是私有的物理组织方式，不能成为调用方必须了解的公开路径。

```text
src/
├── lib.rs                       # 私有模块声明与扁平公共重导出
├── domain.rs                    # 领域子模块声明与领域类型聚合
├── error.rs                     # ZiweiError 公共失败类型
├── rules.rs                     # 私有排盘规则，当前保持单文件
└── domain/                      # 私有领域模块
    ├── primitive.rs             # 阴阳、五行、性别、天干、地支、生肖
    ├── birth.rs                 # 出生值、两类公开输入与 BirthContext
    ├── five_element_bureau.rs   # 五行局
    ├── natal.rs                 # Natal 本命盘结果
    ├── palace.rs                # 宫位与 PalaceKey
    ├── star.rs                  # 星曜与 StarKey
    ├── transformation.rs        # 四化与自化值
    └── period.rs                # 限运领域值
```

### `lib.rs`

crate 根是 Rust 使用者的唯一外部 seam。它重导出已经稳定的输入、基础领域值、错误、`Ziwei` 创建入口与 `Natal` 结果类型；公开排盘入口仅有 `Ziwei::from_birth` 与 `Ziwei::from_input`。不暴露 `rules` 的文件布局或私有计算辅助类型。

使用者应当可以在不理解星曜安置、数组存储次序或规则表位置的前提下使用核心库，也不需要进入子模块构造输入或标注结果类型。`Ziwei` 仅公开 `from_birth` 与 `from_input` 两条关联构造方法；`Natal` 提供命盘事实与查询。内部重构不得迫使调用方改 import 路径。

```rust
use ziwei_core::{Gender, Ziwei, ZiweiBirth};

let birth = ZiweiBirth::new(Gender::Male, 1984, 2, 1, 4).expect("valid normalized birth");
let natal = Ziwei::from_birth(birth);
```

### `domain`

`domain` 是私有模块，集中领域值、输入和命盘对象。它不会形成 `ziwei_core::domain::*` 公开路径；`lib.rs` 从 `domain` 选择性重导出已确认类型，因此内部迁移不能迫使调用方修改 import。

`domain.rs` 只声明子模块并聚合领域类型，不放排盘规则、运行时状态或第二套对象表示。子模块实现继续位于 `domain/*.rs`，避免使用旧式 `domain/mod.rs` 路径。

### `domain/primitive.rs`

此模块提供阴阳、五行、性别、天干、地支和生肖等封闭、可比较的基础领域值，而不是以字符串或裸整数在内部传递。地支索引的子=`0`至亥=`11`约定、天干与地支的阴阳和五行属性、以及地支到生肖的一一映射都在这里得到唯一表示。

五行局不是基础五行：它保留在独立的 `domain/five_element_bureau.rs` 中。

### `domain/birth.rs`

此模块保存 `BirthMonth`、`BirthDay`、`ZiweiBirth`、`ZiweiInput` 与 `BirthContext`。它不进行历法换算，只校验核心已经承诺的输入不变量。

`ZiweiBirth` 保留数字农历年、月、日、时和性别，用排盘规则导出年干支与紫微支。`ZiweiInput` 保留性别、组成有效六十甲子年柱的生年干支、月、紫微支和时；它没有 `birth_year` 与 `birth_day`。

### `domain/natal.rs`

此模块定义不可变 `Natal` 本命盘结果，保存出生上下文、生肖、五行局、十二宫以及已经确认的命宫、身宫、来因宫和紫微星定位事实。`Natal` 不保留原始输入或输入来源。

### `domain/palace.rs`、`domain/star.rs` 与 `domain/transformation.rs`

这三个模块分别保存宫位、星曜和四化领域对象。`PalaceScope` 位于 `domain/palace.rs`，以 `Natal(PalaceKey)`、`Decade(PalaceKey)`、`Yearly(PalaceKey)` 统一表达三类完整宫职，并提供对应的简、繁名称。实际 `Palace` 私有持有本命 `PalaceScope`，不再保存名称字段；`Star` 继续保存名称、简称、生年四化与向心/离心自化事实。

### `domain/period.rs`

`domain/period.rs` 承载完整限运领域；当前保存 `DecadeIndex`、`YearlyIndex`、`DecadeAge` 与 `DecadeYear`，未来实现流月、流日、流时时，其领域值也归入此模块。`period` 是领域模块名称，不因此引入无行为的公开 `Period` 枚举、结构体或 trait。当前大限和流年宫职均由 `PalaceScope` 表达，不再定义 `Decade`、`Yearly`、`DecadePalaceKey` 或 `YearlyPalaceKey`。期间不得在构建 `Natal` 时全量预计算；后续由公开方法按需生成，核心暂不缓存。

### `error.rs`

此模块定义 crate 根公开的 `ZiweiError`。它位于 `domain` 外，因为出生值校验和未来的排盘入口共同使用这一失败类型。

### `rules.rs`

此私有模块保存规则表、公式和布局算法，当前只实现五虎遁十二宫干排布。它不从 crate 根重导出，也不引入流派或规则版本。

在只有一个完整规则族时保持单文件。以后出现多个完整、可独立理解的规则族时，扩展为 `rules.rs` 与 `rules/*.rs`；`rules.rs` 继续承担私有 interface 与编排，子文件只是 implementation，不改变公开 interface。

### `ziwei.rs`（后续）

当开始实现创建命盘的纵切片时再新增此模块。它只定义 `Ziwei`，并提供 `Ziwei::from_birth` 与 `Ziwei::from_input`；具体规则实现仍留在 `rules`，`Ziwei` 本身不保存命盘事实。尚未实现前不创建空壳文件或提前导出无行为类型。

## 公共 interface 的规则

在最终入口名称确认前，`ziwei_core` 的 public interface 必须遵守以下约束：

- 公开输入只有 `ZiweiBirth` 与 `ZiweiInput`；不以带可选字段的联合输入替代它们。
- crate 根重导出 `Ziwei` 与 `Natal`；公开排盘入口为 `Ziwei::from_birth(ZiweiBirth)` 与 `Ziwei::from_input(ZiweiInput)`，它们均返回 `Result<Natal, ZiweiError>`。
- `ZiweiInput::new` 校验生年干、支必须组成有效六十甲子年柱；地支索引有效不代表任意干支组合有效。成功构造后，排盘入口不再重复校验该不变量。
- 成功构建必须得到统一的 `Natal`；调用方不需要选择规则集或流派。
- `Natal` 通过 `BirthContext` 保存可选的 `birth_year` 与 `birth_day`，不保留原始输入或输入来源。
- 所有身份使用稳定 enum 或经过校验的值类型，不能让字符串承担干支、星曜、宫名或四化身份。
- 范围错误、无可用数字农历年、无效期间序号等情况以可匹配错误表达，不能 panic 或依赖错误文案判断；核心 `Display` 为固定中文诊断。
- 返回的事实及查询结果是只读的；调用方不能通过公开引用破坏 `Natal` 不变量。
- 双字名称不是输入身份。宫职名称由 `PalaceScope` 的作用域与 `PalaceKey` 共同确定，星曜名称由 `StarKey` 确定；名称不能反向参与排盘。
- 尚未确认的字段、快照布局、序列化格式、trace 格式和绑定方法名不提前公开。

## 测试与验证布局

### 核心库

每条已经确认的公开行为先以 `ziwei_core/tests/` 下的集成测试表达，再实现最小纵切片。集成测试只跨 crate 根的公共 seam，不检查私有规则模块的文件结构。

私有单元测试可验证公式边界、固定规则表、数组顺序和算术安全性。测试名使用 `CONTEXT.md` 的领域术语，表驱动数据记录来源和预期事实。

`PalaceScope` 的单元测试必须以表驱动形式覆盖三类作用域与十二宫位键组成的全部 36 对 `name_hans`／`name_hant`；`Star` 继续覆盖全部已支持名称与简称。名称测试只固定展示合同，不重复验证排盘规则。

```text
crates/ziwei_core/
├── src/
│   └── ...
├── tests/
│   ├── input_contract.rs
│   ├── ziwei_contract.rs
│   ├── period_contract.rs
│   └── query_contract.rs
└── benches/                         # 有稳定工作负载后才创建
```

### 共享命例

经人工核验的命例输入与期望事实放在 `fixtures/natal/`。fixture 是测试数据，不是一个 Cargo 包，也不应由运行时读取。新增 fixture 必须说明规则来源和适用项目口径。

### Adapter

每个 adapter 在自己的包中维护端到端测试：同一命例经 JavaScript/Wasm 输入后，应得到与核心相同的领域事实和稳定错误代码。adapter 测试不能把绑定层的序列化细节反向变成核心库的约束。

## 性能与依赖策略

- `ziwei_core` 默认零依赖；任何新依赖都必须直接改善已测量的正确性、兼容性或性能。
- 本命构建与查询优先使用固定大小、稳定顺序的数据结构；动态分配或缓存必须有基准证据。
- 大限与流年按需计算；核心暂不缓存。需要缓存时由上层或 adapter 根据其生命周期和容量约束实现。
- 基准测试只在有已确认的命例语料、工作负载和目标环境后引入；不能用尚未验证的“极快”作为提前优化的理由。
- Cargo feature 必须是可叠加的真实可选能力，不能用 feature 选择流派、改变排盘规则或在 Node/Wasm 间切换领域行为。

## 拆包门槛

下列条件同时满足前，不新增 Cargo 包：

1. 新模块面对不同运行时、不同工具链、不同发布周期，具有与排盘规则单向隔离的展示／适配边界，或确有第二种实现。
2. 它能依赖 `ziwei_core` 而不要求核心反向依赖它。
3. 它有独立的 interface、测试和交付物，不只是重导出或转发。
4. 拆分后调用方需要了解的知识不会增加；复杂性会留在新模块内部。

| 候选包 | 允许创建的条件 | 依赖方向 |
| --- | --- | --- |
| `ziwei-napi` | 核心构建、查询、快照和错误合同已稳定，且开始交付 Node 包 | `ziwei-napi -> ziwei_core` |
| `ziwei-wasm` | 核心合同已稳定，且开始交付浏览器/Wasm 产物 | `ziwei-wasm -> ziwei_core` |
| `ziwei-calendar` | 项目明确纳入历法换算，并能作为向 `ZiweiBirth` 提供资料的独立能力 | 可依赖 `ziwei_core` 的输入类型；核心不得依赖它 |
| `ziwei-analysis` | 项目明确纳入解释/断语，并确认其独立语义与安全界限 | `ziwei-analysis -> ziwei_core` |
| `ziwei` 门面 | 证明它提供的版本协调或组合能力超过纯重导出 | `ziwei -> ziwei_core`；否则不创建 |

## 与归档 Rust 结构的关系

归档 Rust 曾拆为 `ziwei_core`、`ziwei_query`、`ziwei`、`ziwei_calendar` 与 `ziwei_analysis`。新设计逐项处理如下：

| 归档包 | 新设计 | 原因 |
| --- | --- | --- |
| `ziwei_core` | 保留并重建为 `ziwei_core` | 它承载不可变领域事实和排盘实现。 |
| `ziwei_query` | 并入 `ziwei_core` | 查询只借用同一张命盘，没有独立运行时或实现变体。 |
| `ziwei` | 暂不创建 | 纯选择性重导出是浅模块。 |
| `ziwei_calendar` | 不创建 | 历法换算明确在引擎外。 |
| `ziwei_analysis` | 不创建 | V1 不做解释或断语。 |

归档结构是交叉参考，不是权威。新代码以本仓库已确认的领域规则、public interface 测试和本文件的依赖约束为准。

## 实施顺序

1. 完成 workspace 与 `ziwei_core` 工具链校验。
2. 在 crate 根确认第一条构建 interface，并写第一条公共集成测试。
3. 按纵切片实现稳定身份与两类输入。
4. 实现最小 `Natal` 构建，再逐步加入宫位、星曜、四化与自化。
5. 在核心中加入按需大限、流年和只读查询。
6. 以固定命例、错误合同和基准验证核心 interface。
7. 仅在 Node/Wasm 交付开始时创建对应 adapter 包。

每一步都必须保持依赖方向、`Natal` 不变量和已通过的公共测试；不以一次性重构替代纵切片。
