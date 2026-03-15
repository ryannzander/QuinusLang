class Point {
    x: int
    y: int

    init(x: int, y: int) {
        this.x = x;
        this.y = y;
    }
}

craft main() -> void {
    make shift p: Point = new Point(1, 2);
    send;
}
