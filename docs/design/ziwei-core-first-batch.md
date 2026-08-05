# `ziwei_core` 首批迁移

> 状态：已执行（2026-08-05）
>
> 目标：把现有 Rust 实现完整迁入 `ziwei_core`，作为后续重写的可编译基线。

## 决定

本次只做机械迁移，不在移动过程中重新划分类型、排盘和查询 interface：

- 原 `crates/ziwei/src` 的实现与测试整体迁入 `crates/ziwei_core/src`；
- crate 内私有构造器、内部坐标和模块关系保持不变；
- `ziwei` 收敛为 SDK facade，显式导出 `ziwei_core` 的当前公开类型；
- calendar、query、analysis 和 bindings 不增加实现。

因此，现有视图和查询代码也暂时随实现进入 `ziwei_core`。这是重写前的代码基线，不代表最终 crate 边界。

## 迁移结果

```text
ziwei
  └── facade / 统一公开入口
        ↓
ziwei_core
  └── 当前全部 Rust 领域实现与测试
```

实现只在 `ziwei_core` 保留一份，`ziwei` 不复制任何紫微规则，也不保留旧实现模块。

## 后续

下一步直接重写 `ziwei_core`。等 `Chart` 稳定后，再设计 `ziwei_query` 的 interface，并将关系查询从 core 移出。
