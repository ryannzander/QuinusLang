extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;

realm fs {
    craft open_file(path: str, mode: str) -> link void {
        send fopen(path, mode);
    }

    craft close(stream: link void) -> i32 {
        send fclose(stream);
    }

    craft read_all(stream: link void) -> str {
        send "x";
    }
}

craft main() -> void {
    send;
}
