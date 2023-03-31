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

// Z algorithm. Calculates an array a[i] = |lcp(s, &s[i..])|,
// where s is the given slice.
// If n = s.length(), the returned array has length n + 1.
// E.g. z_algorithm(b"ababa") = vec![5, 0, 3, 0, 1, 0]
// Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
// Verified by: ABC284-F (https://atcoder.jp/contests/abc284/submissions/38752029)
fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    ret[0] = n;
    let mut i = 1; let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] { j += 1; }
        ret[i] = j;
        if j == 0 { i += 1; continue; }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k; j -= k;
    }
    ret
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: chars,
    }
    let mut s = s;
    for i in 0..2 * n {
        let c = if a[i % n] < a[(i + 1) % n] { '<' } else { '>' };
        s.push(c);
    }
    let z = z_algorithm(&s);
    const INF: i32 = 1 << 28;
    let mut ans = INF;
    for i in 0..n {
        if z[n - 1 + i] >= n - 1 {
            ans = min(ans, i as i32);
        }
    }
    println!("{}", if ans >= INF { -1 } else { ans });
}
