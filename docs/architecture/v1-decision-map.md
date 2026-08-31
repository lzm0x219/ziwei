# V1 设计决策地图

> 状态：设计中。此文档在 V1 规格冻结前记录决策与待决项；现有 Rust 代码、模块布局和公开 interface 均不构成设计权威。

## 设计树

```text
重启范围
└─ V1 目标与规则权威
   ├─ 领域对象、字段结构与事实所有权
   ├─ 两类输入与归一化
   ├─ Natal 输出模型与不变量
   ├─ 大限与流年对象
   ├─ 查询 API
   ├─ Rust / Node / Wasm interface
   ├─ 包与模块结构
   └─ 测试、性能、版本与发布
```

## 已确认决策

| ID | 决策 | 状态 |
| --- | --- | --- |
| D-000 | 保留历史业务规则为候选事实；现有代码、模块布局与公开 interface 不继承。每项候选事实须在 V1 设计中重新确认、获得来源或明确废弃。 | 已确认 |
| D-001 | V1 完成 Rust 核心的本命盘、大限、流年与查询原始事实；不提供解释或断语。Node.js/TypeScript 与 WebAssembly 的 interface 合同同时冻结，但绑定实现后续交付。连续飞化暂不属于 V1。 | 已确认 |
| D-002 | 用户确认并写入 V1 规格的规则为唯一权威；旧 Rust 与旧 Zig 仅作可验证证据。发现冲突时必须重新确认。 | 已确认 |
| D-003 | 现在定义 Node.js/TypeScript 与 WebAssembly 所需的稳定数据、错误与排序合同；不提前实现绑定。 | 已确认 |
| D-004 | `Ziwei` 是公开创建入口，仅承载本命盘创建方法；`Natal` 是不可变本命盘结果。 | 已确认 |
| D-005 | 农历换算、闰月、时区、晚子时与真太阳时均在内核外消解；内核仅接收已归一化的紫微斗数资料。 | 已确认 |
| D-006 | 仅支持唯一项目规则；不支持流派参数、运行时规则版本或调用方注入规则表。规则变更通过新规格与库版本处理。 | 已确认 |
| D-007 | 保留两种公开输入：`ZiweiBirth { gender, birth_year, birth_month, birth_day, birth_hour }` 与 `ZiweiInput { gender, birth_stem, birth_branch, birth_month, ziwei_branch, birth_hour }`。 | 已确认 |
| D-008 | 两种公开输入必须收敛到唯一归一化排盘路径；不预设或公开 `ZiweiSeed`。仅当私有实现确有独立不变量与复用价值时，才引入该类型。 | 已确认 |
| D-009 | Rust 核心公开 interface 使用 `Gender`、`Stem`、`Branch` 等领域值；Node.js/TypeScript 与 WebAssembly adapter 负责数字转换与无效值处理。核心不重复公开裸数字构造器。 | 已确认 |
| D-010 | `Natal` 是唯一、完整、不可变且自足的本命事实源；它保存后续期间、查询和跨语言输出所需的全部归一化事实，不保存原始输入、输入来源或仅为构建服务的临时值。归一化出生上下文由其拥有的 `birth_context: BirthContext` 承载。 | 已确认 |
| D-011 | `Natal` 仅预计算本命事实；大限与流年按需生成，核心不缓存，且期间不新增星曜、宫干四化、生年四化或其他本命事实。 | 已确认 |
| D-012 | 已替换：`Natal` 不设置直接的 `lunar_year` 字段。数字年份由 `BirthContext::birth_year: Option<i32>` 承载：`ZiweiBirth` 有值，`ZiweiInput` 为 `None`；它仅为期间数字年份锚点。 | 已确认 |
| D-013 | 十二个实际宫位以寅至丑为唯一稳定顺序保存；每个宫位自带地支，并支持按地支或宫名查询。 | 已确认 |
| D-014 | 已替换：宫名、星曜、四化等事实仅保存稳定领域身份，并由 `ziwei_locale::ZiweiLabels` 派生标签。 | 已替换 |
| D-015 | 实际宫位实体的地支、宫干、星曜与自化永远属于 `Natal`；宫干四化不作为持久字段，后续由函数按需计算。大限与流年仅返回某一期间的宫职重排视图，不复制或修改 `Natal`。 | 已确认 |
| D-016 | V1 星曜集合固定为十四正曜及左辅、右弼、文昌、文曲，共十八星。 | 已确认 |
| D-017 | 生年四化由目标 `Star::birth_transformation` 保存：全盘恰四星有值。宫干四化不保存为本命对象字段，后续由函数按需计算；由其识别的向心／离心自化保存于目标 `Star::self_transformations`。仅提供原始事实与关系，不提供解释或断语。 | 已确认 |
| D-018 | 命宫以 `ming_palace_branch: Branch` 定位，不重复保存宫位键；身宫以 `shen_palace_key: PalaceKey` 与 `shen_palace_branch: Branch` 定位；来因宫以 `origin_palace_key: PalaceKey` 与 `origin_palace_branch: Branch` 定位。来因宫由生年天干固定映射至地支：甲戌、乙酉、丙申、丁未、戊午、己巳、庚辰、辛卯、壬寅、癸亥；不得从五虎遁结果扫描“唯一同干宫位”。 | 已确认 |
| D-019 | 大限索引 `d` 为 `0..=11`；起始虚岁为五行局数加 `10 × d`；阳男、阴女顺行，阴男、阳女逆行；大限命宫从本命命宫按方向移动 `d` 宫。 | 已确认 |
| D-020 | 流年索引 `i` 为大限内的 `0..=9`；虚岁为五行局数加 `10 × d + i`；流年命宫为生年支加虚岁减一（模十二），并从此逆布十二流年宫职。支持单个流年视图和大限内十条流年枚举。 | 已确认 |
| D-025 | V1 查询提供类型化直接访问与固定高层便捷查询，不提供通用查询语言或可组合筛选 DSL。 | 已确认 |
| D-026 | Rust 内核的细粒度查询返回 `Natal` 内不可变事实的借用视图或迭代器；Node.js/TypeScript 与 WebAssembly adapter 负责转换为独立结果。 | 已确认 |
| D-027 | 先冻结所有领域对象、字段与事实归属，再单独设计查询函数 API；在对象模型完成前，不确认任何具体便捷查询函数。 | 已确认 |
| D-028 | 对象模型按“一个对象、一个字段”逐项讨论并确认；示意代码仅用于讨论，未确认字段不得进入规格。 | 已确认 |
| D-029 | `Natal` 经由 `birth_context: BirthContext` 保留归一化本命事实 `gender: Gender`；它既供大限顺逆行计算，也可由调用方读取。 | 已确认 |
| D-030 | `Natal` 经由 `birth_context: BirthContext` 直接保留归一化本命事实 `birth_stem: Stem`；它是生年四化、来因宫与大限方向的依据，也可由调用方读取。 | 已确认 |
| D-031 | `Natal` 经由 `birth_context: BirthContext` 直接保留归一化本命事实 `birth_branch: Branch`；它是流年按需计算的依据，也可由调用方读取。 | 已确认 |
| D-032 | `Natal` 使用 `birth_context: BirthContext` 聚合归一化出生上下文；该对象的字段逐项确认，`Natal` 不直接设置出生信息字段。 | 已确认 |
| D-033 | `BirthContext` 使用 `birth_year: Option<i32>` 保存数字年份；它只作为期间数字年份锚点，不引入历法换算、范围或日期有效性校验。 | 已确认 |
| D-034 | `BirthContext` 使用 `gender: Gender` 保存完整的归一化性别事实；不以阴阳或大限行进方向替代。 | 已确认 |
| D-035 | `BirthContext` 直接使用 `birth_stem: Stem` 与 `birth_branch: Branch` 承载生年干支；不引入 `SexagenaryYear` 聚合对象。 | 已确认 |
| D-036 | `BirthContext` 使用 `birth_month: BirthMonth` 保存归一化出生月份；两种输入均有该值，不区分闰月。 | 已确认 |
| D-037 | `BirthContext` 使用 `birth_hour` 保存归一化出生时辰；采用出生语义前缀，而不沿用输入字段名 `hour`。具体类型另行确认。 | 已确认 |
| D-038 | `BirthContext::birth_hour` 使用 `Branch`；以字段文档明确其为十二时辰对应的地支，不引入 `Hour` 重复类型或裸数字。 | 已确认 |
| D-039 | `BirthContext` 使用 `birth_day: Option<BirthDay>` 保存归一化农历日；`ZiweiBirth` 有值，`ZiweiInput` 为 `None`。 | 已确认 |
| D-040 | `BirthContext` 不设置 `ziwei_branch`；紫微星所在实际宫位由 `Natal` 顶层的 `ziwei_palace_key: PalaceKey` 与 `ziwei_branch: Branch` 保存。 | 已替换 |
| D-041 | `BirthMonth` 使用仅允许 `1..=12` 的 `u8` 新类型；它不记录闰月信息，也不承担历法换算。 | 已确认 |
| D-042 | `BirthDay` 使用仅允许 `1..=30` 的 `u8` 新类型；它不承担具体月份天数或历法换算校验。 | 已确认 |
| D-043 | `BirthContext` 不保存生肖；`Natal` 顶层以 `zodiac: Zodiac` 保存由 `birth_branch` 映射出的生肖，供调用方直接读取。 | 已确认 |
| D-044 | `Natal` 保存五行局事实，字段名为 `five_element_bureau: FiveElementBureau`；不以裸局数或按需推导替代。 | 已确认 |
| D-045 | `FiveElementBureau` 使用五个变体的枚举；每个变体同时表达固定对应的五行与局数，不使用可构造无效组合的结构体或仅局数新类型。 | 已确认 |
| D-046 | `Natal` 使用 `palaces: [Palace; 12]` 承载十二实际宫位；数量及寅至丑顺序均为类型与领域不变量。 | 已确认 |
| D-047 | 不引入 `PalaceReference`；`Palace` 使用其自身 `key: PalaceKey` 与 `branch: Branch`，命／身／来因宫及关系对象直接使用 `palace_key: PalaceKey` 与 `branch: Branch` 表达实际宫位。 | 已确认 |
| D-048 | `Palace::key` 使用 `PalaceKey` 枚举表达稳定宫位身份；不使用字符串或数组位置推导。 | 已确认 |
| D-049 | `Palace::branch` 使用 `Branch` 保存实际宫位所在的十二地支；不使用裸索引或数组位置推导。 | 已确认 |
| D-050 | `Palace` 使用 `stem: Stem` 保存宫干事实。 | 已确认 |
| D-051 | `Palace` 使用 `stars: Vec<Star>` 保存宫内星曜；星曜由独立 `Star` 对象承载，而不只保存 `StarKey` 或改由全局落宫表保存。 | 已确认 |
| D-052 | `Star::key` 使用 `StarKey` 枚举表达固定十八星身份；简繁体显示名称由该身份映射，不存储字符串或内部数字 ID。 | 已确认 |
| D-053 | 每个 `Star` 保存 `category: StarCategory`；类别划分与 `StarCategory` 变体逐项确认。 | 已确认 |
| D-054 | `StarCategory` 使用 `Major`、`Minor`、`Auxiliary` 三个变体；不使用 `Miscellaneous`。 | 已确认 |
| D-055 | 十四正曜的 `StarCategory` 为 `Major`；左辅、右弼、文昌、文曲为 `Minor`；当前 V1 的十八星没有 `Auxiliary` 成员。 | 已确认 |
| D-056 | `Star` 使用必填字段 `galaxy: StarGalaxy`；`StarGalaxy` 的变体为 `South`、`Central`、`North`。不使用紫微／天府二分的 `StarSystem`。各星归属逐项确认。 | 已确认 |
| D-057 | `StarGalaxy` 归属固定为：`South`＝太阴、贪狼、巨门、天梁、破军；`North`＝太阳、武曲、天同、廉贞、天机；`Central`＝紫微、天府、天相、七杀、左辅、右弼、文昌、文曲。 | 已确认 |
| D-058 | `Star::birth_transformation: Option<Transformation>` 仅表达目标星承接的生年四化；四星有值、其余为 `None`。宫干四化不进入该字段。 | 已确认 |
| D-059 | `Palace` 与 `Natal` 均不保存宫干四化集合；宫干四化由后续函数基于本命事实按需计算。识别出的自化保存于目标 `Star::self_transformations`。相关函数 API 在对象模型冻结后单独设计。 | 已确认 |
| D-060 | `Star::self_transformations` 使用非可选 `SelfTransformations`；其 `inward` 与 `outward` 分别为 `Option<Transformation>`，表达一颗星曜的向心与离心自化。 | 已确认 |
| D-061 | `Natal` 使用 `ming_palace_branch: Branch` 定位命宫；不设置 `life_palace_key` 或 `life_palace_branch`，命宫键由对应实际宫位读取。 | 已确认 |
| D-062 | `Natal` 使用 `shen_palace_key: PalaceKey` 与 `shen_palace_branch: Branch` 定位身宫；二者必须指向同一实际宫位，也不在读取时由出生月份和时辰重算。 | 已确认 |
| D-063 | `Natal` 使用 `origin_palace_key: PalaceKey` 与 `origin_palace_branch: Branch` 定位来因宫；二者必须指向同一实际宫位。 | 已确认 |
| D-064 | `Natal` 的归一化出生上下文字段为 `birth_context: BirthContext`；不使用 `natal_context: NatalContext`。 | 已确认 |
| D-065 | `Natal` 使用顶层字段 `zodiac: Zodiac` 保存由生年地支确定的生肖；它不进入 `BirthContext`。 | 已确认 |
| D-066 | `Natal` 使用顶层字段 `ziwei_palace_key: PalaceKey` 与 `ziwei_branch: Branch` 保存紫微星所在实际宫位；二者必须指向同一宫位，且该宫包含紫微星。 | 已确认 |
| D-067 | `Natal` 顶层字段暂时封闭为：`birth_context`、`zodiac`、`five_element_bureau`、`palaces`、`ming_palace_branch`、身宫定位、来因宫定位与紫微星宫位定位；新增字段须重新开启对象评审。 | 已确认 |
| D-068 | `ZiweiBirth::gender` 使用 `Gender`；Node.js/TypeScript 与 WebAssembly adapter 负责 `0`、`1` 的外部转换。 | 已确认 |
| D-069 | `ZiweiBirth` 使用 `birth_year: i32` 保存外部已归一化的数字年份；不使用 `year` 或额外年份值对象，也不引入年份范围或历法有效性校验。 | 已确认 |
| D-070 | `ZiweiBirth` 使用 `birth_month: BirthMonth`；不使用 `month` 或裸数字。 | 已确认 |
| D-071 | `ZiweiBirth` 使用 `birth_day: BirthDay`；不使用 `day` 或裸数字。 | 已确认 |
| D-072 | `ZiweiBirth` 使用 `birth_hour: Branch` 表达出生时辰对应的地支；不使用 `hour` 或裸数字。 | 已确认 |
| D-073 | `ZiweiInput::gender` 使用 `Gender`；不使用裸数字或布尔值。 | 已确认 |
| D-074 | `ZiweiInput::birth_stem` 使用 `Stem`；不使用裸数字或字符串。 | 已确认 |
| D-075 | `ZiweiInput::birth_branch` 使用 `Branch`；不使用裸数字或字符串。 | 已确认 |
| D-076 | `ZiweiInput` 使用 `birth_month: BirthMonth`；不使用 `month` 或裸数字。 | 已确认 |
| D-077 | `ZiweiInput` 使用 `ziwei_branch: Branch` 直接表达紫微星所在实际宫位地支；不使用 `ziwei_palace_branch` 或裸索引。 | 已确认 |
| D-078 | `ZiweiInput` 使用 `birth_hour: Branch` 表达出生时辰对应的地支；不使用 `hour` 或裸数字。 | 已确认 |
| D-079 | 大限按需视图的类型名为 `Decade`；不使用 `Daxian` 或 `MajorPeriod`。 | 已撤回；重新设计 |
| D-080 | `Decade::index` 使用 `DecadeIndex`；该新类型仅允许大限零基序号 `0..=11`。 | 已撤回；重新设计 |
| D-081 | `Decade` 保存 `start_virtual_age: VirtualAge`；它是已生成大限视图的起始虚岁事实。 | 已撤回；重新设计 |
| D-082 | `Decade` 不保存结束虚岁；大限固定十年，结束虚岁由 `start_virtual_age + 9` 在后续函数中推导。 | 已撤回；重新设计 |
| D-083 | `Decade` 不保存大限命宫地支；它由 `index`、本命命宫与顺逆行规则在后续函数中计算。 | 已撤回；重新设计 |
| D-084 | `Decade` 不保存宫职重排结果；它仅保存 `index` 与 `start_virtual_age`，所有大限宫职视图由后续函数按需计算。 | 已撤回；重新设计 |
| D-085 | 已替换：`Palace` 不使用 `decade_start_age: u8`。 | 已替换 |
| D-086 | `Palace` 使用 `decade_age: DecadeAge` 保存大限年龄区间；`DecadeAge` 内部为 `[u8; 2]`，顺序为 `[start, end]`，并保证 `end == start + 9`。它由五行局的第一大限起始虚岁与实际宫位相对命宫的顺逆位置（`0..=11`）计算：`start = first_start + 10 × position`。 | 已确认 |
| D-087 | `Natal` 不保存十二大限的二维宫职布局，`Palace` 也不保存跨十二大限的一维宫职数组。指定大限序号时按需生成由十二项 `Decade` 组成的宫职重排视图；核心不缓存。`Palace::decade_age` 仍是实际宫位固有的大限年龄区间事实。 | 已被 D-184 替换 |
| D-088 | `Decade` 不保存 `index: DecadeIndex`。大限序号只用于后续创建或查询期间视图，不属于已生成 `Decade` 的持久字段。 | 已被 D-184 替换 |
| D-089 | `Decade` 不保存 `age: DecadeAge`。大限年龄区间仅由实际 `Palace::decade_age` 保存，`Decade` 不重复该本命事实。 | 已被 D-184 替换 |
| D-090 | `Decade` 使用 `key: DecadePalaceKey` 表示一个实际宫位在指定大限中的宫职。`DecadePalaceKey` 与 `PalaceKey` 一一对应，但以大命、大兄、大夫、大子、大财、大疾、大迁、大友、大官、大田、大福、大父等大限宫职名称表达。指定大限的十二宫职排布为按实际宫位固定顺序组成的十二项 `Decade` 数组。 | 已被 D-182、D-184 替换 |
| D-091 | `Decade` 不保存 `ming_palace_branch`。大限命宫地支由十二项 `Decade` 中唯一的命宫位置及实际宫位固定顺序推导。 | 已被 D-184 替换 |
| D-092 | 已指定的流年期间对象定名为 `Yearly`，不使用 `Annual`。`Yearly` 使用 `key: YearlyPalaceKey` 表示一个实际宫位在指定流年中的宫职。`YearlyPalaceKey` 与 `PalaceKey` 一一对应，但以流命、流兄、流夫、流子、流财、流疾、流迁、流友、流官、流田、流福、流父等流年宫职名称表达。指定流年的十二宫职排布为按实际宫位固定顺序组成的十二项 `Yearly` 数组。 | 已被 D-182、D-184 替换 |
| D-093 | `Yearly` 不保存流年序号。流年序号只用于后续创建或查询期间视图，不属于已生成 `Yearly` 的持久字段。 | 已被 D-184 替换 |
| D-094 | `Yearly` 不保存 `virtual_age`。该虚岁由生成流年视图时的大限与流年位置确定；未指定流年时展示的十个虚岁由后续独立的流年列表对象承载。 | 已被 D-184 替换 |
| D-095 | `Yearly` 不保存数字年份。仅当 `BirthContext::birth_year` 有值时，数字年份才可由出生年份与流年虚岁推导；未指定流年时展示的年份由后续独立的流年列表对象承载。 | 已被 D-184 替换 |
| D-096 | 未指定流年时按需生成固定的 `[DecadeYear; 10]`，表达十项虚岁与可用数字年份；它不进入 `PalaceScope`，也不引入额外列表包装对象。 | 已确认 |
| D-097 | 已替换：不定义 `DecadeYears` 类型。十项年度摘要直接使用 `[DecadeYear; 10]`；不使用 `YearlyList`。 | 已替换 |
| D-098 | 单个年度摘要条目类型命名为 `DecadeYear`；它不同于表示流年宫职的 `PalaceScope::Yearly(PalaceKey)`。 | 已确认 |
| D-099 | `DecadeYear` 使用 `age: u8` 保存虚岁；不使用 `virtual_age` 字段名，也不引入 `VirtualAge` 新类型。 | 已确认 |
| D-100 | `DecadeYear` 使用 `year: Option<i32>` 保存可用的数字年份。`ZiweiBirth` 可推导该值；缺少数字出生年份锚点的 `ZiweiInput` 为 `None`。 | 已确认 |
| D-101 | `DecadeYear` 不保存流年序号。其在固定 `[DecadeYear; 10]` 中的位置即为零基流年序号；该类型只保存 `age` 与 `year`。 | 已确认 |
| D-102 | `Natal::zodiac` 使用独立的 `Zodiac` 枚举保存十二生肖的稳定身份，不保存字符串或名称字段。 | 已确认 |
| D-103 | `Zodiac` 的内部稳定变体为 `Rat`、`Ox`、`Tiger`、`Rabbit`、`Dragon`、`Snake`、`Horse`、`Goat`、`Monkey`、`Rooster`、`Dog`、`Pig`；它不提供名称字段或标签 API。 | 已确认 |
| D-104 | `Gender` 的变体为 `Female` 与 `Male`；跨语言映射固定为 `0 = Female`、`1 = Male`。不使用 `Yin`／`Yang` 表示性别。 | 已确认 |
| D-105 | 通用阴阳关系使用独立的 `YinYang` 枚举，其变体为 `Yin` 与 `Yang`；它不替代 `Gender`。 | 已确认 |
| D-106 | `FiveElement` 使用 `Water`、`Wood`、`Metal`、`Earth`、`Fire` 五个变体，只表达五行身份；局数由 `FiveElementBureau` 独立表达。 | 已确认 |
| D-107 | `Stem` 使用 `Jia`、`Yi`、`Bing`、`Ding`、`Wu`、`Ji`、`Geng`、`Xin`、`Ren`、`Gui` 十个拼音稳定变体；固定简体 `Display` 只用于中文错误诊断。 | 已确认 |
| D-108 | `Stem` 的固定零基索引为 `Jia = 0` 至 `Gui = 9`，依甲、乙、丙、丁、戊、己、庚、辛、壬、癸顺序递增。 | 已确认 |
| D-109 | `Branch` 使用 `Zi`、`Chou`、`Yin`、`Mao`、`Chen`、`Si`、`Wu`、`Wei`、`Shen`、`You`、`Xu`、`Hai` 十二个拼音稳定变体；固定简体 `Display` 只用于中文错误诊断。 | 已确认 |
| D-110 | `Branch` 的固定零基索引为 `Zi = 0` 至 `Hai = 11`，依子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥顺序递增。十二实际宫位的寅至丑存储顺序独立于该索引。 | 已确认 |
| D-111 | `PalaceKey` 使用 `Ming`、`XiongDi`、`FuQi`、`ZiNv`、`CaiBo`、`JiE`、`QianYi`、`JiaoYou`、`GuanLu`、`TianZhai`、`FuDe`、`FuMu` 十二个拼音稳定变体；它不提供名称字段或标签 API。 | 已确认 |
| D-112 | `Transformation` 使用 `A`、`B`、`C`、`D` 四个稳定变体，依次映射为禄、权、科、忌；`ALL` 以此顺序公开全集，crate 内 `index()` 与其数组下标对齐；它不提供名称字段或标签 API。 | 已确认 |
| D-113 | `StarKey` 使用 `ZiWei`、`TianJi`、`TaiYang`、`WuQu`、`TianTong`、`LianZhen`、`TianFu`、`TaiYin`、`TanLang`、`JuMen`、`TianXiang`、`TianLiang`、`QiSha`、`PoJun`、`ZuoFu`、`YouBi`、`WenChang`、`WenQu` 十八个拼音稳定变体；`ALL` 以此固定顺序公开全集，crate 内 `index()` 与其数组下标对齐；它不提供名称字段或标签 API。 | 已确认 |
| D-114 | `FiveElementBureau` 使用 `#[repr(u8)]`，并以 `WaterTwo = 2`、`WoodThree = 3`、`MetalFour = 4`、`EarthFive = 5`、`FireSix = 6` 表示五行局；枚举值同时是大限首限起始虚岁。 | 已确认 |
| D-115 | `BirthMonth` 与 `BirthDay` 均为 `#[repr(transparent)]` 的 `u8` 元组新类型，分别约束为 `1..=12` 与 `1..=30`。 | 已确认 |
| D-116 | `DecadeAge` 为 `#[repr(transparent)]` 的 `[u8; 2]` 元组新类型，顺序为 `[start, end]`，并保证 `end == start + 9`。 | 已确认 |
| D-117 | `StarCategory` 使用 `Major`、`Minor`、`Auxiliary` 三个变体。 | 已确认 |
| D-118 | `StarGalaxy` 使用 `South`、`Central`、`North` 三个变体，分别对应南斗、中斗、北斗。 | 已确认 |
| D-119 | `Star` 固定包含 `key: StarKey`、`category: StarCategory`、`galaxy: StarGalaxy`、`birth_transformation: Option<Transformation>`、`self_transformations: SelfTransformations`；其中 `birth_transformation` 仅保存生年四化。 | 已确认 |
| D-120 | `Palace` 固定包含 `key: PalaceKey`、`branch: Branch`、`stem: Stem`、`stars: Box<[Star]>`、`decade_age: DecadeAge`；它始终表示本命实际宫位，不随期间改变。 | 已确认 |
| D-121 | `SelfTransformations` 固定包含 `inward: Option<Transformation>` 与 `outward: Option<Transformation>`；`None` 表示对应方向不存在自化。 | 已确认 |
| D-122 | `BirthContext` 固定包含 `birth_year: Option<i32>`、`gender: Gender`、`birth_stem: Stem`、`birth_branch: Branch`、`birth_month: BirthMonth`、`birth_hour: Branch`、`birth_day: Option<BirthDay>`。`ZiweiBirth` 的年份与日期有值，`ZiweiInput` 中二者为 `None`。 | 已确认 |
| D-123 | `Natal` 固定包含 `birth_context: BirthContext`、`zodiac: Zodiac`、`five_element_bureau: FiveElementBureau`、`palaces: [Palace; 12]`、`ming_palace_branch: Branch`、身宫定位、来因宫定位及紫微星宫位定位字段；所有字段均为本命事实，不保存大限或流年状态。 | 已确认 |
| D-124 | `ZiweiBirth` 固定包含 `gender: Gender`、`birth_year: i32`、`birth_month: BirthMonth`、`birth_day: BirthDay`、`birth_hour: Branch`；它不包含闰月、时区或历法换算信息。 | 已确认 |
| D-125 | `ZiweiInput` 固定包含 `gender: Gender`、`birth_stem: Stem`、`birth_branch: Branch`、`birth_month: BirthMonth`、`ziwei_branch: Branch`、`birth_hour: Branch`；字段保持私有，只能经 `new` 构造。它不包含数字年份与农历日。 | 已确认 |
| D-126 | `Ziwei` 为无字段单元结构体，只作为顶层命盘创建入口；其具体创建方法在 API 设计阶段确认。 | 已确认 |
| D-127 | `Ziwei::from_birth` 与 `Ziwei::from_input` 统一返回 `Result<Natal, ZiweiError>`，维持 Rust、Node.js/TypeScript 与 WebAssembly 的一致错误合同。 | 已确认 |
| D-128 | 已替换：`ZiweiError` 的初始范围仅含 `InvalidSexagenaryYear`；范围型数字错误的统一错误合同由 D-146 与 D-170 确认。 | 已替换 |
| D-129 | `Ziwei` 仅公开 `from_birth(birth: ZiweiBirth) -> Result<Natal, ZiweiError>` 与 `from_input(input: ZiweiInput) -> Result<Natal, ZiweiError>` 两个创建入口。 | 已确认 |
| D-130 | `Natal`、`Palace`、`Star` 等领域对象的字段保持私有；调用方通过只读方法或迭代器读取，不能构造或修改违反排盘不变量的对象。 | 已确认 |
| D-131 | `Natal` 提供 `birth_context(&self) -> &BirthContext`、`zodiac(&self) -> Zodiac`、`five_element_bureau(&self) -> FiveElementBureau` 三个直接本命事实读取方法。 | 已确认 |
| D-132 | `Natal` 提供 `palaces(&self) -> &[Palace; 12]`，以寅至丑稳定顺序零拷贝读取全部实际宫位。 | 已确认 |
| D-133 | `Natal` 提供 `palace(&self, branch: Branch) -> &Palace`，按地支常数时间返回唯一实际宫位；不使用 `Option`，因为十二地支必各有一宫。 | 已确认 |
| D-134 | `Natal` 提供 `palace_by_key(&self, key: PalaceKey) -> &Palace`，按本命宫位键返回唯一实际宫位；不使用 `Option`，因为十二宫位键恰各出现一次。 | 已确认 |
| D-135 | `Natal` 提供 `ming_palace(&self) -> &Palace`，直接返回命宫对应的实际宫位。 | 已确认 |
| D-136 | `Natal` 提供 `shen_palace(&self) -> &Palace`，直接返回身宫对应的实际宫位。 | 已确认 |
| D-137 | `Natal` 提供 `origin_palace(&self) -> &Palace`，直接返回来因宫对应的实际宫位。 | 已确认 |
| D-138 | `Natal` 提供 `ziwei_palace(&self) -> &Palace`，直接返回包含紫微星的唯一实际宫位。 | 已确认 |
| D-139 | `Palace` 提供 `name_hans() -> &'static str`、`name_hant() -> &'static str`，以及 `key() -> PalaceKey`、`branch() -> Branch`、`stem() -> Stem`、`stars() -> &[Star]`、`decade_age() -> DecadeAge` 七个基础只读方法。 | 已被 D-183 替换 |
| D-140 | `Palace` 提供 `star(&self, key: StarKey) -> Option<&Star>`，按星曜键查询宫内星曜。 | 已确认 |
| D-141 | `Star` 提供 `name_hans() -> &'static str`、`name_hant() -> &'static str`，以及 `key() -> StarKey`、`category() -> StarCategory`、`galaxy() -> StarGalaxy`、`birth_transformation() -> Option<Transformation>`、`self_transformations() -> SelfTransformations` 七个基础只读方法。 | 已确认 |
| D-142 | `SelfTransformations` 提供 `inward() -> Option<Transformation>` 与 `outward() -> Option<Transformation>` 两个只读方法。 | 已确认 |
| D-143 | `BirthContext` 提供 `birth_year()`、`gender()`、`birth_stem()`、`birth_branch()`、`birth_month()`、`birth_hour()`、`birth_day()` 七个与字段同名的基础只读方法。 | 已确认 |
| D-144 | `DecadeIndex` 与 `YearlyIndex` 为仅用于 API 入参的 `#[repr(transparent)]` `u8` 新类型，分别约束为 `0..=11` 与 `0..=9`；它们不进入 `PalaceScope`。 | 已确认 |
| D-145 | `DecadeIndex` 与 `YearlyIndex` 均通过标准 `TryFrom<u8>` 构造；越界构造返回明确错误而非 `Option`。 | 已确认 |
| D-146 | `ZiweiError` 包含 `InvalidSexagenaryYear { stem, branch }`、`InvalidDecadeIndex { value }`、`InvalidYearlyIndex { value }` 三个公开变体；前者服务于 `ZiweiInput::new`，后两者服务于期间序号转换。月、日范围错误由 D-170 追加。 | 已确认 |
| D-147 | `Natal` 提供 `decade(&self, index: DecadeIndex) -> [PalaceScope; 12]`，按需生成并按寅至丑实际宫位顺序返回十二项 `PalaceScope::Decade` 宫职；不缓存、不堆分配。 | 已确认 |
| D-148 | `Natal` 提供 `yearly(&self, decade: DecadeIndex, yearly: YearlyIndex) -> [PalaceScope; 12]`，按需生成并按寅至丑实际宫位顺序返回十二项 `PalaceScope::Yearly` 宫职。 | 已确认 |
| D-149 | `Natal` 提供 `decade_years(&self, decade: DecadeIndex) -> [DecadeYear; 10]`，按需返回大限内十项虚岁与可用数字年份摘要，不包含流年宫职。 | 已确认 |
| D-150 | `DecadeYear` 提供 `age() -> u8` 与 `year() -> Option<i32>`。原 `Decade`、`Yearly` 读取接口由 D-182 的 `PalaceScope` 读取接口替换。 | 已确认 |
| D-151 | 宫干四化的单条按需关系结果对象命名为 `PalaceTransformation`；它不同于仅表示禄、权、科、忌类别的 `Transformation` 枚举。 | 已确认 |
| D-152 | `PalaceTransformation` 使用 `transformation: Transformation` 标识该关系的四化类别。 | 已确认 |
| D-153 | `PalaceTransformation` 使用 `star: StarKey` 保存宫干四化命中的目标星曜身份。 | 已确认 |
| D-155 | 已替换：宫位与星曜稳定身份不要求使用 `*Key` 后缀。 | 已替换 |
| D-156 | 已替换：`PalaceKey` 不再提供标签方法。 | 已替换 |
| D-157 | 已替换：`StarKey` 不再提供标签方法。 | 已替换 |
| D-158 | 已替换：`Stem` 与 `Branch` 不再提供标签方法。 | 已替换 |
| D-159 | 已替换：`Zodiac` 不再提供派生标签方法。 | 已替换 |
| D-160 | 已替换：`Gender` 不再提供标签方法。 | 已替换 |
| D-161 | 已替换：`YinYang` 不再提供派生标签方法。 | 已替换 |
| D-162 | 已替换：`FiveElement` 不再提供标签方法。 | 已替换 |
| D-163 | 已替换：`Transformation` 不再提供标签方法。 | 已替换 |
| D-164 | 已替换：`FiveElementBureau` 不再提供标签方法。 | 已替换 |
| D-165 | 已替换：`StarCategory` 不再提供标签方法。 | 已替换 |
| D-166 | 已替换：`StarGalaxy` 不再提供标签方法。 | 已替换 |
| D-167 | `BirthMonth` 与 `BirthDay` 均提供 `get() -> u8`，返回内部已验证数值；它们不提供中文格式化或展示名称。 | 已确认 |
| D-168 | `DecadeAge` 提供 `start() -> u8` 与 `end() -> u8`，不直接暴露内部 `[u8; 2]` 表示或新增区间对象。 | 已确认 |
| D-169 | `DecadeIndex` 与 `YearlyIndex` 均提供 `get() -> u8`，返回已验证的零基期间序号。 | 已确认 |
| D-170 | `BirthMonth` 与 `BirthDay` 均通过标准 `TryFrom<u8>` 构造，分别约束 `1..=12` 与 `1..=30`；越界返回 `ZiweiError::InvalidLunisolarMonth { value }` 或 `ZiweiError::InvalidLunisolarDay { value }`。 | 已确认 |
| D-171 | `Stem` 与 `Branch` 均提供 `index() -> u8`，返回各自固定的零基领域序号；二者还实现固定简体 `Display`，只用于组合核心中文错误诊断，不提供语言选择或 `name()` 标签方法。 | 已确认 |
| D-172 | `FiveElementBureau` 作为不可拆解的领域枚举公开；不提供 `element()` 或 `number()` 等组成部分读取方法，也不承担显示职责。 | 已确认 |
| D-173 | 稳定身份类型使用 `PalaceKey`、`StarKey`、`DecadePalaceKey`、`YearlyPalaceKey` 后缀。`Decade` 与 `Yearly` 均以 `key` 字段和 `key()` 方法公开其期间宫职键。`DecadeKey`、`YearlyKey` 不使用，以免与期间序号混淆。 | 已被 D-182、D-184 替换 |
| D-174 | 已替换：`ziwei` 不公开 `Lang` 或任何标签读取方法，`ziwei_locale` 单向依赖核心提供标签。 | 已替换 |
| D-175 | `DecadePalaceKey` 与 `YearlyPalaceKey` 各自公开 `palace_key() -> PalaceKey`，表达与本命宫位键的一一对应；它们本身不提供名称字段或标签 API。 | 已被 D-182、D-184 替换 |
| D-176 | `ZiweiError::Display` 使用固定中文诊断；无效六十甲子通过 `Stem` 与 `Branch` 的固定简体 `Display` 组合，其余边界错误包含原始数值。它不是可配置的本地化接口，调用方必须通过错误变体和字段而非文案匹配错误。 | 已确认 |
| D-177 | `ziwei_locale` 不保留为 workspace 包，核心不定义 `Lang`、运行时标签读取器或全局语言状态。`Palace`、`Star`、`Decade`、`Yearly` 在创建时从其稳定键派生并私有保存必填的 `name_hans: &'static str` 与 `name_hant: &'static str`；两字段分别表示简体、繁体名称，并由同名只读方法返回，但不参与排盘规则。 | 名称归属已被 D-182、D-183 替换；本地化约束保持确认 |
| D-178 | 已替换：D-130 的私有字段约束不适用于 `Palace`、`Star`、`Decade`、`Yearly` 的名称字段。 | 已替换 |
| D-179 | `Gender`、`Stem`、`Branch` 分别提供 `yin_yang() -> YinYang`，返回各自固定的阴阳归属；`Branch` 另提供 `zodiac() -> Zodiac`，返回一一对应的生肖。上述方法只表达基础领域事实，不提供名称、本地化或排盘逻辑。 | 已确认 |
| D-180 | `birth.rs` 的 crate 私有辅助函数以甲子为数字年份同余 `4` 的基准，从任意 `i32` 数字农历年份导出 `(Stem, Branch)`；实现使用 `rem_euclid`，不以 `birth_year - 4` 直接相减。`ZiweiInput::new` 仅在生年干支阴阳相同时构造成功；十干与十二支的 120 种组合中恰有 60 种有效。 | 已确认 |
| D-181 | 五虎遁按甲己丙寅、乙庚戊寅、丙辛庚寅、丁壬壬寅、戊癸甲寅五组起干，并分别顺布十二宫干；规则空间固定为 `5 × 12`，不将十个生年干误建模为十种独立宫干排布。 | 已确认 |
| D-182 | 宫职统一使用 `PalaceScope` 表达；枚举变体为 `Natal(PalaceKey)`、`Decade(PalaceKey)`、`Yearly(PalaceKey)`。`PalaceScope` 提供 `palace_key()`、`name_hans()` 与 `name_hant()`，由作用域与宫位键共同确定完整宫职身份及简、繁名称。 | 已确认 |
| D-183 | `Palace` 私有持有 `scope: PalaceScope::Natal(PalaceKey)`，提供 `scope() -> PalaceScope`；`key()` 从该作用域投影宫位键。`Palace` 不再保存 `name_hans`、`name_hant`，也不再提供同名方法。 | 已确认 |
| D-184 | 删除 `DecadePalaceKey`、`YearlyPalaceKey`、`Decade` 与 `Yearly`。指定大限和流年时分别按需返回 `[PalaceScope; 12]`；四个限运领域值的文件归属已被 D-185 替换。 | 已确认 |
| D-185 | `domain/period.rs` 承载完整限运领域；当前保存 `DecadeIndex`、`YearlyIndex`、`DecadeAge` 与 `DecadeYear`，未来的流月、流日、流时领域值也归入该模块。不因共同模块而引入无行为的公开 `Period` 枚举、结构体或 trait，crate 根公开类型保持不变。 | 已确认 |
| D-186 | 唯一领域引擎的 Cargo 包名、Rust import 名与目录名统一为 `ziwei`；不保留 `ziwei_core` 兼容包或创建纯重导出门面。未来 adapter 直接单向依赖 `ziwei`。 | 已确认 |
| D-154 | `PalaceTransformation` 的源宫、目标宫定位字段及相关查询暂缓设计；当前不确认 `source_palace`、源／目标地支或目标宫名字段。 | 暂缓 |
## 暂缓决策

| ID | 决策 | 状态 |
| --- | --- | --- |
| D-021 | 连续飞化可从一条确定的宫干四化关系开始；同时支持指定一个源宫并返回禄、权、科、忌四组路径。 | 暂缓；不进入 V1 API、实现或测试 |
| D-022 | 一条飞化到达的目标宫同时存在自化与生年四化时，两种事实均各自生成连续飞化分支。 | 暂缓；恢复功能时重新核验 |
| D-023 | 连续飞化遇自化时的续飞与终止规则。 | 暂缓；未决 |
| D-024 | 连续飞化遇生年四化和无特殊事实时的续飞规则。 | 暂缓；未决 |
