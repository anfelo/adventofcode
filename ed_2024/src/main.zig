const std = @import("std");

pub const day1 = @import("day1.zig");

pub fn main() !void {
    const result = try day1.day1("./src/input/day1_small.txt");
    std.debug.print("{d}", .{result.similarityScore});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
