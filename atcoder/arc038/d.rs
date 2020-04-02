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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize,
        x: [i64; n],
        ab: [(usize1, usize1); m],
    }
    let mut pass = 0;
    let mut fail = 1i64 << 30;
    let mut ans = vec![[-1; 2]; n];
    let mut grev = vec![vec![]; 2 * n];
    let mut indeg_old = vec![0; 2 * n];
    for &(a, b) in &ab {
        grev[b].push(n + a);
        grev[b + n].push(a);
        indeg_old[a] += 1;
        indeg_old[n + a] += 1;
    }
    let mut indeg = vec![0; 2 * n];
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        // [0, n): first, [n, 2n): second
        for i in 0..2 * n {
            indeg[i] = indeg_old[i];
            ans[i / 2][i % 2] = -1;
        }
        let mut que = VecDeque::new();
        for i in 0..n {
            if x[i] >= mid {
                que.push_back((i, 1));
                if indeg[i] == 0 {
                    que.push_back((n + i, 1));
                }
            } else {
                que.push_back((n + i, 0));
                if indeg[i] == 0 {
                    que.push_back((i, 0));
                }
            }
        }
        while let Some((v, stat)) = que.pop_front() {
            if ans[v % n][v / n] != -1 {
                continue;
            }
            ans[v % n][v / n] = stat;
            for &w in &grev[v] {
                let to_player = 1 - (w / n) as i32;
                if to_player != stat {
                    indeg[w] -= 1;
                    if indeg[w] == 0 {
                        que.push_back((w, 1 - to_player));
                    }
                } else {
                    que.push_back((w, to_player));
                }
            }
        }
        if ans[0][0] == 1 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{}\n", pass);
}
