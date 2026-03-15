// QuinusLang standard library: Platform detection
// Compile-time platform checks via #ifdef in emitted C

realm sys {
    craft is_windows() -> i32 {
        hazard {
            cblock { "#ifdef _WIN32\n    return 1;\n#else\n    return 0;\n#endif" };
        }
        send 0;
    }

    craft is_unix() -> i32 {
        hazard {
            cblock { "#ifndef _WIN32\n    return 1;\n#else\n    return 0;\n#endif" };
        }
        send 0;
    }
}
