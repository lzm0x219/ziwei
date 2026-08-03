# Ziwei

MathArts 紫微斗数排盘引擎：从已消解的农历出生资料构造可查询命盘。

## Language

**ZiweiBirth**:
供 `from_birth` 使用的农历出生资料：性别，以及打平的年序号、月位（正月 = 0）、日（初一 = 1）、时辰位（子时 = 0）。不嵌套日期对象。
_Avoid_: BirthInfo、NormalizedDate（作入口嵌套）、公历生日、未消解的时钟时刻

**ZiweiInput**:
`from_input` 的原始量捷径：性别、年干支、月/日/时（零点同 `ZiweiBirth`）。不含命宫、紫微等排盘结果；那些由引擎计算。
_Avoid_: 命宫位注入、紫微位注入、完整十二宫结果图、出生证明

**from_birth**:
从 `ZiweiBirth` 走权威全管道构造命盘的入口；排盘结果一律引擎算。
_Avoid_: calculate、排盘（作方法名时）

**from_input**:
从 `ZiweiInput` 走原始量捷径构造命盘：年柱已由调用方给出，安宫安星仍引擎算；不替代 `from_birth` 的权威年序号路径。
_Avoid_: 注入安星结果、快捷跳过规则

**农历年序号**:
调用方已消解的农历年整数；引擎用 `(year - 4) rem 10/12` 推导年干支，不做公历换算。
_Avoid_: 公历年、年柱字符串
