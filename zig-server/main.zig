const std = @import("std");

const Connection = std.net.StreamServer.Connection;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var clinets = std.ArrayList(Connection).init(allocator);
    defer clinets.deinit();

    var listner = std.net.StreamServer.init(.{});
    defer listner.deinit();

    try listner.listen(std.net.Address.initIp4([_]u8{ 127, 0, 0, 1 }, 8080));
    std.debug.print("listen at {} \n", .{listner.listen_address});

    while (true) {
        const conn = try listner.accept();
        try clinets.append(conn);
        _ = try std.Thread.spawn(.{}, handle_clinet, .{ conn, &clinets });
    }

    return;
}

fn handle_clinet(conn: Connection, clients: *std.ArrayList(Connection)) !void {
    std.debug.print("New Connection ip: {}\n", .{conn.address});

    var buffer: [1024]u8 = undefined;
    while (true) {
        _ = try conn.stream.read(&buffer);
        std.debug.print("message resieved: {s} \n", .{buffer});
        for (clients.*.items) |client| {
            if (client.address.getPort() != conn.address.getPort()) {
                client.stream.writeAll(&buffer) catch |err| {
                    std.debug.print("Could not send a message to : {}, error: {}\n", .{ client.address, err });
                };
            }
        }
    }
}
