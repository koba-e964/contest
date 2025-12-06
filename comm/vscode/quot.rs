fn quot(a: i64, b: i64) -> i64 {
    assert!(b > 0);
    let mut q = a / b;
    let r = a - q * b;
    if r < 0 {
        q -= 1;
    }
    q
}
