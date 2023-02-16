use std::cmp::*;
use std::collections::*;
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

// Ref: https://judge.yosupo.jp/submission/19062
struct TwoBITs<T, F> {
    a: Vec<T>,
    bit1: Vec<T>,
    bit2: Vec<T>,
    f: F,
}

#[inline]
fn lsb(x: usize) -> usize {
    x & x.wrapping_neg()
}

impl<T: Copy + Default, F: Fn(T, T) -> T> TwoBITs<T, F> {
    fn new(a: &[T], f: F) -> Self {
        let n = a.len();
        let mut bit1 = vec![T::default(); n + 1];
        let mut bit2 = vec![T::default(); n + 1];
        bit1[1..].copy_from_slice(&a);
        bit2[1..].copy_from_slice(&a);
        for i in 1..n + 1 {
            let b = lsb(i);
            if i + b <= n {
                bit1[i + b] = f(bit1[i + b], bit1[i]);
            }
        }
        for i in (1..n + 1).rev() {
            let b = lsb(i);
            bit2[i - b] = f(bit2[i - b], bit2[i]);
        }
        TwoBITs {
            a: a.to_vec(),
            bit1: bit1,
            bit2: bit2,
            f: f,
        }
    }
    fn query(&self, l: usize, r: usize) -> T {
        if l == r {
            return self.a[l];
        }
        unsafe {
            let l = l + 1;
            let mut ans = (self.f)(
                *self.a.get_unchecked(l - 1),
                *self.a.get_unchecked(r - 1),
            );
            let mut x = l;
            while x + lsb(x) <= r + 1 {
                ans = (self.f)(ans, *self.bit2.get_unchecked(x));
                x += lsb(x);
            }
            let mut y = r;
            while y != 0 && y - lsb(y) + 1 >= l {
                ans = (self.f)(ans, *self.bit1.get_unchecked(y));
                y &= y - 1;
            }
            if x > r {
                ans
            } else {
                (self.f)(ans, *self.a.get_unchecked(x - 1))
            }
        }
    }
}

// Data structure that is similar to Slope Trick.
#[derive(Default)]
struct Slopes {
    gdiff: BTreeMap<i64, i64>, // Difference of gradients
    gtot: i64, // gradient at 0
}

impl Slopes {
    fn new() -> Self {
        Slopes::default()
    }
    // new(x) := min_{y >= 0, y >= pos - x} newgrad * y + self(x + y)
    // self <- new
    fn convolve(&mut self, pos: i64, newgrad: i64) {
        assert!(newgrad > 0);
        let mut last = 0;
        while !self.gdiff.is_empty() {
            let (&v, &diff) = self.gdiff.iter().next().unwrap();
            if self.gtot >= newgrad || v <= pos {
                last = v;
                self.gdiff.remove(&v);
                self.gtot -= diff;
            } else {
                break;
            }
        }
        *self.gdiff.entry(max(pos, last)).or_insert(0) += newgrad - self.gtot;
        self.gtot = newgrad;
    }
    // returns a(x) + b(x)
    fn merge(mut a: Self, mut b: Self) -> Self {
        if a.gdiff.len() < b.gdiff.len() {
            std::mem::swap(&mut a, &mut b);
        }
        for (v, diff) in b.gdiff {
            *a.gdiff.entry(v).or_insert(0) += diff;
        }
        a.gtot += b.gtot;
        a
    }
    fn at0(&self) -> i64 {
        let mut tot = 0;
        for (&v, &diff) in &self.gdiff {
            tot += v * diff;
        }
        tot
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn rec<G: Fn((i64, usize), (i64, usize)) -> (i64, usize)>(
    a: &[i64],
    stb: &TwoBITs<(i64, usize), G>,
    l: usize, r: usize,
) -> Slopes {
    if l >= r { return Slopes::new(); }
    let ma = stb.query(l, r).0;
    let mut x = r;
    let mut amax = 0;
    let mut recs = vec![];
    while x > l {
        let tmp = stb.query(l, x);
        if tmp.0 != ma {
            recs.push((l, x));
            break;
        }
        let idx = tmp.1;
        recs.push((idx + 1, x));
        amax = max(amax, a[idx]);
        x = idx;
    }
    let mut dp = Slopes::new();
    for (l, r) in recs {
        let sub = rec(a, stb, l, r);
        dp = Slopes::merge(dp, sub);
    }
    dp.convolve(amax, ma);
    dp
}

// Tags: slope-trick, weighted-union-heuristics, min-convolution
fn solve() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut bi = vec![];
    for i in 0..n {
        bi.push((b[i], i));
    }
    let stb = TwoBITs::new(&bi, std::cmp::max);
    println!("{}", rec(&a, &stb, 0, n).at0());
}
