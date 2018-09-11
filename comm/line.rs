// Verified by http://codeforces.com/contest/1036/submission/42744677
// https://creasys.org/rambo/articles/907427cf7cb938c28123
fn ccw((x1, y1): (i64, i64), (x2, y2): (i64, i64), (x3, y3): (i64, i64))
       -> i32 {
    use std::cmp::Ordering;
    let dx2 = x2 - x1;
    let dy2 = y2 - y1;
    let dx3 = x3 - x1;
    let dy3 = y3 - y1;
    match (dx2 * dy3).cmp(&(dx3 * dy2)) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => {
            if dx2 * dx3 < 0 || dy2 * dy3 < 0 {
                return -1;
            }
            if dx2 * dx2 + dy2 * dy2 < dx3 * dx3 + dy3 * dy3 {
                return 1;
            }
            0
        },
    }
}

/// Segments are given by their endpoints.
fn intersect((p11, p12): ((i64, i64), (i64, i64)), (p21, p22): ((i64, i64), (i64, i64)))
             -> Option<(i64, i64)> {
    if ccw(p11, p12, p21) * ccw(p11, p12, p22) <= 0
        && ccw(p21, p22, p11) * ccw(p21, p22, p12) <= 0 {
            let (a00, a01, a10, a11) = (p12.0 - p11.0, p22.0 - p21.0,
                                        p12.1 - p11.1, p22.1 - p21.1);
            let (v0, v1) = (p21.0 - p11.0, p21.1 - p11.1);
            let (s_num, s_den) = (v0 * a11 - v1 * a01, a00 * a11 - a10 * a01);
            if s_num * a00 % s_den == 0 && s_num * a10 % s_den == 0 {
                Some((p11.0 + s_num * a00 / s_den, p11.1 + s_num * a10 / s_den))
            } else {
                None
            }
        } else {
            None
        }
}
