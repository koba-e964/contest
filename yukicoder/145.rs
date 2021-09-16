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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        _n: usize,
        s: chars,
    }
    let mut f = vec![0; 26];
    for c in s {
        f[(c as u8 - b'a') as usize] += 1;
    }
    let mut g = [0; 9];
    let yuki = [8, 10, 20, 24];
    for i in 0..4 {
        g[2 * i + 1] = f[yuki[i]];
    }
    for i in 0..5 {
        let lo = if i > 0 { yuki[i - 1] + 1 } else { 0 };
        let hi = if i == 4 { 26 } else { yuki[i] };
        for j in lo..hi {
            g[2 * i] += f[j];
        }
    }
    eprintln!("g = {:?}", g);
    let ind = vec![
        vec![8],
        vec![6, 7],
        vec![4, 5, 7],
        vec![2, 3, 5, 7],
        vec![0, 1, 3, 5, 7],
    ];
    let mut ans = 0;
    for ind in &ind {
        let mut r = 1 << 30;
        for &i in ind {
            r = min(r, g[i]);
        }
        ans += r;
        for &i in ind {
            g[i] -= r;
        }
    }
    let ind = vec![
        vec![1, 3, 5, 7],
        vec![3, 5, 7],
        vec![5, 7],
        vec![7],
    ];
    eprintln!("g = {:?}", g);
    for ind in &ind {
        let mut r = g[ind[0]] / 2;
        for j in 1..ind.len() {
            r = min(r, g[ind[j]]);
        }
        ans += r;
        g[ind[0]] -= 2 * r;
        for j in 1..ind.len() {
            g[ind[j]] -= r;
        }
    }
    println!("{}", ans);
}
