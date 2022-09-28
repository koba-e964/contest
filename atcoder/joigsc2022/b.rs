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

fn main() {
    input! {
        n: usize,
        ab: [(i64, char); n],
        q: usize,
        st: [(i64, i64); q],
    }
    let mut tbl = vec![vec![]; 4];
    for (a, b) in ab {
        let idx = ['J', 'O', 'I', 'G'].iter().position(|&x| x == b).unwrap();
        tbl[idx].push(a);
    }
    for (s, t) in st {
        let mut mi = 1i64 << 60;
        'outer:
        for bits in 0..16 {
            let mut tot = 0;
            let mut cur = s;
            for i in 0..4 {
                let idx = tbl[i].lower_bound(&cur);
                let p = if (bits & 1 << i) == 0 {
                    if idx >= tbl[i].len() {
                        continue 'outer;
                    } else {
                        tbl[i][idx]
                    }
                } else {
                    if idx == 0 {
                        continue 'outer;
                    } else {
                        tbl[i][idx - 1]
                    }
                };
                tot += (cur - p).abs();
                cur = p;
            }
            mi = min(mi, tot + (cur - t).abs());
        }
        println!("{}", mi);
    }
}
