# `ziwei_query` 公共接口

本文把 [`ziwei-query.md`](ziwei-query.md) 已确认的分层、能力与接口决定收敛成第一版 Rust 公共面。当前实现位于 `crates/ziwei_query`，并由 `ziwei` 门面选择性重导出。

最小用法如下：

```rust
use ziwei::{
    DecadeYearOrdinal, Gender, PalaceName, ZiweiBirth, create_from_birth, query,
};

let birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4)?;
let natal = create_from_birth(birth);
let query = query(&natal);

let ming = query.natal().palace(PalaceName::Ming);
assert_eq!(ming.relative_name(), PalaceName::Ming);

let first_decade = query.decades().next().expect("twelve decades always exist");
let first_year = first_decade.year(DecadeYearOrdinal::try_new(1)?);
assert_eq!(first_year.decade().fact().index(), first_decade.fact().index());
# Ok::<(), Box<dyn std::error::Error>>(())
```

## 公开入口与范围选择

查询只借用 `Natal`。下列稳定类型与 `query` 函数由 `ziwei` 选择性重导出到根路径；普通用户不直接依赖 `ziwei_query`。

```rust
pub fn query(natal: &Natal) -> Query<'_>;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Query<'a> { /* private */ }

impl<'a> Query<'a> {
    pub fn fact(self) -> &'a Natal;
    pub fn natal(self) -> ReframeScope<'a>;

    pub fn decade(self, index: DecadeIndex) -> DecadeScope<'a>;
    pub fn decade_year_at_age(
        self,
        age: u8,
    ) -> Result<DecadeYearSelection<'a>, DecadeAgeError>;
    pub fn decade_year_at_lunar_year(
        self,
        year: i32,
    ) -> Result<DecadeYearSelection<'a>, DecadeLunarYearError>;

    pub fn decades(
        self,
    ) -> impl ExactSizeIterator<Item = DecadeScope<'a>>
           + DoubleEndedIterator
           + 'a;
}
```

按索引只定位大限；按虚岁或农历年一次定位到大限中的具体年份。

```rust
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeScope<'a> { /* private */ }

impl<'a> DecadeScope<'a> {
    pub fn fact(self) -> &'a Decade;
    pub fn year(self, ordinal: DecadeYearOrdinal) -> DecadeYearSelection<'a>;
    pub fn previous_decade(self) -> Option<DecadeScope<'a>>;
    pub fn next_decade(self) -> Option<DecadeScope<'a>>;

    // 直接提供 ReframeScope 的全部宫位、星曜、四化、关系与条件方法。
}

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeYearSelection<'a> { /* private */ }

impl<'a> DecadeYearSelection<'a> {
    pub fn fact(self) -> &'a DecadeYear;
    pub fn ordinal(self) -> DecadeYearOrdinal;
    pub fn decade(self) -> DecadeScope<'a>;
    pub fn previous_year(self) -> Option<DecadeYearSelection<'a>>;
    pub fn next_year(self) -> Option<DecadeYearSelection<'a>>;
}
```

`DecadeYearSelection` 仍属于 L2，定位被选中的 `DecadeYear`，并关联其限内序号与所属 `DecadeScope`；它不是新的 scope，也不产生流年宫位、星曜或四化。需要查询该年份所在大限时，先通过 `decade()` 回到 `DecadeScope`。

全部查询句柄的 `PartialEq / Eq` 遵循不可变命盘事实与 scope 的结构化值语义：由内容相等的 `Natal` 建立且坐标相同的句柄相等，指向不同事实或不同坐标的句柄不相等。

## 立极范围

```rust
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReframeScope<'a> { /* private */ }

impl<'a> ReframeScope<'a> {
    pub fn palace(self, name: PalaceName) -> ScopedPalace<'a>;
    pub fn palace_at(self, branch: Branch) -> ScopedPalace<'a>;
    pub fn palace_by_natal_name(self, name: PalaceName) -> ScopedPalace<'a>;
    pub fn palaces_with_stem(
        self,
        stem: Stem,
    ) -> impl Iterator<Item = ScopedPalace<'a>> + 'a;
    pub fn palaces(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedPalace<'a>>
           + DoubleEndedIterator
           + 'a;

    pub fn shen_palace(self) -> ScopedPalace<'a>;
    pub fn origin_palace(self) -> ScopedPalace<'a>;

    pub fn star(self, name: StarName) -> ScopedStar<'a>;
    pub fn stars(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedStar<'a>>
           + DoubleEndedIterator
           + 'a;
    pub fn shared_palace(self, stars: &[StarName]) -> Option<ScopedPalace<'a>>;

    pub fn birth_transformation(
        self,
        transformation: Transformation,
    ) -> ScopedStar<'a>;
    pub fn birth_transformations(
        self,
    ) -> [(Transformation, ScopedStar<'a>); 4];
    pub fn palace_transformations(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedPalaceTransformation<'a>>
           + DoubleEndedIterator
           + 'a;

    pub fn palace_lines(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedPalaceLine<'a>>
           + DoubleEndedIterator
           + 'a;
    pub fn trine_groups(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 3]>
           + DoubleEndedIterator
           + 'a;
    pub fn four_cardinal_groups(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 4]>
           + DoubleEndedIterator
           + 'a;
    pub fn essence_relations(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 2]>
           + DoubleEndedIterator
           + 'a;
    pub fn six_harmonies(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 2]>
           + DoubleEndedIterator
           + 'a;
}
```

关系方法的英文标识固定为：`converge` 对应会，`trine` 对应三方，`four_cardinals` 对应四正，`essence` 对应河图，`six_harmony` 对应当前宫的六合宫；`six_harmonies` 遍历全部六组六合关系。

`DecadeScope` 对外直接提供上述同名方法，implementation 委托给其内部立极坐标，不通过 `Deref` 暴露。两种 scope 的公共方法保持单一内部声明源；新增 L1 能力不手写修改两份 interface，具体生成或委托机制在 implementation 阶段决定。`DecadeYearSelection` 不直接提供这些方法，避免被误解为流年 scope。

## 宫位与星曜

```rust
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedPalace<'a> { /* private */ }

impl<'a> ScopedPalace<'a> {
    pub fn fact(self) -> &'a Palace;
    pub fn relative_name(self) -> PalaceName;
    pub fn natal_name(self) -> PalaceName;
    pub fn reframe(self) -> ReframeScope<'a>;

    pub fn stars(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedStar<'a>>
           + DoubleEndedIterator
           + 'a;

    pub fn opposite(self) -> ScopedPalace<'a>;
    pub fn converge(self) -> [ScopedPalace<'a>; 2];
    pub fn trine(self) -> [ScopedPalace<'a>; 3];
    pub fn four_cardinals(self) -> [ScopedPalace<'a>; 4];
    pub fn line(self) -> ScopedPalaceLine<'a>;
    pub fn essence(self) -> ScopedPalace<'a>;
    pub fn essence_source(self) -> ScopedPalace<'a>;
    pub fn six_harmony(self) -> ScopedPalace<'a>;

    pub fn palace_transformation(
        self,
        transformation: Transformation,
    ) -> ScopedPalaceTransformation<'a>;
    pub fn palace_transformations(self) -> [ScopedPalaceTransformation<'a>; 4];
    pub fn incoming_palace_transformations(
        self,
    ) -> impl Iterator<Item = ScopedPalaceTransformation<'a>> + 'a;

    pub fn has_star(self, star: StarName) -> bool;
    pub fn has_all_stars(self, stars: &[StarName]) -> bool;
    pub fn has_any_stars(self, stars: &[StarName]) -> bool;
    pub fn has_no_stars(self, stars: &[StarName]) -> bool;
    pub fn is_empty_palace(self) -> bool;

    pub fn converge_has_all_stars(self, stars: &[StarName]) -> bool;
    pub fn converge_has_any_stars(self, stars: &[StarName]) -> bool;
    pub fn converge_has_no_stars(self, stars: &[StarName]) -> bool;
    pub fn converge_birth_transformation(
        self,
        transformation: Transformation,
    ) -> Option<ScopedStar<'a>>;
    pub fn opposite_birth_transformation(
        self,
        transformation: Transformation,
    ) -> Option<ScopedBirthTransformationOpposition<'a>>;
}

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedStar<'a> { /* private */ }

impl<'a> ScopedStar<'a> {
    pub fn fact(self) -> &'a Star;
    pub fn palace(self) -> ScopedPalace<'a>;
    pub fn incoming_palace_transformations(
        self,
    ) -> impl Iterator<Item = ScopedPalaceTransformation<'a>> + 'a;
    pub fn has_inward_self_transformation(
        self,
        transformation: Transformation,
    ) -> bool;
    pub fn has_outward_self_transformation(
        self,
        transformation: Transformation,
    ) -> bool;
}

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedPalaceTransformation<'a> { /* private */ }

impl<'a> ScopedPalaceTransformation<'a> {
    pub fn fact(self) -> &'a PalaceTransformation;
    pub fn source(self) -> ScopedPalace<'a>;
    pub fn target(self) -> ScopedPalace<'a>;
    pub fn star(self) -> ScopedStar<'a>;
}
```

查询对象不重复 `Palace`、`Star`、`PalaceTransformation` 的静态 getter；原始事实统一从 `fact()` 读取。

## 关系结果

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PalaceLine {
    MingQian,
    XiongYou,
    FuGuan,
    ZiTian,
    FuCai,
    FuJi,
}

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedPalaceLine<'a> { /* private */ }

impl<'a> ScopedPalaceLine<'a> {
    pub fn name(self) -> PalaceLine;
    pub fn palaces(self) -> [ScopedPalace<'a>; 2];
}

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopedBirthTransformationOpposition<'a> {
    Zhao(ScopedStar<'a>),
    Chong(ScopedStar<'a>),
}

impl<'a> ScopedBirthTransformationOpposition<'a> {
    pub fn star(self) -> ScopedStar<'a>;
}
```

`PalaceLine` 的六个枚举项依次表示命迁、兄友、夫官、子田、财福、父疾六条宫线；财福线使用已确认的标识符 `FuCai`。

## 限内年份与错误

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecadeYearOrdinal(u8);

impl DecadeYearOrdinal {
    pub fn try_new(value: u8) -> Result<Self, DecadeYearOrdinalError>;
    pub fn get(self) -> u8;
}

impl TryFrom<u8> for DecadeYearOrdinal {
    type Error = DecadeYearOrdinalError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeAgeError { /* private */ }

impl DecadeAgeError {
    pub fn age(self) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecadeLunarYearError {
    BirthYearUnavailable { year: i32 },
    OutsideDecades { year: i32 },
}

impl DecadeLunarYearError {
    pub fn year(self) -> i32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeYearOrdinalError { /* private */ }

impl DecadeYearOrdinalError {
    pub fn value(self) -> u8;
}
```

`DecadeYearOrdinal` 的合法范围固定为一基的 `1..=10`；`try_new()` 与 `TryFrom<u8>` 使用相同验证规则。三个错误都实现 `Display` 与 `std::error::Error`，不定义覆盖全部查询的总 `QueryError`。

## 集合与缺省语义

- `palaces()` 按当前相对 `PalaceName::ALL`；`stars()` 按 `StarName::ALL`；`decades()` 按 `DecadeIndex` 自然顺序。
- 全部四十八条宫位四化按当前相对源宫顺序，再按 `A / B / C / D`；飞入指定宫位或星曜的反查是对该有序集合的稳定过滤，保持相同相对顺序。
- 宫线、三方、四正、河图与暗合保持能力设计中确认的领域顺序。
- 确定存在的宫位、星曜和四化直接返回；正常缺省与首尾导航返回 `Option`；虚岁、农历年等外部范围输入失败返回精确 `Result`。
- 星曜切片按集合语义处理重复项；空集合固定为 `all = true`、`any = false`、`none = true`。
- `shared_palace([])` 返回 `None`；单颗星返回其所在宫；多颗星同宫时返回该宫。
- 反查和跨十二大限条件查询使用上述固定顺序迭代器配合标准 `filter`，不建立 `Condition`、operation 或查询 DSL。

## 尚未进入接口的内容

- `ziwei_query` 私有模块、字段与索引表示。
- 两种 scope 共享方法的具体生成或委托机制。
- `const`、`inline`、缓存与其他微观优化。
- 流年、流月、流日、流时 scope，以及内核未保存的流曜和四化。
