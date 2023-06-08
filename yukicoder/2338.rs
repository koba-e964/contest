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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
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

// https://yukicoder.me/problems/no/2338 (3.5)
// Mo's algorithm を使う。各問題に対し、今着目している範囲内で AC したかどうか、AC 直前の WA 数を高速に更新できれば良いので、
// 問題ごとに AC: 1, WA: 0 とした配列の区間和をとり、二分探索などでこれらの値を求めることにする。
// -> TLE したので定数倍高速化 (lower_bound の範囲を限定) して AC。
// Tags: mos-algorithm
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        ps: [(usize1, String); n],
        lr: [(usize1, usize); q],
    }
    let mut acc = vec![vec![0]; m];
    // state
    let mut lft = vec![0; m];
    let mut rgt = vec![0; m];

    for i in 0..n {
        let (p, ref s) = ps[i];
        let s = if s == "AC" { 1 } else { 0 };
        let last = acc[p][acc[p].len() - 1];
        acc[p].push(last + s);
    }

    const B: usize = 500;
    let mut lri: Vec<_> = (0..q).map(|i| {
        let (l, r) = lr[i];
        (l, r, i)
    }).collect();
    lri.sort_by_key(|&(l, r, _idx)| {
        let q = l / B;
        if q % 2 == 1 {
            (q, n - r)
        } else {
            (q, r)
        }
    });
    let mut ans = vec![(0, 0); q];

    // pointer
    let mut cl = 0;
    let mut cr = 0;

    // state
    let mut ac = 0;
    let mut pen = 0;

    macro_rules! add {
        ($v:expr, $coef:expr) => {
            let idx = $v;
            let (x, y) = if acc[idx][rgt[idx]] != acc[idx][lft[idx]] {
                let pos = acc[idx][lft[idx]..rgt[idx]].lower_bound(&(acc[idx][lft[idx]] + 1));
                (1, pos as i32 - 1)
            } else {
                (0, 0)
            };
            ac += x * $coef;
            pen += y * $coef;
        }
    }

    for &(l, r, idx) in &lri {
        while cr < r {
            let idx = ps[cr].0;
            add!(idx, -1);
            rgt[idx] += 1;
            add!(idx, 1);
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            let idx = ps[cl].0;
            add!(idx, -1);
            lft[idx] -= 1;
            add!(idx, 1);
        }
        while cr > r {
            cr -= 1;
            let idx = ps[cr].0;
            add!(idx, -1);
            rgt[idx] -= 1;
            add!(idx, 1);
        }
        while cl < l {
            let idx = ps[cl].0;
            add!(idx, -1);
            lft[idx] += 1;
            add!(idx, 1);
            cl += 1;
        }
        ans[idx] = (ac, pen);
    }
    for a in ans {
        puts!("{} {}\n", a.0, a.1);
    }
}
