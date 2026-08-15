const std = @import("std");
const contract = @import("report_contract.zig");

const Io = std.Io;
const results_schema_version = contract.results_schema_version;
const docs_root = "docs/benchmarks";
const maximum_artifact_bytes = 64 * 1024 * 1024;

const PublishedRun = struct {
    started_at_unix_ns: i128 = 0,
    revision: []const u8 = "",
    git_commit: []const u8 = "",
    git_dirty: bool = false,
    run_kind: []const u8 = "",
    zig_version: []const u8 = "",
    zig_backend: []const u8 = "",
    optimize_mode: []const u8 = "",
    target: []const u8 = "",
    fixture_id: []const u8 = "",
    cpu_count: usize = 0,
    cpu_model: []const u8 = "",
    os_version: []const u8 = "",
    runner_id: []const u8 = "",
    environment_fingerprint: []const u8 = "",
    natal_size_bytes: usize = 0,
    warmups: usize = 0,
    configured_samples: usize = 0,
    target_sample_ns: u64 = 0,
    verdict: []const u8 = "",
};

const PublishedStatistics = struct {
    median: f64 = 0,
    relative_standard_deviation_percent: f64 = 0,
    operations_per_second: f64 = 0,
};

const PublishedBenchmark = struct {
    name: []const u8 = "",
    operations_per_iteration: usize = 0,
    sample_count: usize = 0,
    unit: []const u8 = "",
    statistics: PublishedStatistics = .{},
};

const PublishedDocument = struct {
    schema_version: u32 = 0,
    run: PublishedRun = .{},
    benchmarks: []const PublishedBenchmark = &.{},
};

const PublishedRecord = struct {
    id: []const u8,
    document: PublishedDocument,
};

const PerformanceRange = struct {
    minimum_median_ns: f64,
    maximum_median_ns: f64,
    minimum_operations_per_second: f64,
    maximum_operations_per_second: f64,
};

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const allocator = init.arena.allocator();
    const args = try init.minimal.args.toSlice(allocator);
    if (args.len == 2 and std.mem.eql(u8, args[1], "--generate")) {
        try regenerateDocumentation(io, allocator, docs_root);
        var stdout_buffer: [256]u8 = undefined;
        var stdout_writer = Io.File.stdout().writer(io, &stdout_buffer);
        try stdout_writer.interface.writeAll("Benchmark documentation regenerated.\n");
        try stdout_writer.interface.flush();
        return;
    }
    if (args.len == 2 and std.mem.eql(u8, args[1], "--check")) {
        try checkDocumentation(io, allocator, docs_root);
        var stdout_buffer: [256]u8 = undefined;
        var stdout_writer = Io.File.stdout().writer(io, &stdout_buffer);
        try stdout_writer.interface.writeAll("Benchmark documentation is up to date.\n");
        try stdout_writer.interface.flush();
        return;
    }
    if (args.len != 3 or std.mem.eql(u8, args[1], "--help")) {
        try printUsage(io);
        if (args.len != 2 or !std.mem.eql(u8, args[1], "--help")) return error.InvalidArguments;
        return;
    }

    try publishReport(io, allocator, args[1], args[2], docs_root);

    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = Io.File.stdout().writer(io, &stdout_buffer);
    const stdout = &stdout_writer.interface;
    try stdout.print("Published benchmark record: {s}/runs/{s}/report.md\n", .{ docs_root, args[2] });
    try stdout.print("Updated benchmark index: {s}/README.md\n", .{docs_root});
    try stdout.flush();
}

fn printUsage(io: Io) !void {
    var buffer: [1024]u8 = undefined;
    var writer = Io.File.stdout().writer(io, &buffer);
    const stdout = &writer.interface;
    try stdout.writeAll(
        \\Usage: zig build benchmark-publish -- <run-directory> <record-id>
        \\
        \\Example:
        \\  zig build benchmark-publish -- benchmark-results/run-123 2026-08-15-a1b2c3d
        \\
        \\Validate generated documentation:
        \\  zig build benchmark-docs-check
        \\
        \\Regenerate the index and trend from published records:
        \\  zig build benchmark-docs-generate
        \\
    );
    try stdout.flush();
}

fn publishReport(
    io: Io,
    allocator: std.mem.Allocator,
    source_directory: []const u8,
    record_id: []const u8,
    destination_root: []const u8,
) !void {
    try validateRecordId(record_id);

    const source_results_path = try std.fmt.allocPrint(allocator, "{s}/results.json", .{source_directory});
    const source_document = try readDocument(io, allocator, source_results_path);
    try validatePublishableDocument(source_document);
    const has_change_chart = try validateReportManifest(io, allocator, source_directory);
    const runs_path = try std.fmt.allocPrint(allocator, "{s}/runs", .{destination_root});
    const destination_directory = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ runs_path, record_id });
    if (try pathExists(io, destination_directory)) return error.RecordAlreadyExists;

    try Io.Dir.cwd().createDirPath(io, runs_path);
    try Io.Dir.cwd().createDirPath(io, destination_directory);
    errdefer Io.Dir.cwd().deleteTree(io, destination_directory) catch {};

    for (contract.required_artifacts) |artifact| {
        try copyArtifact(io, allocator, source_directory, destination_directory, artifact);
    }
    if (has_change_chart) {
        try copyArtifact(io, allocator, source_directory, destination_directory, contract.optional_change_artifact);
    }
    try copyArtifact(io, allocator, source_directory, destination_directory, contract.manifest_artifact);

    try regenerateDocumentation(io, allocator, destination_root);
}

fn validateRecordId(record_id: []const u8) !void {
    if (record_id.len == 0 or record_id.len > 120 or
        std.mem.eql(u8, record_id, ".") or std.mem.eql(u8, record_id, ".."))
    {
        return error.InvalidRecordId;
    }
    for (record_id) |character| {
        if (std.ascii.isAlphanumeric(character) or character == '-' or character == '_' or character == '.') {
            continue;
        }
        return error.InvalidRecordId;
    }
}

fn validatePublishableRun(schema_version: u32, run_kind: []const u8, revision: []const u8) !void {
    if (schema_version != results_schema_version) return error.UnsupportedReportSchema;
    if (!std.mem.eql(u8, run_kind, "full")) return error.SmokeReportCannotBePublished;
    if (revision.len == 0 or std.mem.eql(u8, revision, "unknown")) return error.MissingRevision;
}

fn isGitCommit(commit: []const u8) bool {
    if (commit.len != 40 and commit.len != 64) return false;
    for (commit) |character| {
        if (std.ascii.isDigit(character) or (character >= 'a' and character <= 'f')) continue;
        return false;
    }
    return true;
}

fn isSupportedVerdict(verdict: []const u8) bool {
    return std.mem.eql(u8, verdict, "no-baseline") or
        std.mem.eql(u8, verdict, "no-baseline-noise-warning") or
        std.mem.eql(u8, verdict, "noise-warning") or
        std.mem.eql(u8, verdict, "unstable") or
        std.mem.eql(u8, verdict, "pass") or
        std.mem.eql(u8, verdict, "regression");
}

fn validatePublishableDocument(document: PublishedDocument) !void {
    try validatePublishableRun(document.schema_version, document.run.run_kind, document.run.revision);
    if (document.run.started_at_unix_ns <= 0 or
        !isGitCommit(document.run.git_commit) or
        document.run.zig_version.len == 0 or
        document.run.zig_backend.len == 0 or
        document.run.optimize_mode.len == 0 or
        document.run.target.len == 0 or
        !std.mem.eql(u8, document.run.fixture_id, contract.fixture_id) or
        document.run.cpu_count == 0 or
        document.run.cpu_model.len == 0 or
        document.run.os_version.len == 0 or
        document.run.runner_id.len == 0 or
        document.run.environment_fingerprint.len != 64 or
        document.run.warmups == 0 or
        document.run.configured_samples < 3 or
        document.run.target_sample_ns == 0 or
        !isSupportedVerdict(document.run.verdict))
    {
        return error.IncompleteRunMetadata;
    }
    const expected_revision_length: usize = if (document.run.git_dirty) 18 else 12;
    if (document.run.revision.len != expected_revision_length or
        !std.mem.eql(u8, document.run.revision[0..12], document.run.git_commit[0..12]) or
        (document.run.git_dirty and !std.mem.endsWith(u8, document.run.revision, "-dirty")))
    {
        return error.InconsistentGitIdentity;
    }
    const expected_environment_fingerprint = contract.environmentFingerprint(.{
        .runner_id = document.run.runner_id,
        .cpu_model = document.run.cpu_model,
        .os_version = document.run.os_version,
        .target = document.run.target,
        .zig_version = document.run.zig_version,
        .zig_backend = document.run.zig_backend,
        .optimize_mode = document.run.optimize_mode,
        .cpu_count = document.run.cpu_count,
    });
    if (!std.mem.eql(
        u8,
        document.run.environment_fingerprint,
        &expected_environment_fingerprint,
    )) return error.InconsistentEnvironmentFingerprint;
    if (document.benchmarks.len != contract.benchmark_cases.len) return error.IncompatibleBenchmarkSet;
    var seen = [_]bool{false} ** contract.benchmark_cases.len;
    for (document.benchmarks) |benchmark| {
        const case_index = contract.caseIndex(benchmark.name) orelse return error.IncompatibleBenchmarkSet;
        if (seen[case_index]) return error.IncompatibleBenchmarkSet;
        seen[case_index] = true;
        const expected_case = contract.benchmark_cases[case_index];
        if (benchmark.operations_per_iteration != expected_case.operations_per_iteration or
            !std.mem.eql(u8, benchmark.unit, "ns/op") or
            benchmark.sample_count != contract.expectedSampleCount(expected_case, document.run.configured_samples) or
            !std.math.isFinite(benchmark.statistics.median) or
            benchmark.statistics.median <= 0 or
            !std.math.isFinite(benchmark.statistics.operations_per_second) or
            benchmark.statistics.operations_per_second <= 0)
        {
            return error.IncompleteBenchmarkResult;
        }
    }
}

fn readDocument(io: Io, allocator: std.mem.Allocator, path: []const u8) !PublishedDocument {
    const contents = try Io.Dir.cwd().readFileAlloc(io, path, allocator, .limited(maximum_artifact_bytes));
    return std.json.parseFromSliceLeaky(
        PublishedDocument,
        allocator,
        contents,
        .{ .ignore_unknown_fields = true },
    );
}

fn readManifest(io: Io, allocator: std.mem.Allocator, path: []const u8) !contract.ManifestDocument {
    const contents = try Io.Dir.cwd().readFileAlloc(io, path, allocator, .limited(maximum_artifact_bytes));
    return std.json.parseFromSliceLeaky(
        contract.ManifestDocument,
        allocator,
        contents,
        .{ .ignore_unknown_fields = true },
    );
}

fn validateReportManifest(
    io: Io,
    allocator: std.mem.Allocator,
    report_directory: []const u8,
) !bool {
    const manifest_path = try std.fmt.allocPrint(
        allocator,
        "{s}/{s}",
        .{ report_directory, contract.manifest_artifact },
    );
    const manifest = readManifest(io, allocator, manifest_path) catch |err| switch (err) {
        error.FileNotFound => return error.MissingReportManifest,
        else => return err,
    };
    if (manifest.schema_version != contract.manifest_schema_version or
        manifest.results_schema_version != contract.results_schema_version)
    {
        return error.UnsupportedManifestSchema;
    }

    var seen = [_]bool{false} ** contract.required_artifacts.len;
    var has_change_chart = false;
    for (manifest.artifacts) |artifact| {
        if (artifact.sha256.len != 64) return error.InvalidArtifactDigest;
        const required_index = for (contract.required_artifacts, 0..) |required, index| {
            if (std.mem.eql(u8, required, artifact.name)) break index;
        } else null;
        if (required_index) |index| {
            if (seen[index]) return error.DuplicateReportArtifact;
            seen[index] = true;
        } else if (std.mem.eql(u8, artifact.name, contract.optional_change_artifact)) {
            if (has_change_chart) return error.DuplicateReportArtifact;
            has_change_chart = true;
        } else {
            return error.UnknownReportArtifact;
        }

        const artifact_path = try std.fmt.allocPrint(
            allocator,
            "{s}/{s}",
            .{ report_directory, artifact.name },
        );
        const contents = Io.Dir.cwd().readFileAlloc(
            io,
            artifact_path,
            allocator,
            .limited(maximum_artifact_bytes),
        ) catch |err| switch (err) {
            error.FileNotFound => return error.MissingReportArtifact,
            else => return err,
        };
        const actual_digest = contract.sha256Hex(contents);
        if (!std.mem.eql(u8, artifact.sha256, &actual_digest)) return error.ArtifactDigestMismatch;
    }
    for (seen) |present| if (!present) return error.MissingReportArtifact;
    return has_change_chart;
}

fn pathExists(io: Io, path: []const u8) !bool {
    Io.Dir.cwd().access(io, path, .{}) catch |err| switch (err) {
        error.FileNotFound => return false,
        else => return err,
    };
    return true;
}

fn copyArtifact(
    io: Io,
    allocator: std.mem.Allocator,
    source_directory: []const u8,
    destination_directory: []const u8,
    artifact: []const u8,
) !void {
    const source_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ source_directory, artifact });
    const destination_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ destination_directory, artifact });
    const contents = try Io.Dir.cwd().readFileAlloc(io, source_path, allocator, .limited(maximum_artifact_bytes));
    try Io.Dir.cwd().writeFile(io, .{ .sub_path = destination_path, .data = contents });
}

fn collectPublishedRecords(
    io: Io,
    allocator: std.mem.Allocator,
    destination_root: []const u8,
    records: *std.array_list.Managed(PublishedRecord),
) !void {
    const runs_path = try std.fmt.allocPrint(allocator, "{s}/runs", .{destination_root});
    const runs_directory = try Io.Dir.cwd().openDir(io, runs_path, .{ .iterate = true });
    defer runs_directory.close(io);

    var iterator = runs_directory.iterate();
    while (try iterator.next(io)) |entry| {
        if (entry.kind != .directory) continue;
        const record_id = try allocator.dupe(u8, entry.name);
        const results_path = try std.fmt.allocPrint(
            allocator,
            "{s}/{s}/results.json",
            .{ runs_path, record_id },
        );
        const document = try readDocument(io, allocator, results_path);
        try validatePublishableDocument(document);
        const record_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ runs_path, record_id });
        _ = try validateReportManifest(io, allocator, record_path);
        try records.append(.{ .id = record_id, .document = document });
    }
    if (records.items.len == 0) return error.MissingPublishedRecords;

    std.mem.sort(PublishedRecord, records.items, {}, newestRecordFirst);
}

fn regenerateDocumentation(io: Io, allocator: std.mem.Allocator, destination_root: []const u8) !void {
    var records = std.array_list.Managed(PublishedRecord).init(allocator);
    defer records.deinit();
    try collectPublishedRecords(io, allocator, destination_root, &records);

    const index_contents = try renderIndexAlloc(allocator, records.items);
    const index_path = try std.fmt.allocPrint(allocator, "{s}/README.md", .{destination_root});
    const trend_contents = try renderTrendAlloc(allocator, records.items);
    const trend_path = try std.fmt.allocPrint(allocator, "{s}/trend.svg", .{destination_root});

    // Render both outputs first. The index is the human-facing commit marker, so
    // replace it last if a filesystem failure interrupts the pair of renames.
    try writeFileAtomic(io, allocator, trend_path, trend_contents);
    try writeFileAtomic(io, allocator, index_path, index_contents);
}

fn checkDocumentation(io: Io, allocator: std.mem.Allocator, destination_root: []const u8) !void {
    var records = std.array_list.Managed(PublishedRecord).init(allocator);
    defer records.deinit();
    try collectPublishedRecords(io, allocator, destination_root, &records);

    const expected_index = try renderIndexAlloc(allocator, records.items);
    const index_path = try std.fmt.allocPrint(allocator, "{s}/README.md", .{destination_root});
    try expectFileContents(io, allocator, index_path, expected_index);

    const expected_trend = try renderTrendAlloc(allocator, records.items);
    const trend_path = try std.fmt.allocPrint(allocator, "{s}/trend.svg", .{destination_root});
    try expectFileContents(io, allocator, trend_path, expected_trend);
}

fn expectFileContents(
    io: Io,
    allocator: std.mem.Allocator,
    path: []const u8,
    expected: []const u8,
) !void {
    const actual = Io.Dir.cwd().readFileAlloc(
        io,
        path,
        allocator,
        .limited(maximum_artifact_bytes),
    ) catch |err| switch (err) {
        error.FileNotFound => return error.GeneratedDocumentationMissing,
        else => return err,
    };
    if (!std.mem.eql(u8, actual, expected)) return error.GeneratedDocumentationOutOfDate;
}

fn newestRecordFirst(_: void, left: PublishedRecord, right: PublishedRecord) bool {
    if (left.document.run.started_at_unix_ns != right.document.run.started_at_unix_ns) {
        return left.document.run.started_at_unix_ns > right.document.run.started_at_unix_ns;
    }
    return std.mem.order(u8, left.id, right.id) == .gt;
}

fn performanceRange(document: PublishedDocument) PerformanceRange {
    std.debug.assert(document.benchmarks.len > 0);
    var range: PerformanceRange = .{
        .minimum_median_ns = document.benchmarks[0].statistics.median,
        .maximum_median_ns = document.benchmarks[0].statistics.median,
        .minimum_operations_per_second = document.benchmarks[0].statistics.operations_per_second,
        .maximum_operations_per_second = document.benchmarks[0].statistics.operations_per_second,
    };
    for (document.benchmarks[1..]) |benchmark| {
        range.minimum_median_ns = @min(range.minimum_median_ns, benchmark.statistics.median);
        range.maximum_median_ns = @max(range.maximum_median_ns, benchmark.statistics.median);
        range.minimum_operations_per_second = @min(
            range.minimum_operations_per_second,
            benchmark.statistics.operations_per_second,
        );
        range.maximum_operations_per_second = @max(
            range.maximum_operations_per_second,
            benchmark.statistics.operations_per_second,
        );
    }
    return range;
}

fn verdictPlain(verdict: []const u8) []const u8 {
    if (std.mem.eql(u8, verdict, "pass")) return "没有发现明确变慢";
    if (std.mem.eql(u8, verdict, "regression")) return "发现明确变慢";
    if (std.mem.eql(u8, verdict, "unstable")) return "结果不稳定，不能下结论";
    if (std.mem.eql(u8, verdict, "noise-warning") or
        std.mem.eql(u8, verdict, "no-baseline-noise-warning"))
    {
        return "测量有波动，建议重跑";
    }
    return "当前速度快照，尚无旧结果";
}

fn comparableBaselineId(records: []const PublishedRecord) []const u8 {
    const latest = records[0];
    var baseline_id: []const u8 = "暂无干净基线";
    for (records) |record| {
        if (!samePublishedEnvironment(record, latest) or record.document.run.git_dirty) continue;
        if (!std.mem.eql(u8, record.document.run.verdict, "no-baseline") and
            !std.mem.eql(u8, record.document.run.verdict, "pass")) continue;
        baseline_id = record.id;
    }
    return baseline_id;
}

fn renderIndex(
    writer: *Io.Writer,
    records: []const PublishedRecord,
) !void {
    const latest = records[0];
    const latest_range = performanceRange(latest.document);
    try writer.print(
        "# Ziwei 基准测试结果\n\n" ++
            "> 本页由 `zig build benchmark-publish` 生成，展示经过显式发布的本命盘构建报告。" ++
            "只读查询目前不在测量范围内。\n\n" ++
            "## 最新已发布记录\n\n" ++
            "- 记录：[`{s}`](runs/{s}/report.md)\n" ++
            "- 版本：`{s}`\n" ++
            "- 结论：{s}\n" ++
            "- 当前速度：构建一张命盘约需 {d:.3}–{d:.3} 微秒；连续单线程计算约为每秒 {d:.2}–{d:.2} 百万张。\n" ++
            "- 环境：`{s}`，`{s}`，`{s}`，Zig `{s}`，{d} 个逻辑核心；指纹 `{s}`。\n" ++
            "- 可比较基线：`{s}`。\n\n" ++
            "完整的大白话总结、统计表和全部图表见[本次报告](runs/{s}/report.md)。\n\n" ++
            "![最新已发布记录的耗时图](runs/{s}/latency.svg)\n\n" ++
            "![最新已发布记录的波动图](runs/{s}/variability.svg)\n\n" ++
            "## 同环境历史趋势\n\n" ++
            "趋势图只连接环境指纹与最新记录完全相同的数据；其他机器的结果不会混成一条线。\n\n" ++
            "![同环境历史趋势](trend.svg)\n\n" ++
            "## 历史记录\n\n" ++
            "| 记录 | 版本 | 环境 | 结论 | 每张耗时 | 完整报告 |\n" ++
            "| --- | --- | --- | --- | ---: | --- |\n",
        .{
            latest.id,
            latest.id,
            latest.document.run.revision,
            verdictPlain(latest.document.run.verdict),
            latest_range.minimum_median_ns / 1000,
            latest_range.maximum_median_ns / 1000,
            latest_range.minimum_operations_per_second / 1_000_000,
            latest_range.maximum_operations_per_second / 1_000_000,
            latest.document.run.runner_id,
            latest.document.run.cpu_model,
            latest.document.run.target,
            latest.document.run.zig_version,
            latest.document.run.cpu_count,
            latest.document.run.environment_fingerprint[0..12],
            comparableBaselineId(records),
            latest.id,
            latest.id,
            latest.id,
        },
    );

    for (records) |record| {
        const range = performanceRange(record.document);
        try writer.print(
            "| `{s}` | `{s}` | `{s}` | {s} | {d:.3}–{d:.3} 微秒 | [查看](runs/{s}/report.md) |\n",
            .{
                record.id,
                record.document.run.revision,
                record.document.run.environment_fingerprint[0..12],
                verdictPlain(record.document.run.verdict),
                range.minimum_median_ns / 1000,
                range.maximum_median_ns / 1000,
                record.id,
            },
        );
    }

    try writer.writeAll(
        "\n## 阅读边界\n\n" ++
            "- 这里只收录完整运行；快速自检报告不能发布。\n" ++
            "- 不同机器的结果只能比较数量级，不能直接判断代码变快或变慢。\n" ++
            "- 标为“测量有波动”或“结果不稳定”的记录可以用于追踪问题，但不能作为优化结论。\n" ++
            "- 名称或版本含 `dirty` 的记录来自未提交工作树，只是开发快照，不是干净提交的权威基线。\n" ++
            "- 原始本地报告仍保存在被忽略的 `benchmark-results/`，不会因为发布文档而自动提交。\n",
    );
}

fn renderIndexAlloc(
    allocator: std.mem.Allocator,
    records: []const PublishedRecord,
) ![]u8 {
    var allocating_writer = Io.Writer.Allocating.init(allocator);
    errdefer allocating_writer.deinit();
    try renderIndex(&allocating_writer.writer, records);
    return allocating_writer.toOwnedSlice();
}

fn writeFileAtomic(
    io: Io,
    allocator: std.mem.Allocator,
    destination_path: []const u8,
    contents: []const u8,
) !void {
    const temporary_path = try std.fmt.allocPrint(allocator, "{s}.tmp", .{destination_path});
    try Io.Dir.cwd().writeFile(io, .{ .sub_path = temporary_path, .data = contents });
    errdefer Io.Dir.cwd().deleteFile(io, temporary_path) catch {};
    try Io.Dir.rename(Io.Dir.cwd(), temporary_path, Io.Dir.cwd(), destination_path, io);
}

fn benchmarkForCase(document: PublishedDocument, case_name: []const u8) PublishedBenchmark {
    for (document.benchmarks) |benchmark| {
        if (std.mem.eql(u8, benchmark.name, case_name)) return benchmark;
    }
    unreachable;
}

fn samePublishedEnvironment(left: PublishedRecord, right: PublishedRecord) bool {
    return std.mem.eql(
        u8,
        left.document.run.environment_fingerprint,
        right.document.run.environment_fingerprint,
    );
}

fn renderTrend(writer: *Io.Writer, records: []const PublishedRecord) !void {
    const latest = records[0];
    var comparable_count: usize = 0;
    var minimum_median = std.math.inf(f64);
    var maximum_median: f64 = 0;
    var oldest_id = latest.id;
    for (records) |record| {
        if (!samePublishedEnvironment(record, latest)) continue;
        comparable_count += 1;
        oldest_id = record.id;
        for (record.document.benchmarks) |benchmark| {
            minimum_median = @min(minimum_median, benchmark.statistics.median);
            maximum_median = @max(maximum_median, benchmark.statistics.median);
        }
    }
    std.debug.assert(comparable_count > 0);
    const latency_span = @max(maximum_median - minimum_median, 1);
    const plot_left: f64 = 100;
    const plot_right: f64 = 1140;
    const plot_top: f64 = 75;
    const plot_bottom: f64 = 405;
    const plot_width = plot_right - plot_left;
    const plot_height = plot_bottom - plot_top;
    const colors = [_][]const u8{ "#2563eb", "#f59e0b", "#16a34a", "#dc2626", "#7c3aed" };

    try writer.writeAll(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"620\" viewBox=\"0 0 1200 620\">\n" ++
            "<style>text{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;fill:#172033}.title{font-size:20px;font-weight:700}.label{font-size:13px}.small{font-size:11px}.grid{stroke:#d8dee9;stroke-width:1}</style>\n" ++
            "<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\n" ++
            "<text x=\"24\" y=\"30\" class=\"title\">Natal construction history (ns/op)</text>\n",
    );
    try writer.print(
        "<text x=\"24\" y=\"52\" class=\"small\">Only {d} record(s) with environment {s} are connected.</text>\n",
        .{ comparable_count, latest.document.run.environment_fingerprint[0..12] },
    );
    for (0..6) |grid_index| {
        const ratio = @as(f64, @floatFromInt(grid_index)) / 5;
        const y = plot_top + ratio * plot_height;
        const latency = maximum_median - ratio * latency_span;
        try writer.print(
            "<line x1=\"{d:.2}\" y1=\"{d:.2}\" x2=\"{d:.2}\" y2=\"{d:.2}\" class=\"grid\"/>" ++
                "<text x=\"92\" y=\"{d:.2}\" text-anchor=\"end\" class=\"small\">{d:.1}</text>\n",
            .{ plot_left, y, plot_right, y, y + 4, latency },
        );
    }

    for (contract.benchmark_cases, 0..) |benchmark_case, case_index| {
        const color = colors[case_index];
        try writer.print("<polyline fill=\"none\" stroke=\"{s}\" stroke-width=\"2.5\" points=\"", .{color});
        var comparable_index: usize = 0;
        var reverse_index = records.len;
        while (reverse_index > 0) {
            reverse_index -= 1;
            const record = records[reverse_index];
            if (!samePublishedEnvironment(record, latest)) continue;
            const benchmark = benchmarkForCase(record.document, benchmark_case.name);
            const x = if (comparable_count == 1)
                plot_left + plot_width / 2
            else
                plot_left + @as(f64, @floatFromInt(comparable_index)) /
                    @as(f64, @floatFromInt(comparable_count - 1)) * plot_width;
            const y = plot_bottom - (benchmark.statistics.median - minimum_median) / latency_span * plot_height;
            try writer.print("{d:.2},{d:.2} ", .{ x, y });
            comparable_index += 1;
        }
        try writer.writeAll("\"/>\n");

        comparable_index = 0;
        reverse_index = records.len;
        while (reverse_index > 0) {
            reverse_index -= 1;
            const record = records[reverse_index];
            if (!samePublishedEnvironment(record, latest)) continue;
            const benchmark = benchmarkForCase(record.document, benchmark_case.name);
            const x = if (comparable_count == 1)
                plot_left + plot_width / 2
            else
                plot_left + @as(f64, @floatFromInt(comparable_index)) /
                    @as(f64, @floatFromInt(comparable_count - 1)) * plot_width;
            const y = plot_bottom - (benchmark.statistics.median - minimum_median) / latency_span * plot_height;
            try writer.print(
                "<circle cx=\"{d:.2}\" cy=\"{d:.2}\" r=\"4\" fill=\"{s}\"><title>{s}: {d:.3} ns/op</title></circle>\n",
                .{ x, y, color, record.id, benchmark.statistics.median },
            );
            comparable_index += 1;
        }
    }

    if (comparable_count == 1) {
        try writer.print(
            "<text x=\"{d:.2}\" y=\"430\" text-anchor=\"middle\" class=\"small\">{s}</text>\n",
            .{ plot_left + plot_width / 2, latest.id },
        );
    } else {
        try writer.print(
            "<text x=\"{d:.2}\" y=\"430\" class=\"small\">{s}</text>" ++
                "<text x=\"{d:.2}\" y=\"430\" text-anchor=\"end\" class=\"small\">{s}</text>\n",
            .{ plot_left, oldest_id, plot_right, latest.id },
        );
    }
    for (contract.benchmark_cases, 0..) |benchmark_case, case_index| {
        const y: f64 = 466 + @as(f64, @floatFromInt(case_index)) * 27;
        try writer.print(
            "<line x1=\"100\" y1=\"{d:.2}\" x2=\"130\" y2=\"{d:.2}\" stroke=\"{s}\" stroke-width=\"3\"/>" ++
                "<text x=\"142\" y=\"{d:.2}\" class=\"label\">{s}</text>\n",
            .{ y, y, colors[case_index], y + 5, benchmark_case.display_name },
        );
    }
    try writer.writeAll("</svg>\n");
}

fn renderTrendAlloc(
    allocator: std.mem.Allocator,
    records: []const PublishedRecord,
) ![]u8 {
    var allocating_writer = Io.Writer.Allocating.init(allocator);
    errdefer allocating_writer.deinit();
    try renderTrend(&allocating_writer.writer, records);
    return allocating_writer.toOwnedSlice();
}

test "official report publisher rejects smoke runs" {
    try std.testing.expectError(
        error.SmokeReportCannotBePublished,
        validatePublishableRun(results_schema_version, "smoke", "working-tree"),
    );
}

test "official report publisher accepts a complete full run identity" {
    try validatePublishableRun(results_schema_version, "full", "a1b2c3d");
}

test "publisher accepts only supported verdict values" {
    try std.testing.expect(isSupportedVerdict("no-baseline"));
    try std.testing.expect(isSupportedVerdict("no-baseline-noise-warning"));
    try std.testing.expect(isSupportedVerdict("noise-warning"));
    try std.testing.expect(isSupportedVerdict("unstable"));
    try std.testing.expect(isSupportedVerdict("pass"));
    try std.testing.expect(isSupportedVerdict("regression"));
    try std.testing.expect(!isSupportedVerdict("looks-fast"));
}

test "publisher accepts only full lowercase hex git commits" {
    try std.testing.expect(isGitCommit("a1b2c3d4e5f67890123456789012345678901234"));
    try std.testing.expect(!isGitCommit("a1b2c3d4e5f6"));
    try std.testing.expect(!isGitCommit("A1B2C3D4E5F67890123456789012345678901234"));
    try std.testing.expect(!isGitCommit("z1b2c3d4e5f67890123456789012345678901234"));
}

test "published record id cannot escape the documentation directory" {
    try std.testing.expectError(error.InvalidRecordId, validateRecordId("../outside"));
    try validateRecordId("2026-08-15-a1b2c3d-dirty");
}

test "publisher rejects a report with only part of the benchmark contract" {
    const environment: contract.Environment = .{
        .runner_id = "runner-a",
        .cpu_model = "Example CPU",
        .os_version = "Example OS",
        .target = "aarch64-macos-none",
        .zig_version = "0.16.0",
        .zig_backend = "stage2_llvm",
        .optimize_mode = "ReleaseFast",
        .cpu_count = 8,
    };
    const environment_fingerprint = contract.environmentFingerprint(environment);
    const document: PublishedDocument = .{
        .schema_version = results_schema_version,
        .run = .{
            .started_at_unix_ns = 1,
            .revision = "a1b2c3d4e5f6",
            .git_commit = "a1b2c3d4e5f67890123456789012345678901234",
            .run_kind = "full",
            .zig_version = environment.zig_version,
            .zig_backend = environment.zig_backend,
            .optimize_mode = environment.optimize_mode,
            .target = environment.target,
            .fixture_id = "natal-v1",
            .cpu_count = environment.cpu_count,
            .cpu_model = environment.cpu_model,
            .os_version = environment.os_version,
            .runner_id = environment.runner_id,
            .environment_fingerprint = &environment_fingerprint,
            .warmups = 60,
            .configured_samples = 100,
            .target_sample_ns = 50 * std.time.ns_per_ms,
            .verdict = "no-baseline",
        },
        .benchmarks = &.{.{
            .name = "natal/create_from_input/single",
            .operations_per_iteration = 1,
            .sample_count = 100,
            .unit = "ns/op",
            .statistics = .{ .median = 250, .operations_per_second = 4_000_000 },
        }},
    };
    try std.testing.expectError(error.IncompatibleBenchmarkSet, validatePublishableDocument(document));
}

fn writeSyntheticFullReport(
    io: Io,
    allocator: std.mem.Allocator,
    source_directory: []const u8,
) !void {
    try Io.Dir.cwd().createDirPath(io, source_directory);
    for (contract.required_artifacts) |artifact| {
        if (std.mem.eql(u8, artifact, "results.json")) continue;
        const path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ source_directory, artifact });
        const contents = if (std.mem.endsWith(u8, artifact, ".svg"))
            "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>\n"
        else
            "synthetic artifact\n";
        try Io.Dir.cwd().writeFile(io, .{ .sub_path = path, .data = contents });
    }

    const environment: contract.Environment = .{
        .runner_id = "runner-a",
        .cpu_model = "Example CPU",
        .os_version = "Example OS",
        .target = "aarch64-macos-none",
        .zig_version = "0.16.0",
        .zig_backend = "stage2_llvm",
        .optimize_mode = "ReleaseFast",
        .cpu_count = 8,
    };
    const environment_fingerprint = contract.environmentFingerprint(environment);
    var results_writer = Io.Writer.Allocating.init(allocator);
    defer results_writer.deinit();
    try results_writer.writer.print(
        "{{\n  \"schema_version\": {d},\n  \"run\": {{\n" ++
            "    \"started_at_unix_ns\": 1000,\n" ++
            "    \"revision\": \"a1b2c3d4e5f6\",\n" ++
            "    \"git_commit\": \"a1b2c3d4e5f67890123456789012345678901234\",\n" ++
            "    \"git_dirty\": false,\n" ++
            "    \"run_kind\": \"full\",\n" ++
            "    \"zig_version\": \"{s}\",\n" ++
            "    \"zig_backend\": \"{s}\",\n" ++
            "    \"optimize_mode\": \"{s}\",\n" ++
            "    \"target\": \"{s}\",\n" ++
            "    \"fixture_id\": \"{s}\",\n" ++
            "    \"cpu_count\": {d},\n" ++
            "    \"cpu_model\": \"{s}\",\n" ++
            "    \"os_version\": \"{s}\",\n" ++
            "    \"runner_id\": \"{s}\",\n" ++
            "    \"environment_fingerprint\": \"{s}\",\n" ++
            "    \"warmups\": 60,\n" ++
            "    \"configured_samples\": 100,\n" ++
            "    \"target_sample_ns\": 50000000,\n" ++
            "    \"verdict\": \"no-baseline\"\n  }},\n  \"benchmarks\": [\n",
        .{
            contract.results_schema_version,
            environment.zig_version,
            environment.zig_backend,
            environment.optimize_mode,
            environment.target,
            contract.fixture_id,
            environment.cpu_count,
            environment.cpu_model,
            environment.os_version,
            environment.runner_id,
            &environment_fingerprint,
        },
    );
    for (contract.benchmark_cases, 0..) |benchmark_case, index| {
        try results_writer.writer.print(
            "    {{\"name\": \"{s}\", \"operations_per_iteration\": {d}, " ++
                "\"sample_count\": {d}, \"unit\": \"ns/op\", " ++
                "\"statistics\": {{\"median\": {d}, \"operations_per_second\": {d}}}}}{s}\n",
            .{
                benchmark_case.name,
                benchmark_case.operations_per_iteration,
                contract.expectedSampleCount(benchmark_case, 100),
                240 + index * 10,
                4_000_000 - index * 100_000,
                if (index + 1 == contract.benchmark_cases.len) "" else ",",
            },
        );
    }
    try results_writer.writer.writeAll("  ]\n}\n");
    const results_path = try std.fmt.allocPrint(allocator, "{s}/results.json", .{source_directory});
    try Io.Dir.cwd().writeFile(io, .{
        .sub_path = results_path,
        .data = results_writer.writer.buffered(),
    });

    var manifest_writer = Io.Writer.Allocating.init(allocator);
    defer manifest_writer.deinit();
    try manifest_writer.writer.print(
        "{{\n  \"schema_version\": {d},\n  \"results_schema_version\": {d},\n  \"artifacts\": [\n",
        .{ contract.manifest_schema_version, contract.results_schema_version },
    );
    for (contract.required_artifacts, 0..) |artifact, index| {
        const path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ source_directory, artifact });
        const contents = try Io.Dir.cwd().readFileAlloc(io, path, allocator, .limited(maximum_artifact_bytes));
        const digest = contract.sha256Hex(contents);
        try manifest_writer.writer.print(
            "    {{\"name\": \"{s}\", \"sha256\": \"{s}\"}}{s}\n",
            .{ artifact, &digest, if (index + 1 == contract.required_artifacts.len) "" else "," },
        );
    }
    try manifest_writer.writer.writeAll("  ]\n}\n");
    const manifest_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ source_directory, contract.manifest_artifact });
    try Io.Dir.cwd().writeFile(io, .{
        .sub_path = manifest_path,
        .data = manifest_writer.writer.buffered(),
    });
}

test "publishing is end-to-end verifiable and detects artifact tampering" {
    var temporary = std.testing.tmpDir(.{});
    defer temporary.cleanup();
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    const root = try std.fmt.allocPrint(allocator, ".zig-cache/tmp/{s}", .{temporary.sub_path});
    const source = try std.fmt.allocPrint(allocator, "{s}/source", .{root});
    const destination = try std.fmt.allocPrint(allocator, "{s}/docs", .{root});
    try writeSyntheticFullReport(std.testing.io, allocator, source);

    try publishReport(
        std.testing.io,
        allocator,
        source,
        "2026-08-15-a1b2c3d4e5f6",
        destination,
    );
    try checkDocumentation(std.testing.io, allocator, destination);

    const generated_index = try std.fmt.allocPrint(allocator, "{s}/README.md", .{destination});
    try Io.Dir.cwd().writeFile(std.testing.io, .{
        .sub_path = generated_index,
        .data = "out of date",
    });
    try std.testing.expectError(
        error.GeneratedDocumentationOutOfDate,
        checkDocumentation(std.testing.io, allocator, destination),
    );
    try regenerateDocumentation(std.testing.io, allocator, destination);
    try checkDocumentation(std.testing.io, allocator, destination);

    const published_report = try std.fmt.allocPrint(
        allocator,
        "{s}/runs/2026-08-15-a1b2c3d4e5f6/report.md",
        .{destination},
    );
    try Io.Dir.cwd().writeFile(std.testing.io, .{
        .sub_path = published_report,
        .data = "tampered",
    });
    try std.testing.expectError(
        error.ArtifactDigestMismatch,
        checkDocumentation(std.testing.io, allocator, destination),
    );
}
