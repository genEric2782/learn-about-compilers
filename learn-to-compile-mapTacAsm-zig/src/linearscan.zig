const std = @import("std");
const asmMap = @import("asmMap.zig");

const tacLifetime = struct {
    tacTempValue: []const u8, // Zig doesnt have strings it instead has slices of bytes
    start_pos: usize,
    end_pos: ?usize = null,
};

// !std.ArrayList(tacLifetime) {
pub fn defineLiveInterals(allocator: std.mem.Allocator, tacinstructions: []asmMap.TACInstruction) !std.ArrayList(tacLifetime) {
    var lifetimes = try std.ArrayList(tacLifetime).initCapacity(allocator, 0);

    // var tacInitalPos = std.AutoHashMap(i32, []const u8).init(allocator);
    // var tacFinalPos = std.AutoHashMap(i32, []const u8).init(allocator);
    // defer tacInitalPos.deinit();
    // defer tacFinalPos.deinit();

    // assign a pos to the given IR instructions
    for (tacinstructions, 0..) |instruction, i| {
        // try tacInitalPos.put(i, instruction.tacvar.tacTempValue);
        var tac_intruction_pos = tacLifetime{ .tacTempValue = instruction.tacvar.tacTempValue, .start_pos = i };

        // Look for where a given instruction is next used
        if (i + 1 < tacinstructions.len) {
            for (tacinstructions, i + 1..) |subsequentInstruction, j| {
                if (subsequentInstruction.tacvar.arg1 != null and std.mem.eql(u8, subsequentInstruction.tacvar.arg1.?, instruction.tacvar.tacTempValue) or subsequentInstruction.tacvar.arg2 != null and std.mem.eql(u8, subsequentInstruction.tacvar.arg2.?, instruction.tacvar.tacTempValue)) {
                    // try tacInitalPos.put(j, instruction.tacvar.tacTempValue);
                    tac_intruction_pos.end_pos = j;
                }
            }
        }
        // std.debug.print("Liftetimes: {any}\n", .{tac_intruction_pos});
        try lifetimes.append(allocator, tac_intruction_pos);
    }

    // Probably going to need to deep copy or something stupid
    return lifetimes;
}
