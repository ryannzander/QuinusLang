bring "std.hash";

craft main() -> void {
    make s: str = "hello";
    make h: u64 = hash.fnv1a_str(s);
    print(`hash: ${h}`);
    send;
}
