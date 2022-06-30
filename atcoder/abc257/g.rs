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

// Ref: https://codeforces.com/contest/1200/submission/58594933
// Verified by: https://codeforces.com/contest/1200/submission/58692231
fn kmp_ff<T: PartialEq>(pat: &[T]) -> Vec<usize> {
    let n = pat.len();
    let mut pi = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        while j > 0 && pat[i] != pat[j] {
            j = pi[j - 1];
        }
        j += usize::from(pat[i] == pat[j]);
        pi[i] = j;
    }
    pi
}

// Tags: kmp
fn main() {
    input! {
        s: chars,
        t: chars,
    }
    let ff = kmp_ff(&s);
    let n = t.len();
    const INF: i64 = 1 << 50;
    let mut dp = vec![0; n + 1];
    let mut pos = 0;
    for i in 1..n + 1 {
        let c = t[i - 1];
        while pos > 0 && (pos >= s.len() || s[pos] != c) {
            pos = ff[pos - 1];
        }
        if s[pos] == c {
            pos += 1;
        }
        if pos == 0 {
            dp[i] = INF;
        } else {
            dp[i] = dp[i - pos] + 1;
        }
    }
    println!("{}", if dp[n] >= INF { -1 } else { dp[n] });
}
