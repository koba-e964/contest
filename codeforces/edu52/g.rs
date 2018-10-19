#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};

const DEBUG: bool = false;

// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn calc_dp(s: &[i8], n: usize, fib: &[u64])
           -> (Vec<u64>, Vec<Vec<u64>>, Vec<Vec<u64>>, Vec<Vec<u64>>) {
    let m = s.len();
    // s appears as a substring of A[i]
    let mut dp0 = vec![0u64; n + 1];
    // dp1[i][j]: s.substr(0, j) == A[i].substr(A[i].len() - j, j), not covered by dp0
    let mut dp1 = vec![vec![0u64; m]; n + 1];
    // dp2[i][j]: s.substr(s.len() - j, j) == A[i].substr(0, j), not covered by dp0
    let mut dp2 = vec![vec![0u64; m]; n + 1];
    // dp3[i][j]: s.substr(j, A[i].len()) == A[i], not covered by dp0, dp1, dp2
    let mut dp3 = vec![vec![0u64; m]; n + 1];
    if m == 1 {
        dp0[s[0] as usize] = 1;
    } else {
        for i in 1 .. m - 1 {
            dp3[s[i] as usize][i] = 1;
        }
        dp1[s[0] as usize][1] = 1;
        dp2[s[m - 1] as usize][1] = 1;
    }
    for i in 2 .. n + 1 {
        dp0[i] = dp0[i - 2].saturating_add(dp0[i - 1]);
        for j in 1 .. m {
            dp0[i] = dp0[i].saturating_add(dp1[i - 2][j] * dp2[i - 1][m - j]);
        }
        for j in 1 .. m {
            dp1[i][j] = dp1[i - 1][j];
            dp2[i][j] = dp2[i - 2][j];
        }
        let l_fst = fib[i - 2];
        let l_snd = fib[i - 1];
        for j in 1 .. m {
            if (j as u64).saturating_add(l_snd) < m as u64 {
                dp1[i][j + l_snd as usize] =
                    max(dp1[i][j + l_snd as usize],
                        dp1[i - 2][j] * dp3[i - 1][j]);
            }
            if (j as u64).saturating_add(l_fst) < m as u64 {
                dp2[i][j + l_fst as usize] =
                    max(dp2[i][j + l_fst as usize],
                        dp3[i - 2][m - j - l_fst as usize]
                        * dp2[i - 1][j]);
            }
            if (j as u64).saturating_add(l_fst) < m as u64 {
                dp3[i][j]
                    = max(dp3[i][j], dp3[i - 2][j] * dp3[i - 1][j + l_fst as usize]);
            }
        }
    }
    if DEBUG {
        eprintln!("dp0 = {:?}, dp1 = {:?}, dp2 = {:?}, dp3 = {:?}", dp0, dp1, dp2, dp3);
    }
    (dp0, dp1, dp2, dp3)
    
}

fn calc(s: &[i8], n: usize, fib: &[u64]) -> u64 {
    if s.len() == 0 {
        return fib[n];
    }
    if n == 0 {
        return if s.len() == 1 && s[0] == 0 { 1 } else { 0 }; 
    }
    let (dp0, _, _, _) = calc_dp(s, n, fib);
    if DEBUG {
        eprintln!("s = {:?}, dp0 = {:?}", s, dp0);
    }
    dp0[n]
}

fn terminates(s: &[i8], n: usize, fib: &[u64]) -> u64 {
    if s.len() == 0 {
        return 0;
    }
    let m = s.len();
    if n == 0 {
        return if s.len() == 1 && s[0] == 0 { 1 } else { 0 }; 
    }
    // dp1[i][j]: s.substr(0, j) == A[i].substr(A[i].len() - j, j)
    let mut dp1 = vec![vec![0u64; m + 1]; n + 1];
    // dp3[i][j]: s.substr(j, A[i].len()) == A[i], not covered by dp0, dp1, dp2
    let mut dp3 = vec![vec![0u64; m]; n + 1];
    if m == 1 {
        dp1[s[0] as usize][1] = 1;
    } else {
        for i in 1 .. m {
            dp3[s[i] as usize][i] = 1;
        }
        dp1[s[0] as usize][1] = 1;
    }
    for i in 2 .. n + 1 {
        for j in 1 .. m + 1 {
            dp1[i][j] = dp1[i - 1][j];
        }
        let l_fst = fib[i - 2];
        let l_snd = fib[i - 1];
        for j in 1 .. m + 1 {
            if (j as u64).saturating_add(l_snd) <= m as u64 {
                dp1[i][j + l_snd as usize] = dp1[i - 2][j] * dp3[i - 1][j];
            }
            if (j as u64).saturating_add(l_fst) < m as u64 {
                dp3[i][j]
                    = max(dp3[i][j], dp3[i - 2][j] * dp3[i - 1][j + l_fst as usize]);
            }
        }
    }
    if DEBUG {
        eprintln!("term s = {:?}, dp1 = {:?}, dp3 = {:?}", s, dp1, dp3);
    }
    dp1[n][m]
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let mut fib = vec![0u64; 100];
    fib[0] = 1;
    fib[1] = 1;
    for i in 2 .. 100 {
        fib[i] = fib[i - 1].saturating_add(fib[i - 2]);
    }
    if false {
        calc(&[1, 1], 4, &fib);
        return;
    }
    input! {
        n: usize,
        k: u64,
        m: usize,
    }
    let mut fib = vec![0u64; max(n, 2) + 1];
    fib[0] = 1;
    fib[1] = 1;
    for i in 2 .. n + 1 {
        fib[i] = fib[i - 1].saturating_add(fib[i - 2]);
    }
    let mut det = Vec::new();
    let mut acc: u64 = 0;
    for _ in 0 .. m {
        let exact = terminates(&det, n, &fib);
        if acc < k && acc + exact >= k {
            break;
        }
        if DEBUG {
            eprintln!("det = {:?}, acc = {}, exact = {}", det, acc, exact);
        }
        acc += exact;
        det.push(0i8);
        let u = calc(&det, n, &fib);
        if acc.saturating_add(u) < k {
            acc += u;
            det.pop();
            det.push(1i8);
        }
    }
    for d in det {
        puts!("{}", d);
    }
    puts!("\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
