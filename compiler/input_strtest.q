extern craft fopen(path: str, mode: str) -> link void;

realm fs {
    craft open_file(path: str, mode: str) -> link void {
        send fopen(path, mode);
    }

    craft read_all(stream: link void) -> str {
        send "x";
    }
}

craft main() -> void {
    send;
}
