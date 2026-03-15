# std.gui

Cross-platform GUI via [Raylib](https://www.raylib.com/). Requires Raylib to be installed and linked.

## Setup

- **Windows**: `winget install raylib.raylib` or download from [raylib.com](https://www.raylib.com/)
- **macOS**: `brew install raylib`
- **Linux**: `apt install libraylib-dev` (or equivalent)

The compiler automatically adds `-lraylib` when the gui module is used.

## Functions

| Function | Description |
|----------|-------------|
| `gui.init(width, height, title)` | Open window |
| `gui.close()` | Close window |
| `gui.should_close()` | True if user requested close |
| `gui.begin_draw()` | Start frame drawing |
| `gui.end_draw()` | End frame drawing |
| `gui.clear(r, g, b)` | Clear background (0–255) |
| `gui.rect(x, y, w, h, r, g, b)` | Draw filled rectangle |
| `gui.circle(x, y, radius, r, g, b)` | Draw filled circle |
| `gui.set_fps(fps)` | Target frame rate |
| `gui.text(t, x, y, size, r, g, b)` | Draw text |

## Example

```q
bring "std.gui";

craft main() -> void {
    gui.init(800, 600, "Hello");
    gui.set_fps(60);
    loop {
        check (gui.should_close()) { break; }
        gui.begin_draw();
        gui.clear(30, 30, 46);
        gui.rect(100, 100, 200, 100, 138, 43, 226);
        gui.text("Hello, Raylib!", 120, 130, 20, 255, 255, 255);
        gui.end_draw();
    }
    gui.close();
    send;
}
```
