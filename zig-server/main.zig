const std = @import("std");

const Connection = std.net.StreamServer.Connection;
pub const io_mode = .evented;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var room = Room{ .clinets = std.ArrayList(*Client).init(allocator) };
    defer room.clinets.deinit();

    var listner = std.net.StreamServer.init(.{});
    defer listner.deinit();

    try listner.listen(std.net.Address.initIp4([_]u8{ 127, 0, 0, 1 }, 8080));
    std.debug.print("listen at {} \n", .{listner.listen_address});

    while (true) {
        const clinet = try allocator.create(Client);
        clinet.* = Client{ .conn = try listner.accept(), .room = &room };
        _ = try std.Thread.spawn(.{}, Client.handle, .{clinet});
        try room.clinets.append(clinet);
    }

    return;
}

const Client = struct {
    conn: Connection,
    room: *Room,

    fn handle(client: *Client) !void {
        var buffer: [1024]u8 = undefined;
        while (true) {
            _ = try client.conn.stream.read(&buffer);
            try client.room.broadcast(client, buffer);
        }
    }
};

const Room = struct {
    clinets: std.ArrayList(*Client),
    fn broadcast(self: *Room, clinet: *Client, msg: [1024]u8) !void {
        for (self.clinets.items) |cc| {
            if (cc == clinet) continue;
            try cc.conn.stream.writeAll(&msg);
        }
    }
};
