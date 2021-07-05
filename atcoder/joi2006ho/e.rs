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

const W: usize = 10_001;

fn perimeter(rec: &[((usize, usize), (usize, usize))]) -> i64 {
    let mut app = vec![vec![]; W];
    let mut dis = vec![vec![]; W];
    for &((a, b), (c, d)) in rec {
        app[a].push((b, d));
        dis[c].push((b, d));
    }
    let mut dp = vec![0; W];
    let mut ep = vec![0; W];
    let mut ans = 0;
    for i in 0..W {
        for &(l, r) in &dis[i] {
            for j in l..r {
                dp[j] -= 1;
            }
        }
        for &(l, r) in &app[i] {
            for j in l..r {
                dp[j] += 1;
            }
        }
        for j in 0..W {
            if (ep[j] > 0) != (dp[j] > 0) {
                ans += 1;
            }
            ep[j] = dp[j];
        }
        for j in 0..W {
            if (dp[j] > 0) != (j > 0 && dp[j - 1] > 0) {
                ans += 1;
            }
        }
    }
    ans
}

// Tags: area, perimeter, grid, low-memory
fn main() {
    input! {
        n: usize, r: i32,
        rec: [((usize, usize), (usize, usize)); n],
    }
    let mut app = vec![vec![]; W];
    let mut dis = vec![vec![]; W];
    for &((a, b), (c, d)) in &rec {
        app[a].push((b, d));
        dis[c].push((b, d));
    }
    let mut dp = vec![0; W];
    let mut ans = 0;
    for i in 0..W {
        for &(l, r) in &dis[i] {
            for j in l..r {
                dp[j] -= 1;
            }
        }
        for &(l, r) in &app[i] {
            for j in l..r {
                dp[j] += 1;
            }
        }
        for j in 0..W {
            if dp[j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
    if r == 2 {
        println!("{}", perimeter(&rec));
    }
}
