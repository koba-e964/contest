// Convex hull.
// Returns the points of the convex hull in the counter-clockwise order.
// Verified by: AGC021 B
// https://atcoder.jp/contests/agc021/submissions/22697344
// QUPC 2014 G
// https://atcoder.jp/contests/qupc2014/submissions/23493247
type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}

fn convex_hull(ps: &[P]) -> Vec<P> {
    let n = ps.len();
    if n == 0 {
        return vec![];
    }
    let mut k = 0;
    let mut ps = ps.to_vec();
    ps.sort();
    let mut ch = vec![(0, 0); 2 * n];
    // lower
    for i in 0..n {
        while k >= 2 &&
            det(sub(ps[i], ch[k - 2]), sub(ch[k - 1], ch[k - 2])) >= 0 {
            k -= 1;
        }
        ch[k] = ps[i];
        k += 1;
    }
    // upper
    let t = k + 1;
    for i in (0..n - 1).rev() {
        while k >= t &&
            det(sub(ps[i], ch[k - 2]), sub(ch[k - 1], ch[k - 2])) >= 0 {
            k -= 1;
        }
        ch[k] = ps[i];
        k += 1;
    }
    ch.truncate(k - 1);
    ch
}
