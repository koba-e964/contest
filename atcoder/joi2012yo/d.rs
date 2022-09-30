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
        n: usize, k: usize,
        ab: [(usize1, usize1); k],
    }
    let mut dec = vec![3; n];
    for (a, b) in ab {
        dec[a] = b;
    }
    let mut dp = vec![[[0; 2]; 3]; n];
    if dec[0] == 3 {
        dp[0] = [[1, 0]; 3];
    } else {
        dp[0][dec[0]][0] = 1;
    }
    const MOD: i32 = 10_000;
    for i in 1..n {
        for j in 0..3 {
            for l in 0..3 {
                if dec[i] != 3 && dec[i] != l { continue; }
                for u in 0..2 {
                    if j == l {
                        if u == 1 { continue; }
                        dp[i][l][1] = (dp[i][l][1] + dp[i - 1][l][0]) % MOD;
                    } else {
                        dp[i][l][0] = (dp[i][l][0] + dp[i - 1][j][u]) % MOD;
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..3 {
        for j in 0..2 {
            ans += dp[n - 1][i][j];
        }
    }
    println!("{}", ans % MOD);
}
