const std = @import("std");
const atomicStack = @import("atomicStack.zig");

pub const Stack = atomicStack.AtomicStack([]const u8);
pub var asm_stack = Stack.init();
