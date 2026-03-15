// QuinusLang standard library: Terminal/ANSI
// Colors and cursor control via escape codes

extern craft ql_term_reset() -> void;
extern craft ql_term_red() -> void;
extern craft ql_term_green() -> void;
extern craft ql_term_yellow() -> void;
extern craft ql_term_blue() -> void;
extern craft ql_term_magenta() -> void;
extern craft ql_term_cyan() -> void;
extern craft ql_term_bold() -> void;
extern craft ql_term_clear_screen() -> void;
extern craft ql_term_cursor_hide() -> void;
extern craft ql_term_cursor_show() -> void;

realm term {
    craft reset() -> void {
        ql_term_reset();
        send;
    }

    craft red() -> void {
        ql_term_red();
        send;
    }

    craft green() -> void {
        ql_term_green();
        send;
    }

    craft yellow() -> void {
        ql_term_yellow();
        send;
    }

    craft blue() -> void {
        ql_term_blue();
        send;
    }

    craft magenta() -> void {
        ql_term_magenta();
        send;
    }

    craft cyan() -> void {
        ql_term_cyan();
        send;
    }

    craft bold() -> void {
        ql_term_bold();
        send;
    }

    craft clear_screen() -> void {
        ql_term_clear_screen();
        send;
    }

    craft cursor_hide() -> void {
        ql_term_cursor_hide();
        send;
    }

    craft cursor_show() -> void {
        ql_term_cursor_show();
        send;
    }
}
