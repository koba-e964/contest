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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn dfs(v: usize, l: usize, g: &[Vec<(usize, i64)>], s: i64, t: i64, x: i64, ans: &mut [bool]) {
    if l == 0 {
        if (s..=t).contains(&x) {
            ans[v] = true;
        }
        return;
    }
    for &(w, c) in &g[v] {
        dfs(w, l - 1, g, s, t, x + c, ans);
    }
}

fn main() {
    input! {
        n: usize, m: usize, l: usize, s: i64, t: i64, 
        uvc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for (u, v, c) in uvc {
        g[u].push((v, c));
    }
    let mut ans = vec![false; n];
    dfs(0, l, &g, s, t, 0, &mut ans);
    for i in 0..n {
        if ans[i] {
            print!(" {}", i + 1);
        }
    }
    println!();
}
