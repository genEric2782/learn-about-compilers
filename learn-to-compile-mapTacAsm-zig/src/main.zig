const std = @import("std");
const asmMap = @import("asmMap.zig");
const atomicStack = @import("atomicStack.zig");
const global = @import("global.zig");
const learn_to_compile_mapTacAsm_zig = @import("learn_to_compile_mapTacAsm_zig");

// Yucky
var rax_node = global.Stack.Node{ .data = "rax", .next = null };
var rbx_node = global.Stack.Node{ .data = "rbx", .next = null };
var rcx_node = global.Stack.Node{ .data = "rcx", .next = null };
var rdx_node = global.Stack.Node{ .data = "rdx", .next = null };
var rsi_node = global.Stack.Node{ .data = "rsi", .next = null };
// var rdi_node = global.Stack.Node{ .data = "rdi", .next = null }; this reg is used ot wrtie to stdout so maybe do be using it?
var rbp_node = global.Stack.Node{ .data = "rbp", .next = null };
var rsp_node = global.Stack.Node{ .data = "rsp", .next = null };
var r8_node = global.Stack.Node{ .data = "r8", .next = null };
var r9_node = global.Stack.Node{ .data = "r9", .next = null };
var r10_node = global.Stack.Node{ .data = "r10", .next = null };
var r11_node = global.Stack.Node{ .data = "r11", .next = null };
var r12_node = global.Stack.Node{ .data = "r12", .next = null };
var r13_node = global.Stack.Node{ .data = "r13", .next = null };
var r14_node = global.Stack.Node{ .data = "r14", .next = null };
var r15_node = global.Stack.Node{ .data = "r15", .next = null };

pub fn initGlobalStackWithRegisters() void {
    // These are all the x86_64 64-bit general purpose registers
    global.asm_stack.push(&r15_node);
    global.asm_stack.push(&r14_node);
    global.asm_stack.push(&r13_node);
    global.asm_stack.push(&r12_node);
    global.asm_stack.push(&r11_node);
    global.asm_stack.push(&r10_node);
    global.asm_stack.push(&r9_node);
    global.asm_stack.push(&r8_node);
    global.asm_stack.push(&rsp_node);
    global.asm_stack.push(&rbp_node);
    // global.asm_stack.push(&rdi_node);
    global.asm_stack.push(&rsi_node);
    global.asm_stack.push(&rdx_node);
    global.asm_stack.push(&rcx_node);
    global.asm_stack.push(&rbx_node);
    global.asm_stack.push(&rax_node);
}

pub fn main() !void {
    // Prints to stderr, ignoring potential errors.
    // std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
    // try learn_to_compile_mapTacAsm_zig.bufferedPrint();

    initGlobalStackWithRegisters();

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

    const tacInstrcutionJson = try asmMap.readInTACFile(allocator);
    defer asmMap.freeTACCopy(allocator, tacInstrcutionJson);
    // defer allocator.free(tacInstrcutionJson);

    var asm_map = try asmMap.generateASMInstr(allocator, tacInstrcutionJson);
    defer {
        for (asm_map.items) |item| {
            allocator.free(item);
        }
        asm_map.deinit(allocator);
    }

    try asmMap.generateASMFile(asm_map);
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}
