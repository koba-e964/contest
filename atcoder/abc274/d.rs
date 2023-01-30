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

fn check(a: &[usize], x: i64) -> bool {
    const W: usize = 20_001;
    const S: usize = 10_000;
    let mut dp = vec![false; W];
    dp[S] = true;
    for &a in a {
        let mut ep = vec![false; W];
        for i in 0..W {
            if i + a < W {
                ep[i] |= dp[i + a];
            }
            if i >= a {
                ep[i] |= dp[i - a];
            }
        }
        dp = ep;
    }
    dp[(S as i64 + x) as usize]
}

fn main() {
    input! {
        n: usize, x: i64, y: i64,
        a: [usize; n],
    }
    let mut fst = vec![];
    let mut snd = vec![];
    for i in 1..n {
        if i % 2 == 0 {
            fst.push(a[i]);
        } else {
            snd.push(a[i]);
        }
    }
    println!("{}", if check(&fst, x - a[0] as i64) && check(&snd, y) { "Yes" } else { "No" });
}
