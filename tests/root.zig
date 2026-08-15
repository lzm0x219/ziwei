//! 集成测试入口。

test {
    _ = @import("natal_integration.zig");
    _ = @import("public_api.zig");
}
