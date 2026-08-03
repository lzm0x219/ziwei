#!/usr/bin/env python3
"""THROWAWAY — chart view state shape for wayfinder #249. Not production."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal

# 子=0 … 亥=11（假数据，只验形状）
BRANCHES = list("子丑寅卯辰巳午未申酉戌亥")
NATAL_ROLES = list("命兄夫子财疾迁友官田福父")
NATAL_MING = 2  # 寅

STEMS = list("甲乙丙丁戊己庚辛壬癸")
# 与 Stem::laiyin_branch 一致（示意）
LAIYIN_BY_STEM = {
    0: 10,  # 甲→戌
    1: 9,  # 乙→酉
    2: 4,  # 丙→辰
    3: 7,  # 丁→未
    4: 6,  # 戊→午
    5: 5,  # 己→巳
    6: 4,  # 庚→辰
    7: 3,  # 辛→卯
    8: 2,  # 壬→寅
    9: 11,  # 癸→亥
}

STEM_ON_BRANCH = [(i + 2) % 10 for i in range(12)]

STAR_AT = {
    "紫微": 2,
    "天机": 1,
    "太阳": 10,
    "武曲": 9,
    "天同": 8,
    "廉贞": 3,
    "天府": 2,
    "太阴": 4,
    "贪狼": 5,
    "巨门": 6,
    "天相": 7,
    "天梁": 8,
    "七杀": 9,
    "破军": 0,
    "左辅": 4,
    "右弼": 10,
    "文昌": 10,
    "文曲": 4,
}

# 示意四化表（不全；够演示）
TRANS_TABLE = {
    0: [("禄", "廉贞"), ("权", "破军"), ("科", "武曲"), ("忌", "太阳")],  # 甲
    1: [("禄", "天机"), ("权", "天梁"), ("科", "紫微"), ("忌", "太阴")],  # 乙
    2: [("禄", "天同"), ("权", "天机"), ("科", "文昌"), ("忌", "廉贞")],  # 丙
    3: [("禄", "太阴"), ("权", "天同"), ("科", "天机"), ("忌", "巨门")],  # 丁
    4: [("禄", "贪狼"), ("权", "太阴"), ("科", "右弼"), ("忌", "天机")],  # 戊
    5: [("禄", "武曲"), ("权", "贪狼"), ("科", "天梁"), ("忌", "文曲")],  # 己
    6: [("禄", "太阳"), ("权", "武曲"), ("科", "太阴"), ("忌", "天同")],  # 庚
    7: [("禄", "巨门"), ("权", "太阳"), ("科", "文曲"), ("忌", "文昌")],  # 辛
    8: [("禄", "天梁"), ("权", "紫微"), ("科", "左辅"), ("忌", "武曲")],  # 壬
    9: [("禄", "破军"), ("权", "巨门"), ("科", "太阴"), ("忌", "贪狼")],  # 癸
}


def opposite(b: int) -> int:
    return (b + 6) % 12


def roles_for_ming(ming_branch: int) -> dict[str, int]:
    return {role: (ming_branch - i) % 12 for i, role in enumerate(NATAL_ROLES)}


@dataclass
class ZiweiFly:
    source: int
    hua: str
    target: int
    star: str

    @property
    def self_kind(self) -> str | None:
        if self.target == self.source:
            return "Out"
        if self.target == opposite(self.source):
            return "In"
        return None


@dataclass
class StemHua:
    """某一干的四化落星（生年干固定；大限干/流年干随视图叠加）。"""

    label: str  # 生年 / 大限 / 流年
    stem_index: int
    marks: list[tuple[str, str, int]]  # hua, star, branch


@dataclass
class ZiweiView:
    """
    视图只换两样：
      1) 宫职贴在哪一支
      2) 额外叠加的大限/流年四化（生年四化不在这里、也不被替换）
    """

    kind: Literal["natal", "decade", "annual"]
    role_to_branch: dict[str, int]
    # 叠加层：大限或流年四化；本命视图为空
    overlay_hua: StemHua | None = None
    decade_index: int | None = None
    annual_year: int | None = None


@dataclass
class ZiweiChart:
    """本命固定：星、宫干、飞边、生年干、生年四化、来因宫。"""

    year_stem: int = 0  # 甲
    gender_male: bool = True
    natal_ming: int = NATAL_MING
    star_at: dict[str, int] = field(default_factory=lambda: dict(STAR_AT))
    palace_flies: list[ZiweiFly] = field(default_factory=list)
    year_hua: StemHua = field(init=False)
    laiyin_branch: int = field(init=False)

    def __post_init__(self) -> None:
        self.laiyin_branch = LAIYIN_BY_STEM[self.year_stem]
        self.year_hua = self._stem_hua("生年", self.year_stem)
        if not self.palace_flies:
            self.palace_flies = self._compute_palace_flies()

    def _stem_hua(self, label: str, stem: int) -> StemHua:
        marks = [
            (hua, star, self.star_at.get(star, 0))
            for hua, star in TRANS_TABLE[stem % 10]
        ]
        return StemHua(label=label, stem_index=stem, marks=marks)

    def _compute_palace_flies(self) -> list[ZiweiFly]:
        edges: list[ZiweiFly] = []
        for branch in range(12):
            stem = STEM_ON_BRANCH[branch]
            for hua, star in TRANS_TABLE[stem % 10]:
                edges.append(
                    ZiweiFly(branch, hua, self.star_at.get(star, branch), star)
                )
        return edges

    def natal_roles(self) -> dict[str, int]:
        return roles_for_ming(self.natal_ming)

    def decade_roles(self, step: int) -> dict[str, int]:
        direction = 1 if self.gender_male else -1
        decade_ming = (self.natal_ming + direction * step) % 12
        return roles_for_ming(decade_ming)

    def annual_roles(self, year: int) -> dict[str, int]:
        tai_sui = (year - 4) % 12
        return roles_for_ming(tai_sui)

    def decade_stem(self, step: int) -> int:
        ming_br = self.decade_roles(step)["命"]
        return STEM_ON_BRANCH[ming_br]

    def annual_stem(self, year: int) -> int:
        return (year - 4) % 10

    def view_natal(self) -> ZiweiView:
        return ZiweiView(kind="natal", role_to_branch=self.natal_roles())

    def view_decade(self, step: int) -> ZiweiView:
        return ZiweiView(
            kind="decade",
            role_to_branch=self.decade_roles(step),
            overlay_hua=self._stem_hua("大限", self.decade_stem(step)),
            decade_index=step,
        )

    def view_annual(self, year: int) -> ZiweiView:
        return ZiweiView(
            kind="annual",
            role_to_branch=self.annual_roles(year),
            overlay_hua=self._stem_hua("流年", self.annual_stem(year)),
            annual_year=year,
        )

    def flies_from_role(self, view: ZiweiView, role: str) -> list[ZiweiFly]:
        br = view.role_to_branch[role]
        return [e for e in self.palace_flies if e.source == br]


def print_hua(block: StemHua) -> None:
    print(f"  [{block.label}] 干={STEMS[block.stem_index]}")
    for hua, star, br in block.marks:
        print(f"    {hua} {star} @ {BRANCHES[br]}")


def dump_view(chart: ZiweiChart, view: ZiweiView) -> None:
    print("=" * 60)
    print(f"view.kind = {view.kind!r}")
    if view.decade_index is not None:
        print(f"view.decade_index = {view.decade_index}")
    if view.annual_year is not None:
        print(f"view.annual_year = {view.annual_year}")

    print("--- 固定（生年天干决定，切换视图不变）---")
    print(f"  生年干 = {STEMS[chart.year_stem]}")
    print(f"  来因宫 = {BRANCHES[chart.laiyin_branch]}（由生年干定，不随大限/流年变）")
    print_hua(chart.year_hua)

    print("--- 视图宫职 → 支（会变）---")
    for role, br in view.role_to_branch.items():
        print(f"  {role} → {BRANCHES[br]}  宫干={STEMS[STEM_ON_BRANCH[br]]}")

    print("--- 叠加四化（大限/流年；生年不在这里被替换）---")
    if view.overlay_hua is None:
        print("  （本命视图：无叠加，只有上面的生年四化）")
    else:
        print_hua(view.overlay_hua)
        print("  ↑ 与生年四化同时存在，不是覆盖")

    print("--- flies_from_role(命)：本命宫干边（边集固定）---")
    for e in chart.flies_from_role(view, "命"):
        sk = e.self_kind or "-"
        print(
            f"  {BRANCHES[e.source]} -{e.hua}-> {e.star} @ {BRANCHES[e.target]}  self={sk}"
        )

    print("--- 固定层计数 ---")
    print(f"  palace_flies = {len(chart.palace_flies)}")
    print("  变的：role 贴标 + 可选 overlay 四化")
    print("  不变：生年四化、来因宫、星位、飞宫边")
    print("=" * 60)


def main() -> None:
    chart = ZiweiChart()
    view = chart.view_natal()
    print("THROWAWAY #249 — 生年四化/来因固定；大限流年只叠加")
    print("cmds: natal | decade N | annual YEAR | dump | quit")
    dump_view(chart, view)
    while True:
        try:
            line = input("> ").strip()
        except (EOFError, KeyboardInterrupt):
            print()
            break
        if not line:
            continue
        parts = line.split()
        cmd = parts[0].lower()
        if cmd in ("q", "quit", "exit"):
            break
        if cmd == "natal":
            view = chart.view_natal()
        elif cmd == "decade" and len(parts) == 2:
            view = chart.view_decade(int(parts[1]))
        elif cmd == "annual" and len(parts) == 2:
            view = chart.view_annual(int(parts[1]))
        elif cmd == "dump":
            pass
        else:
            print("natal | decade N | annual YEAR | dump | quit")
            continue
        dump_view(chart, view)


if __name__ == "__main__":
    main()
