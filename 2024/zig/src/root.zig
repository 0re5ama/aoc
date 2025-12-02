const std = @import("std");
const testing = std.testing;

export fn add(a: i32, b: i32) i32 {
    return a + b;
}

pub fn read_file(allocator: std.mem.Allocator, file_name: []const u8) ![]u8 {
    const input_dir = try std.fs.cwd().openDir(
        "../inputs",
        .{ .iterate = true },
    );

    const file = try input_dir.openFile(file_name, .{});
    defer file.close();

    const file_size = try file.getEndPos();
    const file_content = try file.readToEndAlloc(allocator, file_size);
    defer allocator.free(file_content);

    return file_content;
}

test "basic add functionality" {
    try testing.expect(add(3, 7) == 10);
}
