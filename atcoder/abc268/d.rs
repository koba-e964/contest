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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/**
 * Returns the least index of elements that are modified, wrapped with Some.
 * If the entire array is reversed, it returns None instead.
 * v's elements must be pairwise distinct.
 */
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
    let mut tail_dec: usize = 1;
    let n = v.len();
    while tail_dec < n {
        if v[n - tail_dec - 1] > v[n - tail_dec] {
            tail_dec += 1;
        } else {
            break;
        }
    }
    // v[n - tail_dec .. n] is strictly decreasing
    if tail_dec < n {
        let x = n - tail_dec - 1;
        let mut y = n;
        {
            let pivot = &v[x];
            for i in (n - tail_dec .. n).rev() {
                if v[i] > *pivot {
                    y = i;
                    break;
                }
            }
            assert!(y < n);
        }
        v.swap(x, y);
    }
    v[n - tail_dec .. n].reverse();
    if tail_dec < n {
        Some(n - tail_dec - 1)
    } else {
        None
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
fn dfs(s: &[String], t: &HashSet<String>, path: &mut String, now: usize,
       hi: usize) -> Result<(), String> {
    if s.is_empty() {
        if now <= hi && !t.contains(path) {
            return Err(path.clone());
        } else {
            return Ok(());
        }
    }
    let cur = path.len();
    for i in now..if cur == 0 { now + 1 } else { hi + 1 } {
        for _ in 0..i - now + if cur == 0 { 0 } else { 1 } {
            path.push('_');
        }
        path.push_str(&s[0]);
        dfs(&s[1..], t, path, i, hi)?;
        path.truncate(cur);
    }
    Ok(())
}

fn solve() {
    input! {
        n: usize, m: usize,
        s: [String; n],
        t: [String; m],
    }
    let len: usize = s.iter().map(|s| s.len()).sum();
    if n == 1 && len < 3 {
        println!("-1");
        return;
    }
    let t: HashSet<String> = t.into_iter().collect();
    if n == 1 {
        if t.contains(&s[0]) {
            println!("-1");
            return;
        }
        println!("{}", s[0]);
        return;
    }
    let mut p: Vec<_> = (0..n).collect();
    loop {
        let mut tmp = vec!["".to_string(); n];
        for i in 0..n {
            tmp[i] = s[p[i]].clone();
        }
        if let Err(ans) = dfs(&tmp, &t, &mut "".to_string(), 0,
                              16 - len - (n - 1)) {
            println!("{}", ans);
            return;
        }
        if let None = next_permutation(&mut p) { break; }
    }
    println!("-1");
}
