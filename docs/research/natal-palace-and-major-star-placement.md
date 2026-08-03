# Research: 安命身十二宫 · 起寅首/宫干 · 十八主星安星

**Ticket:** [#244](https://github.com/matharts/ziwei/issues/244)  
**Map:** [#242](https://github.com/matharts/ziwei/issues/242)  
**Status:** research only — no placement implementation in library  
**Audience:** later grilling (#251) and implementation tickets

---

## 1. Scope and v1 star set

This note covers the **natal placement pipeline** up through the **default eighteen major stars** already encoded in `crates/ziwei/src/star.rs`:

| Group            | Stars (`Star` enum)                            |
| ---------------- | ---------------------------------------------- |
| 北斗系（紫微系） | 紫微、天机、太阳、武曲、天同、廉贞             |
| 南斗系（天府系） | 天府、太阴、贪狼、巨门、天相、天梁、七杀、破军 |
| 辅佐四星         | 左辅、右弼、文昌、文曲                         |

Together these are the classical **十四正曜 + 左辅右弼文昌文曲** set used as v1 default. Course notes (`course/NOTES.md`, learning-record `0008`) confirm eighteen stars; fifteen is treated as a subset view, not a second placement mode.

**Out of scope here:** 禄存/羊陀、魁钺、火铃、空劫、杂曜神煞全集、大限流年算法细节（map #242 already baseline-decided those separately）、批命解读。

---

## 2. Sources

### 2.1 Classical / first-party algorithm texts (preferred anchors)

| Source                             | Role                                               | Notes                                                                                                                                                                                         |
| ---------------------------------- | -------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 《紫微斗数全书》                   | Canonical print tradition for 安身命、十二宫、四化 | Course already cites [Wikisource 卷二](https://zh.wikisource.org/wiki/紫微斗數全書/卷二) for 安身命例 / 生年干四化. Treat Wikisource as a **retrievable transcript**, not a critical edition. |
| 《紫微斗数全集》                   | Parallel classical print                           | Wikipedia documents **庚/壬四化** divergence vs 《全书》 (see §7).                                                                                                                            |
| 《紫微斗数捷览》(明万历九年, 1581) | Earliest dated title bearing「紫微斗数」           | Historical anchor only; not used as step-by-step engine baseline.                                                                                                                             |
| 《洞微十八星断》(道藏)             | Classical “十八星” naming clue                     | [识典古籍 entry](https://www.shidianguji.com/book/DZ1485/chapter/DZ1485_10) — does **not** automatically equal the modern 14+4 set.                                                           |

### 2.2 Algorithm digests that restate classical 口诀 (reproducible steps)

These are **secondary** but carry the full 起例口诀 chain used by modern manual charting; cross-checked against each other and against repo types:

| Source                         | URL / location                                        | Use                                                                                                |
| ------------------------------ | ----------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| 福山堂 · 起例歌诀总括          | <http://www.fushantang.com/1012/1012c/j3084.html>     | Complete 安命身 / 定寅首 / 起紫微 / 安紫微诸星 / 安天府诸星 / 辅弼昌曲 口诀 + worked 紫微 examples |
| 维基百科 · 紫微斗数 · 推算方法 | <https://zh.wikipedia.org/zh-hans/紫微斗数>           | Ordered pipeline summary; 定紫微 algebraic sketch; 主星/辅星 spacing                               |
| 大纪元 · 排盘方法别            | <https://www.epochtimes.com/gb/9/6/22/n2565978.htm>   | Step list + 三月辰时命宫 worked example; table-driven 紫微 lookup presentation                     |
| 南中堂等口诀页                 | e.g. <https://www.ncc.com.tw/fate/paleo/gv/gv_15.htm> | Same 口诀 family as 福山堂 (cross-check wording)                                                   |

### 2.3 Repo-local fragments already encoding parts of the rules

| Artifact                                     | What it already freezes                                               |
| -------------------------------------------- | --------------------------------------------------------------------- |
| `Star` in `crates/ziwei/src/star.rs`         | v1 eighteen-star catalog                                              |
| `PalaceRole` in `crates/ziwei/src/palace.rs` | 十二宫职 order & labels（交友 = 仆役别名）                            |
| `Palace { role, branch, stem }`              | 宫职 + 宫支 + 宫干 fields                                             |
| `ZiweiInput` in `crates/ziwei/src/input.rs`  | Preprocessed path: gender, 年干支, 月位, 时位, **命宫位**, **紫微位** |
| `FiveElementBureau::from_ming_palace`        | 命宫干支 → 五行局（纳音局数表）                                       |
| `Stem::transformation_star`                  | 生年干四化表（与 §7 冲突注记对照）                                    |
| `position::twelve_index`                     | ring fold 0..=11                                                      |
| Course `RESOURCES.md` / lessons 0008–0011    | 全书卷二 / 十八星 / dual input paths                                  |

---

## 3. Coordinate conventions (critical for code)

Different zero points already coexist in the crate. Placement rules must not mix them silently.

| Quantity                           | Classical origin | Repo encoding     | Zero point           |
| ---------------------------------- | ---------------- | ----------------- | -------------------- |
| 宫支 `Branch`                      | 十二地支         | `Branch::index()` | **子 = 0** … 亥 = 11 |
| `ZiweiInput::birth_month_position` | 农历生月         | `u8` 0..=11       | **正月 = 0**         |
| `ZiweiInput::birth_hour_position`  | 生时             | `u8` 0..=11       | **子时 = 0**         |
| `ZiweiInput::ming_palace_position` | 命宫地支         | `u8` 0..=11       | **寅 = 0**           |
| `ZiweiInput::ziwei_star_position`  | 紫微落宫         | `u8` 0..=11       | **寅 = 0**           |

**Placement arithmetic in this note** uses **寅 = 0** ring indices unless stated otherwise:

```
0寅 1卯 2辰 3巳 4午 5未 6申 7酉 8戌 9亥 10子 11丑
```

Conversion:

```text
branch_index_zi0   = (yin0_index + 2) mod 12   // 寅=0 → 子=0 系
yin0_index         = (branch_index_zi0 + 10) mod 12
```

`twelve_index` already implements Euclidean fold into 0..=11 for signed offsets.

---

## 4. Step-by-step algorithm

Pipeline order (classical manuals and Wikipedia agree on this sequencing):

1. 安命宫、身宫
2. 定十二宫职
3. 起寅首 / 安十二宫干
4. 定命宫五行局
5. 定紫微
6. 定天府（由紫微对称）
7. 安紫微系五星 + 天府系七星
8. 安左辅、右弼、文昌、文曲

Gender is **not** required for any of steps 1–8 (it is required later for 大限顺逆, already baselined on map #242).

### Step A — 安命宫、身宫

**口诀（福山堂等同文）:**

> 寅起正月，顺数至生月，逆数生时为命宫。  
> 寅起正月，顺数至生月，顺数生时为身宫。

**Procedure:**

1. 固定地支盘；**从寅宫起正月**，顺行（寅→卯→…）数至农历生月，得「月宫」\(M\)。
2. **命宫:** 从 \(M\) 起子时，**逆行**数至生时，落宫为命宫 \(Ming\)。
3. **身宫:** 从 \(M\) 起子时，**顺行**数至生时，落宫为身宫 \(Shen\)。
4. 身宫不是第十三宫职；它 **叠落** 在十二宫之一的地支上（可能与命宫同宫：子/午/卯/酉等特定月时组合）。

**Ring formula（寅=0，月位 \(m\)、时位 \(h\) 同 `ZiweiInput`）:**

\[
\begin{aligned}
M &= m \bmod 12 \\
Ming &= (M - h) \bmod 12 \\
Shen &= (M + h) \bmod 12
\end{aligned}
\]

**Worked example（大纪元）:** 三月辰时

- \(m=2\) → \(M=\) 辰；\(h=\) 辰时 \(=4\)
- 命: \((2-4)\bmod 12 = 10\) → **子** ✓
- 身: \((2+4)\bmod 12 = 6\) → **申**

### Step B — 定十二宫职

**口诀:** 由命宫 **逆数** 兄弟、夫妻、子女、财帛、疾厄、迁移、奴仆/交友、事业/官禄、田宅、福德、父母。

Repo `PalaceRole` order (from 命起):

`Ming → XiongDi → FuQi → ZiNv → CaiBo → JiE → QianYi → JiaoYou → GuanLu → TianZhai → FuDe → FuMu`

**Procedure:** 命宫坐某地支后，沿 **地支逆序**（子→亥→戌→…，即 `Branch::index` 递减）依次安十二宫职。

**Ring:** if 命宫的 `Branch::index` is \(b\)，role index \(r \in 0..11\)（0=命）落在:

\[
\text{branch\_index}(r) = (b - r) \bmod 12
\]

Naming alias: classical 奴仆/仆役 = repo `JiaoYou`（交友）; 官禄 aka 事业。

### Step C — 起寅首 / 安十二宫干

**口诀（五虎遁月诀 = 定寅首）:**

> 甲己之年丙作首，乙庚之岁戊为头，  
> 丙辛之年寻庚起，丁壬壬寅顺水流，  
> 若问戊癸何处起，甲寅之上好追求。

| 生年干 | 寅宫起干 |
| ------ | -------- |
| 甲、己 | 丙       |
| 乙、庚 | 戊       |
| 丙、辛 | 庚       |
| 丁、壬 | 壬       |
| 戊、癸 | 甲       |

**Procedure:** 定寅宫天干后，**顺布** 其余十一宫干（寅丙则卯丁、辰戊…；至丑止）。每宫的 `(stem, branch)` 即该宫干支。

Wikipedia alternate wording「年干数×2+1 为寅宫干」与五虎遁 **结果一致**（在甲=1…癸=10 编号下）。

**Repo:** `Palace.stem` is exactly this 宫干; it is the base for later 自化（map #242）and for 命宫五行局.

### Step D — 定命宫五行局

**Rule:** 以 **命宫干支** 查 **纳音五行**，映射局数:

| 纳音五行 | 局     |
| -------- | ------ |
| 水       | 水二局 |
| 木       | 木三局 |
| 金       | 金四局 |
| 土       | 土五局 |
| 火       | 火六局 |

**Repo already implements** the standard 干支组表: `FiveElementBureau::from_ming_palace(stem, branch)` with tests locking the 5×6 group matrix (甲乙×子丑=金四, 甲乙×寅卯=水二, …). This matches 福山堂「以命宫天干地支而定」叙述。

局数 \(n \in \{2,3,4,5,6\}\) is input to 定紫微 and (per map #242) 大限起运虚岁.

### Step E — 定紫微

Requires: 农历 **出生日数** \(d\)（初一=1 …）与局数 \(n\)。

#### E.1 Recommended computational form（口诀「局数除日数」族）

福山堂口诀与例题可整理为（与下表查法等价）:

1. 令 \(q = \lceil d / n \rceil\)，\(e = q\cdot n - d\)（补足整除所需之「差」；若 \(d\) 整除 \(n\) 则 \(e=0\)，\(q = d/n\)）。
2. 以 **寅为第 1 步** 起算，先走到步数 \(q\) 的宫位（「商数宫前走 / 整除起虎口」）。
3. 若 \(e = 0\)：紫微即落该宫。
4. 若 \(e\) 为 **奇数**：从该宫 **逆行** \(e\) 宫。
5. 若 \(e\) 为 **偶数**：从该宫 **顺行** \(e\) 宫。

**Worked examples（福山堂）:**

| 日  | 局   | \(q,e\)           | 落宫                     |
| --- | ---- | ----------------- | ------------------------ |
| 27  | 木三 | \(q=9,e=0\)       | 从寅进 9 → **戌**        |
| 13  | 火六 | \(q=3,e=5\)（奇） | 寅进 3=辰，逆 5 → **亥** |
| 6   | 土五 | \(q=2,e=4\)（偶） | 寅进 2=卯，顺 4 → **未** |

**寅=0 index**（步数按「寅为 1」计）:

\[
\text{ziwei\_yin0} = \bigl((q - 1) + s\cdot e\bigr) \bmod 12,\quad
s = \begin{cases}0 & e=0\\ -1 & e\text{ odd}\\ +1 & e\text{ even}\end{cases}
\]

#### E.2 Table form

大纪元等教材用「局 × 日 → 地支」查表，结果应与 E.1 一致。实现时优先 **公式 + 黄金测例**，表可作为 property test 的期望生成器。

#### E.3 Wikipedia algebraic sketch

Wikipedia 给出另一套「倍数 / 差数 / 奇退偶进」叙述与公式草稿。**措辞比口诀例题更容易误读**（商、倍、差的边界）。以福山堂三例为验收金标，Wiki 公式仅作交叉参考，冲突时以口诀例题为准。

### Step F — 定天府（由紫微）

**口诀（福山堂）:**

> 天府南斗令，常对紫微宫，  
> 丑卯相更迭，未酉互为根。  
> 往来午与戌，蹀躞子和辰，  
> 巳亥交驰骋，同位在寅申。

**Equivalent geometry:** 紫微与天府关于 **寅–申轴** 镜像：

| 紫微    | 天府    |
| ------- | ------- |
| 寅 / 申 | 同宫    |
| 丑 ↔ 卯 | 子 ↔ 辰 | 亥 ↔ 巳 | 戌 ↔ 午 | 酉 ↔ 未 |

**Ring（寅=0）:**

\[
\text{tianfu\_yin0} = (-\text{ziwei\_yin0}) \bmod 12
\]

（与「从寅逆数与紫微顺数相同步数」等价。）

### Step G — 安其余十二正曜

#### G.1 紫微系（从紫微 **逆行**）

**口诀:**

> 紫微逆去天机星，隔一太阳武曲辰，  
> 连接天同空二宫，廉贞居处方是真。

| 相对紫微（逆行步数，寅=0 减） | 星     |
| ----------------------------- | ------ |
| 0                             | 紫微   |
| 1                             | 天机   |
| 2                             | （空） |
| 3                             | 太阳   |
| 4                             | 武曲   |
| 5                             | 天同   |
| 6–7                           | （空） |
| 8                             | 廉贞   |

#### G.2 天府系（从天府 **顺行**）

**口诀:**

> 天府顺行有太阴，贪狼而后巨门临，  
> 随来天相天梁继，七杀空三是破军。

| 相对天府（顺行步数，寅=0 加） | 星     |
| ----------------------------- | ------ |
| 0                             | 天府   |
| 1                             | 太阴   |
| 2                             | 贪狼   |
| 3                             | 巨门   |
| 4                             | 天相   |
| 5                             | 天梁   |
| 6                             | 七杀   |
| 7–9                           | （空） |
| 10                            | 破军   |

**Determinism:** 给定紫微位，十四正曜 **全部** 固定；无需月日时再参与（月日时已用于命宫/局/紫微）。

### Step H — 安辅佐四星（完成十八星）

**口诀:**

> 辰上顺正寻左辅，戌上逆正右弼当，  
> 辰上顺时文曲位，戌上逆时觅文昌。

| 星   | 起点     | 方向 | 计数 | 公式（寅=0）         |
| ---- | -------- | ---- | ---- | -------------------- |
| 左辅 | 辰起正月 | 顺   | 生月 | \((2 + m) \bmod 12\) |
| 右弼 | 戌起正月 | 逆   | 生月 | \((8 - m) \bmod 12\) |
| 文曲 | 辰起子时 | 顺   | 生时 | \((2 + h) \bmod 12\) |
| 文昌 | 戌起子时 | 逆   | 生时 | \((8 - h) \bmod 12\) |

正月左辅在辰、右弼在戌；子时文曲在辰、文昌在戌。

---

## 5. End-to-end dependency graph

```text
农历年干支 ──► 寅首/十二宫干 ──► (命宫干支) ──► 五行局 n
农历月 m、时 h ──► 命宫 Ming、身宫 Shen ──► 十二宫职
农历日 d、局 n ──► 紫微 ──► 天府 ──► 十四正曜
月 m ──► 左辅、右弼
时 h ──► 文昌、文曲
年干 ──► 生年四化（已实现表；见 §7 版本注记）
```

---

## 6. Alignment with existing types and dual entry points

### 6.1 What `ZiweiInput` already carries

From `input.rs` (all positions validated 0..=11):

| Field                         | Meaning    | Zero     |
| ----------------------------- | ---------- | -------- |
| `gender`                      | 命主性别   | —        |
| `birth_stem` / `birth_branch` | 出生年干支 | enums    |
| `birth_month_position`        | 生月       | 正月=0   |
| `birth_hour_position`         | 生时       | 子=0     |
| `ming_palace_position`        | 命宫       | **寅=0** |
| `ziwei_star_position`         | 紫微       | **寅=0** |

**Not carried:** 农历日数、身宫位、十二宫干、五行局、其余十七星、四化结果。

Course history: user defined precomputed path as「命盘各宫位置、紫微星位置、性别、出生天干、出生地支、出生月份位置、出生时辰位置」—— current struct stores **命宫单点** rather than full twelve-role map; twelve roles are always recoverable from 命宫 alone (Step B).

### 6.2 Draft boundary: engine-must-compute vs injectable

> **Non-binding draft for grilling #251.** Do not treat as ADR.

| Quantity                | `from_birth` (planned)                  | `from_input` (current shape) | Draft recommendation                                                                                                                                       |
| ----------------------- | --------------------------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 历法→农历 YMDH / 年干支 | **Outside engine** (map #242)           | N/A                          | Calendar stays external                                                                                                                                    |
| 月位 \(m\)、时位 \(h\)  | Derive from normalized lunar date       | **Inject**                   | Inject OK; pure calendar labels                                                                                                                            |
| 年干支                  | From lunar year                         | **Inject**                   | Inject OK                                                                                                                                                  |
| **命宫**                | **Must compute** (Step A)               | Currently **inject**         | Prefer always compute when \(m,h\) present; if inject, **validate** equals \((m-h)\bmod 12\)                                                               |
| **身宫**                | **Must compute**                        | Missing                      | Always compute; do not inject                                                                                                                              |
| **十二宫职**            | **Must compute** from 命                | Always derive                | Always compute; never inject full role map as source of truth                                                                                              |
| **十二宫干**            | **Must compute** (五虎遁)               | Derive from `birth_stem`     | Always compute; `Palace.stem` is output                                                                                                                    |
| **五行局**              | **Must compute**                        | Derive via existing API      | Always compute; optional inject only if cross-checked                                                                                                      |
| **农历日 \(d\)**        | Required for 紫微                       | **Missing today**            | `from_birth` must supply day; consider adding to input path if verification desired                                                                        |
| **紫微**                | **Must compute** (needs \(d,n\))        | Currently **inject**         | `from_birth`: always compute. `from_input`: inject allowed for fixtures, but **prefer recompute** when day is available; otherwise document trust boundary |
| **天府 + 其余 12 正曜** | **Must compute** from 紫微              | Always derive                | Always compute once 紫微 known — never inject star-by-star for 14 主星                                                                                     |
| **左辅右弼文昌文曲**    | **Must compute** from \(m,h\)           | Always derive                | Always compute                                                                                                                                             |
| **生年四化**            | Compute via `Stem::transformation_star` | Same                         | Always compute from year stem                                                                                                                              |
| **大限序列**            | Compute (map rules)                     | Not in input                 | Always compute; do not inject decade sequence as natal fact                                                                                                |

### 6.3 Risks if injection is unconstrained

1. **Hollow golden tests:** injecting both 命宫 and 紫微 without recomputation lets fixtures hard-code “answers” that never exercise Steps A/E.
2. **Internal inconsistency:** injected 命宫 may disagree with \(m,h\); injected 紫微 may disagree with \(d\)+局.
3. **Missing day:** current `ZiweiInput` **cannot** recompute or verify 紫微; it can only fan out Steps F–H from the injected 紫微 + \(m,h\).

### 6.4 Practical split proposal (for #251)

| Path                | Trust model                                                                                                                                                                                                            |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ziwei::from_birth` | Engine-authoritative full pipeline A–H (+ later 大限/流年).                                                                                                                                                            |
| `Ziwei::from_input` | **Dev/test accelerator**: may accept 命/紫微 as hints, but v1 production semantics should either (1) recompute everything possible from \(m,h,\) year stem, day, or (2) mark injected fields as `TrustedFixture` only. |

Minimal additive field if verification is required on input path: **`birth_day: u8` (1..=30-ish)** so Step E can recompute 紫微.

---

## 7. Explicit conflicts and open questions

| Topic                              | Conflict                                                 | Impact on v1                                                                                                                                                                                    |
| ---------------------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **庚干四化**                       | 《全集》阳武**阴同** vs 《全书》阳武**同阴**（科忌对调） | Code `TRANSFORM_STARS` for `Geng` is 阳武阴同（全集/南派口诀族）。Course text often cites《全书》卷二 — **label mismatch**. Fix at ADR time: either retarget citations to 全集/南派表，或改表。 |
| **壬干四化**                       | 《全集》梁紫**左**武 vs 《全书》梁紫**府**武             | Code uses **左辅** (`ZuoFu`) = 全集系。Same citation issue.                                                                                                                                     |
| **定紫微叙述**                     | Wiki 代数草稿 vs 福山堂口诀例题                          | Implement/test against 口诀三例 + exhaustive \(d,n\) table; do not code Wiki prose literally without tests.                                                                                     |
| **身宫是否入 `Palace` 模型**       | 身宫叠支 vs 十二宫职                                     | Need domain decision: `Ziwei` 是否暴露 `shen_branch` / 查询「身宫落何职」.                                                                                                                      |
| **晚子时 / 日界**                  | 当日 vs 隔日                                             | Affects \(d\) and possibly 时支；map puts calendar outside engine — caller must resolve before `from_birth`.                                                                                    |
| **「命盘各宫位置」字面**           | 课程曾写「各宫位置」复数                                 | 实现上只需命宫一位即可恢复十二职；不必在 `ZiweiInput` 存 12 个 role。                                                                                                                           |
| **十八星 vs 道藏《洞微十八星断》** | 同名不同集                                               | v1 绑定 `Star` enum，不宣称道藏同构。                                                                                                                                                           |

---

## 8. Suggested verification vectors (for later test tickets)

Do **not** implement here; use as future property/fixture checklist:

1. **命身:** 三月辰时 → 命子、身申（大纪元）.
2. **紫微三例:** 27/木三→戌；13/火六→亥；6/土五→未（福山堂）.
3. **紫微–天府镜像:** \(\forall p,\; t=(-p)\bmod 12\)；寅申同宫.
4. **十四正曜间隔:** fix one 紫微，assert relative offsets in §4 G.
5. **辅佐:** 正月 → 左辅辰、右弼戌；子时 → 文曲辰、文昌戌.
6. **五虎遁:** 甲年寅丙、乙年寅戊、… 与 `birth_stem` 全表.
7. **五行局:** reuse existing `FiveElementBureau` unit table; plus 命宫甲子→金四 等纳音抽检.
8. **注入一致性:** 若同时给 \(m,h\) 与 `ming_palace_position`，assert equality（#251 若采纳校验）.
9. **自建验收样例**（规则冻结后）: end-to-end golden fixtures，不绑定商业排盘 App。

---

## 9. Mapping steps → future code modules (non-implementing sketch)

| Step       | Likely home (per course architecture notes) | Existing hooks                           |
| ---------- | ------------------------------------------- | ---------------------------------------- |
| A 命身     | `rules` natal palace                        | `ZiweiInput` month/hour; `twelve_index`  |
| B 十二职   | `rules` → fill `Palace.role`                | `PalaceRole`                             |
| C 宫干     | `rules` 五虎遁                              | `Stem`, `Palace.stem`                    |
| D 局       | already pure fn                             | `FiveElementBureau`                      |
| E 紫微     | `rules`                                     | inject field today; need day for compute |
| F–G 十四曜 | `rules` star placement                      | `Star`                                   |
| H 辅佐     | `rules`                                     | month/hour positions                     |
| 四化       | already pure fn                             | `Stem::transformation_star`              |

---

## 10. Summary for ticket close-out

- **一手/可复现基线:** 安命身、寅首、紫微/天府系、辅弼昌曲 — 以传统 **起例口诀**（福山堂等与《全书》安身命传统一致的南派/三合排盘链）+ 口诀工作例为准；《全书》维基文库作原文入口；道藏「十八星」仅作名称史线索。
- **逐步算法:** §4 Steps A–H，含寅=0 环形公式，与 `ZiweiInput` 零点对齐。
- **已在仓库落地的碎片:** 十八星枚举、宫职、宫干字段、五行局表、四化表、预处理输入中的命宫/紫微位。
- **引擎 vs 注入草案:** 命职/宫干/局/十四正曜+辅佐应由引擎从 \(m,h,d,\) 年干与命宫关系算出；当前 `from_input` 注入命宫+紫微适合夹具，但缺日数且存在「预填答案」风险 — 交给 **#251** 拍板。
- **冲突已记录:** 庚/壬四化 全集 vs 全书；定紫微 Wiki 公式 vs 口诀例题；身宫模型与晚子日界。

---

## 11. References (clickable)

1. 《紫微斗数全书》维基文库总目: <https://zh.wikisource.org/wiki/紫微斗數全書>
2. 《紫微斗数全书·卷二》: <https://zh.wikisource.org/wiki/紫微斗數全書/卷二>
3. 维基百科「紫微斗数」推算方法: <https://zh.wikipedia.org/zh-hans/紫微斗数>
4. 福山堂起例歌诀总括: <http://www.fushantang.com/1012/1012c/j3084.html>
5. 大纪元排盘步骤: <https://www.epochtimes.com/gb/9/6/22/n2565978.htm>
6. 《洞微十八星断》识典: <https://www.shidianguji.com/book/DZ1485/chapter/DZ1485_10>
7. Repo: `crates/ziwei/src/{star,input,palace,five_element_bureau,stem,position}.rs`
8. Course: `course/RESOURCES.md`, `course/NOTES.md`, learning-records `0007`–`0009`
