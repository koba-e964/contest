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

fn rot(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = s.len();
    let mut a = vec![vec!['+'; n]; n];
    for i in 0..n {
        for j in 0..n {
            a[j][n - i - 1] = s[i][j];
        }
    }
    a
}

fn calc(s: &[Vec<char>]) -> usize {
    let n = s.len();
    let mut ans = 0;
    if s[0][..n - 1].iter().all(|&c| c == '.') {
        ans += 1;
    }
    let mut tmp1 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for i in 1..n {
        if s[i][1..n - 1].iter().all(|&c| c == '.')
            && (s[i][0] == '.' || s[i][n - 1] == '.') {
                tmp1 += 1;
                if s[i][0] == '.' && s[i][n - 1] == '.' {
                    z += 1;
                }
            } else {
                if s[i][0] == '.' {
                    x += 1;
                }
                if s[i][n - 1] == '.' {
                    y += 1;
                }
            }
    }
    if max(x, y) + z >= n - 1 {
        tmp1 += 1;
    }
    let mut tmp2 = 0;
    for i in 0..n {
        if (1..n).all(|j| s[j][i] == '.') {
            tmp2 += 1;
        }
    }
    ans + max(tmp1, tmp2)
}

fn main() {
    input! {
        n: usize,
        s: [chars; n],
    }
    let mut s = s;
    let mut ma = 0;
    for _ in 0..2 {
        for _ in 0..4 {
            ma = max(ma, calc(&s));
            s = rot(s);
        }
        s.reverse();
    }
    if s[0].iter().all(|&c| c == '.')
        && s[n - 1].iter().all(|&c| c == '.')
        && (0..n).all(|i| s[i][0] == '.' && s[i][n - 1] == '.') {
            ma = max(ma, 4);
        }
    println!("{}", ma);
}
