const std = @import("std");
const root = @import("root.zig");

const day_1 = @import("day_1");
const day_2 = @import("day_2");
const day_3 = @import("day_3");
const day_4 = @import("day_4");
const day_5 = @import("day_5");
const day_6 = @import("day_6");
const day_7 = @import("day_7");
const day_8 = @import("day_8");
const day_9 = @import("day_9");
const day_10 = @import("day_10");
const day_11 = @import("day_11");
const day_12 = @import("day_12");
const day_13 = @import("day_13");
const day_14 = @import("day_14");
const day_15 = @import("day_15");
const day_16 = @import("day_16");
const day_17 = @import("day_17");
const day_18 = @import("day_18");
const day_19 = @import("day_19");
const day_20 = @import("day_20");
const day_21 = @import("day_21");
const day_22 = @import("day_22");
const day_23 = @import("day_23");
const day_24 = @import("day_24");
const day_25 = @import("day_25");

const days = [_]type{
    day_1,  day_2,  day_3,  day_4,  day_5,  day_6,  day_7,  day_8,  day_9,  day_10,
    day_11, day_12, day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20,
    day_21, day_22, day_23, day_24, day_25,
};

const Allocator = std.mem.Allocator;
const log = std.log.scoped(.AOC);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    var args = std.process.args();

    _ = args.next().?;
    const day_str = args.next() orelse {
        log.err("Please provide a day number as an argument", .{});
        return error.MissingDayArgument;
    };

    const day = try std.fmt.parseInt(u8, day_str, 10);

    var timer = try std.time.Timer.start();

    // const input = @embedFile("../../inputs/1");
    const input = try read_file(day, alloc);
    defer alloc.free(input);

    inline for (1..26) |i| {
        if (day == i) {
            const day_module = @field(@This(), std.fmt.comptimePrint("day_{d}", .{i}));
            const silver_ans = try day_module.silver(input, alloc);
            var time_lapse = @as(f64, @floatFromInt(timer.lap())) / 1.0e6;
            log.info("Silver [{d:.3} ms]: {}", .{ time_lapse, silver_ans });

            const gold_ans = try day_module.gold(input, alloc);
            time_lapse = @as(f64, @floatFromInt(timer.lap())) / 1.0e6;
            log.info("Gold   [{d:.3} ms]: {}", .{ time_lapse, gold_ans });
            return;
        }
    }

    // const silver_ans = try day_module.silver(input, alloc);
    // var time_lapse = @as(f64, @floatFromInt(timer.lap())) / 1.0e6;

    // log.info("Silver [{d:.3} ms]: {}", .{ time_lapse, silver_ans });

    // const gold_ans = try day_module.gold(input, alloc);
    // time_lapse = @as(f64, @floatFromInt(timer.lap())) / 1.0e6;

    // log.info("Gold   [{d:.3} ms]: {}", .{ time_lapse, gold_ans });
}

test "silver test" {
    const alloc = std.testing.allocator;
    const input = try read_file("1t", alloc);
    defer alloc.free(input);

    const silver_ans = try day_1.silver(input, alloc);
    const ans: usize = 11;

    try std.testing.expectEqual(ans, silver_ans);
}

test "gold test" {
    const alloc = std.testing.allocator;
    const input = try read_file("1t", alloc);
    defer alloc.free(input);

    const silver_ans = try day_1.gold(input, alloc);
    const ans: usize = 31;

    try std.testing.expectEqual(ans, silver_ans);
}

pub fn read_file(file_name: []const u8, allocator: std.mem.Allocator) ![]u8 {
    const input_dir = try std.fs.cwd().openDir(
        "../inputs",
        .{ .iterate = true },
    );

    const file = try input_dir.openFile(file_name, .{});
    defer file.close();

    const file_size = try file.getEndPos();
    const file_content = try file.readToEndAlloc(allocator, file_size);

    return file_content;
}
