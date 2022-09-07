use std::cmp::*;
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

fn dfs(arrived: bool,
       used: u32, g: &[Vec<(usize, usize)>], vis: u16, v: usize, c: usize, d: usize, s: &[String]) -> (usize, String) {
    if v == c && arrived {
        return (0, "".to_string());
    }
    let mut mi = (100, "".to_string());
    if used.count_ones() >= 14 {
        return mi;
    }
    if used.count_ones() >= 7 && !arrived {
        return mi;
    }
    for &(w, idx) in &g[v] {
        if (vis & 1 << w) != 0 { continue; }
        if (used & 1 << idx) == 0 {
            let sub = dfs(arrived || w == d, used ^ 1 << idx, g, if w == d { 1 << d } else { vis | 1 << w }, w, c, d, s);
            mi = min(mi, sub);
        }
    }
    mi.0 += 1;
    mi.1 = s[v].clone() + &mi.1;
    mi
}

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        ab: [(String, String); m],
        c: String,
        d: String,
    }
    let mut s = s;
    s.sort();
    let c = s.binary_search(&c).unwrap();
    let d = s.binary_search(&d).unwrap();
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (ref a, ref b) = ab[i];
        let a = s.binary_search(a).unwrap();
        let b = s.binary_search(b).unwrap();
        g[a].push((b, i));
        g[b].push((a, i));
    }
    let (_, ans) = dfs(false, 0, &g, 1 << c, c, c, d, &s);
    println!("{}", ans);
}
