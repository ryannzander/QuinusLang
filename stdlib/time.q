// Q++ standard library: Time
// Uses C FFI: time.h

extern craft ql_time_now() -> i64;

realm time {
    craft now() -> i64 {
        send ql_time_now();
    }
}
