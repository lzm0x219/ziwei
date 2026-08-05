# Research: 五虎遁算法核验

**Status:** research only
**Scope:** `Stem::yin_head_stem`、`compute_palace_stems`，以及它们与来因宫固定表的边界
**Conclusion:** 当前五虎遁实现正确；来因宫由独立表确定，但其宫干必须与生年天干一致

## 1. 结论

[`Stem::yin_head_stem`](../../crates/ziwei_core/src/stem.rs) 与
[`compute_palace_stems`](../../crates/ziwei_core/src/placement.rs) 共同实现了标准五虎遁：

| 生年干 | 寅宫起干 |
| ------ | -------- |
| 甲、己 | 丙       |
| 乙、庚 | 戊       |
| 丙、辛 | 庚       |
| 丁、壬 | 壬       |
| 戊、癸 | 甲       |

从寅宫起干后，天干按 `甲 → 乙 → … → 癸 → 甲` 顺序，沿
`寅 → 卯 → 辰 → 巳 → 午 → 未 → 申 → 酉 → 戌 → 亥 → 子 → 丑`
逐宫顺布。代码的起干表、方向、零点换算和十干循环均与此一致，无须修改。

## 2. 来源与证据边界

### 2.1 历法原典影印本

《钦定协纪辨方书》卷一“本原一”设“五虎遁”条，列出上述五组
“年干 → 正月寅月干”映射，并说明甲年由丙寅起，二月为丁卯，随后依次顺数。

- [韩国国立中央图书馆馆藏记录](https://www.nl.go.kr/NL/contents/search.do?#viewKey=CNTS-00115845363&viewType=C)
- [1741 年木板本影印件及目录定位](https://commons.wikimedia.org/wiki/File:CNTS-00115845363_%E6%AC%BD%E5%AE%9A%E5%8D%94%E7%B4%80%E8%BE%A8%E6%96%B9%E6%9B%B8.pdf)：目录将“五虎遁”定位在 PDF 第 168 页
- [四库全书本文字转录，卷一](https://zh.wikisource.org/zh-hant/%E6%AC%BD%E5%AE%9A%E5%8D%94%E7%B4%80%E8%BE%A8%E6%96%B9%E6%9B%B8_%28%E5%9B%9B%E5%BA%AB%E5%85%A8%E6%9B%B8%E6%9C%AC%29/%E5%8D%B701)：便于检索，关键顺布文字为“二月丁卯以次順數”

这是本次核验的主要证据：馆藏影印本确认版本和原始页面，文字转录用于定位和阅读，
不把转录本本身当作校勘本。

### 2.2 更早的文献交叉证据

宋廖中《五行精纪》卷二十八“起月建例”保存了完整五虎遁歌诀；其后注文再次逐项列出
丙寅、戊寅、庚寅、壬寅、甲寅五组起例。这与清代《钦定协纪辨方书》完全一致。

- [国家图书馆来源影印件，PDF 第 45 页、卷二十八叶五](https://commons.wikimedia.org/w/index.php?page=45&title=File%3ANLC892-411999013122-114503_%E4%BA%94%E8%A1%8C%E7%B2%BE%E7%B4%80_%E7%AC%AC4%E5%86%8A.pdf)
- [《五行精纪》可检索文字](https://zh.wikisource.org/zh-hant/%E4%BA%94%E8%A1%8C%E7%B2%BE%E7%B4%80)

该文献同样证明的是年干起月建规则，不涉及紫微斗数的来因宫定义。

### 2.3 紫微斗数语境

《紫微斗数全书》卷二“起五行寅例”列出同一五组起干；同卷“安身命例”以
甲年寅宫起丙、下一宫卯为丁的例子，说明该规则被用于紫微斗数十二宫排布。

- [《紫微斗数全书》卷二文字转录](https://zh.wikisource.org/zh-hant/%E7%B4%AB%E5%BE%AE%E6%96%97%E6%95%B8%E5%85%A8%E6%9B%B8/%E5%8D%B7%E4%BA%8C)

本次没有找到并核验可公开访问的该书早期影印本，因此这条只作为紫微斗数应用语境的
交叉证据；它的版本沿革和文字校勘不在本结论覆盖范围内。算法本身仍由上面的历法原典
影印本支撑。

## 3. 代码核对

### 3.1 `yin_head_stem`

函数的五个匹配分支逐项对应原典：

```text
Jia | Ji   -> Bing
Yi  | Geng -> Wu
Bing| Xin  -> Geng
Ding| Ren  -> Ren
Wu  | Gui  -> Jia
```

结论：分组与起干均正确。

### 3.2 `compute_palace_stems`

仓库公开宫位数组采用子序，即 `子=0、丑=1、寅=2、…、亥=11`；五虎遁运算采用
寅环，即 `寅=0、卯=1、…、丑=11`。[`position.rs`](../../crates/ziwei_core/src/position.rs)
中的 `branch_index_to_yin0` 映射为：

```text
子  丑  寅  卯  辰  巳  午  未  申  酉  戌  亥
10  11  0   1   2   3   4   5   6   7   8   9
```

`compute_palace_stems` 对每一支计算：

```text
stem = (寅宫起干下标 + 寅环下标) mod 10
```

因此它虽然返回“子为零”的数组，实际顺布方向仍是寅到丑，且跨癸后正确回到甲。
结论：坐标换算、顺布方向和循环边界均正确。

### 3.3 丙年验算

丙年由庚寅起：

| 支 | 寅 | 卯 | 辰 | 巳 | 午 | 未 | 申 | 酉 | 戌 | 亥 | 子 | 丑 |
| -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- |
| 干 | 庚 | 辛 | 壬 | 癸 | 甲 | 乙 | 丙 | 丁 | 戊 | 己 | 庚 | 辛 |

所以当前算法得到“壬辰”和“丙申”都正确。它们只描述丙年十二宫的宫干分布。

## 4. 与来因宫的边界

五虎遁和来因宫固定表是两条独立规则：

- 五虎遁回答：某生年干下，十二地支宫各是什么宫干。
- [`origin_palace_branch`](../../crates/ziwei_core/src/stem.rs) 与
  [ADR-0005](../adr/0005-laiyin-palace.md) 回答：该生年干的来因宫落在哪一地支。

项目同时要求两条规则的结果满足不变量：来因宫宫干等于生年天干。因此丙年的
`origin_palace` 是申；五虎遁独立算得申宫干为丙。实现仍按固定表确定来因宫，不在运行时
扫描宫干。

`Star::origin_transformation` 可直接由生年天干查四化表。由于上述宫干相等不变量，同一组
四化也出现在 `origin_palace` 的 `PalaceTransformation` 中。

## 5. 测试覆盖

`placement.rs` 的表驱动测试
`palace_stems_follow_five_tiger_escape_for_every_year_stem_and_branch` 现已锁定：

1. 五组年干共享各自的寅宫起干；
2. 每组从寅至丑逐宫前进一干；
3. 返回数组仍按 `Branch::index()` 的子序索引。
