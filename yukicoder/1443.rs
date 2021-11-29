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

// https://yukicoder.me/problems/no/1443 (3)
// 1024 未満はその個数があるかどうか、1024 以上は 1024 で割ったあまりの個数を保持しておく。
// 1024 以上の整数は、1024 未満にならない限り足し算しか受け付けないので、足し算によって
// 同じ値にマージされることはない。
fn main() {
    const W: usize = 1 << 10;
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp = vec![false; W];
    let mut ep = vec![0i64; W];
    dp[0] = true;
    for a in a {
        let mut ndp = vec![false; W];
        let mut nep = vec![0i64; W];
        for i in 0..W {
            if dp[i] || ep[i] > 0 {
                ndp[i & a] = true;
            }
        }
        for i in 0..W {
            if dp[i] {
                if i + a >= W {
                    nep[(i + a) % W] += 1;
                } else {
                    ndp[i + a] = true;
                }
            }
            if ep[i] > 0 {
                nep[(i + a) % W] += ep[i];
            }
        }
        dp = ndp;
        ep = nep;
        let mut ans = 0i64;
        for i in 0..W {
            if dp[i] {
                ans += 1;
            }
            ans += ep[i];
        }
        println!("{}", ans);
    }
}
