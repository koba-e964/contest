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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        ab: [(usize1, usize1); n],
    }
    let mut g = vec![BTreeSet::new(); n];
    let mut act = vec![true; n];
    for i in 0..n {
        let (a, b) = ab[i];
        g[a].insert((b, i));
        g[b].insert((a, n + i));
    }
    let mut que = vec![];
    for i in 0..n {
        if g[i].len() == 1 {
            que.push(i);
        }
    }
    let mut ori = vec![""; n];
    while let Some(v) = que.pop() {
        act[v] = false;
        let &(to, eidx) = g[v].iter().next().unwrap();
        ori[eidx % n] = if eidx >= n { "<-" } else { "->" };
        g[to].remove(&(v, (eidx + n) % (2 * n)));
        if g[to].len() == 1 {
            que.push(to);
        }
    }
    // One cycle remains.
    let mut st = 0;
    for i in 0..n {
        if act[i] {
            st = i;
            break;
        }
    }
    let &(to, eidx) = g[st].iter().next().unwrap();
    g[st].remove(&(to, eidx));
    g[to].remove(&(st, (eidx + n) % (2 * n)));
    ori[eidx % n] = if eidx >= n { "<-" } else { "->" };
    que.push(to);
    while let Some(v) = que.pop() {
        act[v] = false;
        let &(to, eidx) = g[v].iter().next().unwrap();
        ori[eidx % n] = if eidx >= n { "<-" } else { "->" };
        g[to].remove(&(v, (eidx + n) % (2 * n)));
        if g[to].len() == 1 {
            que.push(to);
        }
    }
    for i in 0..n {
        puts!("{}\n", ori[i]);
    }
}
