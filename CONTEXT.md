# 紫微斗数上下文

本仓库以 `ziwei_core` 为领域计算内核，以 `ziwei` 为统一门面。当前重写的核心结果称为 `Natal`：它表示由一份已验证出生输入计算出的完整本命盘事实，不承担条件查询、历法换算、绑定或分析职责。

## 统一语言

| 术语 | 含义 |
| --- | --- |
| `ZiweiBirth` | 紫微斗数领域中由历法层归一化后的公开农历出生资料；`year` 是农历年序号，可计算各大限的农历年份与虚岁 |
| `ZiweiInput` | 紫微斗数领域中不含绝对农历年序号的公开输入；只计算虚岁 |
| `Gender` | 命主的阴阳性别；`Yang` 表示阳男，`Yin` 表示阴女，领域值不使用 `Male` / `Female` |
| `NatalContext` | 两种输入归一化后的私有只读上下文，不是第三种公开输入 |
| `Natal` | `ziwei_core` 产出的不可变本命盘 |
| `PalaceName` | 十二宫名；枚举成员沿用原十二宫职顺序 |
| `Palace` | 一个宫位的宫名、地支、天干、星曜与四条宫位四化关系 |
| 立极 | 以任一本命宫为命位起点，重定十二宫的相对宫名关系；立极只改变观察宫位关系的参照，不生成或修改本命事实 |
| 叠宫 | 在一个立极 scope 中，同一实际宫位的相对宫名与本命宫名构成的有方向关系，表述为“当前 scope 某宫叠本命某宫” |
| 空宫 | 没有主星的宫位；即使宫内有辅星，仍属于空宫 |
| 同宫 | 两颗或多颗星曜落在同一个宫位 |
| 对宫 | 地支相隔六支的一对宫位；十二宫名对应为命迁、兄友、夫官、子田、财福、父疾 |
| 宫线 | 一个宫及其对宫组成的固定二宫组：命迁线、兄友线、夫官线、子田线、财福线、父疾线 |
| 三方（`trine`） | 十二宫名组成的四个固定三宫组：命财官、兄疾田、夫迁福、子友父 |
| 河图宫位（`essence`） | 在当前 scope 中，以指定宫为第一宫，按十二宫职顺序数到第六宫所得的有方向关系，即一六共宗 |
| 会（`converge`） | 以指定宫为参照，同一三方中的另外两个宫与该宫相会；两宫共同构成该宫的会关系范围 |
| 照 | 以指定宫为参照，对宫与该宫相照；对宫的生年忌不称照而称冲 |
| 冲 | 生年忌位于指定宫的对宫时，称生年忌冲该宫 |
| 四正（`four_cardinals`） | 十二宫名组成的三个固定四宫组：命迁子田、夫官父疾、兄友财福；本项目不使用“三方四正”作为关系术语 |
| 暗合宫（`six_harmony`） | 按地支六合确定的一对宫位：子丑、寅亥、卯戌、辰酉、巳申、午未；全部六组称 `six_harmonies` |
| `StarName` | 十八颗星曜的稳定身份 |
| `StarCategory` | 星曜类别；`Major` 表示主星，`Minor` 表示当前辅星，`Auxiliary` 为未来其他辅助星预留 |
| `StarGalaxy` | 星曜所属斗系；`South`、`North`、`Central` 分别表示南斗、北斗、中斗 |
| `Star` | 星曜在本命盘中的事实，包含身份、类别、斗系、生年四化与自化 |
| `Transformation` | 四化稳定代码 `A / B / C / D`；展示层可映射为禄、权、科、忌 |
| `origin_transformation` | 由生年天干飞化并落在目标 `Star` 上的生年四化；生年天干与来因宫宫干一致，因此同一事实也是 `origin_palace_name` 所指宫位中对应的 `PalaceTransformation` |
| `PalaceTransformation` | 一条从源宫到目标星曜所在宫的四化关系 |
| `StarSelfTransformations` | 星曜的向心、离心自化结果 |
| `Decade` | 一个十年大限，包含大限命宫地支与十个年份/虚岁条目 |
| `DecadeYear` | 大限中的最小流年事实：可选 `year` 与 `age`；在紫微斗数语境中，`year` 专指农历年序号，`age` 专指虚岁 |
| `ReframeScope` | 以指定实际宫位为命位建立的相对宫名坐标；本命查询与任意立极查询使用这一 scope |
| `DecadeScope` | 以一个 `Decade` 的命宫地支建立的 L2 立极坐标；它复用 `ReframeScope` 的宫位、星曜、四化、关系与条件语义 |
| `DecadeYearOrdinal` | 一个 `DecadeYear` 在所属大限十年中的一基序号，合法范围固定为 `1..=10` |
| `DecadeYearSelection` | 被定位的 `DecadeYear`、其限内序号及所属 `DecadeScope`；它不是流年 scope，不建立新的宫位坐标 |

## 上下文边界

`ziwei_core` 负责：

- 验证两种公开输入并归一化为 `NatalContext`；
- 安十二宫与十八颗星曜；
- 计算生年四化、宫位四化关系与星曜自化；
- 计算生肖、五行局、大限方向及十二个大限；
- 在有农历出生年序号时计算每个大限的 `year` 与虚岁，没有绝对年序号时只保留虚岁。

`ziwei_core` 不负责：

- 按条件查宫、查星、查四化或选择某个大限/流年；
- 流年宫位、流年星曜或流年四化盘；
- 公农历转换、时区、真太阳时；
- 文案、简繁体、国际化；
- bindings、分析系统或插件扩展。

宫位关系、固定分组、条件组合与大限定位由 `ziwei_query` 只读组合；文案与国际化属于外层适配器。`ziwei` 只选择性重导出已经稳定的领域公开面。

## 核心不变量

- `Natal`、`Palace`、`Star`、`PalaceTransformation`、`Decade`、`DecadeYear` 的字段私有，只能由内核装配，对外只读。
- `palaces` 固定从寅宫开始顺行：寅、卯、辰、巳、午、未、申、酉、戌、亥、子、丑。
- 十二宫名各出现一次；十八个 `StarName` 各出现一次。
- 每宫恰有四条 `PalaceTransformation`，顺序固定为 `A / B / C / D`。
- 全盘恰有四颗星的 `origin_transformation` 为 `Some(A/B/C/D)`，每类一次。
- `ming_palace_branch` 必须解析到 `PalaceName::Ming`；`shen_palace_name`、`origin_palace_name` 必须与各自地支分别解析到同一个 `Palace`。
- `origin_palace_name` 所指宫位的天干必须与生年天干一致。
- 十二个大限按 `DecadeIndex(0..=11)` 排列，每个大限恰有十个连续虚岁条目；同一张盘的 `year` 必须全部为 `Some` 或全部为 `None`。

历史算法与边界决策见 `docs/adr/`；当前结果模型以 [ADR 0011](docs/adr/0011-immutable-natal-model.md) 和 `docs/design/ziwei-model.md` 为准。
