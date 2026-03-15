bring "std.term";

craft main() -> void {
    term.red();
    write("red ");
    term.green();
    write("green ");
    term.reset();
    print("done");
    send;
}
