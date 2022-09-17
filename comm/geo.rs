type Coord = i64; // the type of coordinates
type P<C = Coord> = (C, C);

fn inn<C: std::ops::Add<Output = C> + std::ops::Mul<Output = C>>((ax, ay): P<C>, (bx, by): P<C>) -> C {
    ax * bx + ay * by
}

fn det<C: std::ops::Sub<Output = C> + std::ops::Mul<Output = C>>((ax, ay): P<C>, (bx, by): P<C>) -> C {
    ax * by - ay * bx
}

fn sub<C: std::ops::Sub<Output = C>>((ax, ay): P<C>, (bx, by): P<C>) -> P<C> {
    (ax - bx, ay - by)
}

// lines

// (a, b, c) represents ax + by = c
// (a, b) > (0, 0)
type Line = (Coord, Coord, Coord);

fn gcd(mut x: Coord, mut y: Coord) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn perpendicular_bisector((x1, y1): P, (x2, y2): P) -> Line {
    let x = x1 + x2;
    let y = y1 + y2;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut a = 2 * dx;
    let mut b = 2 * dy;
    let mut c = dx * x + dy * y;
    let g = gcd(gcd(a, b), c);
    a /= g;
    b /= g;
    c /= g;
    if (a, b) < (0, 0) {
        a = -a;
        b = -b;
        c = -c;
    }
    // ax + by = c, (a, b) > (0, 0)
    (a, b, c)
}

fn line((x1, y1): P, (x2, y2): P) -> Line {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut a = -dy;
    let mut b = dx;
    let mut c = -dy * x1 + dx * y1;
    let g = gcd(gcd(a, b), c);
    a /= g;
    b /= g;
    c /= g;
    if (a, b) < (0, 0) {
        a = -a;
        b = -b;
        c = -c;
    }
    // ax + by = c, (a, b) > (0, 0)
    (a, b, c)
}

// arg sort
// Verified by: https://yukicoder.me/submissions/706856
fn arg_sort(xy: &mut [(Coord, Coord, i32)]) {
    fn half((x, y, _): (Coord, Coord, i32)) -> i32 {
        assert_ne!((x, y), (0, 0));
        if y >= 0 {
            if x > 0 || y > 0 {
                1
            } else {
                2
            }
        } else {
            2
        }
    }
    xy.sort_unstable_by(|&a, &b| {
        half(a).cmp(&half(b)).then_with(
            || 0.cmp(&(a.0 * b.1 - a.1 * b.0))
                .then_with(|| a.2.cmp(&b.2)))
    });
}

// segments

type Segment<C = Coord> = (P<C>, P<C>);

// Verified by: https://atcoder.jp/contests/math-and-algorithm/submissions/34916093
fn nearest(a: P<f64>, (b, c): Segment<f64>) -> P<f64> {
    let bc = sub(b, c);
    let ac = sub(a, c);
    let x = inn(bc, bc);
    let y = inn(bc, ac);
    if y < 0.0 {
        c
    } else if y > x {
        b
    } else {
        let r = y as f64 / x as f64;
        (c.0 * (1.0 - r) + b.0 * r, c.1 * (1.0 - r) + b.1 * r)
    }
}

fn ccw(a: P<i64>, b: P<i64>, c: P<i64>) -> i8 {
    let b = sub(b, a);
    let c = sub(c, a);
    if det(b, c) > 0 {
        return 1;
    }
    if det(b, c) < 0 {
        return -1;
    }
    if inn(b, c) < 0 {
        return 2;
    }
    if inn(b, c) < inn(c, c) {
        return -2;
    }
    0
}

// Verified by: https://atcoder.jp/contests/math-and-algorithm/submissions/34916560
fn intersect(s: Segment<i64>, t: Segment<i64>) -> bool {
    ccw(s.0, s.1, t.0) * ccw(s.0, s.1, t.1) <= 0 &&
        ccw(t.0, t.1, s.0) * ccw(t.0, t.1, s.1) <= 0
}
