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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        s: chars,
        t: chars,
    }
    let s: Vec<usize> = s.into_iter().map(|c| (c as u8 - b'a') as _).collect();
    let t: Vec<usize> = t.into_iter().map(|c| (c as u8 - b'a') as _).collect();
    let n = s.len();
    let m = t.len();
    let mut dp1 = vec![[n + 1; 26]; n + 1];
    let mut dp2 = vec![[m + 1; 26]; m + 1];
    for i in (0..n).rev() {
        dp1[i] = dp1[i + 1];
        dp1[i][s[i]] = i;
    }
    for i in (0..m).rev() {
        dp2[i] = dp2[i + 1];
        dp2[i][t[i]] = i;
    }
    // j < pos[i] <=> s[i..] is a subsequence of t[j..]
    let mut pos = vec![m + 1; n + 1];
    for i in (0..n).rev() {
        let mut p = max(1, pos[i + 1]) - 1;
        while p > 0 {
            if t[p - 1] == s[i] {
                break;
            }
            p -= 1;
        }
        pos[i] = p;
    }
    eprintln!("pos = {:?}", pos);
    let mut ans = vec![];
    let mut cur1 = 0;
    let mut cur2 = 0;
    if pos[0] > 0 {
        puts!("-1\n");
        return;
    }
    while cur1 < n && cur2 <= m {
        let mut nxt = None;
        for i in 0..26 {
            let nxtcur1 = dp1[cur1][i] + 1;
            let nxtcur2 = dp2[cur2][i] + 1;
            if nxtcur1 <= n && pos[nxtcur1] <= nxtcur2 {
                nxt = Some(i);
                break;
            }
        }
        if let Some(nxt) = nxt {
            cur1 = dp1[cur1][nxt] + 1;
            cur2 = dp2[cur2][nxt] + 1;
            ans.push(nxt);
        } else {
            panic!();
        }
    }
    for v in ans {
        puts!("{}", (b'a' + v as u8) as char);
    }
    puts!("\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
