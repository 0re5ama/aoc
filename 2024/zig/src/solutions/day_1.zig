const std = @import("std");

const Allocator = std.mem.Allocator;

pub fn silver(input: []const u8, alloc: Allocator) !usize {
    var arr1 = std.ArrayList(usize).init(alloc);
    var arr2 = std.ArrayList(usize).init(alloc);
    defer arr1.deinit();
    defer arr2.deinit();

    var lines = std.mem.split(u8, input, "\n");

    while (lines.next()) |line| {
        if (line.len == 0) break;
        var iter = std.mem.tokenizeAny(u8, line, " ");

        const a = try std.fmt.parseInt(usize, iter.next().?, 10);
        const b = try std.fmt.parseInt(usize, iter.next().?, 10);
        try arr1.append(a);
        try arr2.append(b);
    }

    std.sort.heap(usize, arr1.items, {}, comptime std.sort.asc(usize));
    std.sort.heap(usize, arr2.items, {}, comptime std.sort.asc(usize));

    var sum: usize = 0;
    for (arr1.items, arr2.items) |a, b| {
        sum += @max(a, b) - @min(a, b);
    }

    return sum;
}

pub fn gold(input: []const u8, alloc: Allocator) !usize {
    var arr1 = std.ArrayList(usize).init(alloc);
    var arr2 = std.ArrayList(usize).init(alloc);
    defer arr1.deinit();
    defer arr2.deinit();

    var lines = std.mem.split(u8, input, "\n");

    while (lines.next()) |line| {
        if (line.len == 0) break;
        var iter = std.mem.tokenizeAny(u8, line, " ");

        const a = try std.fmt.parseInt(usize, iter.next().?, 10);
        const b = try std.fmt.parseInt(usize, iter.next().?, 10);
        try arr1.append(a);
        try arr2.append(b);
    }

    var sum: usize = 0;

    for (arr1.items) |a| {
        const count = std.mem.count(usize, arr2.items, &[1]usize{a});
        sum += a * count;
    }

    return sum;
}
