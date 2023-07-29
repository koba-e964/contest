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

fn calc(
    mut rec1: Vec<((usize, usize), (usize, usize), usize)>,
    mut rec2: Vec<((usize, usize), (usize, usize), usize)>,
    ans: &mut [i32],
) {
    rec1.sort();
    rec2.sort();
    for &(a, b, idx) in &rec1 {
        let hi = rec2.lower_bound(&((b.0, 0), (0, 0), 0));
        for i in 0..hi {
            let (c, d, _) = rec2[i];
            if a.0 < d.0 && c.0 < b.0 && a.1 < d.1 && c.1 < b.1 {
                ans[idx] += 1;
            }
        }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        cub: [[[usize; 3]; 2]; n],
    }
    let mut ans = vec![0i32; n];
    for dim in 0..3 {
        let mut hm1 = HashMap::new();
        let mut hm2 = HashMap::new();
        for i in 0..n {
            for j in 0..2 {
                let pos = cub[i][j][dim];
                let a = (cub[i][0][(dim + 1) % 3], cub[i][0][(dim + 2) % 3]);
                let b = (cub[i][1][(dim + 1) % 3], cub[i][1][(dim + 2) % 3]);
                if j == 0 {
                    hm1.entry(pos).or_insert(vec![]).push((a, b, i));
                } else {
                    hm2.entry(pos).or_insert(vec![]).push((a, b, i));
                }
            }
        }
        for (k, v) in hm1 {
            if let Some(v2) = hm2.get(&k) {
                calc(v.clone(), v2.clone(), &mut ans);
                calc(v2.clone(), v, &mut ans);
            }
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
