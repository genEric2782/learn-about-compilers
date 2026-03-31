const std = @import("std");

// TODO: Get to read a JSON File now :D
// Here void - the function returns no value
//      ! - return error or return type
// so !void means to return either an error or nothing
pub fn readInTACFile() !std.ArrayList([]const u8) {
    // using GeneralPurposeAllocator to define a pattern for how memory is allocated
    // the GeneralPurposeAllocator (debug) allocator
    // "This is a safe allocator that can prevent double-free, use-after-free and can detect leaks" per documentation
    // .{} is for configuring the allocator
    // . -> infer struct type
    // {} -> empty strcut (Default settings)
    // the {} after the argument creates an instance of the allocator
    // var -> mutable var
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    // defer - run this code when the current scope exits
    //      this ensures the allocator is properly cleaned up
    defer {
        // de initialzes the heap returns a leak check if a memory leak was found
        const check = gpa.deinit();
        if (check == .leak) {
            std.debug.print("Mem leak found\n", .{});
            @panic("Mem Leak");
        }
    }

    // the .allocator is an interface for using the newly created gpa
    const allocator = gpa.allocator();

    // Now that all the setup is out of the way time to read the file :D
    const file_path = "../../Tac.txt";

    const buffer = try std.fs.cwd().readFileAlloc(allocator, file_path, std.math.maxInt(usize));
    defer allocator.free(buffer);

    var lines = try std.ArrayList([]const u8).initCapacity(allocator, 0);
    defer lines.deinit(allocator);

    // std.debug.print("File contents:\n{s}\n", .{buffer});

    // this method got depricated but still useful info about while loops
    // | value | while something produces a value bind to value and run the loop
    var iter = std.mem.splitScalar(u8, buffer, '\n');
    while (iter.next()) |line| {
        const copy = try allocator.dupe(u8, line);
        try lines.append(allocator, copy);
        // std.debug.print("Each Line: {s}\n", .{line});
    }

    return lines;
}

pub fn generateASMInstr(tacInstr: std.ArrayList([]const u8)) !void {
    for (tacInstr) |instr| {}
}

// TODO
// pub fn determinInstructionType
