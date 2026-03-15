extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;
extern craft fseek(stream: link void, offset: i64, whence: i32) -> i32;
extern craft ftell(stream: link void) -> i64;
extern craft malloc(size: usize) -> link void;
extern craft fread(buf: link void, size: usize, n: usize, stream: link void) -> usize;
extern craft __ql_null_at(buf: link void, i: usize) -> void;

realm fs {
    craft open_file(path: str, mode: str) -> link void {
        send fopen(path, mode);
    }

    craft close(stream: link void) -> i32 {
        send fclose(stream);
    }

    craft read_all(stream: link void) -> str {
        fseek(stream, 0, 2);
        make size: i64 = ftell(stream);
        fseek(stream, 0, 0);
        check (size <= 0) {
            send "";
        }
        make cap: int = (size as int) + 1;
        make buf: link void = malloc(cap as usize);
        fread(buf, 1, size as usize, stream);
        __ql_null_at(buf, size as usize);
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

    craft write_all(path: str, content: str) -> i32 {
        make f: link void = fopen(path, "w");
        check (f == 0) {
            send 0;
        }
        send 0;
    }
}
extern craft strlen(s: str) -> usize;
craft main() -> void {
    send 0;
}
