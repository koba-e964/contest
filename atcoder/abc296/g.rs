use std::cmp::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

type Coord = i64;

fn cmp(a: (Coord, Coord), b: (Coord, Coord)) -> Ordering {
    fn half((x, y): (Coord, Coord)) -> i32 {
        assert_ne!((x, y), (0, 0));
        if y >= 0 {
            if x > 0 || y > 0 {
                1
            } else {
                0
            }
        } else {
            0
        }
    }
    half(a).cmp(&half(b)).then_with(
        || 0.cmp(&(a.0 * b.1 - a.1 * b.0)))
}

// The author read the editorial before implementing this.
// どう頑張っても「ある点が特定の角度の内部にある」かどうか判定する処理は書かないとならないので、
// 横着せずに書く。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        xy: [(i64, i64); n],
        q: usize,
        ab: [(i64, i64); q],
    }
    let mut mi = (xy[0].1, xy[0].0, 0);
    let mut ans = vec![""; q];
    for i in 0..n {
        let (x, y) = xy[i];
        mi = min(mi, (y, x, i));
    }
    let (y0, x0, idx0) = mi;
    let mut s = vec![];
    for i in 1..n {
        let (x, y) = xy[(i + idx0) % n];
        s.push((x - x0, y - y0));
    }
    for i in 0..q {
        let (x, y) = ab[i];
        if (x, y) == (x0, y0) {
            ans[i] = "ON";
            continue;
        }
        let p = (x - x0, y - y0);
        if cmp(p, s[0]) == Ordering::Less || cmp(p, s[n - 2]) == Ordering::Greater {
            ans[i] = "OUT";
            continue;
        }
        let mut pass = 0;
        let mut fail = n - 2;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if cmp(p, s[mid]) != Ordering::Less {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        assert_ne!(cmp(p, s[pass]), Ordering::Less);
        let idx = min(pass, n - 3);
        let (x, y) = p;
        let (a, b) = s[idx];
        let (c, d) = s[idx + 1];
        let outer = (c - a) * (y - b) - (d - b) * (x - a);
        if idx == 0 {
            if cmp(s[0], p) == Ordering::Equal {
                ans[i] = if a * a + b * b >= x * x + y * y {
                    "ON"
                } else {
                    "OUT"
                };
                continue;
            }
        }
        if idx == n - 3 {
            if cmp(s[n - 2], p) == Ordering::Equal {
                ans[i] = if d >= y {
                    "ON"
                } else {
                    "OUT"
                };
                continue;
            }
        }
        if outer == 0 {
            ans[i] = "ON";
        } else if outer > 0 {
            ans[i] = "IN";
        } else {
            ans[i] = "OUT";
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
