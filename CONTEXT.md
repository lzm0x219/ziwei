# Ziwei

MathArts 紫微斗数排盘引擎：从已消解的农历出生资料构造可查询命盘。

## Language

**ZiweiBirth**:
供 `from_birth` 使用的农历出生资料：性别，以及打平的年序号、月位（正月 = 0）、日（初一 = 1）、时辰位（子时 = 0）。不嵌套日期对象。
_Avoid_: BirthInfo、NormalizedDate（作入口嵌套）、公历生日、未消解的时钟时刻

**ZiweiInput**:
已预处理的排盘捷径资料（含月时位、命宫位、紫微位等），供 `from_input` 测例与注入路径使用，不是权威出生叙述。
_Avoid_: 出生证明、完整历法结果

**from_birth**:
从 `ZiweiBirth` 走权威全管道构造命盘的入口。
_Avoid_: calculate、排盘（作方法名时）

**from_input**:
从 `ZiweiInput` 构造命盘的预处理/测例入口；不替代 `from_birth` 的权威语义。
_Avoid_: 快捷构造（若暗示可跳过规则）

**农历年序号**:
调用方已消解的农历年整数；引擎用 `(year - 4) rem 10/12` 推导年干支，不做公历换算。
_Avoid_: 公历年、年柱字符串
