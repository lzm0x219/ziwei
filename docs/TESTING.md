# 测试规范

本文定义 Ziwei Zig 项目中单元测试、集成测试、测试入口和构建入口的放置规则。新增或迁移测试时，以本文为准。

当前项目使用 `mise.toml` 固定的 Zig `0.16.0`。本文中的 `build.zig` 写法以该版本和仓库当前构建图为基准。

## 按测试边界选择位置

| 测试边界 | 放置位置 | 允许依赖 | 典型内容 |
| --- | --- | --- | --- |
| 单个模块的内部实现 | 对应的 `src/*.zig` | 当前文件的公开和私有声明 | 单个函数、类型、错误、边界和内部不变量 |
| 公开 API 或多个模块协作 | `tests/*.zig` | 通过 `@import("ziwei")` 使用 public API | 排盘流程、查询流程、模块协作和回归命例 |
| 命令行、文件、网络或端到端行为 | `tests/*.zig` | public API 和必要的外部边界 | 黑盒行为、I/O 和完整流程 |

判断时看测试保护的行为边界，而不是测试代码的长度：

- 需要访问私有声明才能验证的局部行为属于单元测试。
- 只应通过 `src/root.zig` 导出的接口观察结果，或需要真实协作多个模块的行为属于集成测试。
- 一个测试同时满足两类特征时，优先按更外层的行为边界放入 `tests/`。

## 编写单元测试

单元测试必须直接放在被测试的源码文件中，并位于生产声明之后：

```zig
pub fn add(left: i32, right: i32) i32 {
    return left + right;
}

test "两个整数相加" {
    const std = @import("std");
    try std.testing.expectEqual(@as(i32, 3), add(1, 2));
}
```

单元测试遵守以下规则：

- 测试名称描述可观察行为，不复述函数名。
- 覆盖正常路径、边界、预期错误和重要不变量；涉及分配时还要覆盖清理路径。
- 测试专用常量、样例数据和辅助逻辑应声明在对应的 `test` 块内。
- 不得把仅供测试使用的顶层函数、类型或常量混入生产实现区。需要较复杂或跨测试复用的测试夹具时，先判断该测试是否实际属于集成测试；属于集成测试的夹具一并移入 `tests/`。
- 不得为了测试而扩大生产声明的可见性。

## 聚合源码中的单元测试

Zig 不会自动扫描 `src/`。每个含有单元测试的源码模块都必须由 `src/root.zig` 的匿名测试块显式导入。

当前入口为：

```zig
test {
    _ = @import("models/decade.zig");
    _ = @import("models/five_element_bureau.zig");
    _ = @import("models/input.zig");
    _ = @import("models/natal.zig");
    _ = @import("models/palace.zig");
    _ = @import("models/placement.zig");
    _ = @import("models/primitive.zig");
    _ = @import("models/star.zig");
    _ = @import("models/transformation.zig");
    _ = @import("query.zig");
}
```

新增含测试的源码文件时，必须同时更新该列表。删除某文件的最后一个单元测试时，应删除对应的测试聚合导入；生产代码所需的普通导入不受此规则影响。

## 编写集成测试

集成测试统一放在 `tests/`，并按验证场景命名。测试文件应从包模块导入公开接口：

```zig
const std = @import("std");
const ziwei = @import("ziwei");

test "公开入口完成一个排盘流程" {
    // 仅通过 ziwei 的 public API 构造输入并验证结果。
    _ = std.testing;
    _ = ziwei;
}
```

集成测试遵守以下规则：

- 不得通过 `../src/*.zig` 绕过 `src/root.zig` 直接访问内部实现。
- 测试夹具和辅助函数与测试一起留在 `tests/`，不得放入生产源码。
- 一个文件聚焦一组相关场景；公共 API 契约、完整命例和其他流程应分文件维护。
- 新增集成测试文件时，必须在 `tests/root.zig` 中显式导入。

当前集成测试入口为：

```zig
//! 集成测试入口。

test {
    _ = @import("natal_integration.zig");
    _ = @import("public_api.zig");
}
```

## 在构建图中执行全部测试

`build.zig` 必须分别创建并运行单元测试产物和集成测试产物，再让同一个 `test` step 依赖两个运行步骤。只调用 `b.addTest` 只能证明测试能够编译，不能证明测试已经执行。

Zig `0.16.0` 对应的项目配置为：

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const ziwei = b.addModule("ziwei", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const unit_tests = b.addTest(.{
        .root_module = ziwei,
    });
    const run_unit_tests = b.addRunArtifact(unit_tests);

    const integration_test_module = b.createModule(.{
        .root_source_file = b.path("tests/root.zig"),
        .target = target,
        .optimize = optimize,
        .imports = &.{.{
            .name = "ziwei",
            .module = ziwei,
        }},
    });
    const integration_tests = b.addTest(.{
        .root_module = integration_test_module,
    });
    const run_integration_tests = b.addRunArtifact(integration_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_unit_tests.step);
    test_step.dependOn(&run_integration_tests.step);
}
```

因此，以下命令必须编译并执行全部单元测试和集成测试：

```bash
mise exec -- zig build test
```

提交 Zig 源码、测试或构建配置前，还必须运行仓库统一检查：

```bash
mise run check
```

## 推荐目录结构

```text
src/
├── root.zig                    public API 和单元测试聚合入口
├── ziwei.zig                   排盘实现；存在局部单元行为时可附单元测试
├── query.zig                   查询实现和查询模块单元测试
└── models/
    └── *.zig                   领域实现和对应单元测试
tests/
├── root.zig                    集成测试聚合入口
├── natal_integration.zig       完整命盘与多模块协作测试
├── public_api.zig              public API 黑盒测试
└── query_integration.zig       查询协作测试；完成暂缓迁移后新增
build.zig                       注册并运行两类测试
mise.toml                       固定 Zig 版本并提供统一检查命令
```

## 当前仓库检查结果

当前放置方式整体符合本规范：

- `src/models/*.zig` 中的测试验证对应模型或放置规则的局部行为，应继续与实现同文件维护。
- 原位于 `src/ziwei.zig` 的完整命例和跨模块测试已迁移至 `tests/natal_integration.zig`；`src/ziwei.zig` 当前只保留生产实现。
- 原位于 `src/root.zig` 的 public API 黑盒测试已迁移至 `tests/public_api.zig`；`src/root.zig` 当前只保留公开导出和单元测试聚合入口。
- `src/models/decade.zig` 的测试专用边界常量已局部化到对应 `test` 块，不再混入生产声明区。
- `build.zig` 已同时运行单元测试和集成测试，`zig build test` 是全部测试的统一入口。

当前唯一已知例外是 `src/query.zig`。以下两个测试属于查询模块自身的单元测试，应继续留在源码中：

- `限内年份序号仅接受一至十`
- `固定宫位分组完整覆盖领域身份`

其余五个测试通过真实命盘验证查询句柄、十二宫映射、宫位关系、星曜与四化查询以及大限定位，属于多模块协作测试；`sampleNatal` 和 `palaceNames` 也是这些测试的夹具。按本规范，它们最终应一起迁移到 `tests/query_integration.zig`。该迁移目前按项目决定暂缓，不改变长期放置规则。

## 变更检查清单

- 测试保护的是模块内部行为，还是 public API、跨模块或端到端行为？
- 单元测试是否位于对应源码文件，集成测试是否位于 `tests/`？
- 测试专用声明是否误入生产实现区？
- 新增测试文件是否已由 `src/root.zig` 或 `tests/root.zig` 显式导入？
- `build.zig` 的 `test` step 是否依赖两个测试产物的运行步骤？
- 是否使用 `mise.toml` 固定的 Zig 版本运行了 `mise run check`？
