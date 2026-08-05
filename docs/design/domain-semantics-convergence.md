# 领域语义收敛

本文件记录重写后必须只有一种表达的核心语义，防止同一事实以顶层数组、边标记和查询返回值重复存在。

## 四化

- `Transformation` 只使用稳定代码 `A / B / C / D`。
- 禄、权、科、忌及简繁体文案属于外层 i18n，不进入 `ziwei_core`。
- 生年四化只存为 `Star.origin_transformation`，不再定义 `YearTransformation` 或 `OriginTransformation` 结果类型。
- 宫位四化关系只存为源 `Palace.transformations`，每宫固定四条。
- 自化只存为目标 `Star.self_transformations`，不在 `Palace` 或关系边上重复保存。

`PalaceTransformation` 同时保留源宫名/支和目标宫名/支。这些字段虽然能从外层与目标星曜推导，但作为一条完整领域关系，其源、目标坐标必须能独立校验；它不保留额外的 `self_transformation` 标记。

## 宫位坐标

`PalaceName` 表示宫名，`Branch` 表示空间地支。`Natal` 明确保存命宫、身宫、来因宫各自的宫名和地支：

```text
ming_palace / ming_palace_branch
body_palace / body_palace_branch
origin_palace / origin_palace_branch
```

三组坐标都必须解析到同一张盘中的唯一 `Palace`。`palaces` 对外顺序固定以寅为零，避免公开数组顺序依赖 `Branch` 的内部编码。

## 星曜身份与落位

`StarKey` 是星曜身份；`Star` 是一张具体本命盘内的落位结果。分类与斗系是 `StarKey` 的固有元数据，生年四化与自化是 `Star` 的盘内事实。

这一区分避免：

- 用显示字符串充当身份；
- 在宫位、关系边和顶层数组重复存同一颗星；
- 为尚未存在的 i18n 消费者预建 label 类型。

## 时间语义

`Decade` 是命盘整体的一部分，因此保存在 `Natal`。`DecadeYear` 只保存当前确定且无需查询才能成立的两个事实：`year: Option<i32>`（农历年序号）和虚岁 `age: u8`。

没有出生年份的输入仍可产生完整大限与虚岁，但不能伪造年份。流年盘、流年宫位和流年四化不是 `Natal` 的存储事实。

## 公开面

结果类型字段私有，只允许内核构造。公开 API 负责读取完整事实，不提供 `palace_at`、`branch_of_star`、按年龄找大限等条件查询；这些操作由未来 `ziwei_query` 组合完成。
