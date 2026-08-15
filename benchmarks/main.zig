//! 本命盘构建基准与自包含报告生成器。

const builtin = @import("builtin");
const contract = @import("report_contract.zig");
const std = @import("std");
const ziwei = @import("ziwei");

const Io = std.Io;
const results_schema_version = contract.results_schema_version;
const fixture_id = contract.fixture_id;
const max_samples = 100;
const bootstrap_resample_count = 1000;
const comparison_threshold_percent: f64 = 5;
const representative_case_count = 60;
const chart_width_px: usize = 1200;
const chart_label_x_px: f64 = 24;
const chart_label_font_size_px: f64 = 13;
const chart_monospace_advance_em: f64 = 0.62;
const chart_label_plot_gap_px: f64 = 24;
const chart_plot_x_px: f64 = 440;
const chart_plot_width_px: f64 = 650;

const Config = struct {
    warmup_count: usize = 60,
    sample_count: usize = 100,
    target_ns: u64 = 50 * std.time.ns_per_ms,
    output_root: []const u8 = "benchmark-results",
    revision: []const u8 = "unknown",
    seed: u64 = 0x5eed_2026,
    quick: bool = false,
    baseline_name: ?[]const u8 = null,
    baseline_path: ?[]const u8 = null,
};

const RunIdentity = struct {
    git_commit: []const u8,
    git_dirty: bool,
    revision: []const u8,
    environment: contract.Environment,
    environment_fingerprint: [64]u8,
};

const Context = struct {
    births: [representative_case_count]ziwei.ZiweiBirth,
    inputs: [representative_case_count]ziwei.ZiweiInput,
};

const WorkloadFn = *const fn (*const Context) void;

const Benchmark = struct {
    name: []const u8,
    operation_count: usize,
    workload: WorkloadFn,
    expensive: bool = false,
};

const Statistics = struct {
    minimum: f64,
    q1: f64,
    median: f64,
    q3: f64,
    p90: f64,
    p95: f64,
    p99: f64,
    maximum: f64,
    mean: f64,
    standard_deviation: f64,
    median_absolute_deviation: f64,
    relative_standard_deviation_percent: f64,
    confidence_95_lower: f64,
    confidence_95_upper: f64,
    outlier_count: usize,
    severe_outlier_count: usize,
    operations_per_second: f64,
};

const BenchmarkResult = struct {
    benchmark: Benchmark,
    iterations_per_sample: usize,
    sample_count: usize,
    samples_elapsed_ns: [max_samples]u64,
    samples_ns_per_operation: [max_samples]f64,
    statistics: Statistics,
};

const SampleComparison = struct {
    baseline_median: f64,
    current_median: f64,
    median_change_percent: f64,
    baseline_p95: f64,
    current_p95: f64,
    p95_change_percent: f64,
    confidence_95_lower_percent: f64,
    confidence_95_upper_percent: f64,
};

const ComparisonVerdict = enum { pass, regression };

const BaselineRun = struct {
    started_at_unix_ns: i128 = 0,
    run_kind: []const u8 = "",
    revision: []const u8 = "",
    git_commit: []const u8 = "",
    git_dirty: bool = false,
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
    warmups: usize = 0,
    configured_samples: usize = 0,
    target_sample_ns: u64 = 0,
    verdict: []const u8 = "",
};

const BaselineBenchmark = struct {
    name: []const u8 = "",
    operations_per_iteration: usize = 0,
    sample_count: usize = 0,
    unit: []const u8 = "",
    samples_ns_per_operation: []const f64 = &.{},
};

const BaselineDocument = struct {
    schema_version: u32 = 0,
    run: BaselineRun = .{},
    benchmarks: []const BaselineBenchmark = &.{},
};

const Baseline = struct {
    name: []const u8,
    path: []const u8,
    document: BaselineDocument,
};

const BenchmarkComparison = struct {
    values: SampleComparison,
    verdict: ComparisonVerdict,
};

const benchmarks = [_]Benchmark{
    .{
        .name = contract.benchmark_cases[0].name,
        .operation_count = contract.benchmark_cases[0].operations_per_iteration,
        .workload = runSingleInput,
    },
    .{
        .name = contract.benchmark_cases[1].name,
        .operation_count = contract.benchmark_cases[1].operations_per_iteration,
        .workload = runSingleBirth,
    },
    .{
        .name = contract.benchmark_cases[2].name,
        .operation_count = contract.benchmark_cases[2].operations_per_iteration,
        .workload = runRepresentativeInputs,
    },
    .{
        .name = contract.benchmark_cases[3].name,
        .operation_count = contract.benchmark_cases[3].operations_per_iteration,
        .workload = runRepresentativeBirths,
    },
    .{
        .name = contract.benchmark_cases[4].name,
        .operation_count = contract.benchmark_cases[4].operations_per_iteration,
        .workload = runExhaustiveInputs,
        .expensive = contract.benchmark_cases[4].expensive,
    },
};

fn estimatedChartLabelRightPx(label: []const u8) f64 {
    return chart_label_x_px +
        @as(f64, @floatFromInt(label.len)) * chart_label_font_size_px * chart_monospace_advance_em;
}

test "benchmark chart labels stay outside the plot area" {
    for (benchmarks) |benchmark| {
        try std.testing.expect(
            estimatedChartLabelRightPx(benchmark.name) + chart_label_plot_gap_px <= chart_plot_x_px,
        );
    }
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const arena = init.arena.allocator();
    const args = try init.minimal.args.toSlice(arena);
    const parsed = try parseConfig(args);
    if (parsed == null) {
        try printUsage(io);
        return;
    }
    var config = parsed.?;
    const identity = try collectRunIdentity(io, arena, init.environ_map);
    config.revision = identity.revision;
    const baseline = if (config.baseline_path != null)
        try loadBaseline(io, arena, config, &identity.environment_fingerprint)
    else
        null;
    const started_at_ns = Io.Clock.real.now(io).nanoseconds;

    const context = createContext();
    var results: [benchmarks.len]BenchmarkResult = undefined;

    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = Io.File.stdout().writer(io, &stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print(
        "Ziwei natal benchmarks: mode={s}, warmups={d}, samples={d}, target_ms={d}, seed={d}\n",
        .{ @tagName(builtin.mode), config.warmup_count, config.sample_count, config.target_ns / std.time.ns_per_ms, config.seed },
    );
    try stdout.flush();

    runBenchmarksInterleaved(io, config, &context, &results);
    var comparison_storage: [benchmarks.len]BenchmarkComparison = undefined;
    const comparisons: ?[]const BenchmarkComparison = if (baseline) |value| blk: {
        buildComparisons(&results, value.document, config.seed, &comparison_storage);
        break :blk &comparison_storage;
    } else null;
    for (results) |result| {
        const stats = result.statistics;
        try stdout.print(
            "  {s}: median={d:.3} ns/op, p95={d:.3} ns/op, RSD={d:.2}%\n",
            .{ result.benchmark.name, stats.median, stats.p95, stats.relative_standard_deviation_percent },
        );
        try stdout.flush();
    }

    var directory_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const report_directory = try std.fmt.bufPrint(
        &directory_buffer,
        "{s}/run-{d}",
        .{ config.output_root, started_at_ns },
    );
    try Io.Dir.cwd().createDirPath(io, report_directory);

    try writeReports(io, arena, report_directory, started_at_ns, config, identity, &results, baseline, comparisons);
    try stdout.print("Report: {s}/report.md\n", .{report_directory});
    try stdout.flush();
}

fn parseConfig(args: []const []const u8) !?Config {
    var config: Config = .{};
    var index: usize = 1;
    while (index < args.len) : (index += 1) {
        const argument = args[index];
        if (std.mem.eql(u8, argument, "--help")) return null;
        if (std.mem.eql(u8, argument, "--quick")) {
            config.quick = true;
            continue;
        }
        if (std.mem.eql(u8, argument, "--warmups")) {
            index += 1;
            if (index == args.len) return error.MissingArgumentValue;
            config.warmup_count = try std.fmt.parseUnsigned(usize, args[index], 10);
            if (config.warmup_count > 200) return error.InvalidWarmupCount;
            continue;
        }
        if (std.mem.eql(u8, argument, "--samples")) {
            index += 1;
            if (index == args.len) return error.MissingArgumentValue;
            config.sample_count = try std.fmt.parseUnsigned(usize, args[index], 10);
            if (config.sample_count < 3 or config.sample_count > max_samples) {
                return error.InvalidSampleCount;
            }
            continue;
        }
        if (std.mem.eql(u8, argument, "--target-ms")) {
            index += 1;
            if (index == args.len) return error.MissingArgumentValue;
            const target_ms = try std.fmt.parseUnsigned(u64, args[index], 10);
            if (target_ms == 0 or target_ms > 10_000) return error.InvalidTargetDuration;
            config.target_ns = try std.math.mul(u64, target_ms, std.time.ns_per_ms);
            continue;
        }
        if (std.mem.eql(u8, argument, "--output-root")) {
            index += 1;
            if (index == args.len or args[index].len == 0) return error.MissingArgumentValue;
            config.output_root = args[index];
            continue;
        }
        if (std.mem.eql(u8, argument, "--seed")) {
            index += 1;
            if (index == args.len) return error.MissingArgumentValue;
            config.seed = try std.fmt.parseUnsigned(u64, args[index], 10);
            continue;
        }
        if (std.mem.eql(u8, argument, "--baseline")) {
            index += 1;
            if (index == args.len or args[index].len == 0) return error.MissingArgumentValue;
            config.baseline_name = args[index];
            index += 1;
            if (index == args.len or args[index].len == 0) return error.MissingArgumentValue;
            config.baseline_path = args[index];
            continue;
        }
        return error.UnknownArgument;
    }

    if (config.quick and config.baseline_path != null) return error.BaselineRequiresFullRun;
    if (config.quick) {
        config.warmup_count = @min(config.warmup_count, 1);
        config.sample_count = @min(config.sample_count, 5);
        config.target_ns = @min(config.target_ns, 2 * std.time.ns_per_ms);
    }
    return config;
}

fn printUsage(io: Io) !void {
    var buffer: [2048]u8 = undefined;
    var writer = Io.File.stdout().writer(io, &buffer);
    const stdout = &writer.interface;
    try stdout.writeAll(
        \\Usage: zig build benchmark -- [options]
        \\
        \\Options:
        \\  --warmups <count>       Warm-up measurements per case (default: 60)
        \\  --samples <count>       Samples per case, 3-100 (default: 100)
        \\  --target-ms <ms>        Target duration per sample (default: 50)
        \\  --output-root <path>    Parent directory for run reports
        \\  --seed <integer>        Seed for interleaved case ordering
        \\  --baseline <name> <results.json>
        \\                          Named full-run baseline used for report-only comparison
        \\  --quick                 Small smoke run for harness verification
        \\  --help                  Show this help
        \\
    );
    try stdout.flush();
}

fn createContext() Context {
    var context: Context = undefined;
    for (0..representative_case_count) |index| {
        const month: u8 = @intCast((index * 5) % 12);
        const day: u8 = @intCast((index * 7) % 30 + 1);
        const hour: u8 = @intCast((index * 7 + 3) % 12);
        const gender: ziwei.Gender = if (index % 4 < 2) .yang else .yin;
        context.births[index] = ziwei.ZiweiBirth.init(
            gender,
            1984 + @as(i32, @intCast(index)),
            month,
            day,
            hour,
        ) catch unreachable;
        context.inputs[index] = ziwei.ZiweiInput.init(
            gender,
            ziwei.Stem.all[index % ziwei.Stem.all.len],
            ziwei.Branch.all[index % ziwei.Branch.all.len],
            month,
            day,
            hour,
        ) catch unreachable;
    }
    return context;
}

fn runRepresentativeInputs(context: *const Context) void {
    for (context.inputs) |input| {
        const natal = ziwei.createFromInput(input) catch unreachable;
        std.mem.doNotOptimizeAway(&natal);
    }
}

fn runSingleInput(context: *const Context) void {
    const natal = ziwei.createFromInput(context.inputs[0]) catch unreachable;
    std.mem.doNotOptimizeAway(&natal);
}

fn runRepresentativeBirths(context: *const Context) void {
    for (context.births) |birth| {
        const natal = ziwei.createFromBirth(birth) catch unreachable;
        std.mem.doNotOptimizeAway(&natal);
    }
}

fn runSingleBirth(context: *const Context) void {
    const natal = ziwei.createFromBirth(context.births[0]) catch unreachable;
    std.mem.doNotOptimizeAway(&natal);
}

fn runExhaustiveInputs(_: *const Context) void {
    const genders = [_]ziwei.Gender{ .yang, .yin };
    for (genders) |gender| {
        for (0..60) |pillar_index| {
            const stem = ziwei.Stem.all[pillar_index % ziwei.Stem.all.len];
            const branch = ziwei.Branch.all[pillar_index % ziwei.Branch.all.len];
            for (0..12) |month| {
                for (1..31) |day| {
                    for (0..12) |hour| {
                        const input: ziwei.ZiweiInput = .{
                            .gender = gender,
                            .birth_stem = stem,
                            .birth_branch = branch,
                            .month = @intCast(month),
                            .day = @intCast(day),
                            .hour = @intCast(hour),
                        };
                        const natal = ziwei.createFromInput(input) catch unreachable;
                        std.mem.doNotOptimizeAway(&natal);
                    }
                }
            }
        }
    }
}

fn runBenchmarksInterleaved(
    io: Io,
    config: Config,
    context: *const Context,
    results: *[benchmarks.len]BenchmarkResult,
) void {
    for (benchmarks, 0..) |benchmark, index| {
        const sample_count = if (benchmark.expensive) @min(config.sample_count, 10) else config.sample_count;
        results[index] = .{
            .benchmark = benchmark,
            .iterations_per_sample = calibrate(io, context, benchmark, config.target_ns),
            .sample_count = sample_count,
            .samples_elapsed_ns = undefined,
            .samples_ns_per_operation = undefined,
            .statistics = undefined,
        };
    }

    var random_state = config.seed;
    for (0..config.warmup_count) |round| {
        const order = shuffledBenchmarkOrder(&random_state);
        for (order) |index| {
            const benchmark = results[index].benchmark;
            const warmup_count = if (benchmark.expensive) @min(config.warmup_count, 1) else config.warmup_count;
            if (round < warmup_count) {
                _ = measure(io, context, benchmark, results[index].iterations_per_sample);
            }
        }
    }

    for (0..config.sample_count) |sample_index| {
        const order = shuffledBenchmarkOrder(&random_state);
        for (order) |index| {
            var result = &results[index];
            if (sample_index >= result.sample_count) continue;
            const elapsed_ns = measure(io, context, result.benchmark, result.iterations_per_sample);
            const total_operations = result.iterations_per_sample * result.benchmark.operation_count;
            result.samples_elapsed_ns[sample_index] = elapsed_ns;
            result.samples_ns_per_operation[sample_index] = @as(f64, @floatFromInt(elapsed_ns)) /
                @as(f64, @floatFromInt(total_operations));
        }
    }

    for (results) |*result| {
        result.statistics = calculateStatistics(result.samples_ns_per_operation[0..result.sample_count]);
    }
}

fn shuffledBenchmarkOrder(random_state: *u64) [benchmarks.len]usize {
    var order: [benchmarks.len]usize = undefined;
    for (&order, 0..) |*value, index| value.* = index;

    var remaining = order.len;
    while (remaining > 1) {
        remaining -= 1;
        const swap_index: usize = @intCast(nextRandom(random_state) % (remaining + 1));
        std.mem.swap(usize, &order[remaining], &order[swap_index]);
    }
    return order;
}

fn nextRandom(state: *u64) u64 {
    state.* = state.* *% 6_364_136_223_846_793_005 +% 1_442_695_040_888_963_407;
    return state.*;
}

fn calibrate(io: Io, context: *const Context, benchmark: Benchmark, target_ns: u64) usize {
    var iterations: usize = 1;
    while (iterations < 1_000_000) {
        const elapsed_ns = measure(io, context, benchmark, iterations);
        if (elapsed_ns >= target_ns or benchmark.expensive) return iterations;
        if (elapsed_ns == 0) {
            iterations = @min(iterations * 10, 1_000_000);
            continue;
        }
        const scaled = @as(u128, target_ns) * iterations / elapsed_ns;
        const next = @max(iterations + 1, @as(usize, @intCast(@min(scaled, 1_000_000))));
        iterations = @min(next, iterations * 10);
    }
    return iterations;
}

fn measure(io: Io, context: *const Context, benchmark: Benchmark, iterations: usize) u64 {
    const started_at = Io.Clock.awake.now(io).nanoseconds;
    for (0..iterations) |_| benchmark.workload(context);
    const elapsed = Io.Clock.awake.now(io).nanoseconds - started_at;
    return @intCast(@max(elapsed, 0));
}

fn calculateStatistics(samples: []const f64) Statistics {
    var ordered: [max_samples]f64 = undefined;
    @memcpy(ordered[0..samples.len], samples);
    insertionSort(ordered[0..samples.len]);

    var sum: f64 = 0;
    for (samples) |sample| sum += sample;
    const mean = sum / @as(f64, @floatFromInt(samples.len));

    var squared_difference_sum: f64 = 0;
    for (samples) |sample| {
        const difference = sample - mean;
        squared_difference_sum += difference * difference;
    }
    const standard_deviation = if (samples.len > 1)
        @sqrt(squared_difference_sum / @as(f64, @floatFromInt(samples.len - 1)))
    else
        0;
    const standard_error = standard_deviation / @sqrt(@as(f64, @floatFromInt(samples.len)));
    const confidence_margin = 1.96 * standard_error;

    const q1 = percentile(ordered[0..samples.len], 0.25);
    const q3 = percentile(ordered[0..samples.len], 0.75);
    const interquartile_range = q3 - q1;
    const lower_fence = q1 - 1.5 * interquartile_range;
    const upper_fence = q3 + 1.5 * interquartile_range;
    const severe_lower_fence = q1 - 3 * interquartile_range;
    const severe_upper_fence = q3 + 3 * interquartile_range;
    var outlier_count: usize = 0;
    var severe_outlier_count: usize = 0;
    for (samples) |sample| {
        if (sample < lower_fence or sample > upper_fence) outlier_count += 1;
        if (sample < severe_lower_fence or sample > severe_upper_fence) severe_outlier_count += 1;
    }

    const median = percentile(ordered[0..samples.len], 0.5);
    var absolute_deviations: [max_samples]f64 = undefined;
    for (samples, 0..) |sample, index| absolute_deviations[index] = @abs(sample - median);
    insertionSort(absolute_deviations[0..samples.len]);
    return .{
        .minimum = ordered[0],
        .q1 = q1,
        .median = median,
        .q3 = q3,
        .p90 = percentile(ordered[0..samples.len], 0.90),
        .p95 = percentile(ordered[0..samples.len], 0.95),
        .p99 = percentile(ordered[0..samples.len], 0.99),
        .maximum = ordered[samples.len - 1],
        .mean = mean,
        .standard_deviation = standard_deviation,
        .median_absolute_deviation = percentile(absolute_deviations[0..samples.len], 0.5),
        .relative_standard_deviation_percent = if (mean == 0) 0 else standard_deviation / mean * 100,
        .confidence_95_lower = @max(0, mean - confidence_margin),
        .confidence_95_upper = mean + confidence_margin,
        .outlier_count = outlier_count,
        .severe_outlier_count = severe_outlier_count,
        .operations_per_second = if (median == 0) 0 else std.time.ns_per_s / median,
    };
}

fn insertionSort(values: []f64) void {
    for (1..values.len) |index| {
        const value = values[index];
        var position = index;
        while (position > 0 and values[position - 1] > value) : (position -= 1) {
            values[position] = values[position - 1];
        }
        values[position] = value;
    }
}

fn percentile(ordered: []const f64, fraction: f64) f64 {
    if (ordered.len == 1) return ordered[0];
    const position = fraction * @as(f64, @floatFromInt(ordered.len - 1));
    const lower_index: usize = @intFromFloat(@floor(position));
    const upper_index = @min(lower_index + 1, ordered.len - 1);
    const weight = position - @as(f64, @floatFromInt(lower_index));
    return ordered[lower_index] * (1 - weight) + ordered[upper_index] * weight;
}

fn compareSamples(baseline_samples: []const f64, current_samples: []const f64, seed: u64) SampleComparison {
    std.debug.assert(baseline_samples.len >= 3 and baseline_samples.len <= max_samples);
    std.debug.assert(current_samples.len >= 3 and current_samples.len <= max_samples);

    const baseline_statistics = calculateStatistics(baseline_samples);
    const current_statistics = calculateStatistics(current_samples);
    std.debug.assert(baseline_statistics.median > 0);

    var random_state = seed;
    var changes: [bootstrap_resample_count]f64 = undefined;
    var baseline_resample: [max_samples]f64 = undefined;
    var current_resample: [max_samples]f64 = undefined;
    for (&changes) |*change| {
        for (baseline_resample[0..baseline_samples.len]) |*sample| {
            const index: usize = @intCast(nextRandom(&random_state) % baseline_samples.len);
            sample.* = baseline_samples[index];
        }
        for (current_resample[0..current_samples.len]) |*sample| {
            const index: usize = @intCast(nextRandom(&random_state) % current_samples.len);
            sample.* = current_samples[index];
        }
        insertionSort(baseline_resample[0..baseline_samples.len]);
        insertionSort(current_resample[0..current_samples.len]);
        const baseline_median = percentile(baseline_resample[0..baseline_samples.len], 0.5);
        const current_median = percentile(current_resample[0..current_samples.len], 0.5);
        change.* = (current_median / baseline_median - 1) * 100;
    }
    insertionSort(&changes);

    return .{
        .baseline_median = baseline_statistics.median,
        .current_median = current_statistics.median,
        .median_change_percent = (current_statistics.median / baseline_statistics.median - 1) * 100,
        .baseline_p95 = baseline_statistics.p95,
        .current_p95 = current_statistics.p95,
        .p95_change_percent = (current_statistics.p95 / baseline_statistics.p95 - 1) * 100,
        .confidence_95_lower_percent = percentile(&changes, 0.025),
        .confidence_95_upper_percent = percentile(&changes, 0.975),
    };
}

fn loadBaseline(
    io: Io,
    allocator: std.mem.Allocator,
    config: Config,
    current_environment_fingerprint: []const u8,
) !Baseline {
    const path = config.baseline_path.?;
    const contents = try Io.Dir.cwd().readFileAlloc(io, path, allocator, .limited(8 * 1024 * 1024));
    const document = try std.json.parseFromSliceLeaky(
        BaselineDocument,
        allocator,
        contents,
        .{ .ignore_unknown_fields = true },
    );
    try validateBaseline(document, config, current_environment_fingerprint);
    return .{
        .name = config.baseline_name.?,
        .path = path,
        .document = document,
    };
}

fn buildComparisons(
    results: []const BenchmarkResult,
    baseline: BaselineDocument,
    seed: u64,
    output: *[benchmarks.len]BenchmarkComparison,
) void {
    for (results, 0..) |result, index| {
        const baseline_benchmark = for (baseline.benchmarks) |candidate| {
            if (std.mem.eql(u8, candidate.name, result.benchmark.name)) break candidate;
        } else unreachable;
        const values = compareSamples(
            baseline_benchmark.samples_ns_per_operation,
            result.samples_ns_per_operation[0..result.sample_count],
            seed +% @as(u64, @intCast(index)),
        );
        output[index] = .{
            .values = values,
            .verdict = comparisonVerdict(values),
        };
    }
}

fn comparisonVerdict(comparison: SampleComparison) ComparisonVerdict {
    return if (comparison.confidence_95_lower_percent > comparison_threshold_percent)
        .regression
    else
        .pass;
}

fn currentTarget(buffer: []u8) []const u8 {
    return std.fmt.bufPrint(
        buffer,
        "{s}-{s}-{s}",
        .{ @tagName(builtin.target.cpu.arch), @tagName(builtin.target.os.tag), @tagName(builtin.target.abi) },
    ) catch unreachable;
}

fn runCommand(
    io: Io,
    allocator: std.mem.Allocator,
    argv: []const []const u8,
) ![]const u8 {
    const result = try std.process.run(allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1024 * 1024),
        .stderr_limit = .limited(1024 * 1024),
    });
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    switch (result.term) {
        .exited => |code| if (code != 0) return error.IdentityCommandFailed,
        else => return error.IdentityCommandFailed,
    }
    return allocator.dupe(u8, std.mem.trim(u8, result.stdout, " \t\r\n"));
}

fn collectCpuModel(io: Io, allocator: std.mem.Allocator) ![]const u8 {
    if (builtin.target.os.tag == .macos) {
        const model = try runCommand(io, allocator, &.{ "sysctl", "-n", "machdep.cpu.brand_string" });
        if (model.len == 0) return error.MissingCpuModel;
        return model;
    }
    if (builtin.target.os.tag == .linux) {
        const contents = Io.Dir.cwd().readFileAlloc(
            io,
            "/proc/cpuinfo",
            allocator,
            .limited(1024 * 1024),
        ) catch return allocator.dupe(u8, builtin.target.cpu.model.name);
        var lines = std.mem.splitScalar(u8, contents, '\n');
        while (lines.next()) |line| {
            const separator = std.mem.indexOfScalar(u8, line, ':') orelse continue;
            const key = std.mem.trim(u8, line[0..separator], " \t");
            if (!std.mem.eql(u8, key, "model name") and !std.mem.eql(u8, key, "Hardware")) continue;
            const model = std.mem.trim(u8, line[separator + 1 ..], " \t\r");
            if (model.len > 0) return allocator.dupe(u8, model);
        }
    }
    return allocator.dupe(u8, builtin.target.cpu.model.name);
}

fn collectRunIdentity(
    io: Io,
    allocator: std.mem.Allocator,
    environment_variables: *const std.process.Environ.Map,
) !RunIdentity {
    const git_commit = try runCommand(io, allocator, &.{ "git", "rev-parse", "HEAD" });
    if (git_commit.len < 12) return error.MissingGitCommit;
    const git_status = try runCommand(
        io,
        allocator,
        &.{ "git", "status", "--porcelain", "--untracked-files=normal" },
    );
    const git_dirty = git_status.len > 0;
    const revision = try std.fmt.allocPrint(
        allocator,
        "{s}{s}",
        .{ git_commit[0..12], if (git_dirty) "-dirty" else "" },
    );

    const runner_id = if (environment_variables.get("ZIWEI_BENCHMARK_RUNNER_ID")) |value|
        if (value.len == 0) return error.MissingRunnerId else try allocator.dupe(u8, value)
    else blk: {
        const hostname = try runCommand(io, allocator, &.{"hostname"});
        if (hostname.len == 0) return error.MissingRunnerId;
        break :blk hostname;
    };
    const cpu_model = try collectCpuModel(io, allocator);
    const os_version = try runCommand(io, allocator, &.{ "uname", "-sr" });
    if (os_version.len == 0) return error.MissingOsVersion;
    var target_buffer: [128]u8 = undefined;
    const target = try allocator.dupe(u8, currentTarget(&target_buffer));
    const environment: contract.Environment = .{
        .runner_id = runner_id,
        .cpu_model = cpu_model,
        .os_version = os_version,
        .target = target,
        .zig_version = builtin.zig_version_string,
        .zig_backend = @tagName(builtin.zig_backend),
        .optimize_mode = @tagName(builtin.mode),
        .cpu_count = std.Thread.getCpuCount() catch 0,
    };
    if (environment.cpu_count == 0) return error.MissingCpuCount;
    return .{
        .git_commit = git_commit,
        .git_dirty = git_dirty,
        .revision = revision,
        .environment = environment,
        .environment_fingerprint = contract.environmentFingerprint(environment),
    };
}

fn sameEnvironmentFingerprint(baseline: []const u8, current: []const u8) bool {
    return baseline.len > 0 and std.mem.eql(u8, baseline, current);
}

fn runKindName(config: Config) []const u8 {
    return if (config.quick) "smoke" else "full";
}

fn validateBaseline(
    baseline: BaselineDocument,
    config: Config,
    current_environment_fingerprint: []const u8,
) !void {
    if (baseline.schema_version != results_schema_version) return error.UnsupportedBaselineSchema;
    if (!std.mem.eql(u8, baseline.run.run_kind, "full")) return error.BaselineMustBeFullRun;
    if (baseline.run.warmups != config.warmup_count or
        baseline.run.configured_samples != config.sample_count or
        baseline.run.target_sample_ns != config.target_ns)
    {
        return error.IncompatibleBaselineSampling;
    }
    if (!sameEnvironmentFingerprint(
        baseline.run.environment_fingerprint,
        current_environment_fingerprint,
    )) {
        return error.IncompatibleBaselineEnvironment;
    }
    if (!std.mem.eql(u8, baseline.run.fixture_id, fixture_id)) return error.IncompatibleBaselineFixture;
    if (baseline.run.revision.len == 0 or std.mem.eql(u8, baseline.run.revision, "unknown")) {
        return error.MissingBaselineRevision;
    }
    if (config.revision.len == 0 or std.mem.eql(u8, config.revision, "unknown")) {
        return error.MissingCurrentRevision;
    }
    if (!std.mem.eql(u8, baseline.run.verdict, "no-baseline") and
        !std.mem.eql(u8, baseline.run.verdict, "pass")) return error.UnstableBaseline;
    if (baseline.benchmarks.len != benchmarks.len) return error.IncompatibleBaselineBenchmarks;

    var seen = [_]bool{false} ** benchmarks.len;
    for (baseline.benchmarks) |baseline_benchmark| {
        const benchmark_index = for (benchmarks, 0..) |benchmark, index| {
            if (std.mem.eql(u8, baseline_benchmark.name, benchmark.name)) break index;
        } else return error.IncompatibleBaselineBenchmarks;
        if (seen[benchmark_index]) return error.IncompatibleBaselineBenchmarks;
        seen[benchmark_index] = true;

        const benchmark = benchmarks[benchmark_index];
        const expected_sample_count = if (benchmark.expensive) @min(config.sample_count, 10) else config.sample_count;
        if (baseline_benchmark.operations_per_iteration != benchmark.operation_count or
            !std.mem.eql(u8, baseline_benchmark.unit, "ns/op") or
            baseline_benchmark.sample_count != expected_sample_count or
            baseline_benchmark.samples_ns_per_operation.len != expected_sample_count)
        {
            return error.IncompatibleBaselineBenchmarks;
        }
        for (baseline_benchmark.samples_ns_per_operation) |sample| {
            if (!std.math.isFinite(sample) or sample <= 0) return error.InvalidBaselineSamples;
        }
    }
}

fn reportVerdict(
    config: Config,
    results: []const BenchmarkResult,
    comparisons: ?[]const BenchmarkComparison,
) []const u8 {
    if (config.quick) return "smoke-only";
    var maximum_rsd: f64 = 0;
    var maximum_outlier_ratio: f64 = 0;
    var maximum_severe_outlier_ratio: f64 = 0;
    for (results) |result| {
        maximum_rsd = @max(maximum_rsd, result.statistics.relative_standard_deviation_percent);
        const sample_count: f64 = @floatFromInt(result.sample_count);
        maximum_outlier_ratio = @max(
            maximum_outlier_ratio,
            @as(f64, @floatFromInt(result.statistics.outlier_count)) / sample_count,
        );
        maximum_severe_outlier_ratio = @max(
            maximum_severe_outlier_ratio,
            @as(f64, @floatFromInt(result.statistics.severe_outlier_count)) / sample_count,
        );
    }
    if (maximum_rsd > 10 or maximum_severe_outlier_ratio > 0.10) return "unstable";
    if (maximum_rsd > 5 or maximum_outlier_ratio > 0.10) {
        return if (comparisons == null) "no-baseline-noise-warning" else "noise-warning";
    }
    if (comparisons) |values| {
        for (values) |comparison| {
            if (comparison.verdict == .regression) return "regression";
        }
        return "pass";
    }
    return "no-baseline";
}

fn plainLanguageSummaryLead(verdict: []const u8) []const u8 {
    if (std.mem.eql(u8, verdict, "smoke-only")) {
        return "这次只是快速自检，确认基准程序和报告能正常生成。样本很少，不能据此判断性能变快或变慢。";
    }
    if (std.mem.eql(u8, verdict, "unstable")) {
        return "这次反复测同一项工作时，结果忽快忽慢，波动太大。下面的速度只能看大概，不能用来判断代码改动让程序变快还是变慢。";
    }
    if (std.mem.eql(u8, verdict, "noise-warning") or
        std.mem.eql(u8, verdict, "no-baseline-noise-warning"))
    {
        return "这次反复测同一项工作时，有些结果忽快忽慢。下面会直接指出是哪一项在波动；先重跑一次，再判断性能有没有变化。";
    }
    if (std.mem.eql(u8, verdict, "regression")) {
        return "和保存的旧结果相比，这次确实有项目变慢，需要进一步排查。";
    }
    if (std.mem.eql(u8, verdict, "pass")) {
        return "和保存的旧结果相比，这次没有发现哪一项明确变慢。";
    }
    return "这份报告能说明程序现在大约有多快，但没有同一环境下的旧结果，所以还不能判断是变快还是变慢。";
}

fn benchmarkPlainName(name: []const u8) []const u8 {
    if (std.mem.eql(u8, name, "natal/create_from_input/single")) {
        return "预处理输入的单张构建";
    }
    if (std.mem.eql(u8, name, "natal/create_from_birth/single")) {
        return "出生资料的单张构建";
    }
    if (std.mem.eql(u8, name, "natal/create_from_input/sexagenary_cycle")) {
        return "预处理输入的 60 个干支年";
    }
    if (std.mem.eql(u8, name, "natal/create_from_birth/sexagenary_cycle")) {
        return "出生资料的 60 个干支年";
    }
    if (std.mem.eql(u8, name, "natal/create_from_input/exhaustive_valid_space")) {
        return "全部合法输入组合";
    }
    return name;
}

fn writeReports(
    io: Io,
    allocator: std.mem.Allocator,
    report_directory: []const u8,
    started_at_ns: i96,
    config: Config,
    identity: RunIdentity,
    results: []const BenchmarkResult,
    baseline: ?Baseline,
    comparisons: ?[]const BenchmarkComparison,
) !void {
    try writeJson(io, report_directory, started_at_ns, config, identity, results, baseline, comparisons);
    try writeSummaryCsv(io, report_directory, results);
    try writeSamplesCsv(io, report_directory, results);
    try writeLatencyChart(io, report_directory, results);
    try writeDistributionChart(io, report_directory, results);
    try writeVariabilityChart(io, report_directory, results);
    if (comparisons) |values| try writeChangeChart(io, report_directory, results, values);
    try writeMarkdown(io, report_directory, started_at_ns, config, identity, results, baseline, comparisons);
    try writeManifest(io, allocator, report_directory, comparisons != null);
}

fn writeManifest(
    io: Io,
    allocator: std.mem.Allocator,
    report_directory: []const u8,
    has_change_chart: bool,
) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, contract.manifest_artifact, &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    try writer.print(
        "{{\n  \"schema_version\": {d},\n  \"results_schema_version\": {d},\n  \"artifacts\": [\n",
        .{ contract.manifest_schema_version, contract.results_schema_version },
    );
    const artifact_count = contract.required_artifacts.len + @as(usize, @intFromBool(has_change_chart));
    var artifact_index: usize = 0;
    for (contract.required_artifacts) |artifact| {
        try writeManifestArtifact(io, allocator, writer, report_directory, artifact);
        artifact_index += 1;
        try writer.writeAll(if (artifact_index == artifact_count) "\n" else ",\n");
    }
    if (has_change_chart) {
        try writeManifestArtifact(
            io,
            allocator,
            writer,
            report_directory,
            contract.optional_change_artifact,
        );
        try writer.writeByte('\n');
    }
    try writer.writeAll("  ]\n}\n");
    try writer.flush();
}

fn writeManifestArtifact(
    io: Io,
    allocator: std.mem.Allocator,
    writer: *Io.Writer,
    report_directory: []const u8,
    artifact: []const u8,
) !void {
    const artifact_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ report_directory, artifact });
    const contents = try Io.Dir.cwd().readFileAlloc(io, artifact_path, allocator, .limited(64 * 1024 * 1024));
    const digest = contract.sha256Hex(contents);
    try writer.writeAll("    {\"name\": ");
    try writeJsonString(writer, artifact);
    try writer.writeAll(", \"sha256\": ");
    try writeJsonString(writer, &digest);
    try writer.writeByte('}');
}

fn createReportFile(io: Io, report_directory: []const u8, file_name: []const u8, path_buffer: []u8) !Io.File {
    const path = try std.fmt.bufPrint(path_buffer, "{s}/{s}", .{ report_directory, file_name });
    return Io.Dir.cwd().createFile(io, path, .{ .truncate = true });
}

fn writeJson(
    io: Io,
    report_directory: []const u8,
    started_at_ns: i96,
    config: Config,
    identity: RunIdentity,
    results: []const BenchmarkResult,
    baseline: ?Baseline,
    comparisons: ?[]const BenchmarkComparison,
) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "results.json", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    try writer.print("{{\n  \"schema_version\": {d},\n  \"run\": {{\n", .{results_schema_version});
    try writer.print("    \"started_at_unix_ns\": {d},\n", .{started_at_ns});
    try writer.writeAll("    \"revision\": ");
    try writeJsonString(writer, identity.revision);
    try writer.writeAll(",\n    \"git_commit\": ");
    try writeJsonString(writer, identity.git_commit);
    try writer.print(",\n    \"git_dirty\": {s},\n    \"run_kind\": ", .{if (identity.git_dirty) "true" else "false"});
    try writeJsonString(writer, runKindName(config));
    try writer.writeAll(",\n    \"zig_version\": ");
    try writeJsonString(writer, identity.environment.zig_version);
    try writer.writeAll(",\n    \"zig_backend\": ");
    try writeJsonString(writer, identity.environment.zig_backend);
    try writer.writeAll(",\n    \"optimize_mode\": ");
    try writeJsonString(writer, identity.environment.optimize_mode);
    try writer.writeAll(",\n    \"target\": ");
    try writeJsonString(writer, identity.environment.target);
    try writer.writeAll(",\n    \"fixture_id\": ");
    try writeJsonString(writer, fixture_id);
    try writer.print(",\n    \"cpu_count\": {d},\n    \"cpu_model\": ", .{identity.environment.cpu_count});
    try writeJsonString(writer, identity.environment.cpu_model);
    try writer.writeAll(",\n    \"os_version\": ");
    try writeJsonString(writer, identity.environment.os_version);
    try writer.writeAll(",\n    \"runner_id\": ");
    try writeJsonString(writer, identity.environment.runner_id);
    try writer.writeAll(",\n    \"environment_fingerprint\": ");
    try writeJsonString(writer, &identity.environment_fingerprint);
    try writer.print(
        ",\n    \"natal_size_bytes\": {d},\n    \"warmups\": {d},\n" ++
            "    \"configured_samples\": {d},\n    \"target_sample_ns\": {d},\n    \"interleave_seed\": {d},\n" ++
            "    \"verdict\": \"{s}\"\n  }},\n  \"baseline\": ",
        .{
            @sizeOf(ziwei.Natal),
            config.warmup_count,
            config.sample_count,
            config.target_ns,
            config.seed,
            reportVerdict(config, results, comparisons),
        },
    );
    if (baseline) |value| {
        try writer.writeAll("{\n    \"name\": ");
        try writeJsonString(writer, value.name);
        try writer.writeAll(",\n    \"path\": ");
        try writeJsonString(writer, value.path);
        try writer.print(",\n    \"run_id\": {d},\n    \"revision\": ", .{value.document.run.started_at_unix_ns});
        try writeJsonString(writer, value.document.run.revision);
        try writer.writeAll("\n  }");
    } else {
        try writer.writeAll("null");
    }
    try writer.writeAll(",\n  \"benchmarks\": [\n");
    for (results, 0..) |result, result_index| {
        const stats = result.statistics;
        try writer.writeAll("    {\n      \"name\": ");
        try writeJsonString(writer, result.benchmark.name);
        try writer.print(
            ",\n      \"operations_per_iteration\": {d},\n      \"iterations_per_sample\": {d},\n" ++
                "      \"sample_count\": {d},\n      \"unit\": \"ns/op\",\n      \"statistics\": {{\n" ++
                "        \"minimum\": {d:.6},\n        \"q1\": {d:.6},\n        \"median\": {d:.6},\n" ++
                "        \"q3\": {d:.6},\n        \"p90\": {d:.6},\n        \"p95\": {d:.6},\n" ++
                "        \"p99\": {d:.6},\n        \"maximum\": {d:.6},\n        \"mean\": {d:.6},\n" ++
                "        \"standard_deviation\": {d:.6},\n        \"median_absolute_deviation\": {d:.6},\n" ++
                "        \"relative_standard_deviation_percent\": {d:.6},\n        \"confidence_95_lower\": {d:.6},\n" ++
                "        \"confidence_95_upper\": {d:.6},\n        \"outlier_count\": {d},\n" ++
                "        \"severe_outlier_count\": {d},\n        \"operations_per_second\": {d:.3}\n" ++
                "      }},\n      \"samples_elapsed_ns\": [",
            .{
                result.benchmark.operation_count,
                result.iterations_per_sample,
                result.sample_count,
                stats.minimum,
                stats.q1,
                stats.median,
                stats.q3,
                stats.p90,
                stats.p95,
                stats.p99,
                stats.maximum,
                stats.mean,
                stats.standard_deviation,
                stats.median_absolute_deviation,
                stats.relative_standard_deviation_percent,
                stats.confidence_95_lower,
                stats.confidence_95_upper,
                stats.outlier_count,
                stats.severe_outlier_count,
                stats.operations_per_second,
            },
        );
        for (result.samples_elapsed_ns[0..result.sample_count], 0..) |sample, sample_index| {
            if (sample_index != 0) try writer.writeAll(", ");
            try writer.print("{d}", .{sample});
        }
        try writer.writeAll("],\n      \"samples_ns_per_operation\": [");
        for (result.samples_ns_per_operation[0..result.sample_count], 0..) |sample, sample_index| {
            if (sample_index != 0) try writer.writeAll(", ");
            try writer.print("{d:.6}", .{sample});
        }
        try writer.writeAll("],\n      \"comparison\": ");
        if (comparisons) |values| {
            const comparison = values[result_index];
            try writer.print(
                "{{\n        \"baseline_median_ns_per_operation\": {d:.6},\n" ++
                    "        \"current_median_ns_per_operation\": {d:.6},\n" ++
                    "        \"median_change_percent\": {d:.6},\n" ++
                    "        \"baseline_p95_ns_per_operation\": {d:.6},\n" ++
                    "        \"current_p95_ns_per_operation\": {d:.6},\n" ++
                    "        \"p95_change_percent\": {d:.6},\n" ++
                    "        \"median_change_confidence_95_lower_percent\": {d:.6},\n" ++
                    "        \"median_change_confidence_95_upper_percent\": {d:.6},\n" ++
                    "        \"threshold_percent\": {d:.2},\n" ++
                    "        \"verdict\": \"{s}\"\n      }}",
                .{
                    comparison.values.baseline_median,
                    comparison.values.current_median,
                    comparison.values.median_change_percent,
                    comparison.values.baseline_p95,
                    comparison.values.current_p95,
                    comparison.values.p95_change_percent,
                    comparison.values.confidence_95_lower_percent,
                    comparison.values.confidence_95_upper_percent,
                    comparison_threshold_percent,
                    @tagName(comparison.verdict),
                },
            );
        } else {
            try writer.writeAll("null");
        }
        try writer.writeAll("\n    }");
        if (result_index + 1 != results.len) try writer.writeAll(",");
        try writer.writeAll("\n");
    }
    try writer.writeAll("  ]\n}\n");
    try writer.flush();
}

fn writeJsonString(writer: *Io.Writer, value: []const u8) !void {
    try writer.writeByte('"');
    for (value) |byte| {
        switch (byte) {
            '"' => try writer.writeAll("\\\""),
            '\\' => try writer.writeAll("\\\\"),
            '\n' => try writer.writeAll("\\n"),
            '\r' => try writer.writeAll("\\r"),
            '\t' => try writer.writeAll("\\t"),
            0...8, 11, 12, 14...0x1f => try writer.print("\\u00{x:0>2}", .{byte}),
            else => try writer.writeByte(byte),
        }
    }
    try writer.writeByte('"');
}

fn writeSummaryCsv(io: Io, report_directory: []const u8, results: []const BenchmarkResult) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "summary.csv", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;
    try writer.writeAll(
        "benchmark,operations_per_iteration,iterations_per_sample,samples,minimum_ns_per_op," ++
            "q1_ns_per_op,median_ns_per_op,q3_ns_per_op,p90_ns_per_op,p95_ns_per_op,p99_ns_per_op," ++
            "maximum_ns_per_op,mean_ns_per_op,stddev_ns_per_op,mad_ns_per_op,rsd_percent," ++
            "confidence_95_lower_ns_per_op,confidence_95_upper_ns_per_op,outliers,severe_outliers,operations_per_second\n",
    );
    for (results) |result| {
        const stats = result.statistics;
        try writer.print(
            "{s},{d},{d},{d},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d:.6},{d},{d},{d:.3}\n",
            .{
                result.benchmark.name,
                result.benchmark.operation_count,
                result.iterations_per_sample,
                result.sample_count,
                stats.minimum,
                stats.q1,
                stats.median,
                stats.q3,
                stats.p90,
                stats.p95,
                stats.p99,
                stats.maximum,
                stats.mean,
                stats.standard_deviation,
                stats.median_absolute_deviation,
                stats.relative_standard_deviation_percent,
                stats.confidence_95_lower,
                stats.confidence_95_upper,
                stats.outlier_count,
                stats.severe_outlier_count,
                stats.operations_per_second,
            },
        );
    }
    try writer.flush();
}

fn writeSamplesCsv(io: Io, report_directory: []const u8, results: []const BenchmarkResult) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "samples.csv", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;
    try writer.writeAll("benchmark,sample_index,elapsed_ns,ns_per_operation\n");
    for (results) |result| {
        for (result.samples_ns_per_operation[0..result.sample_count], 0..) |sample, sample_index| {
            try writer.print(
                "{s},{d},{d},{d:.6}\n",
                .{ result.benchmark.name, sample_index, result.samples_elapsed_ns[sample_index], sample },
            );
        }
    }
    try writer.flush();
}

fn writeLatencyChart(io: Io, report_directory: []const u8, results: []const BenchmarkResult) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "latency.svg", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    var maximum: f64 = 0;
    for (results) |result| maximum = @max(maximum, @max(result.statistics.p95, result.statistics.confidence_95_upper));
    const height = 100 + results.len * 86;
    try writer.print(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{d}\" height=\"{d}\" viewBox=\"0 0 {d} {d}\">\n" ++
            "<style>text{{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;fill:#172033}}.title{{font-size:20px;font-weight:700}}.label{{font-size:{d:.0}px}}.value{{font-size:12px}}.grid{{stroke:#d8dee9;stroke-width:1}}</style>\n" ++
            "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/><text x=\"24\" y=\"30\" class=\"title\">Natal construction latency (ns/op)</text>\n" ++
            "<rect x=\"650\" y=\"17\" width=\"14\" height=\"10\" rx=\"2\" fill=\"#2463eb\"/><text x=\"670\" y=\"27\" class=\"value\">median</text>" ++
            "<rect x=\"760\" y=\"17\" width=\"14\" height=\"10\" rx=\"2\" fill=\"#f59e0b\"/><text x=\"780\" y=\"27\" class=\"value\">p95</text>" ++
            "<line x1=\"835\" y1=\"22\" x2=\"865\" y2=\"22\" stroke=\"#7c3aed\" stroke-width=\"2\"/><text x=\"873\" y=\"27\" class=\"value\">mean 95% CI</text>\n",
        .{ chart_width_px, height, chart_width_px, height, chart_label_font_size_px },
    );
    for (results, 0..) |result, index| {
        const y = 64 + index * 86;
        const median_width = if (maximum == 0) 0 else result.statistics.median / maximum * chart_plot_width_px;
        const p95_width = if (maximum == 0) 0 else result.statistics.p95 / maximum * chart_plot_width_px;
        const confidence_lower_x = chart_plot_x_px + result.statistics.confidence_95_lower / maximum * chart_plot_width_px;
        const confidence_upper_x = chart_plot_x_px + result.statistics.confidence_95_upper / maximum * chart_plot_width_px;
        const mean_x = chart_plot_x_px + result.statistics.mean / maximum * chart_plot_width_px;
        try writer.print(
            "<text x=\"{d:.2}\" y=\"{d}\" class=\"label\">{s}</text>\n" ++
                "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" class=\"grid\"/>\n" ++
                "<rect x=\"{d:.2}\" y=\"{d}\" width=\"{d:.2}\" height=\"18\" rx=\"3\" fill=\"#2463eb\"/>" ++
                "<text x=\"{d:.2}\" y=\"{d}\" class=\"value\">{d:.3}</text>\n" ++
                "<rect x=\"{d:.2}\" y=\"{d}\" width=\"{d:.2}\" height=\"18\" rx=\"3\" fill=\"#f59e0b\"/>" ++
                "<text x=\"{d:.2}\" y=\"{d}\" class=\"value\">{d:.3}</text>\n",
            .{
                chart_label_x_px,
                y,
                result.benchmark.name,
                chart_plot_x_px,
                y + 10,
                chart_plot_x_px + chart_plot_width_px,
                y + 10,
                chart_plot_x_px,
                y + 17,
                median_width,
                chart_plot_x_px + 6 + median_width,
                y + 31,
                result.statistics.median,
                chart_plot_x_px,
                y + 41,
                p95_width,
                chart_plot_x_px + 6 + p95_width,
                y + 55,
                result.statistics.p95,
            },
        );
        try writer.print(
            "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#7c3aed\" stroke-width=\"2\"/>" ++
                "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#7c3aed\"/>" ++
                "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#7c3aed\"/>" ++
                "<circle cx=\"{d:.2}\" cy=\"{d}\" r=\"4\" fill=\"#7c3aed\"/>\n",
            .{
                confidence_lower_x,
                y + 70,
                confidence_upper_x,
                y + 70,
                confidence_lower_x,
                y + 65,
                confidence_lower_x,
                y + 75,
                confidence_upper_x,
                y + 65,
                confidence_upper_x,
                y + 75,
                mean_x,
                y + 70,
            },
        );
    }
    try writer.writeAll("</svg>\n");
    try writer.flush();
}

fn writeDistributionChart(io: Io, report_directory: []const u8, results: []const BenchmarkResult) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "distribution.svg", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    var maximum: f64 = 0;
    for (results) |result| maximum = @max(maximum, result.statistics.maximum);
    const height = 86 + results.len * 62;
    try writer.print(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{d}\" height=\"{d}\" viewBox=\"0 0 {d} {d}\">\n" ++
            "<style>text{{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;fill:#172033}}.title{{font-size:20px;font-weight:700}}.label{{font-size:{d:.0}px}}.value{{font-size:12px}}</style>\n" ++
            "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/><text x=\"24\" y=\"30\" class=\"title\">Sample distribution (ns/op)</text>\n" ++
            "<rect x=\"650\" y=\"17\" width=\"18\" height=\"10\" fill=\"#bfdbfe\" stroke=\"#2463eb\"/><text x=\"674\" y=\"27\" class=\"value\">Q1-Q3</text>" ++
            "<circle cx=\"780\" cy=\"22\" r=\"4\" fill=\"#f59e0b\"/><text x=\"790\" y=\"27\" class=\"value\">P95</text>" ++
            "<circle cx=\"855\" cy=\"22\" r=\"3\" fill=\"#dc2626\"/><text x=\"864\" y=\"27\" class=\"value\">outlier</text>\n",
        .{ chart_width_px, height, chart_width_px, height, chart_label_font_size_px },
    );
    for (results, 0..) |result, index| {
        const stats = result.statistics;
        const y: usize = 60 + index * 62;
        const scale = if (maximum == 0) 0 else chart_plot_width_px / maximum;
        const minimum_x = chart_plot_x_px + stats.minimum * scale;
        const q1_x = chart_plot_x_px + stats.q1 * scale;
        const median_x = chart_plot_x_px + stats.median * scale;
        const q3_x = chart_plot_x_px + stats.q3 * scale;
        const p95_x = chart_plot_x_px + stats.p95 * scale;
        const maximum_x = chart_plot_x_px + stats.maximum * scale;
        try writer.print(
            "<text x=\"{d:.2}\" y=\"{d}\" class=\"label\">{s}</text>\n",
            .{ chart_label_x_px, y + 5, result.benchmark.name },
        );
        try writer.print(
            "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#64748b\" stroke-width=\"2\"/>\n",
            .{ minimum_x, y, maximum_x, y },
        );
        try writer.print(
            "<rect x=\"{d:.2}\" y=\"{d}\" width=\"{d:.2}\" height=\"24\" fill=\"#bfdbfe\" stroke=\"#2463eb\"/>\n",
            .{ q1_x, y - 12, q3_x - q1_x },
        );
        try writer.print(
            "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#172033\" stroke-width=\"3\"/>" ++
                "<circle cx=\"{d:.2}\" cy=\"{d}\" r=\"5\" fill=\"#f59e0b\"/>\n",
            .{ median_x, y - 12, median_x, y + 12, p95_x, y },
        );

        const lower_fence = stats.q1 - 1.5 * (stats.q3 - stats.q1);
        const upper_fence = stats.q3 + 1.5 * (stats.q3 - stats.q1);
        for (result.samples_ns_per_operation[0..result.sample_count]) |sample| {
            if (sample < lower_fence or sample > upper_fence) {
                try writer.print(
                    "<circle cx=\"{d:.2}\" cy=\"{d}\" r=\"3\" fill=\"#dc2626\" fill-opacity=\"0.7\"/>\n",
                    .{ chart_plot_x_px + sample * scale, y },
                );
            }
        }
        try writer.print(
            "<text x=\"{d:.2}\" y=\"{d}\" class=\"value\">n={d}</text>\n",
            .{ chart_plot_x_px + chart_plot_width_px + 20, y + 5, result.sample_count },
        );
    }
    try writer.writeAll("</svg>\n");
    try writer.flush();
}

fn writeVariabilityChart(io: Io, report_directory: []const u8, results: []const BenchmarkResult) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "variability.svg", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    var maximum: f64 = 1;
    for (results) |result| maximum = @max(maximum, result.statistics.relative_standard_deviation_percent);
    const height = 76 + results.len * 58;
    try writer.print(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{d}\" height=\"{d}\" viewBox=\"0 0 {d} {d}\">\n" ++
            "<style>text{{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;fill:#172033}}.title{{font-size:20px;font-weight:700}}.label{{font-size:{d:.0}px}}.value{{font-size:12px}}</style>\n" ++
            "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/><text x=\"24\" y=\"30\" class=\"title\">Relative variability (RSD %)</text>\n",
        .{ chart_width_px, height, chart_width_px, height, chart_label_font_size_px },
    );
    for (results, 0..) |result, index| {
        const y = 58 + index * 58;
        const width = result.statistics.relative_standard_deviation_percent / maximum * chart_plot_width_px;
        try writer.print(
            "<text x=\"{d:.2}\" y=\"{d}\" class=\"label\">{s}</text>" ++
                "<rect x=\"{d:.2}\" y=\"{d}\" width=\"{d:.2}\" height=\"20\" rx=\"3\" fill=\"#10b981\"/>" ++
                "<text x=\"{d:.2}\" y=\"{d}\" class=\"value\">{d:.2}%</text>\n",
            .{
                chart_label_x_px,
                y + 15,
                result.benchmark.name,
                chart_plot_x_px,
                y,
                width,
                chart_plot_x_px + 6 + width,
                y + 15,
                result.statistics.relative_standard_deviation_percent,
            },
        );
    }
    try writer.writeAll("</svg>\n");
    try writer.flush();
}

fn changeChartX(change_percent: f64, domain_absolute_percent: f64) f64 {
    return chart_plot_x_px +
        (change_percent + domain_absolute_percent) / (2 * domain_absolute_percent) * chart_plot_width_px;
}

fn writeChangeChart(
    io: Io,
    report_directory: []const u8,
    results: []const BenchmarkResult,
    comparisons: []const BenchmarkComparison,
) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "change.svg", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    var maximum_absolute_change = comparison_threshold_percent;
    for (comparisons) |comparison| {
        maximum_absolute_change = @max(maximum_absolute_change, @abs(comparison.values.median_change_percent));
        maximum_absolute_change = @max(maximum_absolute_change, @abs(comparison.values.confidence_95_lower_percent));
        maximum_absolute_change = @max(maximum_absolute_change, @abs(comparison.values.confidence_95_upper_percent));
    }
    const domain_absolute_percent = maximum_absolute_change * 1.2;
    const zero_x = changeChartX(0, domain_absolute_percent);
    const threshold_upper_x = changeChartX(comparison_threshold_percent, domain_absolute_percent);
    const height = 106 + results.len * 62;
    const plot_bottom = height - 18;

    try writer.print(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{d}\" height=\"{d}\" viewBox=\"0 0 {d} {d}\">\n" ++
            "<style>text{{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;fill:#172033}}.title{{font-size:20px;font-weight:700}}.label{{font-size:{d:.0}px}}.value{{font-size:12px}}.axis{{font-size:11px;fill:#64748b}}</style>\n" ++
            "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>" ++
            "<text x=\"24\" y=\"30\" class=\"title\">Median change vs baseline (%)</text>" ++
            "<text x=\"720\" y=\"28\" class=\"value\">point estimate + 95% bootstrap CI; threshold +{d:.0}%</text>\n" ++
            "<rect x=\"{d:.2}\" y=\"66\" width=\"{d:.2}\" height=\"{d}\" fill=\"#dcfce7\" fill-opacity=\"0.75\"/>" ++
            "<line x1=\"{d:.2}\" y1=\"58\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#475569\" stroke-width=\"2\"/>" ++
            "<line x1=\"{d:.2}\" y1=\"62\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"#dc2626\" stroke-dasharray=\"4 4\"/>" ++
            "<text x=\"{d:.2}\" y=\"52\" text-anchor=\"middle\" class=\"axis\">0%</text>" ++
            "<text x=\"{d:.2}\" y=\"56\" text-anchor=\"middle\" class=\"axis\">+{d:.0}%</text>\n",
        .{
            chart_width_px,
            height,
            chart_width_px,
            height,
            chart_label_font_size_px,
            comparison_threshold_percent,
            chart_plot_x_px,
            threshold_upper_x - chart_plot_x_px,
            plot_bottom - 66,
            zero_x,
            zero_x,
            plot_bottom,
            threshold_upper_x,
            threshold_upper_x,
            plot_bottom,
            zero_x,
            threshold_upper_x,
            comparison_threshold_percent,
        },
    );
    for (results, comparisons, 0..) |result, comparison, index| {
        const y = 88 + index * 62;
        const lower_x = changeChartX(comparison.values.confidence_95_lower_percent, domain_absolute_percent);
        const upper_x = changeChartX(comparison.values.confidence_95_upper_percent, domain_absolute_percent);
        const point_x = changeChartX(comparison.values.median_change_percent, domain_absolute_percent);
        const color = if (comparison.verdict == .regression) "#dc2626" else "#2563eb";
        try writer.print(
            "<text x=\"{d:.2}\" y=\"{d}\" class=\"label\">{s}</text>" ++
                "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"{s}\" stroke-width=\"3\"/>" ++
                "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"{s}\"/>" ++
                "<line x1=\"{d:.2}\" y1=\"{d}\" x2=\"{d:.2}\" y2=\"{d}\" stroke=\"{s}\"/>" ++
                "<circle cx=\"{d:.2}\" cy=\"{d}\" r=\"5\" fill=\"{s}\"/>" ++
                "<text x=\"1170\" y=\"{d}\" text-anchor=\"end\" class=\"value\">{s}{d:.2}%</text>\n",
            .{
                chart_label_x_px,
                y + 5,
                result.benchmark.name,
                lower_x,
                y,
                upper_x,
                y,
                color,
                lower_x,
                y - 6,
                lower_x,
                y + 6,
                color,
                upper_x,
                y - 6,
                upper_x,
                y + 6,
                color,
                point_x,
                y,
                color,
                y + 5,
                if (comparison.values.median_change_percent >= 0) "+" else "",
                comparison.values.median_change_percent,
            },
        );
    }
    try writer.writeAll("</svg>\n");
    try writer.flush();
}

fn writePlainLanguageSummary(
    writer: *Io.Writer,
    config: Config,
    results: []const BenchmarkResult,
    baseline: ?Baseline,
    comparisons: ?[]const BenchmarkComparison,
) !void {
    std.debug.assert(results.len > 0);
    const verdict = reportVerdict(config, results, comparisons);
    var fastest_index: usize = 0;
    var slowest_index: usize = 0;
    var noisiest_index: usize = 0;
    var most_outliers_index: usize = 0;
    var single_input_index: ?usize = null;
    var single_birth_index: ?usize = null;
    var exhaustive_index: ?usize = null;
    for (results, 0..) |result, index| {
        if (result.statistics.median < results[fastest_index].statistics.median) fastest_index = index;
        if (result.statistics.median > results[slowest_index].statistics.median) slowest_index = index;
        if (result.statistics.relative_standard_deviation_percent >
            results[noisiest_index].statistics.relative_standard_deviation_percent)
        {
            noisiest_index = index;
        }
        if (result.statistics.outlier_count > results[most_outliers_index].statistics.outlier_count) {
            most_outliers_index = index;
        }
        if (std.mem.eql(u8, result.benchmark.name, "natal/create_from_input/single")) {
            single_input_index = index;
        } else if (std.mem.eql(u8, result.benchmark.name, "natal/create_from_birth/single")) {
            single_birth_index = index;
        } else if (std.mem.eql(u8, result.benchmark.name, "natal/create_from_input/exhaustive_valid_space")) {
            exhaustive_index = index;
        }
    }

    const fastest = results[fastest_index];
    const slowest = results[slowest_index];
    const speed_gap_percent = if (fastest.statistics.median == 0)
        0
    else
        (slowest.statistics.median / fastest.statistics.median - 1) * 100;

    try writer.print(
        "## 大白话总结\n\n{s}\n\n" ++
            "- 这次只测本命盘构建，不包含只读查询，也不包含文件读写、网络请求和历法换算。\n" ++
            "- 在这台机器的纯计算循环里，构建一张本命盘通常约需 {d:.3}–{d:.3} 微秒；连续单线程计算时，相当于每秒约 {d:.2}–{d:.2} 百万张。\n" ++
            "- 本轮最快的是“{s}”，约 {d:.3} 微秒/张；最慢的是“{s}”，约 {d:.3} 微秒/张，两者相差约 {d:.1}%。\n",
        .{
            plainLanguageSummaryLead(verdict),
            fastest.statistics.median / 1000,
            slowest.statistics.median / 1000,
            slowest.statistics.operations_per_second / 1_000_000,
            fastest.statistics.operations_per_second / 1_000_000,
            benchmarkPlainName(fastest.benchmark.name),
            fastest.statistics.median / 1000,
            benchmarkPlainName(slowest.benchmark.name),
            slowest.statistics.median / 1000,
            speed_gap_percent,
        },
    );

    if (single_input_index) |input_index| {
        if (single_birth_index) |birth_index| {
            const input_median = results[input_index].statistics.median;
            const birth_median = results[birth_index].statistics.median;
            const smaller = @min(input_median, birth_median);
            const difference_percent = if (smaller == 0) 0 else @abs(birth_median - input_median) / smaller * 100;
            try writer.print(
                "- 两个单张入口中，预处理输入约需 {d:.3} 微秒，出生资料约需 {d:.3} 微秒；",
                .{ input_median / 1000, birth_median / 1000 },
            );
            if (difference_percent < 0.5) {
                try writer.print("两者几乎一样快，只差约 {d:.1}%。\n", .{difference_percent});
            } else if (birth_median > input_median) {
                try writer.print("从出生资料开始慢约 {d:.1}%。\n", .{difference_percent});
            } else {
                try writer.print("从出生资料开始快约 {d:.1}%。\n", .{difference_percent});
            }
        }
    }
    if (exhaustive_index) |index| {
        try writer.print(
            "- 把全部合法输入组合都跑一遍后，折算到每张约需 {d:.3} 微秒；这个数字已经包含在上面的最快—最慢范围里。\n",
            .{results[index].statistics.median / 1000},
        );
    }

    if (config.quick) {
        try writer.writeAll("- 这只是快速自检，测量次数太少；上面的速度只看数量级，不能拿来判断优化是否有效，也不能替换正式旧结果。\n\n");
        return;
    }

    if (std.mem.eql(u8, verdict, "unstable") or
        std.mem.eql(u8, verdict, "noise-warning") or
        std.mem.eql(u8, verdict, "no-baseline-noise-warning"))
    {
        const noisiest = results[noisiest_index];
        try writer.print(
            "- 波动最大的是“{s}”：重复测了 {d} 次，测得速度之间的波动约为 {d:.1}%。\n",
            .{
                benchmarkPlainName(noisiest.benchmark.name),
                noisiest.sample_count,
                noisiest.statistics.relative_standard_deviation_percent,
            },
        );
        const most_outliers = results[most_outliers_index];
        if (most_outliers.statistics.outlier_count > 0) {
            try writer.print(
                "- 明显跑偏的结果最多出现在“{s}”：{d} 次测量中有 {d} 次和其余结果差得比较远。\n",
                .{
                    benchmarkPlainName(most_outliers.benchmark.name),
                    most_outliers.sample_count,
                    most_outliers.statistics.outlier_count,
                },
            );
        }
        if (baseline) |baseline_value| {
            const values = comparisons.?;
            var largest_change_index: usize = 0;
            for (values[1..], 1..) |comparison, index| {
                if (@abs(comparison.values.median_change_percent) >
                    @abs(values[largest_change_index].values.median_change_percent))
                {
                    largest_change_index = index;
                }
            }
            const change = values[largest_change_index].values.median_change_percent;
            try writer.print(
                "- 和保存的“{s}”旧结果相比，表面变化最大的是“{s}”：现在{s} {d:.1}%；但本轮波动太大，不能确认这是代码造成的真实变化。\n\n",
                .{
                    baseline_value.name,
                    benchmarkPlainName(results[largest_change_index].benchmark.name),
                    if (change < 0) "快了约" else "慢了约",
                    @abs(change),
                },
            );
        } else {
            try writer.writeAll("- 这次没有同一环境下的旧结果可比较；建议保持机器、电源和后台负载不变，再正式跑一次。\n\n");
        }
        return;
    }

    if (baseline) |baseline_value| {
        const values = comparisons.?;
        if (std.mem.eql(u8, verdict, "pass")) {
            var largest_change_index: usize = 0;
            for (values[1..], 1..) |comparison, index| {
                if (@abs(comparison.values.median_change_percent) >
                    @abs(values[largest_change_index].values.median_change_percent))
                {
                    largest_change_index = index;
                }
            }
            const change = values[largest_change_index].values.median_change_percent;
            try writer.print(
                "- 和保存的“{s}”旧结果相比，变化最大的是“{s}”：现在{s} {d:.1}%。把每次测量本身的波动算进去后，没有哪一项能确认慢了超过 {d:.0}%。\n" ++
                    "- 报告只负责提醒，不会自动阻止构建或合并。\n\n",
                .{
                    baseline_value.name,
                    benchmarkPlainName(results[largest_change_index].benchmark.name),
                    if (change < 0) "快了约" else "慢了约",
                    @abs(change),
                    comparison_threshold_percent,
                },
            );
            return;
        }
        if (std.mem.eql(u8, verdict, "regression")) {
            var regression_count: usize = 0;
            var worst_index: usize = 0;
            var worst_change = -std.math.inf(f64);
            for (values, 0..) |comparison, index| {
                if (comparison.verdict != .regression) continue;
                regression_count += 1;
                if (comparison.values.median_change_percent > worst_change) {
                    worst_change = comparison.values.median_change_percent;
                    worst_index = index;
                }
            }
            const worst = values[worst_index].values;
            try writer.print(
                "- 共有 {d} 项可以确认变慢。最明显的是“{s}”：现在慢了约 {d:.1}%；把测量波动算进去，实际变慢幅度大约在 {d:.1}%–{d:.1}% 之间。\n" ++
                    "- 建议先检查这项最近的代码变化，再在同一环境复跑确认；报告只负责提醒，不会自动阻止构建或合并。\n\n",
                .{
                    regression_count,
                    benchmarkPlainName(results[worst_index].benchmark.name),
                    worst.median_change_percent,
                    worst.confidence_95_lower_percent,
                    worst.confidence_95_upper_percent,
                },
            );
            return;
        }
        unreachable;
    }
    try writer.writeAll("- 这次没有同一环境下的旧结果，所以只能说明当前速度，不能判断升降；可以保存本次 `results.json`，供以后在同一环境中比较。\n\n");
}

fn writeMarkdown(
    io: Io,
    report_directory: []const u8,
    started_at_ns: i96,
    config: Config,
    identity: RunIdentity,
    results: []const BenchmarkResult,
    baseline: ?Baseline,
    comparisons: ?[]const BenchmarkComparison,
) !void {
    var path_buffer: [std.fs.max_path_bytes]u8 = undefined;
    const file = try createReportFile(io, report_directory, "report.md", &path_buffer);
    defer file.close(io);
    var output_buffer: [4096]u8 = undefined;
    var file_writer = file.writer(io, &output_buffer);
    const writer = &file_writer.interface;

    try writer.print(
        "# Ziwei 本命盘构建基准报告\n\n" ++
            "- 结论：`{s}`\n- Run kind：`{s}`\n- Run ID：`{d}`\n- Revision：`{s}`\n" ++
            "- Git commit：`{s}`（工作树{s}）\n- Runner：`{s}`\n" ++
            "- Zig：`{s}`（`{s}` backend）\n- Target：`{s}`\n- Optimize：`{s}`\n" ++
            "- CPU：`{s}`（{d} 个逻辑核心）\n- OS：`{s}`\n- Environment：`{s}`\n" ++
            "- Fixture：`{s}`\n- `@sizeOf(Natal)`：`{d} bytes`\n- 配置：warm-up `{d}`，样本 `{d}`，目标样本时长 `{d} ms`，交错 seed `{d}`\n\n",
        .{
            reportVerdict(config, results, comparisons),
            runKindName(config),
            started_at_ns,
            identity.revision,
            identity.git_commit,
            if (identity.git_dirty) "有未提交改动" else "干净",
            identity.environment.runner_id,
            identity.environment.zig_version,
            identity.environment.zig_backend,
            identity.environment.target,
            identity.environment.optimize_mode,
            identity.environment.cpu_model,
            identity.environment.cpu_count,
            identity.environment.os_version,
            &identity.environment_fingerprint,
            fixture_id,
            @sizeOf(ziwei.Natal),
            config.warmup_count,
            config.sample_count,
            config.target_ns / std.time.ns_per_ms,
            config.seed,
        },
    );
    try writePlainLanguageSummary(writer, config, results, baseline, comparisons);
    try writer.writeAll(
        "## 结果\n\n" ++
            "| 基准 | P50 | P95 | P99 | Mean | MAD | 95% mean CI | RSD | Outliers (severe) | Throughput |\n" ++
            "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n",
    );
    for (results) |result| {
        const stats = result.statistics;
        try writer.print(
            "| `{s}` | {d:.3} | {d:.3} | {d:.3} | {d:.3} | {d:.3} | [{d:.3}, {d:.3}] | {d:.2}% | {d} ({d})/{d} | {d:.0} op/s |\n",
            .{
                result.benchmark.name,
                stats.median,
                stats.p95,
                stats.p99,
                stats.mean,
                stats.median_absolute_deviation,
                stats.confidence_95_lower,
                stats.confidence_95_upper,
                stats.relative_standard_deviation_percent,
                stats.outlier_count,
                stats.severe_outlier_count,
                result.sample_count,
                stats.operations_per_second,
            },
        );
    }
    if (baseline) |baseline_value| {
        const values = comparisons.?;
        try writer.writeAll("\n## 基线比较\n\n- Baseline：`");
        try writer.writeAll(baseline_value.name);
        try writer.writeAll("`\n- Baseline revision：`");
        try writer.writeAll(baseline_value.document.run.revision);
        try writer.print("`\n- Baseline run ID：`{d}`\n- Baseline file：`", .{baseline_value.document.run.started_at_unix_ns});
        try writer.writeAll(baseline_value.path);
        try writer.print(
            "`\n- 判定：median 变化的 95% bootstrap CI 下界超过 `+{d:.0}%` 时标记 `regression`；当前仅报告，不作为退出码门禁。\n\n" ++
                "| 基准 | Baseline P50 | Current P50 | P50 change (95% CI) | Baseline P95 | Current P95 | P95 change | 判定 |\n" ++
                "| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n",
            .{comparison_threshold_percent},
        );
        for (results, values) |result, comparison| {
            const change = comparison.values;
            try writer.print(
                "| `{s}` | {d:.3} | {d:.3} | {s}{d:.2}% ([{s}{d:.2}%, {s}{d:.2}%]) | {d:.3} | {d:.3} | {s}{d:.2}% | `{s}` |\n",
                .{
                    result.benchmark.name,
                    change.baseline_median,
                    change.current_median,
                    if (change.median_change_percent >= 0) "+" else "",
                    change.median_change_percent,
                    if (change.confidence_95_lower_percent >= 0) "+" else "",
                    change.confidence_95_lower_percent,
                    if (change.confidence_95_upper_percent >= 0) "+" else "",
                    change.confidence_95_upper_percent,
                    change.baseline_p95,
                    change.current_p95,
                    if (change.p95_change_percent >= 0) "+" else "",
                    change.p95_change_percent,
                    @tagName(comparison.verdict),
                },
            );
        }
    }
    try writer.writeAll(
        "\n## 图表\n\n![本命盘构建延迟](latency.svg)\n\n![样本分布](distribution.svg)\n\n![相对波动](variability.svg)\n\n",
    );
    if (comparisons != null) try writer.writeAll("![相对基线变化](change.svg)\n\n");
    try writer.writeAll(
        "## 测量边界\n\n" ++
            "- 本轮只测量 `createFromInput` 与 `createFromBirth` 的本命盘构建，不包含只读查询。\n" ++
            "- `sexagenary_cycle` 每次迭代覆盖 60 个干支年，并轮换性别、月、日、时辰。\n" ++
            "- `exhaustive_valid_space` 每次迭代覆盖 518,400 个合法 `ZiweiInput` 组合；为控制总时长，最多采集 10 个样本。\n" ++
            "- 夹具准备、统计与报告写出均位于计时区外；构建结果通过 `std.mem.doNotOptimizeAway` 保留。\n" ++
            "- 不同 case 的 warm-up 与正式样本按固定 seed 随机交错，以减轻时间顺序偏差。\n" ++
            "- 单次结果的 95% mean CI 使用 `mean ± 1.96 × standard error`；相对 median 变化使用 1,000 次确定性 bootstrap；异常值使用 `1.5 × IQR` 判定。\n" ++
            "- `smoke` 只验证基准与报告管线，不产生可比较的性能结论。\n" ++
            "- baseline 比较要求环境指纹、schema、run kind、采样配置、fixture 与 case 合同一致；仍应在同一本地固定基准机上运行。\n\n" ++
            "原始样本与机器可读摘要见 [`results.json`](results.json)、[`summary.csv`](summary.csv) 和 [`samples.csv`](samples.csv)。\n",
    );
    try writer.flush();
}

test "percentile interpolates between ordered samples" {
    const ordered = [_]f64{ 10, 20, 30, 40 };
    try std.testing.expectApproxEqAbs(@as(f64, 25), percentile(&ordered, 0.5), 0.0001);
    try std.testing.expectApproxEqAbs(@as(f64, 37), percentile(&ordered, 0.9), 0.0001);
}

test "statistics retain latency distribution and outliers" {
    const samples = [_]f64{ 10, 10, 10, 10, 100 };
    const stats = calculateStatistics(&samples);
    try std.testing.expectEqual(@as(f64, 10), stats.median);
    try std.testing.expectEqual(@as(usize, 1), stats.outlier_count);
    try std.testing.expect(stats.p95 > stats.median);
    try std.testing.expect(stats.confidence_95_upper > stats.confidence_95_lower);
}

test "baseline comparison reports relative median change with confidence interval" {
    const baseline_samples = [_]f64{ 100, 100, 100, 100, 100 };
    const current_samples = [_]f64{ 110, 110, 110, 110, 110 };
    const comparison = compareSamples(&baseline_samples, &current_samples, 42);

    try std.testing.expectApproxEqAbs(@as(f64, 10), comparison.median_change_percent, 0.0001);
    try std.testing.expectApproxEqAbs(@as(f64, 100), comparison.baseline_median, 0.0001);
    try std.testing.expectApproxEqAbs(@as(f64, 110), comparison.current_median, 0.0001);
    try std.testing.expectApproxEqAbs(@as(f64, 10), comparison.p95_change_percent, 0.0001);
    try std.testing.expectApproxEqAbs(@as(f64, 10), comparison.confidence_95_lower_percent, 0.0001);
    try std.testing.expectApproxEqAbs(@as(f64, 10), comparison.confidence_95_upper_percent, 0.0001);
    try std.testing.expectEqual(ComparisonVerdict.regression, comparisonVerdict(comparison));
}

test "smoke run rejects a named performance baseline" {
    const args = [_][]const u8{ "ziwei-benchmark", "--quick", "--baseline", "main", "baseline.json" };
    try std.testing.expectError(error.BaselineRequiresFullRun, parseConfig(&args));
}

test "smoke report never emits a performance verdict" {
    try std.testing.expectEqualStrings("smoke-only", reportVerdict(.{ .quick = true }, &.{}, null));
}

test "smoke plain-language summary refuses a performance conclusion" {
    try std.testing.expectEqualStrings(
        "这次只是快速自检，确认基准程序和报告能正常生成。样本很少，不能据此判断性能变快或变慢。",
        plainLanguageSummaryLead("smoke-only"),
    );
}

test "plain-language report names workloads without internal benchmark ids" {
    try std.testing.expectEqualStrings(
        "预处理输入的单张构建",
        benchmarkPlainName("natal/create_from_input/single"),
    );
    try std.testing.expectEqualStrings(
        "全部合法输入组合",
        benchmarkPlainName("natal/create_from_input/exhaustive_valid_space"),
    );
}

test "baseline compatibility rejects unsupported result schema" {
    const baseline: BaselineDocument = .{ .schema_version = 1 };
    try std.testing.expectError(error.UnsupportedBaselineSchema, validateBaseline(baseline, .{}, "current"));
}

test "baseline compatibility rejects smoke results" {
    const baseline: BaselineDocument = .{
        .schema_version = results_schema_version,
        .run = .{ .run_kind = "smoke" },
    };
    try std.testing.expectError(error.BaselineMustBeFullRun, validateBaseline(baseline, .{}, "current"));
}

test "baseline compatibility rejects a different sampling contract" {
    const baseline: BaselineDocument = .{
        .schema_version = results_schema_version,
        .run = .{
            .run_kind = "full",
            .warmups = 60,
            .configured_samples = 99,
            .target_sample_ns = 50 * std.time.ns_per_ms,
        },
    };
    try std.testing.expectError(error.IncompatibleBaselineSampling, validateBaseline(baseline, .{}, "current"));
}

test "baseline compatibility rejects a different environment fingerprint" {
    const baseline: BaselineDocument = .{
        .schema_version = results_schema_version,
        .run = .{
            .run_kind = "full",
            .environment_fingerprint = "different",
            .warmups = 60,
            .configured_samples = 100,
            .target_sample_ns = 50 * std.time.ns_per_ms,
        },
    };
    try std.testing.expectError(error.IncompatibleBaselineEnvironment, validateBaseline(baseline, .{}, "current"));
}

test "baseline comparison requires an exact environment fingerprint" {
    try std.testing.expect(!sameEnvironmentFingerprint("runner-a", "runner-b"));
    try std.testing.expect(sameEnvironmentFingerprint("runner-a", "runner-a"));
}

test "baseline compatibility requires the complete benchmark set" {
    var target_buffer: [128]u8 = undefined;
    const baseline: BaselineDocument = .{
        .schema_version = results_schema_version,
        .run = .{
            .run_kind = "full",
            .revision = "baseline-revision",
            .zig_version = builtin.zig_version_string,
            .zig_backend = @tagName(builtin.zig_backend),
            .optimize_mode = @tagName(builtin.mode),
            .target = currentTarget(&target_buffer),
            .fixture_id = fixture_id,
            .cpu_count = std.Thread.getCpuCount() catch 0,
            .environment_fingerprint = "current",
            .warmups = 60,
            .configured_samples = 100,
            .target_sample_ns = 50 * std.time.ns_per_ms,
            .verdict = "no-baseline",
        },
    };
    try std.testing.expectError(
        error.IncompatibleBaselineBenchmarks,
        validateBaseline(baseline, .{ .revision = "current-revision" }, "current"),
    );
}

test "interleaved benchmark order is a deterministic permutation" {
    var first_state: u64 = 42;
    var second_state: u64 = 42;
    const first = shuffledBenchmarkOrder(&first_state);
    const second = shuffledBenchmarkOrder(&second_state);
    try std.testing.expectEqual(first, second);

    var seen = [_]bool{false} ** benchmarks.len;
    for (first) |index| {
        try std.testing.expect(index < benchmarks.len);
        try std.testing.expect(!seen[index]);
        seen[index] = true;
    }
}

test "all benchmark declarations can be analyzed" {
    std.testing.refAllDecls(@This());
}
