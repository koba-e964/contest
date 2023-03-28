use std::cmp::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn rec(lim: i64, a: &[i64], fib: &[i64]) -> i64 {
    assert!(lim >= 0);
    let n = a.len();
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return if lim >= a[0] { fib[0] } else { 0 };
    }
    if a[n - 1] + a[n - 2] <= lim {
        return rec(lim - a[n - 1], &a[..n - 1], fib) + fib[n - 1];
    }
    if a[n - 1] > lim {
        return rec(lim, &a[..n - 1], fib);
    }
    if a[n - 2] > lim {
        return fib[n - 1] + rec(lim - a[n - 1], &a[..n - 2], fib);
    }
    if n == 2 {
        return fib[1];
    }
    if n == 3 {
        let mut ans = fib[n - 1]; // 100
        if a[n - 1] + a[n - 3] <= lim {
            ans = max(ans, fib[n - 1] + fib[n - 3]); // 101
        }
        return ans;
    }
    assert!(n >= 4);
    if a[n - 1] + a[n - 3] + a[n - 4] <= lim {
        let mut ans = rec(lim - (a[n - 1] + a[n - 3] + a[n - 4]), &a[..n - 4], fib) + fib[n - 1] + fib[n - 2];
        ans = max(ans, rec(lim - (a[n - 1] + a[n - 3]), &a[..n - 4], fib) + fib[n - 1] + fib[n - 3]);
        return ans;
    }
    if a[n - 1] + a[n - 3] <= lim {
        let mut ans = rec(lim - min(a[n - 1], a[n - 2] + a[n - 3]), &a[..n - 3], fib) + fib[n - 1];
        ans = max(ans, rec(lim - (a[n - 1] + a[n - 3]), &a[..n - 4], fib) + fib[n - 1] + fib[n - 3]);
        return ans;
    }
    if min(a[n - 1], a[n - 2] + a[n - 3]) + a[n - 4] <= lim {
        let mut ans = rec(lim - (min(a[n - 1], a[n - 2] + a[n - 3]) + a[n - 4]), &a[..n - 4], fib) + fib[n - 1] + fib[n - 4];
        ans = max(ans, rec(lim - (min(a[n - 1], a[n - 2] + a[n - 3])), &a[..n - 4], fib) + fib[n - 1]);
        return ans;
    }
    let mut ans = rec(lim - min(a[n - 1], a[n - 2] + a[n - 3]), &a[..n - 4], fib) + fib[n - 1];
    if lim >= a[n - 2] {
        ans = max(ans, rec(lim - a[n - 2], &a[..n - 3], fib) + fib[n - 2]);
    }
    ans
}

// https://yukicoder.me/problems/no/2167 (3.5)
// \sum_{1 <= i <= N} F_i = F_{N+2} - 2 であるため、N と N-1 が同時に取れるのであれば N は必ず取る必要がある。
// 片方は取れるが両方は取れない場合、どちらかを取るのが最善。最大の 3 個の取り方は 101, 100, 011, 010 の 4 通りだが、101 > 010 であり (F_N + F_{N-2} > F_{N-1} + \sum_{1 <= i <= N - 3} F_i)、100 と 011 は残りの重さが大きいほうが良いので、実質 2 通り。こうすると再帰のパターン数は 2^{N/3} であることがわかる。
// N-3 も考慮に入れると 1011 > 100, 1011 > 011 であることがわかる。よって 1011 が取れるなら 1011, 1010 の 2 通りに再帰し、そうでないなら 1010 と 100 or 011 に再帰するのが良い。オーダーは x^4 = x + 1 の唯一の正の根を r ~= 1.221 として O(r^n) である。r^{75} ~= 3.2 * 10^6 なのでこれでよい。
// -> 101 が取れない時の処理が誤っている。このときは 100 or 011 と 010 に再帰する必要がある。
// 1001 or 0111 がとれるときは 010 より常に良いのでそちらを採用するのがよく、x1 と x0 (x は 100 or 011) に再帰するのが良い。そうでないときは 1000 or 0110 と 010 に再帰する。
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        cs: [([i64], i64); t],
    }
    let mut fib = vec![1, 2];
    while fib.len() < 75 {
        let tmp = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(tmp);
    }
    for (mut cs, app) in cs {
        let lim = cs[0];
        cs.remove(0);
        cs.push(app);
        puts!("{}\n", rec(lim, &cs, &fib));
    }
}
