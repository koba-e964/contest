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
    input! {
        n: usize, m: usize, k: usize, s: usize1, t: usize1, x: usize1,
        uv: [(usize1, usize1); m],
    }
    const MOD: i64 = 998_244_353;
    let mut dp = vec![0; 2 * n];
    dp[s] = 1;
    for _ in 0..k {
        let mut ep = vec![0; 2 * n];
        for &(u, v) in &uv {
            for &(u, v) in &[(u, v), (v, u)] {
                for j in 0..2 {
                    let a = u + j * n;
                    let b = if v == x {
                        v + (1 - j) * n
                    } else {
                        v + j * n
                    };
                    ep[b] = (ep[b] + dp[a]) % MOD;
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp[t]);
}
