use std::collections::*;
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

struct BIT2D {
    n: i32,
    m: i32,
    dat: HashMap<(i32, i32), i64>,
}

impl BIT2D {
    pub fn new(n: i32, m: i32) -> Self {
        BIT2D {
            n: n,
            m: m,
            dat: HashMap::new(),
        }
    }
    pub fn add(&mut self, x: i32, y: i32, val: i64) {
        let mut x = x;
        while x <= self.n {
            let mut y = y;
            while y <= self.m {
                *self.dat.entry((x, y)).or_insert(0) += val;
                y += y & -y;
            }
            x += x & -x;
        }
    }
    pub fn sum(&self, mut xl: i32, mut xr: i32, y: i32) -> i64 {
        let mut sum = 0;
        let get = |x: i32| -> i64 {
            let mut y = y;
            let mut sum = 0;
            while y > 0 {
                sum += self.dat.get(&(x, y)).unwrap_or(&0);
                y -= y & -y;
            }
            sum
        };
        while xr != xl {
            if xr > xl {
                sum += get(xr);
                xr -= xr & -xr;
            } else {
                sum -= get(xl);
                xl -= xl & -xl;
            }
        }
        sum
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        abc: [(i64, i64, i64); q],
    }
    let mut occ = a.clone();
    occ.sort(); occ.dedup();
    let mut bit = BIT2D::new(n as i32, occ.len() as i32);
    for i in 0..n {
        let idx = occ.binary_search(&a[i]).unwrap();
        bit.add(i as i32 + 1, idx as i32 + 1, a[i]);
    }
    let mut g = 0;
    for (a, b, c) in abc {
        let l = (a ^ g) as i32;
        let r = (b ^ g) as i32;
        let x = c ^ g;
        let idx = occ.upper_bound(&x) as i32;
        g = bit.sum(l - 1, r, idx);
        puts!("{}\n", g);
    }
}
