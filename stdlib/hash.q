// Q++ standard library: Hashing
// FNV-1a and djb2 via C runtime

extern craft ql_hash_fnv1a(ptr: link void, len: usize) -> u64;
extern craft ql_hash_djb2(ptr: link void, len: usize) -> u64;

realm hash {
    craft fnv1a(s: str, len: usize) -> u64 {
        send ql_hash_fnv1a(s, len);
    }

    craft fnv1a_str(s: str) -> u64 {
        send ql_hash_fnv1a(s, strlen(s));
    }

    craft djb2(s: str, len: usize) -> u64 {
        send ql_hash_djb2(s, len);
    }

    craft djb2_str(s: str) -> u64 {
        send ql_hash_djb2(s, strlen(s));
    }
}
