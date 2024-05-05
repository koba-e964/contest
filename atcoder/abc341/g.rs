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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: convex-hull
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut st = vec![(n as i64, acc[n])];
    let mut dp = vec![0.0; n];
    for i in (0..n).rev() {
        let (x, y) = (i as i64, acc[i]);
        while st.len() >= 2 {
            let (x2, y2) = st[st.len() - 1];
            let (x1, y1) = st[st.len() - 2];
            if (y2 - y1) * (x1 - x) > (y1 - y) * (x2 - x1) {
                break;
            }
            st.pop();
        }
        let (x2, y2) = st[st.len() - 1];
        dp[i] = (y2 - y) as f64 / (x2 - x) as f64;
        st.push((x, y));
    }
    for i in 0..n {
        puts!("{}\n", dp[i]);
    }
}
