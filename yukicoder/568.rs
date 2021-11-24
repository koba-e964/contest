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

// https://yukicoder.me/problems/no/568 (3)
// SA を全探索して SB を二分探索。
// Tags: ad-hoc-data-structure, binary-search, occurrence
fn main() {
    input! {
        n: usize, m: i32,
        xab: [(usize, usize, usize); n],
    }
    const W: usize = 100_100;
    let mut ans = n as i32;
    let mut aidd = vec![vec![]; W];
    let mut two = 0;
    let mut three = 0;
    let mut freq = vec![[0i32; 6]; W];
    let mut pos = 0;
    for i in 0..n {
        let (x, a, b) = xab[i];
        aidd[a].push(i);
        if x >= 2 {
            three += 1;
        }
        if x == 1 {
            two += 1;
        }
        freq[b][x + 1] += 1;
    }
    for sa in (0..W).rev() {
        for &idx in &aidd[sa] {
            let (x, _a, b) = xab[idx];
            let nx = x + if pos <= b { 1 } else { 0 };
            freq[b][nx] -= 1;
            freq[b][nx + 1] += 1;
            match nx {
                1 => two += 1,
                2 => {
                    two -= 1;
                    three += 1;
                }
                _ => {}
            }
        }
        while pos < W && two + three >= m {
            if two + three - freq[pos][2] >= m {
                three -= freq[pos][3];
                two += freq[pos][3] - freq[pos][2];
                for j in 0..5 {
                    freq[pos][j] = freq[pos][j + 1];
                }
                pos += 1;
            } else {
                break;
            }
        }
        if two + three >= m {
            ans = min(ans, three);
        }
    }
    println!("{}", ans);
}
