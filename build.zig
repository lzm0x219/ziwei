const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    const ziwei = b.addModule("ziwei", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });

    const tests = b.addTest(.{
        .root_module = ziwei,
    });
    const run_tests = b.addRunArtifact(tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_tests.step);
}
