const std = @import("std");
const expect = std.testing.expect;

const DataInfo = struct { totalDistance: u32, similarityScore: i32 };

pub fn day1(path: []const u8) !DataInfo {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile(path, .{});
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

    var list2_map = std.AutoHashMap(i32, u8).init(allocator);
    defer list2_map.deinit();

    var dataInfo = DataInfo{ .totalDistance = 0, .similarityScore = 0 };

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

        const value = list2_map.get(num2);
        if (value) |v| {
            try list2_map.put(num2, v + 1);
        } else {
            try list2_map.put(num2, 1);
        }

        arr.clearRetainingCapacity();
    }

    std.mem.sort(i32, list1.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, list2.items, {}, comptime std.sort.asc(i32));

    for (list1.items, 0..) |num, i| {
        dataInfo.totalDistance += @abs(num - list2.items[i]);

        if (list2_map.get(num)) |v| {
            dataInfo.similarityScore += num * v;
        }
    }

    return dataInfo;
}

test "day1" {
    var result = try day1("./src/input/day1.txt");
    try expect(result.totalDistance == 2000468);
    try expect(result.similarityScore == 18567089);

    result = try day1("./src/input/day1_small.txt");
    try expect(result.totalDistance == 11);
    try expect(result.similarityScore == 31);
}
