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

fn contains<T: PartialEq>(s: &[T], pat: &[T]) -> bool {
    if pat.is_empty() {
        return true;
    }
    let ff = kmp_ff(pat);
    let mut pos = 0;
    for i in 0..s.len() {
        let c = &s[i];
        while pos > 0 && &pat[pos] != c {
            pos = ff[pos - 1];
        }
        if &pat[pos] == c {
            pos += 1;
        }
        if pos == pat.len() {
            return true;
        }
    }
    false
}

fn calc(k: usize, mut a: Vec<i32>, mut b: Vec<i32>) -> bool {
    let n = a.len();
    if k > n {
        return a == b;
    }
    if k <= n - 2 {
        a.sort();
        b.sort();
        return a == b;
    }
    if k == n {
        let mut brev = b.clone();
        brev.reverse();
        return a == b || a == brev;
    }
    let mut brev = b.clone();
    brev.reverse();
    let mut p = a.clone();
    p.extend_from_slice(&a);
    contains(&p, &b) || contains(&p, &brev)
}

fn main() {
    input! {
        n: usize, k: usize,
        a: [i32; n],
        b: [i32; n],
    }
    println!("{}", if calc(k, a, b) { "Yes" } else { "No" });
}
