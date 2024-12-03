const std = @import("std");

pub const day1 = @import("day1.zig");

test "aoc" {
    std.testing.refAllDecls(@This());
}
