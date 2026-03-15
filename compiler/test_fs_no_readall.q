extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;
extern craft __ql_null_at(buf: link void, i: usize) -> void;

realm fs {
    craft open_file(path: str, mode: str) -> link void {
        send fopen(path, mode);
    }

    craft close(stream: link void) -> i32 {
        send fclose(stream);
    }

    craft read_all(stream: link void) -> str {
        make buf: link void = 0;
        __ql_null_at(buf, 0 as usize);
        send buf;
    }

    craft exists(path: str) -> bool {
        make f: link void = fopen(path, "r");
        check (f == 0) {
            send false;
        }
        fclose(f);
        send true;
    }
}
extern craft strlen(s: str) -> usize;
craft main() -> void {
    send 0;
}
