# 公开接口与领域类型

## 构造入口

`ziwei_core` 只公开两种输入和两条结果构造入口：

```rust
let natal = Natal::from_birth(ZiweiBirth::try_new(/* ... */)?);
let natal = Natal::from_input(ZiweiInput::try_new(/* ... */)?);
```

`NatalContext` 是内核保存的归一化只读事实，不公开构造，也不接受调用方直接传入。输入验证完成后，两个 `Natal` 构造入口均不再返回错误。

## 读取接口

多字段结果类型字段全部私有，按值返回小型 `Copy` 枚举和数字，按借用返回聚合：

```text
Natal.context() / zodiac() / palaces() / decades()
Palace.name() / branch() / stem() / stars() / transformations()
Star.key() / origin_transformation() / self_transformations()
Decade.index() / ming_palace_branch() / years() / age_start() / age_end()
DecadeYear.year() / age()
```

`year` 与 `age` 是公开内核中的规范领域名，分别指农历年序号与虚岁；不增加 `lunar_year` / `virtual_age` 或对应长前缀别名。

公开接口不提供可构造非法结果的 `new`，也不暴露可变集合。crate 内装配使用 `pub(crate)` 构造函数。

## 身份与文案

`Gender`、`Branch`、`Stem`、`PalaceName`、`StarKey`、`StarType`、`StarGalaxy`、`Transformation`、`Zodiac`、`DecadeDirection`、`DecadeIndex` 是稳定领域身份或值。

`Gender` 固定使用 `Yang / Yin` 表示阳男/阴女；阴阳是 core 的领域值，`Male / Female` 不属于公开内核语言。

`StarKey::as_str()` 提供机器稳定 key。宫名、星名、斗系及四化的简繁体文案不属于 core，待真实消费者出现后由 `ziwei` 外层适配器或独立 i18n 模块提供。

## 不属于公开内核的接口

以下旧式便利方法删除，不提供兼容别名：

- 按地支、宫名、星曜查找宫位或落位；
- 按年龄、年份或索引查找大限；
- 筛选四化来源或目标；
- `ZiweiView`、handle、查询错误类型；
- 旧 `Ziwei`、`PalaceRole`、`DecadeStep`、`ZiweiFly`、`YearTransformation` 名称。

这些需求出现时，由 `ziwei_query` 接收 `&Natal` 并返回借用或轻量查询结果，不能反向污染 `Natal` 的存储结构。

## 门面

`ziwei` crate 只选择性重导出已经确认的 core 公开类型，避免 glob 导出内部实现细节。其他 workspace crate 当前只保留空边界，不提前建设。
