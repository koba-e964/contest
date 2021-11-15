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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const PI: f64 = 3.1415926535897932;

fn zeta(k: usize) -> f64 {
    if k == 2 {
        return PI * PI / 6.0;
    }
    if k == 3 {
        return 1.20205690315959428539973816151144999076498629;
    }
    if k == 4 {
        return PI.powf(4.0) / 90.0;
    }
    if k >= 50 {
        return 1.0;
    }
    let mut ans = 0.0;
    for i in (1..100).rev() {
        let mut v = 1.0;
        for _ in 0..k {
            v /= i as f64;
        }
        ans += v;
    }
    ans
}

// Tags: multiple-zeta-function
fn main() {
    input!(x: i32, k: usize);
    if x == 1 {
        let mut prod = 1.0;
        for _ in 0..k + 1 {
            prod /= 2.0;
        }
        println!("{}", 1.0 - prod);
        return;
    }
    if k == 1 {
        println!("0.5");
        return;
    }
    let mut dp = vec![0.0; k + 1];
    let mut acc = vec![0.0; k + 1];
    acc[1] = 1.0;
    dp[1] = 1.0;
    for i in 2..k + 1 {
        dp[i] = zeta(i) * (i - 1) as f64 - acc[i - 1];
        acc[i] = acc[i - 1] + dp[i];
    }
    println!("{}", dp[k]);
}
