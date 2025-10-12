fn digitsum(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        ans += x % 10;
        x /= 10;
    }
    ans
}
