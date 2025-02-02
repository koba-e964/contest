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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, w: usize,
        xy: [(usize1, i64); n],
        q: usize,
        ta: [(i64, usize1); q],
    }
    let mut occ = vec![vec![]; w];
    for i in 0..n {
        let (x, y) = xy[i];
        occ[x].push((y, i));
    }
    let mut index = vec![1 << 30; n];
    let mut mi = 1 << 30;
    for i in 0..w {
        occ[i].sort_unstable();
        mi = mi.min(occ[i].len());
        for j in 0..occ[i].len() {
            index[occ[i][j].1] = j;
        }
    }
    let mut disap = vec![0; mi];
    for i in 0..mi {
        let mut tmp = 0;
        for j in 0..w {
            tmp = tmp.max(occ[j][i].0);
        }
        disap[i] = if i == 0 {
            tmp
        } else {
            tmp.max(disap[i - 1] + 1)
        };
    }
    eprintln!("{:?}", disap);
    for (t, a) in ta {
        let idx = disap.upper_bound(&t);
        puts!("{}\n", if index[a] >= idx { "Yes" } else { "No" });
    }
}
