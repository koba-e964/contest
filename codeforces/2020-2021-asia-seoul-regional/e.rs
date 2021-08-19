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

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    if a[0] >= 2 || a[n - 1] >= 2 {
        println!("NO");
        return;
    }
    let mut dp = vec![[[false; 2]; 2]; n + 1];
    for i in 0..2 {
        dp[0][i][i] = true;
    }
    for i in 0..n {
        for j in 0..2 {
            for k in 0..2 {
                if !dp[i][j][k] {
                    continue;
                }
                for l in 0..2 {
                    for m in 0..2 {
                        let fst = j as i32 + 1 - l as i32;
                        let snd = k as i32 + 1 - m as i32;
                        if (fst - snd).abs() == a[i] {
                            dp[i + 1][l][m] = true;
                        }
                    }
                }
            }
        }
    }
    println!("{}", if dp[n][0][0] || dp[n][1][1] { "YES" } else { "NO" });
}
