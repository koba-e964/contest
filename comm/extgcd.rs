fn extgcd(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        return (x, 1, 0);
    }
    let r = x % y;
    let q = x / y;
    let (g, a, b) = extgcd(y, r);
    (g, b, a - b * q)
}
