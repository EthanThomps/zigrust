const std = @import("std");

// zig run main.zig
// zig test main.zig
pub fn main() !void {
    std.debug.print("{s}\n", .{"Hello World!"});
}

test "equal" {
    1 == 2;
}
