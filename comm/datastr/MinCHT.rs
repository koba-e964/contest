// https://github.com/kth-competitive-programming/kactl/blob/main/content/data-structures/LineContainer.h
// Verified by: https://judge.yosupo.jp/submission/63195
type Coord = i64;

#[derive(Clone, Debug, Default)]
struct MinCHT {
    lines: std::collections::BTreeSet<Entry>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point(Coord);

#[derive(Eq, Debug, Clone)]
struct Entry(Coord, Coord, std::cell::Cell<Point>);
impl PartialEq for Entry {
    fn eq(&self, o: &Self) -> bool {
        (self.0, self.1) == (o.0, o.1)
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        (o.0, o.1).partial_cmp(&(self.0, self.1))
    }
}
impl Ord for Entry {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        (o.0, o.1).cmp(&(self.0, self.1))
    }
}
impl std::borrow::Borrow<Point> for Entry {
    fn borrow(&self) -> &Point {
        unsafe { &*self.2.as_ptr() }
    }
}

impl MinCHT {
    const INF: i64 = 1 << 62;
    fn div(a: Coord, b: Coord) -> Coord {
        a / b - if (a ^ b) < 0 && a % b != 0 { 1 } else { 0 }
    }
    // Should we erase y from lines?
    // As well as modifying x.2 to the appropriate value
    fn isect(x: &Entry, y: Option<&Entry>) -> bool {
        let y = if let Some(y) = y {
            y
        } else {
            x.2.set(Point(Self::INF));
            return false;
        };
        assert!(!std::ptr::eq(&x.2, &y.2));
        if x.0 == y.0 {
            let p = if x.1 < y.1 {
                Self::INF
            } else {
                -Self::INF
            };
            x.2.set(Point(p));
        } else {
            x.2.set(Point(Self::div(y.1 - x.1, x.0 - y.0)));
        }
        x.2 >= y.2
    }
    fn new() -> Self {
        Default::default()
    }
    // Adds y = ax + b
    fn add(&mut self, a: Coord, b: Coord) {
        let e = Entry(a, b, std::cell::Cell::new(Point(0)));
        if self.lines.contains(&e) {
            return;
        }
        self.lines.insert(e.clone());
        loop {
            let y = self.lines.get(&e).unwrap();
            let z = self.lines.range(Entry(a, b - 1, std::cell::Cell::new(Point(0)))..).next();
            if Self::isect(y, z) {
                let z = z.unwrap().clone();
                self.lines.remove(&z);
            } else {
                break;
            }
        }
        let mut now;
        {
            let y = self.lines.range(e.clone()..).next();
            let x = self.lines.range(..e.clone()).rev().next();
            if let Some(x) = x {
                now = x.clone();
                if Self::isect(x, y) {
                    let y = y.unwrap().clone();
                    self.lines.remove(&y);
                    let xx = self.lines.range(..e.clone()).rev().next().unwrap();
                    let yy = self.lines.range(e.clone()..).next();
                    Self::isect(xx, yy);
                }
            } else {
                return;
            }
        }
        loop {
            let y = self.lines.range(now.clone()..).next();
            let x = self.lines.range(..now.clone()).rev().next();
            if let Some(x) = x {
                if Self::isect(x, y) {
                    let x = x.clone();
                    let y = y.unwrap().clone();
                    self.lines.remove(&y);
                    let xx = self.lines.range(..now.clone()).rev().next().unwrap();
                    let yy = self.lines.range(now.clone()..).next();
                    Self::isect(xx, yy);
                    now = x;
                    continue;
                }
            }
            break;
        }
    }
    fn query(&self, x: Coord) -> Coord {
        assert!(!self.lines.is_empty());
        let &Entry(a, b, _) = self.lines.range(Point(x)..).next().unwrap();
        a * x + b
    }
}
