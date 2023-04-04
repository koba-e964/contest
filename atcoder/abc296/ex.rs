use std::cmp::*;
use std::collections::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn conn_forget(conns: &[u8], pat: u8) -> Option<Vec<u8>> {
    let mut new_conns = vec![];
    for &c in conns {
        if (c & pat) == 0 {
            return None;
        }
        new_conns.push(c & pat);
    }
    let mut cur = vec![];
    {
        let mut rem = pat;
        while rem > 0 {
            let lsb = rem & rem.wrapping_neg();
            let cont = rem - (rem & (rem + lsb));
            cur.push(cont);
            rem -= cont;
        }
    }
    for &v in &new_conns {
        for i in 0..cur.len() {
            for j in 0..i {
                if (cur[i] & v) != 0 && (cur[j] & v) != 0 {
                    cur[j] |= cur[i];
                    cur[i] = 0;
                }
            }
        }
    }
    cur.sort_unstable();
    let mut pos = 0;
    while pos < cur.len() && cur[pos] == 0 {
        pos += 1;
    }
    Some(cur[pos..].to_vec())
}

// 状態は連結成分を並べた Vec である。これにより連結状態とどのセルが黒かを同時に管理できる。
// Tags: connectedness-dp
fn main() {
    input! {
        n: usize, m: usize,
        s: [chars; n],
    }
    let mut mi = n;
    let mut ma = 0;
    for i in 0..n {
        if s[i].iter().any(|&c| c == '#') {
            mi.chmin(i);
            ma.chmax(i);
        }
    }
    let mut dp = HashMap::new();
    dp.insert(vec![], 0);
    const INF: u32 = 1 << 28;
    for i in mi..ma + 1 {
        let mut ep = HashMap::new();
        let mut black = 0;
        for j in 0..m {
            if s[i][j] == '#' {
                black |= 1 << j;
            }
        }
        for bits in 0u8..1 << m {
            if (bits & black) != black { continue; }
            let added = (bits ^ black).count_ones();
            for (k, &v) in &dp {
                if let Some(new) = conn_forget(k, bits) {
                    if !new.is_empty() {
                        ep.entry(new).or_insert(INF).chmin(v + added);
                    }
                }
            }
        }
        dp = ep;
    }
    let mut ans = INF;
    for (k, v) in dp {
        if k.len() == 1 {
            ans.chmin(v);
        }
    }
    println!("{}", ans);
}
