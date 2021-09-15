use std::cmp::*;
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        n: usize, k: usize1,
        x: [i64; n],
        a: [i64; n],
    }
    let mut hi = x[k] + a[k];
    let mut lo = x[k] - a[k];
    let mut hiidx = k + 1;
    let mut loidx = k;
    loop {
        if (hiidx == n || x[hiidx] > hi) && (loidx == 0 || x[loidx - 1] < lo) {
            break;
        }
        if hiidx < n && x[hiidx] <= hi {
            hi = max(hi, x[hiidx] + a[hiidx]);
            lo = min(lo, x[hiidx] - a[hiidx]);
            hiidx += 1;
        }
        if loidx > 0 && x[loidx - 1] >= lo {
            loidx -= 1;
            hi = max(hi, x[loidx] + a[loidx]);
            lo = min(lo, x[loidx] - a[loidx]);
        }
    }
    println!("{}", hiidx - loidx);
}
