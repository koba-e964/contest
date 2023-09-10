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
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut acc = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j]
                + if s[i][j] == '.' {
                    1
                } else {
                    0
                };
        }
    }
    let mut fail = min(h, w) + 1;
    let mut pass = 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut imos = vec![vec![0; w + 1]; h + 1];
        for i in 0..h + 1 - mid {
            for j in 0..w + 1 - mid {
                if acc[i + mid][j + mid] + acc[i][j] == acc[i + mid][j] + acc[i][j + mid] {
                    imos[i][j] += 1;
                    imos[i][j + mid] -= 1;
                    imos[i + mid][j] -= 1;
                    imos[i + mid][j + mid] += 1;
                }
            }
        }
        for i in 0..h + 1 {
            for j in 0..w {
                imos[i][j + 1] += imos[i][j];
            }
        }
        for i in 0..h {
            for j in 0..w + 1 {
                imos[i + 1][j] += imos[i][j];
            }
        }
        let mut ok = true;
        for i in 0..h {
            for j in 0..w {
                ok &= (imos[i][j] != 0) == (s[i][j] == '#');
            }
        }
        if ok {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
