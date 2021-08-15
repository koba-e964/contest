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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let st = TwoBITs::new(&a, std::cmp::min);
    for (l, r) in lr {
        puts!("{}\n", st.query(l, r));
    }
}
