// Q++ standard library: Math operations
// Uses C FFI to wrap math.h and stdlib.h

extern craft abs(x: i32) -> i32;
extern craft fabs(x: f64) -> f64;
extern craft sqrt(x: f64) -> f64;
extern craft fmin(a: f64, b: f64) -> f64;
extern craft fmax(a: f64, b: f64) -> f64;
extern craft ql_add_checked_i32(a: i32, b: i32) -> Result(i32, i32);
extern craft ql_sub_checked_i32(a: i32, b: i32) -> Result(i32, i32);
extern craft ql_mul_checked_i32(a: i32, b: i32) -> Result(i32, i32);

realm math {
    craft abs_i32(x: i32) -> i32 {
        send abs(x);
    }

    craft abs_f64(x: f64) -> f64 {
        send fabs(x);
    }

    craft min_i32(a: i32, b: i32) -> i32 {
        check (a < b) {
            send a;
        }
        send b;
    }

    craft max_i32(a: i32, b: i32) -> i32 {
        check (a > b) {
            send a;
        }
        send b;
    }

    craft min_f64(a: f64, b: f64) -> f64 {
        send fmin(a, b);
    }

    craft max_f64(a: f64, b: f64) -> f64 {
        send fmax(a, b);
    }

    craft sqrt_f64(x: f64) -> f64 {
        send sqrt(x);
    }

    craft add_checked_i32(a: i32, b: i32) -> Result(i32, i32) {
        send ql_add_checked_i32(a, b);
    }

    craft sub_checked_i32(a: i32, b: i32) -> Result(i32, i32) {
        send ql_sub_checked_i32(a, b);
    }

    craft mul_checked_i32(a: i32, b: i32) -> Result(i32, i32) {
        send ql_mul_checked_i32(a, b);
    }
}
