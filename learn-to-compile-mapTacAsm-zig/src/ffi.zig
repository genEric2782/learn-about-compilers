const std = @import("std");
const asmMap = @import("asmMap.zig");
const global = @import("global.zig");
const main_mod = @import("main.zig");

pub const AsmResult = extern struct {
    ptr: [*]u8,
    len: usize,
    err: bool, // true → ptr/len contain an error message string
};

// Rust need to initalize the global stack
export fn tac_init() void {
    main_mod.initGlobalStackWithRegisters();
}

export fn tac_compile(json_ptr: [*]const u8, json_len: usize) AsmResult {
    const json = json_ptr[0..json_len];
    return compile(json) catch |err| {
        const msg = std.fmt.allocPrint(
            std.heap.c_allocator,
            "zig error: {s}",
            .{@errorName(err)},
        ) catch return .{ .ptr = undefined, .len = 0, .err = true };
        return .{ .ptr = msg.ptr, .len = msg.len, .err = true };
    };
}

export fn tac_free_result(result: AsmResult) void {
    if (result.len > 0) {
        std.heap.c_allocator.free(result.ptr[0..result.len]);
    }
}

fn compile(json: []const u8) !AsmResult {
    // Use a GPA for internal work; only the final joined string
    // crosses the boundary via c_allocator.
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // 1. Parse JSON → []TACInstruction
    const parsed = try std.json.parseFromSlice(
        []asmMap.TACInstruction,
        allocator,
        json,
        .{},
    );
    defer parsed.deinit();

    const instructions = try asmMap.deepCopyJsonObject(allocator, parsed.value);
    defer asmMap.freeTACCopy(allocator, instructions);

    // 2. Run codegen
    var asm_lines = try asmMap.generateASMInstr(allocator, instructions);
    // Create the asm file

    defer {
        for (asm_lines.items) |line| allocator.free(line);
        asm_lines.deinit(allocator);
    }

    // Header Boiler plate
    try asm_lines.insert(allocator, 0, try allocator.dupe(u8, "section .text")); // TODO This is dumb and i hate since
    try asm_lines.insert(allocator, 1, try allocator.dupe(u8, "global _start")); // since a normal insert of a string literal isnt heap allocated (even though i pass an allocator)
    try asm_lines.insert(allocator, 2, try allocator.dupe(u8, "")); // there is nothing to free
    try asm_lines.insert(allocator, 3, try allocator.dupe(u8, "_start:")); // so the defer above panics and breaks

    // the move the exit code 60 to the rax reg?
    try asm_lines.append(allocator, try allocator.dupe(u8, "    mov rax, 60")); // TODO make this dynamic
    try asm_lines.append(allocator, try allocator.dupe(u8, "    syscall")); // TODO make this dynamic

    // 3. Join lines into a single buffer allocated with c_allocator
    //    so Rust can hand it back to tac_free_result safely.

    // TODO Add the spaces in under the start tag even though it isnt required
    var total: usize = 0;
    for (asm_lines.items) |line| total += line.len + 1; // +1 for '\n'

    const out = try std.heap.c_allocator.alloc(u8, total);
    var pos: usize = 0;
    for (asm_lines.items) |line| {
        @memcpy(out[pos .. pos + line.len], line);
        pos += line.len;
        out[pos] = '\n';
        pos += 1;
    }

    return AsmResult{ .ptr = out.ptr, .len = out.len, .err = false };
}
