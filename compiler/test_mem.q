bring "std.mem";

extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;

craft main() -> void {
    make buf: link void = malloc(16);
    mem.set(buf, 0, 16);
    free(buf);
    send;
}
