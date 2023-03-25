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
    ($next:expr, (switch, $t:tt, $u:tt )) => {{
        let ty = read_value!($next, i32);
        if ty == 1 {
            Ok(read_value!($next, $t))
        } else {
            Err(read_value!($next, $u))
        }
    }};
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: weighted-union-heuristics, edge-contraction
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        p: [usize1; n - 1],
        q: usize,
        qs: [(switch, (usize1, usize1), usize1); q],
    }
    let mut qf = vec![BTreeSet::new(); n];
    let mut rep = vec![0; n];
    let mut ch = vec![BTreeSet::new(); n];
    let mut par = vec![n; n];
    for i in 0..n - 1 {
        ch[p[i]].insert(i + 1);
        par[i + 1] = p[i];
    }
    for i in 0..n {
        qf[i].insert(i);
        rep[i] = i;
    }
    for q in qs {
        match q {
            Ok((u, v)) => {
                let mut u = rep[u];
                let v = rep[v];
                while u != v {
                    let p = par[u];
                    ch[p].remove(&u);
                    let mut fst = std::mem::take(&mut ch[p]);
                    let mut snd = std::mem::take(&mut ch[u]);
                    let mut fstr = std::mem::take(&mut qf[p]);
                    let mut sndr = std::mem::take(&mut qf[u]);
                    if fst.len() + fstr.len() < snd.len() + sndr.len() {
                        for &v in &fstr {
                            sndr.insert(v);
                            rep[v] = u;
                        }
                        for &v in &fst {
                            snd.insert(v);
                            par[v] = u;
                        }
                        ch[u] = snd;
                        qf[u] = sndr;
                        par[u] = par[p];
                        if par[p] < n {
                            ch[par[p]].remove(&p);
                            ch[par[p]].insert(u);
                        }
                        if p == v {
                            break;
                        }
                    } else {
                        for &v in &sndr {
                            fstr.insert(v);
                            rep[v] = p;
                        }
                        for &v in &snd {
                            fst.insert(v);
                            par[v] = p;
                        }
                        ch[p] = fst;
                        qf[p] = fstr;
                        u = p;
                    }
                }
            }
            Err(x) => {
                let id = rep[x];
                puts!("{}\n", qf[id].iter().next().unwrap() + 1);
            }
        }
    }
}
