const std = @import("std");

pub const results_schema_version: u32 = 3;
pub const manifest_schema_version: u32 = 1;
pub const fixture_id = "natal-v1";

pub const BenchmarkCase = struct {
    name: []const u8,
    display_name: []const u8,
    operations_per_iteration: usize,
    expensive: bool = false,
};

pub const benchmark_cases = [_]BenchmarkCase{
    .{
        .name = "natal/create_from_input/single",
        .display_name = "预处理输入的单张构建",
        .operations_per_iteration = 1,
    },
    .{
        .name = "natal/create_from_birth/single",
        .display_name = "出生资料的单张构建",
        .operations_per_iteration = 1,
    },
    .{
        .name = "natal/create_from_input/sexagenary_cycle",
        .display_name = "预处理输入的 60 个干支年",
        .operations_per_iteration = 60,
    },
    .{
        .name = "natal/create_from_birth/sexagenary_cycle",
        .display_name = "出生资料的 60 个干支年",
        .operations_per_iteration = 60,
    },
    .{
        .name = "natal/create_from_input/exhaustive_valid_space",
        .display_name = "全部合法输入组合",
        .operations_per_iteration = 518_400,
        .expensive = true,
    },
};

pub const required_artifacts = [_][]const u8{
    "report.md",
    "latency.svg",
    "distribution.svg",
    "variability.svg",
    "results.json",
    "summary.csv",
    "samples.csv",
};

pub const optional_change_artifact = "change.svg";
pub const manifest_artifact = "manifest.json";

pub const ManifestArtifact = struct {
    name: []const u8 = "",
    sha256: []const u8 = "",
};

pub const ManifestDocument = struct {
    schema_version: u32 = 0,
    results_schema_version: u32 = 0,
    artifacts: []const ManifestArtifact = &.{},
};

pub const Environment = struct {
    runner_id: []const u8,
    cpu_model: []const u8,
    os_version: []const u8,
    target: []const u8,
    zig_version: []const u8,
    zig_backend: []const u8,
    optimize_mode: []const u8,
    cpu_count: usize,
};

pub fn environmentFingerprint(environment: Environment) [64]u8 {
    const Sha256 = std.crypto.hash.sha2.Sha256;
    var hash = Sha256.init(.{});
    inline for (.{
        environment.runner_id,
        environment.cpu_model,
        environment.os_version,
        environment.target,
        environment.zig_version,
        environment.zig_backend,
        environment.optimize_mode,
    }) |field| {
        hash.update(field);
        hash.update(&.{0});
    }
    var cpu_count_bytes: [@sizeOf(usize)]u8 = undefined;
    std.mem.writeInt(usize, &cpu_count_bytes, environment.cpu_count, .little);
    hash.update(&cpu_count_bytes);

    var digest: [Sha256.digest_length]u8 = undefined;
    hash.final(&digest);
    return std.fmt.bytesToHex(digest, .lower);
}

pub fn sha256Hex(contents: []const u8) [64]u8 {
    const Sha256 = std.crypto.hash.sha2.Sha256;
    var digest: [Sha256.digest_length]u8 = undefined;
    Sha256.hash(contents, &digest, .{});
    return std.fmt.bytesToHex(digest, .lower);
}

pub fn expectedSampleCount(case: BenchmarkCase, configured_samples: usize) usize {
    return if (case.expensive) @min(configured_samples, 10) else configured_samples;
}

pub fn caseIndex(name: []const u8) ?usize {
    for (benchmark_cases, 0..) |case, index| {
        if (std.mem.eql(u8, case.name, name)) return index;
    }
    return null;
}

test "environment fingerprint changes with the physical runner" {
    const base: Environment = .{
        .runner_id = "runner-a",
        .cpu_model = "Example CPU",
        .os_version = "Example OS",
        .target = "aarch64-macos-none",
        .zig_version = "0.16.0",
        .zig_backend = "stage2_llvm",
        .optimize_mode = "ReleaseFast",
        .cpu_count = 8,
    };
    var changed = base;
    changed.runner_id = "runner-b";
    try std.testing.expect(!std.mem.eql(
        u8,
        &environmentFingerprint(base),
        &environmentFingerprint(changed),
    ));
}

test "artifact digest matches the SHA-256 standard vector" {
    try std.testing.expectEqualStrings(
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        &sha256Hex("abc"),
    );
}
