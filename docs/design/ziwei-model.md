# `Natal` 领域模型

本文定义 `ziwei_core` 重写后的权威结果结构。它描述已确认的存储事实，不把查询便利方法或未来能力混入内核。

## 根结构

```rust
pub struct Natal {
    context: NatalContext,
    zodiac: Zodiac,
    palaces: [Palace; 12],

    ming_palace_branch: Branch,
    shen_palace_name: PalaceName,
    shen_palace_branch: Branch,
    origin_palace_name: PalaceName,
    origin_palace_branch: Branch,

    bureau: FiveElementBureau,
    decade_direction: DecadeDirection,
    decades: [Decade; 12],
}
```

所有字段私有。`create_from_birth(ZiweiBirth)` 与 `create_from_input(ZiweiInput)` 是两条公开、无失败的构造入口；两者必须进入同一条归一化计算流水线。

```rust
pub struct NatalContext {
    gender: Gender,
    year: Option<i32>,
    birth_stem: Stem,
    birth_branch: Branch,
    month: u8,
    day: u8,
    hour: u8,
}
```

`NatalContext` 只由已验证输入生成，不公开构造，也不成为第三种输入。`ZiweiBirth.year` 是历法层归一化后的农历年序号，必须保证十二个大限中的全部农历年份都能以 `i32` 表示；`ZiweiInput` 的 `year` 固定为 `None`。

## 宫位与星曜

```rust
pub struct Palace {
    name: PalaceName,
    branch: Branch,
    stem: Stem,
    stars: Vec<Star>,
    transformations: [PalaceTransformation; 4],
}

pub struct Star {
    name: StarName,
    category: StarCategory,
    galaxy: Option<StarGalaxy>,
    origin_transformation: Option<Transformation>,
    self_transformations: StarSelfTransformations,
}

pub struct StarSelfTransformations {
    inward: Option<Transformation>,
    outward: Option<Transformation>,
}
```

`PalaceName` 只改原 `PalaceRole` 的类型名，十二个枚举成员及次序保持不变：

```text
Ming, XiongDi, FuQi, ZiNv, CaiBo, JiE,
QianYi, JiaoYou, GuanLu, TianZhai, FuDe, FuMu
```

`palaces` 的数组坐标不是 `Branch` 的内部序号，而是固定以寅为零：

```text
0 Yin, 1 Mao, 2 Chen, 3 Si, 4 Wu, 5 Wei,
6 Shen, 7 You, 8 Xu, 9 Hai, 10 Zi, 11 Chou
```

每宫的 `stars` 只存落在该宫的星曜，并按 `StarName::ALL` 的相对次序稳定排列。全盘十八个 `StarName` 各出现一次。

`StarName` 只承载稳定身份。类别与斗系存储在 `Star`：

- `StarCategory`: `Major / Minor / Auxiliary`；十四主星为 `Major`，左辅、右弼、文昌、文曲为 `Minor`，首批没有 `Auxiliary` 成员。
- `StarGalaxy`: `South / North / Central`，分别表示南斗、北斗、中斗；没有斗系的星返回 `None`。

`StarName::ALL` 沿用当前十八星算法顺序，不参与决定安星先后；安星结果由计算规则决定，数组只提供稳定遍历顺序。

## 四化关系

```rust
pub enum Transformation { A, B, C, D }

pub struct PalaceTransformation {
    source_name: PalaceName,
    source_branch: Branch,
    transformation: Transformation,
    target_name: PalaceName,
    target_branch: Branch,
    star_name: StarName,
}
```

`A / B / C / D` 是领域稳定代码，展示层可映射为禄、权、科、忌。每个 `Palace` 保存以自身为源宫的四条关系，顺序为 `A / B / C / D`。每条关系的目标宫必须包含且只包含对应 `star_name`。

由生年天干所飞化的生年四化直接存于目标 `Star.origin_transformation`。来因宫与五虎遁宫干各自独立计算，但来因宫宫干必须与生年天干一致，因此同一组四化也存在于 `origin_palace_name` 所指宫位的 `PalaceTransformation` 中，两者按目标 `star_name` 与 `transformation` 一一对应。全盘恰有四个 `Some`，并且 `A / B / C / D` 各一次，不再另存顶层数组。

自化也存于目标星曜：

- 源宫地支等于目标宫地支时，`outward = Some(transformation)`；
- 源宫与目标宫相对时，`inward = Some(transformation)`；
- 两者可同时存在，各自至多一个；
- 它们必须能从 `PalaceTransformation` 唯一复算。

## 生肖与大限

`Zodiac` 由出生地支计算并存入 `Natal`：子鼠、丑牛、寅虎、卯兔、辰龙、巳蛇、午马、未羊、申猴、酉鸡、戌狗、亥猪。

```rust
pub enum DecadeDirection { Forward, Reverse }

pub struct Decade {
    index: DecadeIndex,
    ming_palace_branch: Branch,
    years: [DecadeYear; 10],
}

pub struct DecadeYear {
    year: Option<i32>,
    age: u8,
}
```

`DecadeDirection::Forward` 当且仅当出生年干阴阳与性别阴阳相同。第零大限命宫地支等于本命命宫地支，后续按方向逐宫移动。

十二个大限按 `DecadeIndex(0..=11)` 排列。每个大限存十个连续虚岁；`age_start()` 和 `age_end()` 分别读取首尾条目，不重复存储。`create_from_birth` 的 `year` 为 `birth_year + age - 1`，`create_from_input` 的 `year` 全部为 `None`；这里的年份均指农历年序号。

这里只保存“年份 + 虚岁”的最小流年事实。流年宫位、星曜、四化和按年份/年龄选择大限属于 `ziwei_query` 或后续独立计算，不进入 `Natal`。
