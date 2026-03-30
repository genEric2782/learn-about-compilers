const std = @import("std");

// Here void - the function returns no value
//      ! - return error or return type
// so !void means to return either an error or nothing
pub fn readInTACFile() !void {
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
    var file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const buffer = try std.fs.cwd().readFileAlloc(allocator, file_path, std.math.maxInt(usize));
    defer allocator.free(buffer);

    std.debug.print("File contents:\n{s}\n", .{buffer});

    // this method got depricated but still useful info about while loops
    // | value | while something produces a value bind to value and run the loop
    // while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', &line_buf)) |line| {
    //     defer allocator.free(line);

    //     std.debug.print("Line: {s}\n", .{line});
    // }
}
