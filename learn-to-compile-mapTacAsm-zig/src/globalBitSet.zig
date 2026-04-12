const std = @import("std");

// General Purpose 64 bit registers
pub const Register = enum(u64) {
    rax = 0,
    rbx,
    rcx,
    rdx,
    rsi,
    rdi,
    rbp,
    rsp,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,

    pub fn toString(self: Register) []const u8 {
        return switch (self) {
            .rax => "rax",
            .rbx => "rbx",
            .rcx => "rcx",
            .rdx => "rdx",
            .rsi => "rsi",
            .rdi => "rdi",
            .rbp => "rbp",
            .rsp => "rsp",
            .r8 => "r8",
            .r9 => "r9",
            .r10 => "r10",
            .r11 => "r11",
            .r12 => "r12",
            .r13 => "r13",
            .r14 => "r14",
            .r15 => "r15",
        };
    }
};

pub const RegisterMap = struct {
    in_use: std.atomic.Value(u16),

    pub fn init() @This() {
        return .{
            .in_use = std.atomic.Value(u16).init(0),
        };
    }

    pub fn tryAcquire(self: *@This(), reg: Register) bool {
        // @intFromEnum - Zig builtin function
        const mask: u16 = @as(u16, 1) << @as(u4, @intCast(@intFromEnum(reg)));

        // While true is here for racing threads if thread 1 get the value thread 2 wants beofre it can
        // the logic will have it try again
        while (true) {
            // Get current state of atomic
            const current = self.in_use.load(.acquire);

            // if register is already in use
            if ((current & mask) != 0) return false;

            // Updating register mapping with taken register
            const new = current | mask;

            // If state is still equal to the current state i.e. another thread hasnt changed it
            // replace it with the new state
            // success_order: acq_rel -> publish your right
            // fail -> .aquire new current state of reg
            if (self.in_use.cmpxchgWeak(current, new, .acq_rel, .acquire) == null) {
                return true;
            }
        }
    }

    pub fn acquireAny(self: *@This()) ?Register {
        while (true) {
            const current = self.in_use.load(.acquire);

            // invert: 1 = free
            const free_mask = ~current;

            if (free_mask == 0) return null; // none available

            // ctz = count trailing zeros
            const bit_index: u4 = @intCast(@ctz(free_mask)); // first free bit
            const mask: u16 = @as(u16, 1) << bit_index;

            const new = current | mask;

            if (self.in_use.cmpxchgWeak(current, new, .acq_rel, .acquire) == null) {
                return @enumFromInt(bit_index);
            }
        }
    }

    pub fn release(self: *@This(), reg: Register) void {
        const mask: u16 = @as(u16, 1) << @as(u4, @intCast(@intFromEnum(reg))); // <--- This nasty no way this is right thing to do

        while (true) {
            const current = self.in_use.load(.acquire);
            const new = current & ~mask;

            if (self.in_use.cmpxchgWeak(current, new, .acq_rel, .acquire) == null) {
                return;
            }
        }
    }
};
