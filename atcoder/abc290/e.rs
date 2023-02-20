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
    input! {
        n: usize,
        a: [usize1; n],
    }
    let mut occ = vec![vec![]; n];
    for i in 0..n {
        occ[a[i]].push(i);
    }
    let nn = n as i64;
    let mut tot = 0;
    for i in 0..nn {
        let x = min(i + 1, nn - i - 1);
        tot += x * (x + 1) / 2;
        if i + 1 <= nn - x {
            tot += (nn - x - i - 1) * (i + 1);
        }
    }
    for i in 0..n {
        if occ[i].is_empty() {
            continue;
        }
        let m = occ[i].len();
        let mut acc = vec![0; m + 1];
        for j in 0..m {
            acc[j + 1] = acc[j] + occ[i][j] as i64;
        }
        for j in 1..m {
            let idx = occ[i][..j].lower_bound(&(n - occ[i][j] - 1));
            tot -= acc[idx] + idx as i64;
            tot -= (n - occ[i][j]) as i64 * (j - idx) as i64;
        }
    }
    println!("{}", tot);
}
