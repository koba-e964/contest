use std::cmp::*;
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

fn main() {
    input! {
        h: usize, w: usize, h1: usize, w1: usize, h2: usize, w2: usize,
        a: [[i64; w]; h],
    }
    let h2 = min(h1, h2);
    let w2 = min(w1, w2);
    let mut acc = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j] + a[i][j];
        }
    }
    let mut sm = vec![vec![0; w - w2 + 1]; h - h2 + 1];
    for i in 0..h - h2 + 1 {
        for j in 0..w - w2 + 1 {
            let s = acc[i + h2][j + w2] - acc[i][j + w2] - acc[i + h2][j] + acc[i][j];
            sm[i][j] = s;
        }
    }
    let mut tmp = vec![vec![0; h - h2 + 1]; w - w1 + 1];
    let mut big = vec![vec![0; w - w1 + 1]; h - h1 + 1];
    for i in 0..h - h2 + 1 {
        let a = &sm[i];
        let rmq = TwoBITs::new(&a, max);
        for j in 0..w - w1 + 1 {
            tmp[j][i] = rmq.query(j, j + w1 - w2 + 1);
        }
    }
    let mut ans = 0;
    for j in 0..w - w1 + 1 {
        let a = &tmp[j];
        let rmq = TwoBITs::new(&a, max);
        for i in 0..h - h1 + 1 {
            big[i][j] = rmq.query(i, i + h1 - h2 + 1);
            let s = acc[i + h1][j + w1] - acc[i][j + w1] - acc[i + h1][j] + acc[i][j];
            ans = max(ans, s - big[i][j]);
        }
    }
    println!("{}", ans);
}
