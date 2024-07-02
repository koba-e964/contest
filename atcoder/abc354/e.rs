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

fn main() {
    input! {
        n: usize,
        ab: [(i32, u32); n],
    }
    let mut dp = vec![false; 1 << n];
    for bits in 3..1 << n {
        let mut win = false;
        for i in 0..n {
            for j in i + 1..n {
                if (bits & 1 << i) != 0 && (bits & 1 << j) != 0 {
                    let (a, b) = ab[i];
                    let (c, d) = ab[j];
                    if a == c || b == d {
                        if !dp[bits ^ 1 << i ^ 1 << j] {
                            win = true;
                        }
                    }
                }
            }
        }
        dp[bits] = win;
    }
    println!("{}", if dp[(1 << n) - 1] { "Takahashi" } else { "Aoki" });
}
