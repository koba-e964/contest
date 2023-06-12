use std::io::{Write, BufWriter};
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn f(mut a: Vec<i32>) -> bool {
    let mut s = [0; 2];
    for i in 0..a.len() {
        s[a[i] as usize] += 1;
    }
    a.dedup();
    let m = a.len();
    if m % 2 == 0 { return true; }
    if m >= 9 { return true; }
    if a[0] == 0 && s[1] == m / 2 {
        return false;
    }
    if a[0] == 1 && s[0] == m / 2 {
        return false;
    }
    true
}

// https://yukicoder.me/problems/no/2343 (3.5)
// 0 と 1 が同数であれば簡単。そうでないとき、一般性を失わず 0 が多いとして良い。
// 0+(10+)* にマッチしない文字列であれば 0 を 1 と同数にまで減らすことができるので可能。
// マッチする文字列の場合はおそらく不可能と思い提出したら WA。
// -> 010101010 (len = 9) の場合、0101 を 5/8 にし、 01 を 1/2、 010 を 3/8 にして
// 01010 で 3/8 を作れば最終的に 1/2 にできる。よって長さが 9 以上で 0+(10+)* にマッチする場合も可能。
// これで提出したら AC だった。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(a: [[i32]]);
    for a in a {
        puts!("{}\n", if f(a) { "Yes" } else { "No" });
    }
}
