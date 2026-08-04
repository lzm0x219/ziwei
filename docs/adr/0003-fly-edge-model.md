# 飞宫单跳模型与查询面

飞宫只算一套：十二宫的本命宫干各自查四化表，最多 48 条单跳边。大限、流年不重排宫干，只改「哪个宫职贴在哪一支」；查的时候先按视图把宫职落到地支，再读那一支宫干已经算好的边。

自化不单独存。目标落在本宫就是出，落在对宫就是入。生年、大限、流年的四化是另一条线：用年干 / 大限干 / 流年干各化四星并落宫，别当成第二套「十二宫再飞一遍」。多跳串链、三爻变、断象文案都不做。

## 接口草图

```text
ZiweiFly { source_branch, transformation, target_branch, star }
// Out | In | None 由几何派生，不入库

palace_flies() -> &[ZiweiFly; 48]              // 全量，只来自本命宫干
// 布局：Branch::index 升序，每支 4 条，四化序 = Transformation::ALL
flies_from_branch(branch) -> &[ZiweiFly; 4]    // O(1) 切片
flies_from_role(role, view) -> &[ZiweiFly; 4]  // 先贴标再切片

stem_transformations(stem)      // 层用干 → 四星落宫
```

## 否决过的做法

- 大限飞只飞大限命那一支的四条：不对。大限视图仍是十二宫干飞，换的是宫职索引。
- 大限/流年重布十二宫干再飞：和「宫干固定」冲突，地图也排除了。
- 再搞一套「层干飞」边集和宫干飞并列：名字容易混。层四化就做成「单干化四星」的 API。
