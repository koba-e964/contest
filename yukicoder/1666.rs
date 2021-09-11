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

const B: usize = 60;

// #powers <= x
fn calc(x: i64, moe: &[i64]) -> i64 {
    let mut tot = 1;
    for b in 2..60 {
        if moe[b] == 0 {
            continue;
        }
        let mut pass = 1;
        let mut fail = 1i64 << ((60 + b - 1) / b);
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            let mut tmp = mid;
            for _ in 1..b {
                tmp = tmp.saturating_mul(mid);
            }
            if tmp <= x {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        tot += -(pass - 1) * moe[b];
    }
    tot
}

// Tags: inclusion-exclusion-on-divisors
fn main() {
    input! {
        t: usize,
        a: [i64; t],
    }
    let mut moe = vec![0; B];
    moe[1] = 1;
    for i in 1..B {
        for j in 2..(B - 1) / i + 1 {
            moe[i * j] -= moe[i];
        }
    }
    for k in a {
        let mut pass = k * k;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if calc(mid, &moe) >= k {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        println!("{}", pass);
    }
}
