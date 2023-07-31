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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 998_244_353;

// m <= 7 なので 2^m <= 128
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 1_000_001;
    let mut fac = vec![vec![]; W];
    for i in 2..W {
        if !fac[i].is_empty() {
            continue;
        }
        for j in 1..(W - 1) / i + 1 {
            fac[i * j].push(i);
        }
    }
    let mut dp = vec![0; W];
    for a in a {
        let f = &fac[a];
        let m = f.len();
        let mut tot = 0;
        for bits in 1usize..1 << m {
            let mut prod = 1;
            for i in 0..m {
                if (bits & 1 << i) != 0 {
                    prod *= f[i];
                }
            }
            if bits.count_ones() % 2 == 0 {
                tot = tot + MOD - dp[prod];
            } else {
                tot += dp[prod];
            }
            if tot >= MOD {
                tot -= MOD;
            }
        }
        for bits in 0..1 << m {
            let mut prod = 1;
            for i in 0..m {
                if (bits & 1 << i) != 0 {
                    prod *= f[i];
                }
            }
            dp[prod] = (dp[prod] + tot + 1) % MOD;
        }
    }
    let mut tot = 0;
    for i in (1..W).rev() {
        for j in 2..(W - 1) / i + 1 {
            dp[i] += MOD - dp[i * j];
            if dp[i] >= MOD {
                dp[i] -= MOD;
            }
        }
        tot += dp[i];
        if tot >= MOD {
            tot -= MOD;
        }
    }
    println!("{}", tot);
}
