#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
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

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn fake_dp(abc: &[i64], s: &[usize]) -> Option<Vec<usize>> {
    // sum abc = 2
    let n = s.len();
    let mut dp = vec![[false; 8]; n + 1];
    let mut init = 0;
    for i in 0..3 {
        if abc[i] > 0 {
            init |= 1 << i;
        }
    }
    dp[0][init] = true;
    let mut trans = vec![
        vec![vec![]; 8]; 3
    ];
    for i in 0..3 {
        let mask = [3usize, 5, 6][i];
        let lo = mask & mask.wrapping_neg();
        let hi = mask - lo;
        for j in 1..7 {
            if (j & mask) == 0 {
                continue;
            }
            if (j & mask) == mask {
                trans[i][j].push((j - lo, hi));
                trans[i][j].push((j - hi, lo));
            } else if j.count_ones() == 1 {
                trans[i][j].push((mask, mask - j));
            } else if (j & mask) == lo {
                trans[i][j].push((j - lo + hi, hi));
            } else {
                trans[i][j].push((j - hi + lo, lo));
            }
        }
    }
    for i in 0..n {
        for j in 0..8 {
            if !dp[i][j] { continue; }
            for &(tr, _) in &trans[s[i]][j] {
                dp[i + 1][tr] = true;
            }
        }
    }
    let ok = dp[n].iter().position(|&x| x);
    if ok.is_none() {
        return None;
    }
    let mut cur = ok.unwrap();
    let mut ans = vec![0; n];
    for i in (0..n).rev() {
        let mut pre = 8;
        'finder:
        for j in 0..8 {
            if !dp[i][j] { continue; }
            for &(tr, added) in &trans[s[i]][j] {
                if tr == cur {
                    pre = j;
                    ans[i] = added;
                    break 'finder;
                }
            }
        }
        assert!(pre < 8);
        cur = pre;
    }
    Some(ans)
}

fn naive(abc: &[i64], s: &[usize]) -> Option<Vec<usize>> {
    let mut abc = abc.to_vec();
    let n = s.len();
    let mut ans = vec![0; n];
    for i in 0..n {
        let (a, b) = if s[i] == 0 {
            // AB
            (0, 1)
        } else if s[i] == 1 {
            // AC
            (0, 2)
        } else {
            // BC
            (1, 2)
        };
        if abc[a] < abc[b] {
            ans[i] = 1 << a;
            abc[a] += 1;
            abc[b] -= 1;
        } else {
            ans[i] = 1 << b;
            abc[a] -= 1;
            abc[b] += 1;
        }
        if abc.iter().any(|&x| x < 0) {
            return None;
        }
    }
    Some(ans)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, abc: [i64; 3],
        s: [chars; n],
    }
    let s: Vec<usize> = s.into_iter().map(|s| {
        if &s[..] == &['A', 'B'] {
            0
        } else if &s[..] == &['A', 'C'] {
            1
        } else if &s[..] == &['B', 'C'] {
            2
        } else {
            panic!()
        }
    }).collect();
    let ans = if abc.iter().sum::<i64>() == 2 {
        fake_dp(&abc, &s)
    } else {
        naive(&abc, &s)
    };
    let ans = if let Some(ans) = ans {
        ans
    } else {
        puts!("No\n");
        return;
    };
    puts!("Yes\n");
    for i in 0..n {
        puts!("{}\n", match ans[i] {
            1 => "A",
            2 => "B",
            4 => "C",
            _ => panic!(),
        });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
