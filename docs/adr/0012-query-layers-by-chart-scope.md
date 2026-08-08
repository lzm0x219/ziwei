# 查询按命盘 scope 分层

`ziwei_query` 按回答问题所需的最小宫位坐标 scope 分层，而不按返回数量、查询动作或领域对象分层。当前只有 `ReframeScope` 与 `DecadeScope`：前者表达本命或任意立极坐标，后者以大限命宫为起点表达大限坐标。同一 scope 内可以查询事实、导航、关系和条件，只有 `ziwei_core` 拥有相应的完整时态事实后，才继续增加流年、流月等查询层。

选择 scope 作为分层标准，是因为调用者首先需要确定自己正在查看本命还是某一步运限；按“查一个／查多个”“宫／星／四化”切分会把同一问题拆散，并把不同上下文的问题错误地放在一起。

返回一个还是多个结果不改变层级。对固定十二个大限逐一建立同一种 L2 scope、应用同一个条件并返回匹配项，仍属于 L2 跨 scope 条件查询，不因此成为新的时间线层。

`DecadeYearSelection` 只定位大限内的一个 `DecadeYear`，不建立新的宫位坐标，因此不是 scope；要继续查宫位，必须经由其 `decade()` 回到 `DecadeScope`。按岁数或农历年定位的入口分别命名为 `decade_year_at_age` 与 `decade_year_at_lunar_year`，限内年份次序使用经验证的 `DecadeYearOrdinal`；这些名称刻意表达“年份定位”而非流年 scope。

本决策取代 [ADR-0004](0004-chart-query-api.md) 中旧 `ZiweiView` 公开面及当前支持流年 scope 的计划，但保留其“不复制命盘、不重算星曜、宫干与四化事实”的计算约束。当前只设计本命与大限 scope；流年及更细层级必须等 `ziwei_core` 拥有相应的完整事实后再决定。
