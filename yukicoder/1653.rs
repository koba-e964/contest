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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn sqrt(x: i64) -> i64 {
    if x <= 1 { return x; }
    let mut pass = 0;
    let mut fail = 1 << 30;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if mid * mid <= x {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    const W: usize = 1_000_001;
    input!(l: i64, r: i64);
    let n = (r - l + 1) as usize;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] {
            continue;
        }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut rem = vec![0; n];
    let mut sq = vec![false; n];
    for i in 0..n {
        rem[i] = l + i as i64;
    }
    for p in 2..W {
        if !pr[p] { continue; }
        let pp = p as i64;
        let mut x = (l + pp - 1) / pp * pp;
        while x <= r {
            let idx = (x - l) as usize;
            if x % (pp * pp) == 0 {
                sq[idx] = true;
            } else {
                rem[idx] /= pp;
            }
            x += pp;
        }
    }
    for i in 0..n {
        let x = sqrt(rem[i]);
        if x > 1 && rem[i] == x * x {
            sq[i] = true;
        }
    }
    println!("{}", sq.iter().filter(|&&x| !x).count());
}
