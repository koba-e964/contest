use std::collections::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn rec(n: &[char], k: i64, pos: usize, st: bool, eq: bool,
       memo: &mut HashMap<(i64, usize, bool, bool), i64>) -> i64 {
    let key = (k, pos, st, eq);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let l = n.len();
    if pos >= l {
        return if st && k >= 1 { 1 } else { 0 };
    }
    let me = (n[pos] as u8 - b'0') as i64;
    let mut ans = 0;
    // 0
    if st {
        let mut c = 1;
        if eq && me == 0 {
            c = 0;
            for i in pos + 1..l {
                c = 10 * c + (n[i] as u8 - b'0') as i64;
            }
            c += 1;
        } else {
            for _ in pos + 1..l {
                c *= 10;
            }
        }
        ans += c;
    } else {
        ans += rec(n, k, pos + 1, false, false, memo);
    }
    for i in 1..if eq { me + 1 } else { 10 } {
        ans += rec(n, k / i, pos + 1, true, eq && i == me, memo);
    }
    memo.insert(key, ans);
    ans
}

fn main() {
    input!(n: chars, k: i64);
    let mut memo = HashMap::new();
    let ans = rec(&n, k, 0, false, true, &mut memo);
    println!("{}", ans);
}
