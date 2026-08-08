# Workspace 边界

## 当前 crate 职责

| crate | 当前职责 |
| --- | --- |
| `ziwei_core` | 验证紫微输入并计算不可变 `Natal`；包含宫位、星曜、四化、自化、生肖、五行局和最小大限年份/虚岁事实 |
| `ziwei` | 面向调用方的统一门面，选择性重导出稳定 core 事实与 query 查询类型 |
| `ziwei_query` | 借用 `Natal` 的只读查询层；提供立极、大限、宫位、星曜、四化、固定关系与首批条件组合 |
| `ziwei_calendar` | 保留的历法边界；当前不实现产品能力 |
| `ziwei_analysis` | 保留的分析边界；当前不实现产品能力 |
| bindings crates | 未来语言绑定；当前不实现 |

依赖方向保持单向：`ziwei_query -> ziwei_core`，`ziwei -> ziwei_core + ziwei_query`；`ziwei_core` 不依赖查询、历法、分析或 bindings。

## 子包边界

`ziwei_query` 已通过 `ziwei` 门面交付可工作的端到端查询能力，只依赖 `ziwei_core`。`ziwei_calendar` 与 `ziwei_analysis` 仍是保留边界，不代表对应产品能力已经实现；只有出现真实消费者并能交付可工作的端到端切片时，才加入实现和必要依赖。

## 为什么大限留在 core

大限方向、起限年龄、命宫移动和十个连续虚岁都是由同一出生输入确定的命盘事实，因此十二个 `Decade` 属于 `Natal`。有绝对农历年序号的输入同时产生 `DecadeYear.year`；无绝对年序号输入只产生虚岁。这里的 `year` 是历法层归一化后的农历年序号。

“从某年查哪个大限”“为某年生成流年盘”是条件查询或新一轮计算，不是存储事实，留给 `ziwei_query` 或后续独立能力。
