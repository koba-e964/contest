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

/// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
/// T is a commutative monoid. Indices are 1 .. n.
/// Verified by yukicoder No.404 (http://yukicoder.me/submissions/155373)
struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    /**
     * gets the sum in [1 .. idx]
     * @param idx
     * @return sum
     */
    fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    /**
     * performs data[idx] += val;
     */
    fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        assert!(idx > 0);
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Make sure that 1 <= idx <= n.
    #[allow(unused)]
    fn single(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(idx) - self.accum(idx - 1)
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        qs: [(i32, usize, usize); q],
    }
    let mut bit = BIT::new(n, 0);
    for i in 0..n {
        bit.add(i + 1, a[i]);
    }
    for (ty, l, r) in qs {
        if ty == 0 {
            bit.add(l + 1, r as i64);
        } else {
            puts!("{}\n", bit.accum(r) - bit.accum(l));
        }
    }
}
