# 如何渐进重构 Ziwei workspace

> 内容类型：Conceptual
>
> 状态：草案
>
> 目标：确认顶层职责、依赖方向和迁移顺序。具体类型与接口留到下一步讨论。
>
> 读者：维护 Rust 核心和各语言 SDK 的开发者。

## 1. 当前架构

当前 workspace 已完成目标 crate 的骨架初始化：

| Crate            | 职责                                      |
| ---------------- | ----------------------------------------- |
| `ziwei`          | 统一 facade，显式导出当前 core interface  |
| `ziwei_core`     | 已承载既有实现，等待按目标模型重写         |
| `ziwei_calendar` | 独立历法骨架                              |
| `ziwei_query`    | 领域查询骨架                              |
| `ziwei_analysis` | 分析骨架                                  |

既有实现已整体迁入 `ziwei_core`，内部仍按 `placement`、`pipeline`、`decade`、`fly` 等模块分工。排盘逻辑只在 Rust 中实现，后续直接在 core 中重写，不重复实现。

当前 Cargo 依赖方向为：

```text
ziwei_query    -> ziwei_core
ziwei_analysis -> ziwei_core + ziwei_query
ziwei          -> ziwei_calendar + ziwei_core + ziwei_query + ziwei_analysis
```

现有实现只作为可编译基线，不继续作为目标结构演化。下一步按 [#280](https://github.com/matharts/ziwei/issues/280) 围绕不可变 `Ziwei` 重写 `ziwei_core`。

## 2. 主要问题

- 排盘数据和领域查询仍集中在一个 crate，历法能力尚未实现。
- `Ziwei` 同时负责数据、构造和查询，职责会继续增长。
- 当前 Rust interface 不适合直接作为多语言契约。
- 分析层尚未建立，未来规则不应进入 `Ziwei`。

## 3. 目标 workspace

```text
workspace/
├── ziwei
├── ziwei_core
├── ziwei_calendar
├── ziwei_query
├── ziwei_analysis
└── bindings       # Rust API 稳定后建立
```

数据流保持为：

```text
ziwei_calendar
        ↓
生成独立的历法结果
        ↓
ziwei 组合与转换
        ↓
ziwei_core
        ↓
生成 Ziwei
        ↓
ziwei_query
        ↓
查询领域关系
        ↓
ziwei_analysis
```

`ziwei` 是 Rust SDK facade，也是组合流程的唯一公开入口。未来的 bindings 位于最外层，只依赖 `ziwei`。

## 4. Crate 边界

| Crate            | 负责                                                  | 不负责                   |
| ---------------- | ----------------------------------------------------- | ------------------------ |
| `ziwei_calendar` | 公历/农历转换、节气和独立历法结果                     | 紫微类型、排盘与分析     |
| `ziwei_core`     | `Ziwei`、领域模型、安星、排盘、大限和流年计算         | 历法转换、关系查询与解释 |
| `ziwei_query`    | 基于 `Ziwei` 的只读领域关系查询                       | 排盘与命理解释           |
| `ziwei_analysis` | 格局、规则匹配、分析和解读结果                        | 排盘与修改 `Ziwei`       |
| `ziwei`          | SDK facade、能力组合、公开 interface、features 和版本 | 领域规则与语言绑定       |
| `bindings`       | FFI、类型转换、错误映射和 SDK 外观                    | 任何紫微领域规则         |

`Ziwei` 是稳定、不可变的领域数据对象。领域查询由 `ziwei_query` 实现，不继续增加到 `Ziwei`。

## 5. 依赖原则

- `ziwei_core` 与 `ziwei_calendar` 相互独立。
- `ziwei_calendar` 不依赖 `ziwei_core` 的 `Stem`、`Branch` 或其他紫微领域类型。
- `Stem`、`Branch` 属于 `ziwei_core`；`ziwei` 将历法结果转换为 core 的标准化排盘输入。
- `ziwei_query` 依赖 `ziwei_core`。
- `ziwei_analysis` 依赖 `ziwei_core` 和 `ziwei_query`。
- `ziwei` 依赖并聚合 calendar、core、query 和 analysis。
- 未来的 bindings 只依赖 `ziwei`。
- 领域 crate 不依赖 bindings。
- TypeScript、Python、Java 和 Swift SDK 不实现领域规则。

Cargo 依赖与运行时数据流不是同一件事。外层先调用 calendar，再把时间上下文传给 core。

## 6. 渐进迁移

### 阶段 0：初始化子包

建立 `ziwei_core`、`ziwei_calendar`、`ziwei_query` 和 `ziwei_analysis` 的最小可编译 crate，并将 `ziwei` 定位为 facade。只固定 Cargo 依赖方向，不提前定义 interface 或迁移业务逻辑。

### 阶段 1：整理现有 crate

先按 [领域语义收口](domain-semantics-convergence.md)清理现有实现，再分开命盘数据、排盘实现和查询实现。保持 workspace 可编译，为迁移建立清晰边界。

### 阶段 2：迁移 core 与建立 query

[`ziwei_core` 首批迁移](ziwei-core-first-batch.md)已经完成。下一步执行 [#280](https://github.com/matharts/ziwei/issues/280) 重写 core；关系查询等 `Ziwei` 数据模型稳定后再迁入 `ziwei_query`，`ziwei` facade 始终保留统一入口。

### 阶段 3：建立 calendar

待 epheon 提供所需能力后，在 `ziwei_calendar` 中建立公农历转换与节气能力。calendar 保持独立，不迁入 `Stem`、`Branch` 等紫微领域类型。

### 阶段 4：建立 bindings

Rust 公共 API 稳定后，再设计并实现 TypeScript、Python、Java 和 Swift bindings。当前不保留任何语言绑定代码或占位实现。

### 阶段 5：建立 analysis

出现第一个可验证分析规则时，再向已初始化的 `ziwei_analysis` 迁入实际能力；此前不定义分析 interface。

每个阶段结束时，workspace 必须保持编译，并通过现有 Rust 测试。

## 7. 下一步

按以下顺序继续：

1. 执行 [#280](https://github.com/matharts/ziwei/issues/280)，围绕不可变 `Ziwei` 重写 `ziwei_core`；
2. `ziwei_query` 采用什么 interface（待 `Ziwei` 数据模型稳定后讨论）；
3. calendar 向 core 提供什么时间上下文；
4. bindings 使用什么跨语言数据契约；
5. 不同流派和规则版本如何进入 core 与 analysis。
