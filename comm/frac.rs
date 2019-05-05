// Verified by https://atcoder.jp/contests/cpsco2019-s3/submissions/5279150
#[derive(Clone, Copy, PartialEq, Eq)]
struct Frac {
    x: i64, y: i64,
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y; y = r;
    }
    x.abs()
}

#[allow(unused)]
impl Frac {
    fn new() -> Self {
        Frac {x: 0, y: 1}
    }
    fn add(self, o: Self) -> Self {
        Frac { x: self.x * o.y + self.y * o.x, y: o.y * self.y }.red()
    }
    fn sub(self, o: Self) -> Self {
        Frac { x: self.x * o.y - self.y * o.x, y: o.y * self.y }.red()
    }
    fn mul(self, o: Self) -> Self {
        Frac { x: self.x * o.x, y: self.y * o.y }.red()
    }
    fn div(self, o: Self) -> Self {
        Frac { x: self.x * o.y, y: self.y * o.x }.red()
    }
    fn neg(self) -> Self {
        Frac {x: -self.x, y: self.y }
    }
    fn red(mut self) -> Self {
        let g = gcd(self.x, self.y);
        self.x /= g;
        self.y /= g;
        if self.y < 0 {
            Frac {x: -self.x, y: -self.y }
        } else {
            self
        }
    }
}

fn frac(x: i64, y: i64) -> Frac {
    Frac {x: x, y: y}.red()
}

impl Ord for Frac {
    fn cmp(&self, o: &Frac) -> std::cmp::Ordering {
        (self.x * o.y).cmp(&(self.y * o.x))
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, o: &Frac) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl std::fmt::Debug for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}
impl std::fmt::Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}
