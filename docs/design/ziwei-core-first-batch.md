# `ziwei_core` 首批重写

## 状态

不可变 `Natal` 重写由 [#280](https://github.com/matharts/ziwei/issues/280) 跟踪，并已通过 [#282](https://github.com/matharts/ziwei/pull/282) 落地完成。下文保留为已完成切片的范围与验收记录。

## 本切片目标

在不保留旧 `Ziwei` 结果模型的前提下，以一个可工作的端到端切片建立不可变 `Natal`：

1. 两种已验证输入进入同一条归一化流水线；
2. 生成十二宫、十八星、生年四化、宫位四化关系与星曜自化；
3. 生成生肖、五行局、大限方向、十二大限及每限十个年份/虚岁；
4. 通过 `ziwei` 门面选择性重导出稳定领域类型；
5. 删除旧结果类型、查询选择器和兼容路径。

之所以作为一个切片完成，是因为宫位、星曜、四化边与自化共同构成一个所有权图。保留新旧双模型会制造重复事实和临时转换层，不符合 Extreme Simplicity。

## 精确范围

包含：

- `Ziwei` 重命名为 `Natal`，无别名；
- `PalaceRole` 重命名为 `PalaceName`，成员顺序不变；
- 原星曜身份枚举重命名为 `StarKey`，新增盘内 `Star`；
- 四化统一为 `Transformation::A/B/C/D`；
- `PalaceTransformation` 下沉到源 `Palace`，生年四化与自化下沉到目标 `Star`；
- `DecadeStep` 重命名为 `Decade`，每限保存十个 `DecadeYear`；
- 出生年份增加覆盖十二大限的可表示性校验；
- 结果字段私有、只读，由 crate 内统一装配。

不包含：

- `ziwei_query` 的查询实现；
- 流年盘、流年宫位、流年星曜或流年四化；
- 历法、bindings、analysis、插件机制；
- 简繁体或其他 i18n 实现；
- 规则集标识或扩展框架。

## 验收标准

- `from_birth` 与 `from_input` 共享计算路径，前者年份全部为 `Some`，后者全部为 `None`。
- 十二宫按寅为零排列，宫名与地支各自唯一。
- 十八个 `StarKey` 全盘各一次，宫内顺序遵循 `StarKey::ALL`。
- 生年 `A/B/C/D` 各一次；每宫四条关系均能解析到唯一目标星曜。
- 星曜自化与宫位四化关系可相互校验。
- 生肖映射、大限方向、十二乘十大限条目及年份/虚岁公式有测试覆盖。
- 旧查询接口不再从 `ziwei_core` 或 `ziwei` 导出。
- `cargo test --workspace`、`cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo doc --workspace --no-deps` 全部通过。
