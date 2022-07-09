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
        n: usize, k: usize, q: usize,
        ab: [(usize1, usize1); k],
        lrx: [(usize1, usize, usize1); q],
    }
    // We want x^p[r]^{p[l]^{-1}}. 
    let mut que = vec![vec![]; k + 1];
    for i in 0..q {
        let (l, r, x) = lrx[i];
        que[r].push((l, x, i));
    }
    let mut p: Vec<usize> = (0..n).collect();
    let mut invp: Vec<usize> = (0..n).collect();
    let mut que2 = vec![vec![]; k + 1];
    for i in 0..k {
        let (a, b) = ab[i];
        p.swap(a, b);
        for &(l, x, idx) in &que[i + 1] {
            que2[l].push((p[x], idx));
        }
    }
    p = (0..n).collect();
    let mut ans = vec![0; q];
    for i in 0..k {
        for &(x, idx) in &que2[i] {
            ans[idx] = invp[x];
        }
        let (a, b) = ab[i];
        p.swap(a, b);
        invp.swap(p[a], p[b]);
    }
    for v in ans {
        puts!("{}\n", v + 1);
    }
}
