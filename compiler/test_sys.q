bring "std.sys";

craft main() -> void {
    make w: i32 = sys.is_windows();
    make u: i32 = sys.is_unix();
    print(`windows=${w} unix=${u}`);
    send;
}
