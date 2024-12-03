const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile("./src/input/day1.txt", .{});
    defer file.close();

    // Things are _a lot_ slower if we don't use a BufferedReader
    var buffered = std.io.bufferedReader(file.reader());
    var reader = buffered.reader();

    // lines will get read into this
    var arr = std.ArrayList(u8).init(allocator);
    defer arr.deinit();

    var list1 = std.ArrayList(i32).init(allocator);
    defer list1.deinit();
    var list2 = std.ArrayList(i32).init(allocator);
    defer list2.deinit();

    var totalDistance: u64 = 0;

    while (true) {
        reader.streamUntilDelimiter(arr.writer(), '\n', null) catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };

        var num1: i32 = 0;
        var num2: i32 = 0;

        var i: usize = 0;
        var start: usize = 0;
        while (i < arr.items.len) {
            if (arr.items[i] == ' ' and arr.items[start] != ' ') {
                num1 = try std.fmt.parseInt(i32, arr.items[start..i], 10);
                start = i;
            } else if (arr.items[i] != ' ' and arr.items[start] == ' ') {
                start = i;
            }

            i += 1;
        }
        num2 = try std.fmt.parseInt(i32, arr.items[start..], 10);

        try list1.insert(list1.items.len, num1);
        try list2.insert(list2.items.len, num2);

        arr.clearRetainingCapacity();
    }

    std.mem.sort(i32, list1.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, list2.items, {}, comptime std.sort.asc(i32));

    for (list1.items, 0..) |num, i| {
        totalDistance += @abs(num - list2.items[i]);
    }
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
