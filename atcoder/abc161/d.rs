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

fn dfs(cnt: usize, len: usize, path: &mut Vec<i32>, ord: &mut usize) -> Result<(), i64> {
    if path.len() >= len {
        *ord += 1;
        if *ord == cnt {
            let mut sum: i64 = 0;
            for &v in path.iter() {
                sum = 10 * sum + v as i64;
            }
            return Err(sum);
        }
        return Ok(());
    }
    if path.len() == 0 {
        for i in 1..10 {
            path.push(i);
            dfs(cnt, len, path, ord)?;
            path.pop();
        }
        return Ok(());
    }
    for dif in -1..2 {
        let nxt = path[path.len() - 1] + dif;
        if nxt >= 0 && nxt <= 9 {
            path.push(nxt);
            dfs(cnt, len, path, ord)?;
            path.pop();
        }
    }
    Ok(())
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(k: usize);
    let mut past = 0;
    let mut len = 1;
    let ans;
    loop {
        let mut ord = 0;
        match dfs(k - past, len, &mut vec![], &mut ord) {
            Ok(()) => past += ord,
            Err(val) => {
                ans = val;
                break;
            }
        }
        len += 1;
    };
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
