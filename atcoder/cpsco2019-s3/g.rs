#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/*
a[i] の小さい人はあまりずらしてはいけない気がする
X = 2, 1 が 10 個と 10 が 1 個の場合は?
X/A = 0.1 なので、0.1, ..., 1 こ振り分けるのが理想。
0, ..., 2 の場合のコストは 10 * 0.1 + 0.1 = 1.1
1, 0, ..., 0, 1 の場合のコストは 0.9 + 9 * 0.1 + 0 = 1.8

- の人が何人か、+ の人が何人かで場合分け?

O(N^2) は自明だけど計算量を落とす方法がわからない
1 iteration:
x * a[i] / A を計算し、 round to nearest する。差分がわかるので差分を一番コストが低いものから順に埋める。

解説を見た。単にソートするだけ。座圧のようなテクニックで O(N log N)
 */
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
    fn cmp(&self, o: &Frac) -> Ordering {
        (self.x * o.y).cmp(&(self.y * o.x))
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, o: &Frac) -> Option<Ordering> {
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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, x: i64,
        a: [i64; n],
    }
    let asum: i64 = a.iter().sum();
    let mut actions = vec![];
    for i in 0..n {
        let count = (x * a[i]) / asum;
        actions.push((frac(-1, a[i]), n - i, count));
        let cross = frac(2 * count, a[i]).add(frac(1, a[i]))
            .add(frac(-2 * x, asum));
        actions.push((cross, n - i, 1));
        actions.push((frac(1, a[i]), n - i, x));
    }
    actions.sort();
    actions.reverse();

    let mut ans = vec![0; n];
    let mut rem = x;
    while rem > 0 {
        let (_prof, idx, c) = actions.pop().unwrap();
        let c = min(c, rem);
        let idx = n - idx;
        ans[idx] += c;
        rem -= c;
    }
    for a in ans {
        puts!("{}\n", a);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
