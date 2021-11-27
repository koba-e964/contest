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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        s: chars,
        k: i64,
    }
    let n = s.len();
    let mut ans = 0;
    let mut dif = vec![];
    for i in 0..n {
        if s[i] == '.' { continue; }
        let pos = dif.len();
        dif.push((i - pos) as i64);
    }
    let m = dif.len();
    let mut acc = vec![0; m + 1];
    for i in 0..m {
        acc[i + 1] = acc[i] + dif[i];
    }
    for i in 0..m {
        for &(dx, dy) in &[(0, 0), (0, 1), (1, 0)] {
            if i < dx || m - i - 1 < dy { continue; }
            let mut pass = 0;
            let mut fail = min(i - dx, m - 1 - i - dy) + 1;
            while fail - pass > 1 {
                let mid = (fail + pass) / 2;
                let l = mid + dx;
                let r = mid + dy;
                let mut tmp = dif[i] * l as i64 - (acc[i] - acc[i - l]);
                tmp += acc[i + r + 1] - acc[i + 1] - dif[i] * r as i64;
                if tmp <= k {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            ans = max(ans, pass * 2 + dx + dy + 1);
        }
    }
    println!("{}", ans);
}
