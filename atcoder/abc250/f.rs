use std::cmp::*;
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

fn f((x, y): (i64, i64), (z, w): (i64, i64)) -> i64 {
    (y + w) * (z - x)
}

// Tags: geometry
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut a = 0;
    for i in 0..n {
        a -= f(xy[i], xy[(i + 1) % n]);
    }
    const INF: i64 = 1 << 62;
    let mut mi = INF;
    let mut pos = 2;
    let mut cur = -f(xy[0], xy[1]) - f(xy[1], xy[2]);
    // Slightly exceeds
    for i in 0..n {
        while pos < i + n - 2 {
            if (cur + f(xy[i], xy[pos % n])) * 4 >= a {
                break;
            }
            cur -= f(xy[pos % n], xy[(pos + 1) % n]);
            pos += 1;
        }
        let tmp = (cur + f(xy[i], xy[pos % n])) * 4 - a;
        mi = min(mi, tmp.abs());
        cur += f(xy[i], xy[(i + 1) % n]);
    }
    // Slightly falls short
    pos = 2 * n - 3;
    cur = a + f(xy[n - 3], xy[n - 2]) + f(xy[n - 2], xy[n - 1]);
    for i in (0..n).rev() {
        while pos > i + 2 {
            if (cur + f(xy[i], xy[pos % n])) * 4 < a {
                break;
            }
            pos -= 1;
            cur += f(xy[pos % n], xy[(pos + 1) % n]);
        }
        let tmp = a - (cur + f(xy[i], xy[pos % n])) * 4;
        mi = min(mi, tmp.abs());
        if i > 0 {
            cur -= f(xy[i - 1], xy[i]);
        }
    }
    println!("{}", mi);
}
