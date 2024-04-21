use std::cmp::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        tws: [(i64, i64, i64); m],
    }
    let mut que = BinaryHeap::new();
    for (t, w, s) in tws {
        que.push((Reverse(t), 0, w, s));
    }
    let mut rem = (0..n).collect::<BTreeSet<_>>();
    let mut ans = vec![0; n];
    while let Some((Reverse(t), ty, w, s)) = que.pop() {
        if ty == 0 {
            if let Some(&i) = rem.range(..).next() {
                rem.remove(&i);
                ans[i] += w;
                que.push((Reverse(t + s), 1, i as i64, 0));
            }
        } else {
            rem.insert(w as usize);
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
