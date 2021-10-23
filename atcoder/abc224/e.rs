use std::cmp::*;
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize, n: usize,
        rca: [(usize1, usize1, i64); n],
    }
    let mut row = vec![-1; h];
    let mut col = vec![-1; w];
    let mut coo = vec![];
    for &(_r, _c, a) in &rca {
        coo.push(a);
    }
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut pop = vec![vec![]; m];
    for i in 0..n {
        let (r, c, a) = rca[i];
        let a = coo.binary_search(&a).unwrap();
        pop[a].push((r, c, i));
    }
    let mut ans = vec![-1; n];
    for i in (0..m).rev() {
        for &(r, c, idx) in &pop[i] {
            ans[idx] = max(row[r], col[c]) + 1;
        }
        for &(r, c, idx) in &pop[i] {
            row[r] = max(row[r], ans[idx]);
            col[c] = max(col[c], ans[idx]);
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
