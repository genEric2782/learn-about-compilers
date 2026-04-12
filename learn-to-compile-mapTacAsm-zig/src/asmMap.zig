const std = @import("std");
const linearscan = @import("linearscan.zig");
const atomicStack = @import("atomicStack.zig");
const global = @import("global.zig");
const globalBitSet = @import("globalBitSet.zig");

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
    MINUS,
    MULTIPLY,
    DIVIDE,
    ERROR,
};

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

// Deep Copy :(
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

// Since Zig cant switch on strings just map opcade json value to an enum which is can switch on
pub fn determinInstructionOpCode(opcode: []const u8) OpCode {
    if (std.mem.eql(u8, opcode, "LOAD_CONSTANT")) {
        return OpCode.LOAD_CONSTANT;
    }
    if (std.mem.eql(u8, opcode, "ADD")) {
        return OpCode.ADD;
    }
    if (std.mem.eql(u8, opcode, "MINUS")) {
        return OpCode.MINUS;
    }
    if (std.mem.eql(u8, opcode, "MULTIPLY")) {
        return OpCode.MULTIPLY;
    }
    if (std.mem.eql(u8, opcode, "DIVIDE")) {
        return OpCode.DIVIDE;
    } else {
        return OpCode.ERROR;
    }
}

pub fn generateASMInstr(allocator: std.mem.Allocator, tacinstructions: []TACInstruction) !std.ArrayList([]const u8) {
    const r_enum = globalBitSet.Register;
    var mappedAsmInstructions = try std.ArrayList([]const u8).initCapacity(allocator, 0);
    var inserted_instructions: u64 = 0;
    // defer mappedAsmInstructions.deinit(allocator); // TODO: Free here?

    // var tacToRegMap = std.StringHashMap(*global.Stack.Node).init(allocator); // defer tacInitalPos.deinit();
    var tacToRegMap = std.StringHashMap(r_enum).init(allocator);
    defer tacToRegMap.deinit(); // TODO: Free here?

    // determine life times of IR instructions
    var lifetimes = try linearscan.defineLiveInterals(allocator, tacinstructions);
    defer lifetimes.deinit(allocator);

    var asmInstr: []const u8 = &[_]u8{}; // initalize to an empty slice doing there here to aovid lifetime issues? of initalizing in the switch

    // for (lifetimes.items) |lifetime| {
    //     std.debug.print("Liftetimes: {any}\n", .{lifetime});
    // }
    // std.debug.print("Liftetimes: {any}\n", .{lifetimes});
    var lifetimeCounter: i32 = 0;
    for (tacinstructions, 0..) |instruciton, i| {
        std.debug.print("Op Codes: {s}\n", .{instruciton.opcode});
        const instrOpCode = determinInstructionOpCode(instruciton.opcode);
        switch (instrOpCode) {
            OpCode.LOAD_CONSTANT => {
                const reg: r_enum = global.register_bit_map.acquireAny() orelse return error.OutOfRegisters; // TODO Need to handle spilling varibale if out of stack space
                // const reg: *global.Stack.Node = global.asm_stack.pop() orelse return error.EmptyStack; // TODO Need to handle spilling varibale if out of stack space
                try tacToRegMap.put(instruciton.tacvar.tacTempValue, reg); // .* -> deref
                asmInstr = try std.fmt.allocPrint(allocator, "mov {s}, {s}", .{
                    reg.toString(),
                    instruciton.tacvar.value orelse return error.MissingValue,
                });
                // defer allocator.free(asmInstr);
                // mappedAsmInstructions.append(allocator, asmInstr);
                try mappedAsmInstructions.append(allocator, asmInstr);
                std.debug.print("Asembly Instruction: {s}\n", .{asmInstr});
            },
            OpCode.ADD => {
                const operand1 = tacToRegMap.get(instruciton.tacvar.arg1 orelse return error.MissingValue) orelse null;
                const operand2 = tacToRegMap.get(instruciton.tacvar.arg2 orelse return error.MissingValue) orelse null;
                // Look for what to instructions are getting added together
                // since the instructions that are going to be added should be in the map that can just cross reference
                // Ziggy why no support for multiple bindings on an if stament :(
                if (operand1) |reg1| {
                    if (operand2) |reg2| {
                        asmInstr = try std.fmt.allocPrint(allocator, "add {s}, {s}", .{
                            reg1.toString(),
                            reg2.toString(),
                        });
                        try mappedAsmInstructions.append(allocator, asmInstr);
                    } else {
                        return error.MissingValue;
                    }
                } else {
                    return error.MissingValue;
                }
                // defer allocator.free(asmInstr);

                std.debug.print("Asembly Instruction: {s}\n", .{asmInstr});
            },
            OpCode.MINUS => {
                const operand1 = tacToRegMap.get(instruciton.tacvar.arg1 orelse return error.MissingValue) orelse null;
                const operand2 = tacToRegMap.get(instruciton.tacvar.arg2 orelse return error.MissingValue) orelse null;
                // Look for what to instructions are getting added together
                // since the instructions that are going to be added should be in the map that can just cross reference
                // Ziggy why no support for multiple bindings on an if stament :(
                if (operand1) |reg1| {
                    if (operand2) |reg2| {
                        asmInstr = try std.fmt.allocPrint(allocator, "sub {s}, {s}", .{
                            reg1.toString(),
                            reg2.toString(),
                        });
                        try mappedAsmInstructions.append(allocator, asmInstr);
                    } else {
                        return error.MissingValue;
                    }
                } else {
                    return error.MissingValue;
                }
                // defer allocator.free(asmInstr);

                std.debug.print("Asembly Instruction: {s}\n", .{asmInstr});
            },
            // TODO In A shocking turn of event mul only takes one operand (and assumes the other?)
            // mul takes register and multiplies it with a value in rax
            // therefore i need to mv val 1 into rax and then * with val2 and store the answer in rax and rdx
            // so if either of those are taken i need to mv them into other registers
            OpCode.MULTIPLY => {
                const operand1 = tacToRegMap.get(instruciton.tacvar.arg1 orelse return error.MissingValue) orelse null;
                const operand2 = tacToRegMap.get(instruciton.tacvar.arg2 orelse return error.MissingValue) orelse null;
                // Look for what to instructions are getting multiplied together
                // Since multiplication expects one of the operands to be in rax
                // and then sotred in rax and rdx,
                // Need to check if those reg are in use and if they are save off there contents and then free them
                if (operand1) |reg1| {
                    if (operand2) |reg2| {
                        // if rax is already aprat of this mul instruction no need to do this
                        // TODO: THis is fragile i probably need to account for if rax is reg1 or reg 2 but i dont wanna think about that atm
                        if (!(reg1 == r_enum.rax or reg2 == r_enum.rax)) {
                            // if rax isnt free, free it
                            if (!global.register_bit_map.tryAcquire(r_enum.rax)) {
                                try moveContentsToFreeRegister(allocator, &tacToRegMap, &inserted_instructions, &mappedAsmInstructions, globalBitSet.Register.rax);
                            }
                            // Need to move the value of arg1 into rax so it can be multipled with arg2
                            asmInstr = try std.fmt.allocPrint(allocator, "mov {s} {s}", .{
                                r_enum.rax.toString(),
                                reg1.toString(),
                            });
                            try mappedAsmInstructions.append(allocator, asmInstr);
                        }

                        // Since half the result is stored in rdx need to make sure it is free at well
                        // TODO Implement a lock flag maybe since rax and rdx cant be used after this multiply?
                        // TODO: for now only "support 1 mul insturrction" will need to implement storing sutff in memory
                        // else am going to run out on registers very quickly if having multiple mul instructions
                        if (!global.register_bit_map.tryAcquire(r_enum.rdx)) {
                            try moveContentsToFreeRegister(allocator, &tacToRegMap, &inserted_instructions, &mappedAsmInstructions, globalBitSet.Register.rdx);
                        }
                        // TODO For now only multiply small values as we currently cant handle outputting numbers that
                        // overflow into rdx until i implement write in asm :D
                        const tmp_reg: globalBitSet.Register = if (reg1 == r_enum.rax) reg2 else reg1;
                        asmInstr = try std.fmt.allocPrint(allocator, "mul {s}", .{
                            tmp_reg.toString(),
                        });
                        try mappedAsmInstructions.append(allocator, asmInstr);

                        // How to handle the fact that it gets stored in rax:rdx?
                    } else {
                        return error.MissingValue;
                    }
                } else {
                    return error.MissingValue;
                }
                // defer allocator.free(asmInstr);

                std.debug.print("Asembly Instruction: {s}\n", .{asmInstr});
            },
            // Gunna go ahead and guess this is like mul i could do a 5 second google but where is the fun in that
            // TODO
            OpCode.DIVIDE => {
                const operand1 = tacToRegMap.get(instruciton.tacvar.arg1 orelse return error.MissingValue) orelse null;
                const operand2 = tacToRegMap.get(instruciton.tacvar.arg2 orelse return error.MissingValue) orelse null;
                // Look for what to instructions are getting added together
                // since the instructions that are going to be added should be in the map that can just cross reference
                // Ziggy why no support for multiple bindings on an if stament :(
                if (operand1) |reg1| {
                    if (operand2) |reg2| {
                        asmInstr = try std.fmt.allocPrint(allocator, "div {s}, {s}", .{
                            reg1.toString(),
                            reg2.toString(),
                        });
                        try mappedAsmInstructions.append(allocator, asmInstr);
                    } else {
                        return error.MissingValue;
                    }
                } else {
                    return error.MissingValue;
                }
                // defer allocator.free(asmInstr);

                std.debug.print("Asembly Instruction: {s}\n", .{asmInstr});
            },
            else => {
                std.debug.print("Bazz\n", .{});
            },
        }
        lifetimeCounter += 1;
        // try mappedAsmInstructions.append(allocator, asmInstr);

        // TODO perform lifetime check to free up registers here

        // If this the last iteration of the loop
        if (i == tacinstructions.len - 1) {
            // Add instructions for program exit
            // TODO this will only work for instructions like add
            const reg_with_output = tacToRegMap.get(instruciton.tacvar.arg1 orelse return error.MissingValue) orelse return error.MissingValue;
            const asmOut = try std.fmt.allocPrint(allocator, "mov {s}, {s}", .{
                "rdi",
                reg_with_output.toString(),
            });
            try mappedAsmInstructions.append(allocator, asmOut);
        }
    }

    return mappedAsmInstructions;
}

// DEBUGGING
// allocator: std.mem.Allocator,
pub fn generateASMFile(mapped_asm_instructions: std.ArrayList([]const u8)) !void {
    const file_path = "../../output.asm";

    const file = try std.fs.cwd().createFile(file_path, .{ .truncate = true });
    defer file.close();

    // Seems like alot of boiler plate for a writer...
    var buffer: [1024]u8 = undefined;
    var writer_wrapper = file.writer(&buffer);
    const writer = &writer_wrapper.interface;

    // Create the asm file
    // Header Boiler plate
    try writer.print("section .text\n", .{});
    try writer.print("global _start\n", .{});
    try writer.print("\n", .{});
    try writer.print("_start:\n", .{});

    for (mapped_asm_instructions.items) |mapped_asm_instruction| {
        try writer.print("    {s}\n", .{mapped_asm_instruction});
    }

    // the move the exit code 60 to the rax reg?
    try writer.print("    mov rax, 60\n", .{});
    try writer.print("    syscall", .{});

    try writer.flush(); // this is what actually writes all the writer lines to the file
}

pub fn addHeaderAndFootertoASM(allocator: std.mem.Allocator, asm_lines: *std.ArrayList([]const u8)) !void {
    // Header Boiler plate
    try asm_lines.insert(allocator, 0, try allocator.dupe(u8, "section .text")); // TODO This is dumb and i hate since
    try asm_lines.insert(allocator, 1, try allocator.dupe(u8, "global _start")); // since a normal insert of a string literal isnt heap allocated (even though i pass an allocator)
    try asm_lines.insert(allocator, 2, try allocator.dupe(u8, "")); // there is nothing to free
    try asm_lines.insert(allocator, 3, try allocator.dupe(u8, "_start:")); // so the defer above panics and breaks

    // the move the exit code 60 to the rax reg?
    try asm_lines.append(allocator, try allocator.dupe(u8, "mov rax, 60")); // TODO make this dynamic
    try asm_lines.append(allocator, try allocator.dupe(u8, "syscall")); // TODO make this dynamic
}

pub fn moveContentsToFreeRegister(allocator: std.mem.Allocator, tacToRegMap: *std.StringHashMap(globalBitSet.Register), inserted_instructions: *u64, mappedAsmInstructions: *std.ArrayList([]const u8), register_to_free: globalBitSet.Register) !void {
    const reg_store: globalBitSet.Register = global.register_bit_map.acquireAny() orelse return error.OutOfRegisters;

    inserted_instructions.* += 1;
    const inserted_instructions_string = try std.fmt.allocPrint(allocator, "i{}", .{inserted_instructions});
    defer allocator.free(inserted_instructions_string);
    try tacToRegMap.put(inserted_instructions_string, reg_store); // .* -> deref

    const asmInstr = try std.fmt.allocPrint(allocator, "mov {s}, {s}", .{
        reg_store.toString(),
        register_to_free.toString(),
    });
    try mappedAsmInstructions.append(allocator, asmInstr);

    // global.register_bit_map.release(register_to_free);
}
