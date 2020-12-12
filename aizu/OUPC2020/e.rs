#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

const INF: i64 = 1 << 50;

struct Trie {
    ch: [Option<Box<Trie>>; 2],
    a: i64, // max obtained
}

fn new() -> Trie {
    Trie {
        ch: [None, None],
        a: -INF,
    }
}

fn add(t: &mut Trie, x: i64, val: i64, dep: usize) {
    if dep == 0 {
        t.a = max(t.a, val);
        return;
    }
    let idx = (x >> (dep - 1)) & 1;
    let idx = idx as usize;
    if t.ch[idx].is_none() {
        t.ch[idx] = Some(Box::new(new()));
    }
    add(t.ch[idx].as_mut().unwrap(), x, val, dep - 1);
}

fn dfs(t: &mut Trie) {
    let mut ma = t.a;
    for i in 0..2 {
        if let Some(x) = t.ch[i].as_mut() {
            dfs(x);
            ma = max(ma, x.a);
        }
    }
    t.a = ma;
}

fn get(t: Option<&Trie>, x: i64, m: i64, dep: usize) -> i64 {
    let t = if let Some(t) = t {
        t
    } else {
        return -INF;
    };
    if dep == 0 {
        return t.a;
    }
    let xbit = ((x >> (dep - 1)) & 1) as usize;
    let mbit = ((m >> (dep - 1)) & 1) as usize;
    if mbit == 0 {
        return get(t.ch[xbit].as_ref().map(|x| x.as_ref()), x, m, dep - 1);
    }
    
    let mut ma = get(t.ch[xbit ^ 1].as_ref().map(|x| x.as_ref()), x, m, dep - 1);
    if let Some(ch) = t.ch[xbit].as_ref() {
        ma = max(ma, ch.a);
    }
    ma
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: i64,
        a: [i64; n],
        b: [i64; n],
    }
    const B: usize = 30;
    let mut trie = new();
    for bits in 0..1 << (n / 2) {
        let mut ax = 0;
        let mut bs = 0;
        for i in 0..n / 2 {
            if (bits & 1 << i) != 0 {
                ax ^= a[i];
                bs += b[i];
            }
        }
        add(&mut trie, ax, bs, B);
    }
    dfs(&mut trie);
    let mut ma = 0;
    for bits in 0..1 << (n - n / 2) {
        let mut ax = 0;
        let mut bs = 0;
        for i in n / 2..n {
            if (bits & 1 << (i - n / 2)) != 0 {
                ax ^= a[i];
                bs += b[i];
            }
        }
        let res = get(Some(&trie), ax, m, B);
        ma = max(ma, bs + res);
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
