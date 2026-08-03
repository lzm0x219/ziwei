# PROTOTYPE — throwaway (wayfinder #249)

**问题：** 本命 / 大限 / 流年切换时，数据形状好不好用？

**修正：** 生年天干固定 → **生年四化、来因宫**挂在本命盘上，切换视图**不替换**。  
大限/流年四化是 **overlay 叠加**，与生年四化并存。

运行：

```bash
python3 .scratch/wayfinder-249-chart-view/view_proto.py
```

命令：`natal` | `decade N` | `annual YEAR` | `dump` | `quit`

对比 `natal` 与 `decade 1`：上面「固定」块应一字不变；只有宫职贴标和「叠加四化」变。

## Verdict（已接受）

**形状：固定本命盘 + 视图叠层。**

- 本命固定：星位、宫干、飞宫边（≤48）、生年干、**生年四化**、**来因宫**（生年干定，不随大限/流年变）。
- 视图可变：`kind` + `role_to_branch`；大限/流年 **overlay 四化** 与生年四化并存，不替换。
- 查询：`flies_from_role(view, role)` = 视图宫职→支 → 读固定飞边。
- 否决：每步大限/流年物化整盘副本（除非日后有强需求）。

喂给 #246：公开 API 采用 `Ziwei`（固定）+ `ZiweiView` / view 参数（叠层）。
