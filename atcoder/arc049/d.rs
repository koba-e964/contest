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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: data-structure, lazy-segment-trees
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        tab: [(i32, usize, usize); q],
    }
    let mut lazy = vec![0; 1 << n];
    for (t, a, b) in tab {
        if t == 1 {
            let a = a - 1;
            let mut c = 1;
            let mut acc = 0;
            for i in 0..n {
                acc ^= lazy[c];
                if ((a & 1 << (n - 1 - i)) != 0) ^ ((acc & 1 << i) != 0) {
                    c = 2 * c + 1;
                } else {
                    c = 2 * c;
                }
            }
            puts!("{}\n", c + 1 - (1 << n));
            continue;
        }
        let mut st = vec![];
        let b = b + 1;
        for i in (0..n).rev() {
            let a = max(a, 1 << i);
            let b = min(b, 2 << i);
            st.push((a, b, 1, 1 << i, 2 << i, i, 0, 0));
        }
        while let Some((a, b, c, l, r, dep, acc, pos)) = st.pop() {
            if a >= b { continue; }
            if a <= l && b >= r {
                lazy[c] ^= 1 << dep;
                continue;
            }
            assert!(r - l >= 2);
            let mid = (l + r) / 2;
            let newacc = acc ^ lazy[c];
            let flip = (newacc & 1 << pos) != 0;
            st.push((a, min(mid, b), 2 * c + if flip { 1 } else { 0 }, l, mid, dep, newacc, pos + 1));
            st.push((max(a, mid), b, 2 * c + if flip { 0 } else { 1 }, mid, r, dep, newacc, pos + 1));
        }
    }
}
