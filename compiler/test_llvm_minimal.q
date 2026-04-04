// Minimal program for LLVM backend testing
// Run: cargo build --features llvm && qpp build compiler/test_llvm_minimal.q --backend llvm
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}

craft main() -> void {
    make x: i32 = add(1, 2);
    send;
}
