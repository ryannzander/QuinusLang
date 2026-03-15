class Point {
    x: int
    y: int

    init(x: int, y: int) {
        this.x = x;
        this.y = y;
    }
}

func main() -> void {
    var p: Point = new Point(1, 2);
    return;
}
