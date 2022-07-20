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

// Solved with hints
// http://lealgorithm.blogspot.com/2019/06/blog-post.html
fn main() {
    input! {
        s: usize, t: usize, m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; s + t];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut mat = vec![vec![None; t]; t];
    for i in 0..s {
        for j in 0..g[i].len() {
            for k in 0..g[i].len() {
                if j == k { continue; }
                if let Some(v) = mat[g[i][j] - s][g[i][k] - s] {
                    println!("{} {} {} {}", i + 1, v + 1, g[i][j] + 1, g[i][k] + 1);
                    return;
                }
                mat[g[i][j] - s][g[i][k] - s] = Some(i);
            }
        }
    }
    println!("-1");
}
