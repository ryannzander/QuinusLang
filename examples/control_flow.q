craft main() -> void {
    make shift x: int = 5;
    check ((x > 0)) {
        make shift y: int = 1;
    }
    make shift i: int = 0;
    loopwhile ((i < 10)) {
        /* unsupported in fmt */
    }
    send;
}
