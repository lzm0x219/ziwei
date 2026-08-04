# 引擎结果类型字段不变量

引擎产出的多字段结果类型不得被外部自由构造。`Palace`、`ZiweiFly`、`DecadeStep`、`DecadeYear` 的字段私有，只经 crate 内装配；对外只提供 `const` 只读 getter。与输入侧 `ZiweiBirth` / `ZiweiInput` / `DecadeIndex` 同一模式：不变量由类型系统保证，不靠调用方自觉。

## 哪些类型要封

| 类型 | 字段组合不变量（引擎保证） |
| ---- | -------------------------- |
| `Palace` | 宫职逆布 + 宫支 + 五虎遁宫干一致 |
| `ZiweiFly` | 源支宫干 × 四化 → 星 → 本命落宫为目标支；自化标注与源支/目标支一致 |
| `DecadeStep` | `age_end = age_start + 9`；`ming_branch` 为大限命宫叠落地支 |
| `DecadeYear` | `lunar_year = birth_year + virtual_age - 1`（仅 `from_birth`） |

## 明确不封

- 纯显示标签：`PalaceRoleLabel`、`StarLabel`（字符串载荷，无组合不变量）。
- 身份枚举与查询选择器：`Branch` / `Stem` / `Star` / `PalaceRole` / `Transformation` / `ZiweiView` 等。

## 接口形态

```text
// 外部：只读
palace.role() / palace.branch() / palace.stem()
fly.source_branch() / fly.transformation() / fly.target_branch() / fly.star()
step.step() / step.ming_branch() / step.age_start() / step.age_end()
year.lunar_year() / year.virtual_age()

// 内部：pub(crate) new(...)，无公开构造、无兼容 struct 字面量
```

## 否决过的做法

- 公开字段 + 文档“请勿伪造” — 否决：黄金测例与 binding 仍可拼出非法组合。
- 公开 `try_new` 再校验组合 — 否决：结果不是输入；校验表会泄漏或重复整条排盘规则。
- 为显示标签也做私有字段 — 否决：无意义封装。
- 兼容层（`From` 旧公开字段 struct、双轨 API）— 否决：见 AGENTS 极端简单规则。
