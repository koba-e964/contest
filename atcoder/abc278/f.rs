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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        s: [chars; n],
    }
    let mut g = vec![0; n + 1];
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if s[i][s[i].len() - 1] == s[j][0] {
                g[i] |= 1 << j;
            }
        }
    }
    g[n] = (1 << n) - 1;
    let mut dp = vec![vec![false; n + 1]; 1 << n];
    for bits in 1..1 << n {
        for i in 0..n + 1 {
            if (bits & 1 << i) != 0 { continue; }
            let mut me = false;
            for j in 0..n {
                if (bits & 1 << j) == 0 { continue; }
                if (g[i] & 1 << j) != 0 && !dp[bits ^ 1 << j][j] {
                    me = true;
                }
            }
            dp[bits][i] = me;
        }
    }
    println!("{}", if dp[(1 << n) - 1][n] { "First" } else { "Second" });
}
