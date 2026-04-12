const std = @import("std");
const atomicStack = @import("atomicStack.zig");
const globalBitSet = @import("globalBitSet.zig");

pub const Stack = atomicStack.AtomicStack([]const u8);
pub var asm_stack = Stack.init();

pub var register_bit_map = globalBitSet.RegisterMap.init();
