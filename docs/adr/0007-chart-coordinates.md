# 盘面坐标约定

排盘**规则内部**一律用**寅起环**（`yin0`，0=寅 … 10=子 11=丑），与口诀「寅起正月」及 backup 算法环一致。  
**对外 API** 用地支枚举 `Branch` 及「正常下标」：`Branch::index()` 为 **子=0** … 亥=11。禁止把未标注零点的裸 `u8` 当作宫位传出公共边界。

转换（`twelve_index` 只做模 12，不携带零点语义）：

```text
yin0         = (branch_index + 10) mod 12   // 子序 → 寅环
branch_index = (yin0 + 2) mod 12            // 寅环 → 子序
```

其它零点（与宫位环无关）：

- `ZiweiBirth.month`：正月 = 0
- `ZiweiBirth.hour`：子时 = 0（与 `Branch::Zi` 同序）

安命等口诀在寅环上算完，再转为 `Branch` 写入 `Ziwei`。迁移期若仍见 `ZiweiInput` 命宫/紫微 `u8`，文档曾写寅=0；目标形态不再注入这些结果字段（ADR-0002）。
