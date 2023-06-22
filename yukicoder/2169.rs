use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

// https://yukicoder.me/problems/no/2169 (3.5)
// A の階差数列 B = (A[2] - A[1], ..., A[N] - A[N - 1]) に対する以下のような操作とみなすことができる。
// 1. どこかに 1 を足す
// 2. どこかに -1 を足す
// 3. 0 <= i < j <= N - 2 に対し、 B[i] += 1, B[j] -= 1 を行う
// 最終的な目標は、B の要素がすべて d[i] に等しいことである。
// これの答えは、\sum_{j} max(0, -(B[j] - d[i])) + (B[j] - d[i] の累積和の max) である。
// 証明: d[i] = 0 としてよい。B の全ての要素を 0 にするための操作回数が \sum_{j} max(0, -B[j]) + (B[j] の累積和の max) であることを示す。
// B[0] > 0 のとき、操作 2 を行うと第 2 項が 1 減る。 B[0] < 0 のとき、第 2 項が max になるような和を 0..x とすれば、
// x = 0 であるか B[x-1] > 0 であるかのいずれかである。それぞれ操作 1、操作 3 を行えば第 2 項が変わらずに第 1 項が減る。(証明終)
// 第 1 項は B をソートしておけば O(Q log N)-time で、第 2 項は CHT で O(Q log N)-time 計算できる。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        d: [i64; q],
    }
    let mut cht = MinCHT::new();
    for i in 0..n {
        cht.add(i as i64, -(a[i] - a[0]));
    }
    let mut b = vec![0; n - 1];
    for i in 0..n - 1 {
        b[i] = a[i + 1] - a[i];
    }
    b.sort();
    let mut bacc = vec![0; n];
    for i in 0..n - 1 {
        bacc[i + 1] = bacc[i] + b[i];
    }
    for d in d {
        let idx = b.lower_bound(&d);
        let term1 = idx as i64 * d - bacc[idx];
        let term2 = -cht.query(d);
        puts!("{}\n", term1 + term2);
    }
}
