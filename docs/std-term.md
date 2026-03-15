# std.term

Terminal/ANSI escape codes for colors and cursor control.

## Functions

| Function | Description |
|----------|-------------|
| `term.reset()` | Reset all attributes |
| `term.red()`, `term.green()`, etc. | Set foreground color |
| `term.bold()` | Bold text |
| `term.clear_screen()` | Clear screen and move cursor |
| `term.cursor_hide()` | Hide cursor |
| `term.cursor_show()` | Show cursor |

## Example

```q
bring "std.term";

craft main() -> void {
    term.red();
    write("Error: ");
    term.reset();
    term.green();
    write("OK\n");
    term.reset();
    send;
}
```
