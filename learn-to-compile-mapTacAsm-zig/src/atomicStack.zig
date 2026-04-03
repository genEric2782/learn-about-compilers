const std = @import("std");

// Probs do need an atomic stack but better safe than sorry with global objects
// comptime - keyword use to forcaeably execute something at compile time
// we do this becuase this stack takes type type which is a generic type
// that will have a type by compile time
pub fn AtomicStack(comptime item: type) type {
    return struct {
        pub const Node = struct {
            data: item,
            next: ?*Node,
        };

        head: std.atomic.Value(?*Node),

        // @This returns type of current container i.e this struct
        pub fn init() @This() {
            return .{
                .head = std.atomic.Value(?*Node).init(null),
            };
        }

        pub fn push(self: *@This(), node: *Node) void {
            while (true) {
                const current = self.head.load(.acquire);
                node.next = current;

                if (self.head.cmpxchgWeak(current, node, .acq_rel, .acquire) == null) {
                    return;
                }
            }
        }

        pub fn pop(self: *@This()) ?*Node {
            while (true) {
                const current = self.head.load(.acquire) orelse return null;
                const next = current.next;

                if (self.head.cmpxchgWeak(current, next, .acq_rel, .acquire) == null) {
                    return current;
                }
            }
            return null;
        }
    };
}
