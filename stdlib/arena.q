// Q++ standard library: Arena allocator
// Simple alloc/free wrappers; full bump allocator requires hazard blocks

extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;

realm arena {
    craft alloc(size: usize) -> link void {
        send malloc(size);
    }

    craft dealloc(ptr: link void) -> void {
        free(ptr);
        send;
    }
}
