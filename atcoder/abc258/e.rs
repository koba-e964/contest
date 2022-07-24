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

const B: usize = 40;

fn calc(bin: &[Vec<(i64, usize)>], k: i64) -> i64 {
    let (mut x, mut y) = (0, 0);
    for i in 0..B {
        if (k & 1 << i) != 0 {
            let (z, w) = bin[i][y];
            x += z;
            y = w;
        }
    }
    x * bin[0].len() as i64 + y as i64
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize, x: i64,
        w: [i64; n],
        k: [i64; q],
    }
    let ws: i64 = w.iter().sum();
    let mut wacc = vec![0; n + 1];
    for i in 0..n {
        wacc[i + 1] = wacc[i] + w[i];
    }
    let mut bin = vec![vec![(0, 0); n]; B];
    for i in 0..n {
        let q = (x + wacc[i]) / ws;
        let rem = (x + wacc[i]) % ws;
        let idx = wacc.lower_bound(&rem);
        if idx >= n {
            bin[0][i] = (q + 1, 0);
        } else {
            bin[0][i] = (q, idx);
        }
    }
    for i in 0..B - 1 {
        for j in 0..n {
            let (x, y) = bin[i][j];
            let (z, w) = bin[i][y];
            bin[i + 1][j] = (x + z, w);
        }
    }
    for k in k {
        let l1 = calc(&bin, k - 1);
        let l2 = calc(&bin, k);
        puts!("{}\n", l2 - l1);
    }
}
