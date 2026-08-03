# 飞宫单跳模型与查询面

v1 飞宫只有一种物理计算：**十二宫本命宫干**各查四化表，得到最多 48 条单跳边。大限/流年不重布宫干；它们只改变 **宫职→地支** 的贴标，查询时按视图角色解析到支，再读该支宫干对应的边。自化（离心出 / 向心入）由边的几何派生（目标=本宫 / 对宫），不另存真相。生年/大限/流年 **四化落星** 是另一条线：用层用干（年干、大限干、流年干）各飞 4 化标在星上，**不是**第二套「十二宫飞」算法。v1 不做多跳串链、三爻变、断象文案。

## Domain sketch

```text
FlyEdge { source_branch, transformation, target_branch, star }
// self_kind ∈ {Out, In, None} derived

palace_fly_edges()                         // full ≤48, natal stems only
flies_from_branch(branch)
flies_from_role(role, view: Natal|Decade|Annual)

stem_transformations(stem) -> 4 star placements  // year / decade / annual stems
```

## Considered Options

- 大限飞 = 仅大限命支宫干 4 条 — 否决：大限视图仍是十二宫干飞，只换宫职索引。
- 大限/流年重布十二宫干再飞 — 否决：与「宫干固定、只换宫职」冲突，地图已排除。
- 层干飞作为与宫干飞并列的第二套边集 — 否决：易误解；层四化保留为「单干 4 化落星」API。
