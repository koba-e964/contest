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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: usize,
        p: [usize1; n],
    }
    let mut tip = BTreeSet::new();
    let mut len = vec![0; n];
    let mut nxt = vec![0; n];
    let mut eaten = vec![-1; n];
    for i in 0..n {
        let v = p[i];
        let t = tip.range(v..).next();
        if let Some(&t) = t {
            nxt[v] = t;
            len[v] = len[t] + 1;
            tip.remove(&t);
            if len[v] == k {
                // eaten
                let mut c = v;
                for _ in 0..k {
                    eaten[c] = (i + 1) as i32;
                    c = nxt[c];
                }
            } else {
                tip.insert(v);
            }
        } else {
            if k == 1 {
                eaten[v] = (i + 1) as i32;
            } else {
                tip.insert(v);
                len[v] = 1;
            }
        }
    }
    for i in 0..n {
        puts!("{}\n", eaten[i]);
    }
}
