// Q++ standard library: SIMD intrinsics (SSE)
// Use hazard { cblock { "#include <xmmintrin.h>" } } then call via extern
// Or use hazard { cblock { " __m128 x = _mm_loadu_ps(p); " } } for inline

extern craft _mm_loadu_ps(p: link f32) -> link void;
extern craft _mm_storeu_ps(p: link f32, a: link void) -> void;
extern craft _mm_add_ps(a: link void, b: link void) -> link void;
extern craft _mm_mul_ps(a: link void, b: link void) -> link void;

realm simd {
    craft loadu_ps(p: link f32) -> link void {
        send _mm_loadu_ps(p);
    }

    craft storeu_ps(p: link f32, a: link void) -> void {
        _mm_storeu_ps(p, a);
        send;
    }

    craft add_ps(a: link void, b: link void) -> link void {
        send _mm_add_ps(a, b);
    }

    craft mul_ps(a: link void, b: link void) -> link void {
        send _mm_mul_ps(a, b);
    }
}
