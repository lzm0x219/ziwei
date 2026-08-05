# 大限顺逆

大限顺逆由**生年天干阴阳**与**性别**是否同属阳/阴决定，与口诀「阳男阴女顺、阴男阳女逆」及 backup `decadeDirection = (YIN_YANG[gender] === stemAttr) ? 1 : -1` 等价。

```text
year_yang   := year_stem ∈ {甲, 丙, 戊, 庚, 壬}
person_yang := gender = Yang   // Gender::Yang 阳/男；Gender::Yin 阴/女
forward     := year_yang == person_yang   // true=顺(+1), false=逆(-1)
```

- **顺：** 第一限在命宫，之后命 → 父 → 福 → 田 → …
- **逆：** 第一限在命宫，之后命 → 兄 → 夫 → 子 → …
- **起运：** 虚岁起点 = 五行局数；每限 10 年 `[start, start+9]`（地图 Notes，本 ADR 不改）。
- **Gender** 固定为 `Yang` / `Yin`：阴阳是排盘语义，男/女是对应解释；不使用 `Male` / `Female` 作为核心领域值，并与年干阴阳直接比较。
- 大限命宫叠落到某地支后，使用该支的本命宫干，不生成新的宫干。

样例：甲+Yang 顺、甲+Yin 逆、乙+Yang 逆、乙+Yin 顺。
