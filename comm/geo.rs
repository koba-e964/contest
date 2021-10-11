type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn inn((ax, ay): P, (bx, by): P) -> Coord {
    ax * bx + ay * by
}

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
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
