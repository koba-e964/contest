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

fn dfs(a: &[Vec<i32>], s: &mut HashSet<i32>, x: usize, y: usize) -> u32 {
    let h = a.len() - 1;
    let w = a[0].len() - 1;
    if x == h && y == w {
        return 1;
    }
    let mut ans = 0;
    if x < h {
        let val = a[x + 1][y];
        if !s.contains(&val) {
            s.insert(val);
            ans += dfs(a, s, x + 1, y);
            s.remove(&val);
        }
    }
    if y < w {
        let val = a[x][y + 1];
        if !s.contains(&val) {
            s.insert(val);
            ans += dfs(a, s, x, y + 1);
            s.remove(&val);
        }
    }
    ans
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [[i32; w]; h],
    }
    let mut s = HashSet::new();
    s.insert(a[0][0]);
    println!("{}", dfs(&a, &mut s, 0, 0));
}
