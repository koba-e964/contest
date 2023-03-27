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

#[inline]
fn zadd(mut a: u32, b: u32, m: u32) -> u32 {
    a += b;
    if a >= m {
        a -= m;
    }
    a
}

// https://yukicoder.me/problems/no/1956 (5)
// S := \sum A_i とする。普通の DP だと NS ワード (~= 36MB) 程度のメモリを要するので、
// 制限の 15MB に間に合わせるためには 1/3 程度に圧縮する必要がある。
// L ~= N/3 として、選んだ整数の個数 i が floor(i / L) = q を満たすときに重みを r^q だけつけるときの問題が解ければ、
// r = 1, -1, 0 で解くことで元の問題も解ける。
// (線形代数を解くときに 2 で割る操作が必要なので、mod は 2M で見る必要があることに注意。)
// ただ単に 15MB 以下にするだけだと動かないので、13 MB 程度以下にする必要があると思われる。
fn main() {
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                print!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: u32, c: usize,
        a: [usize; n],
    }
    let s: usize = a.iter().sum();
    if c == n {
        let mut v = vec![0; s];
        v[s - 1] = 1;
        putvec!(v);
        return;
    }
    const L: usize = 30;
    // 0.1 M entris, 0.4 MB
    let mut res = vec![0u32; s + 1];
    // 3.1 M entries, 12.4 MB
    let mut dp = vec![[0u32; L]; s + 1];
    let mut tmp = vec![0u32; s + 1];
    // coefs:
    // 0: 0 0 1
    // 1: 1/2 -1/2 0
    // 2: 1/2 1/2 -1
    dp[0][0] = 1;
    // r = 1
    for i in 0..n {
        for j in 0..s + 1 {
            tmp[j] = dp[j][0];
        }
        for k in 0..s - a[i] + 1 {
            dp[k + a[i]][0] = zadd(dp[k + a[i]][0], dp[k][L - 1], 2 * m);
        }
        for j in (1..L - 1).rev() {
            for k in 0..s - a[i] + 1 {
                dp[k + a[i]][j + 1] = zadd(dp[k + a[i]][j + 1], dp[k][j], 2 * m);
            }
        }
        for k in 0..s - a[i] + 1 {
            dp[k + a[i]][1] = zadd(dp[k + a[i]][1], tmp[k], 2 * m);
        }
    }
    for i in 0..s + 1 {
        if c >= L {
            res[i] = dp[i][c % L];
        }
    }
    for v in &mut dp {
        for v in v {
            *v = 0;
        }
    }
    dp[0][0] = 1;
    // r = -1
    for i in 0..n {
        for j in 0..s + 1 {
            tmp[j] = dp[j][0];
        }
        for k in 0..s - a[i] + 1 {
            dp[k + a[i]][0] = zadd(dp[k + a[i]][0], 2 * m - dp[k][L - 1], 2 * m);
        }
        for j in (1..L - 1).rev() {
            for k in 0..s - a[i] + 1 {
                dp[k + a[i]][j + 1] = zadd(dp[k + a[i]][j + 1], dp[k][j], 2 * m);
            }
        }
        for k in 0..s - a[i] + 1 {
            dp[k + a[i]][1] = zadd(dp[k + a[i]][1], tmp[k], 2 * m);
        }
    }
    for i in 0..s + 1 {
        if c >= L {
            if c < 2 * L {
                res[i] = zadd(res[i], 2 * m - dp[i][c % L], 2 * m);
            } else {
                res[i] = zadd(res[i], dp[i][c % L], 2 * m);
            }
        }
    }
    for v in &mut dp {
        for v in v {
            *v = 0;
        }
    }
    dp[0][0] = 1;
    // r = 0
    for i in 0..n {
        for j in (0..L - 1).rev() {
            for k in 0..s - a[i] + 1 {
                dp[k + a[i]][j + 1] = zadd(dp[k + a[i]][j + 1], dp[k][j], 2 * m);
            }
        }
    }
    for i in 0..s + 1 {
        if c / L == 0 {
            res[i] = zadd(dp[i][c % L], dp[i][c % L], 2 * m);
        } else if c / L == 2 {
            res[i] = zadd(res[i], (4 * m - 2 * dp[i][c % L]) % (2 * m), 2 * m);
        }
    }
    drop(dp);
    for i in 1..s + 1 {
        tmp[i - 1] = res[i] / 2;
    }
    putvec!(tmp[..s]);
}
