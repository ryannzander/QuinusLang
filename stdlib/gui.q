// QuinusLang standard library: GUI via Raylib
// Requires: link with -lraylib
// Install: https://www.raylib.com/
// Build: quinus build && gcc -o out build/output.c -lraylib -I. (or use --link-extra)

extern craft ql_gui_init(width: i32, height: i32, title: str) -> void;
extern craft ql_gui_close() -> void;
extern craft ql_gui_should_close() -> bool;
extern craft ql_gui_begin_draw() -> void;
extern craft ql_gui_end_draw() -> void;
extern craft ql_gui_clear(r: i32, g: i32, b: i32) -> void;
extern craft ql_gui_rect(x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32) -> void;
extern craft ql_gui_circle(x: i32, y: i32, radius: f32, r: i32, g: i32, b: i32) -> void;
extern craft ql_gui_set_fps(fps: i32) -> void;
extern craft ql_gui_text(t: str, x: i32, y: i32, size: i32, r: i32, g: i32, b: i32) -> void;

realm gui {
    craft init(width: i32, height: i32, title: str) -> void {
        ql_gui_init(width, height, title);
        send;
    }

    craft close() -> void {
        ql_gui_close();
        send;
    }

    craft should_close() -> bool {
        send ql_gui_should_close();
    }

    craft begin_draw() -> void {
        ql_gui_begin_draw();
        send;
    }

    craft end_draw() -> void {
        ql_gui_end_draw();
        send;
    }

    craft clear(r: i32, g: i32, b: i32) -> void {
        ql_gui_clear(r, g, b);
        send;
    }

    craft rect(x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32) -> void {
        ql_gui_rect(x, y, w, h, r, g, b);
        send;
    }

    craft circle(x: i32, y: i32, radius: f32, r: i32, g: i32, b: i32) -> void {
        ql_gui_circle(x, y, radius, r, g, b);
        send;
    }

    craft set_fps(fps: i32) -> void {
        ql_gui_set_fps(fps);
        send;
    }

    craft text(t: str, x: i32, y: i32, size: i32, r: i32, g: i32, b: i32) -> void {
        ql_gui_text(t, x, y, size, r, g, b);
        send;
    }
}
