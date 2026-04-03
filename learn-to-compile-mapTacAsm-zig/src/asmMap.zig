const std = @import("std");
const linearscan = @import("linearscan.zig");

pub const TACvar = struct {
    @"$type": []const u8,
    tacTempValue: []const u8,
    value: ?[]const u8 = null,
    op: ?[]const u8 = null, // ? used to mark as optional
    arg1: ?[]const u8 = null,
    arg2: ?[]const u8 = null,
};

pub const TACInstruction = struct {
    opcode: []const u8, // Zig doesnt have strings it instead has slices of bytes
    tacvar: TACvar,
};

const OpCode = enum {
    LOAD_CONSTANT,
    ADD,
    ERROR,
};

// TODO: Get to read a JSON File now :D
// Here void - the function returns no value
//      ! - return error or return type
// so !void means to return either an error or nothing
pub fn readInTACFile(allocator: std.mem.Allocator) ![]TACInstruction { // !std.ArrayList([]const u8) {

    // Now that all the setup is out of the way time to read the file :D
    const file_path = "../../TacJson.json";

    const buffer = try std.fs.cwd().readFileAlloc(allocator, file_path, std.math.maxInt(usize));
    defer allocator.free(buffer);

    var parsedJson = try std.json.parseFromSlice([]TACInstruction, allocator, buffer, .{});
    defer parsedJson.deinit();

    const tac_instructions = parsedJson.value;
    const copy = deepCopyJsonObject(allocator, tac_instructions);
    // const copy = try allocator.dupe(TACInstruction, tac_instructions);

    // for (tac_instructions) |tac_instruction| {
    //     std.debug.print("Op Codes: \n{s}\n", .{tac_instruction.opcode});
    // }

    // std.debug.print("File contents:\n{any}\n", .{root});

    // var lines = try std.ArrayList([]const u8).initCapacity(allocator, 0);
    // defer lines.deinit(allocator);

    // std.debug.print("File contents:\n{s}\n", .{buffer});

    // | value | while something produces a value bind to value and run the loop
    // var iter = std.mem.splitScalar(u8, buffer, '\n');
    // while (iter.next()) |line| {
    //     const copy = try allocator.dupe(u8, line);
    //     try lines.append(allocator, copy);
    //     // std.debug.print("Each Line: {s}\n", .{line});
    // }

    return copy;
}

// TODO: Deep Copy Uggghhhhhh
pub fn deepCopyJsonObject(allocator: std.mem.Allocator, tacinstructions: []TACInstruction) ![]TACInstruction {
    var copy = try allocator.alloc(TACInstruction, tacinstructions.len);

    // |Value, iter|
    for (tacinstructions, 0..) |instruction, i| {
        copy[i].opcode = try allocator.dupe(u8, instruction.opcode);

        const innerObjCopy = instruction.tacvar;
        var deepInnerObjCopy = TACvar{
            .@"$type" = try allocator.dupe(u8, innerObjCopy.@"$type"),
            .tacTempValue = try allocator.dupe(u8, innerObjCopy.tacTempValue),
            .op = null,
            .value = null,
            .arg1 = null,
            .arg2 = null,
        };
        if (instruction.tacvar.arg1) |val| {
            deepInnerObjCopy.arg1 = try allocator.dupe(u8, val);
        }
        if (instruction.tacvar.arg2) |val| {
            deepInnerObjCopy.arg2 = try allocator.dupe(u8, val);
        }
        if (instruction.tacvar.op) |val| {
            deepInnerObjCopy.op = try allocator.dupe(u8, val);
        }
        if (instruction.tacvar.value) |val| {
            deepInnerObjCopy.value = try allocator.dupe(u8, val);
        }

        copy[i].tacvar = deepInnerObjCopy;
    }

    return copy;
}

pub fn freeTACCopy(allocator: std.mem.Allocator, deepCopy: []TACInstruction) void {
    for (deepCopy) |copiedInstruction| {
        // free opcode
        allocator.free(copiedInstruction.opcode);

        const innerObjCopy = copiedInstruction.tacvar;
        allocator.free(innerObjCopy.@"$type");
        allocator.free(innerObjCopy.tacTempValue);

        if (innerObjCopy.value) |val| {
            allocator.free(val);
        }
        if (innerObjCopy.op) |val| {
            allocator.free(val);
        }
        if (innerObjCopy.arg1) |val| {
            allocator.free(val);
        }
        if (innerObjCopy.arg2) |val| {
            allocator.free(val);
        }
    }

    // finally, free the top-level array
    allocator.free(deepCopy);
}

pub fn generateASMInstr(tacinstructions: []TACInstruction) !void {

    // determine life times of IR instructions
    try linearscan.defineLiveInterals(tacinstructions);

    for (tacinstructions) |instruciton| {
        std.debug.print("Op Codes: {s}\n", .{instruciton.opcode});
        const instrOpCode = determinInstructionOpCode(instruciton.opcode);
        switch (instrOpCode) {
            OpCode.LOAD_CONSTANT => {
                std.debug.print("Foo\n", .{});
            },
            OpCode.ADD => {
                std.debug.print("Bar\n", .{});
            },
            else => {
                std.debug.print("Bazz\n", .{});
            },
        }
    }
}

// Since Zig cant switch on strings just map opcade json value to an enum which is can switch on
pub fn determinInstructionOpCode(opcode: []const u8) OpCode {
    if (std.mem.eql(u8, opcode, "LOAD_CONSTANT")) {
        return OpCode.LOAD_CONSTANT;
    }
    if (std.mem.eql(u8, opcode, "ADD")) {
        return OpCode.ADD;
    } else {
        return OpCode.ERROR;
    }
}
