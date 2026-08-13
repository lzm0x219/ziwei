//! MathArts 开源生态的标准驱动紫微斗数排盘引擎

// ==================== 核心类型 ====================
pub const Stem = @import("primitive.zig").Stem;
pub const Branch = @import("primitive.zig").Branch;

// ==================== 主要入口函数 ====================
pub const createZiwei = @import("ziwei.zig").createZiwei();
pub const createZiweiByOriginal = @import("ziwei.zig").createZiweiByOriginal();
