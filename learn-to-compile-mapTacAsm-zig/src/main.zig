const std = @import("std");
const asmMap = @import("asmMap.zig");
const atomicStack = @import("atomicStack.zig");
const learn_to_compile_mapTacAsm_zig = @import("learn_to_compile_mapTacAsm_zig");

const Stack = atomicStack.AtomicStack([]const u8);
var asm_stack = Stack.init();

// Yucky
var rax_node = Stack.Node{ .data = "rax", .next = null };
var rbx_node = Stack.Node{ .data = "rbx", .next = null };
var rcx_node = Stack.Node{ .data = "rcx", .next = null };
var rdx_node = Stack.Node{ .data = "rdx", .next = null };
var rsi_node = Stack.Node{ .data = "rsi", .next = null };
var rdi_node = Stack.Node{ .data = "rdi", .next = null };
var rbp_node = Stack.Node{ .data = "rbp", .next = null };
var rsp_node = Stack.Node{ .data = "rsp", .next = null };
var r8_node = Stack.Node{ .data = "r8", .next = null };
var r9_node = Stack.Node{ .data = "r9", .next = null };
var r10_node = Stack.Node{ .data = "r10", .next = null };
var r11_node = Stack.Node{ .data = "r11", .next = null };
var r12_node = Stack.Node{ .data = "r12", .next = null };
var r13_node = Stack.Node{ .data = "r13", .next = null };
var r14_node = Stack.Node{ .data = "r14", .next = null };
var r15_node = Stack.Node{ .data = "r15", .next = null };

pub fn initGlobalStackWithRegisters() void {
    // These are all the x86_64 64-bit general purpose registers
    asm_stack.push(&rax_node);
    asm_stack.push(&rbx_node);
    asm_stack.push(&rcx_node);
    asm_stack.push(&rdx_node);
    asm_stack.push(&rsi_node);
    asm_stack.push(&rdi_node);
    asm_stack.push(&rbp_node);
    asm_stack.push(&rsp_node);
    asm_stack.push(&r8_node);
    asm_stack.push(&r9_node);
    asm_stack.push(&r10_node);
    asm_stack.push(&r11_node);
    asm_stack.push(&r12_node);
    asm_stack.push(&r13_node);
    asm_stack.push(&r14_node);
    asm_stack.push(&r15_node);
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

    try asmMap.generateASMInstr(tacInstrcutionJson);
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
