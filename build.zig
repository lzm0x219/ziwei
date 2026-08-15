const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const benchmark_optimize = b.option(
        std.builtin.OptimizeMode,
        "benchmark-optimize",
        "Optimization mode used only by the benchmark executable",
    ) orelse .ReleaseFast;

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

    const benchmark_test_module = b.createModule(.{
        .root_source_file = b.path("benchmarks/main.zig"),
        .target = target,
        .optimize = optimize,
        .imports = &.{.{
            .name = "ziwei",
            .module = ziwei,
        }},
    });
    const benchmark_tests = b.addTest(.{
        .root_module = benchmark_test_module,
    });
    const run_benchmark_tests = b.addRunArtifact(benchmark_tests);

    const benchmark_publish_tests = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("benchmarks/publish.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    const run_benchmark_publish_tests = b.addRunArtifact(benchmark_publish_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_unit_tests.step);
    test_step.dependOn(&run_integration_tests.step);
    test_step.dependOn(&run_benchmark_tests.step);
    test_step.dependOn(&run_benchmark_publish_tests.step);

    const benchmark_ziwei = b.createModule(.{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = benchmark_optimize,
    });
    const benchmark_module = b.createModule(.{
        .root_source_file = b.path("benchmarks/main.zig"),
        .target = target,
        .optimize = benchmark_optimize,
        .imports = &.{.{
            .name = "ziwei",
            .module = benchmark_ziwei,
        }},
    });
    const benchmark_executable = b.addExecutable(.{
        .name = "ziwei-benchmark",
        .root_module = benchmark_module,
    });
    const run_benchmark = b.addRunArtifact(benchmark_executable);
    run_benchmark.setCwd(b.path("."));
    if (b.args) |args| run_benchmark.addArgs(args);

    const benchmark_step = b.step("benchmark", "Run natal construction benchmarks and write reports");
    benchmark_step.dependOn(&run_benchmark.step);

    const benchmark_publish_module = b.createModule(.{
        .root_source_file = b.path("benchmarks/publish.zig"),
        .target = target,
        .optimize = .ReleaseSafe,
    });
    const benchmark_publish_executable = b.addExecutable(.{
        .name = "ziwei-benchmark-publish",
        .root_module = benchmark_publish_module,
    });
    const run_benchmark_publish = b.addRunArtifact(benchmark_publish_executable);
    run_benchmark_publish.setCwd(b.path("."));
    if (b.args) |args| run_benchmark_publish.addArgs(args);

    const benchmark_publish_step = b.step(
        "benchmark-publish",
        "Publish a full benchmark report into versioned documentation",
    );
    benchmark_publish_step.dependOn(&run_benchmark_publish.step);

    const run_benchmark_docs_check = b.addRunArtifact(benchmark_publish_executable);
    run_benchmark_docs_check.setCwd(b.path("."));
    run_benchmark_docs_check.addArg("--check");
    const benchmark_docs_check_step = b.step(
        "benchmark-docs-check",
        "Validate published benchmark artifacts and generated documentation",
    );
    benchmark_docs_check_step.dependOn(&run_benchmark_docs_check.step);

    const run_benchmark_docs_generate = b.addRunArtifact(benchmark_publish_executable);
    run_benchmark_docs_generate.setCwd(b.path("."));
    run_benchmark_docs_generate.addArg("--generate");
    const benchmark_docs_generate_step = b.step(
        "benchmark-docs-generate",
        "Regenerate the benchmark index and trend from published records",
    );
    benchmark_docs_generate_step.dependOn(&run_benchmark_docs_generate.step);
}
