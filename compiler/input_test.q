extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;

realm fs {
    craft open_file(path: str, mode: str) -> link void {
        send fopen(path, mode);
    }
}

craft main() -> void {
    send;
}
