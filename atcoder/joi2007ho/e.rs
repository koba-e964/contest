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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn dfs(v: usize, g: &[(usize, usize)], rat: &[(i64, i64)]) -> i64 {
    let (x, y) = g[v];
    let sub1 = if x > 0 { dfs(x - 1, g, rat) } else { 1 };
    let sub2 = if y > 0 { dfs(y - 1, g, rat) } else { 1 };
    let (p, q) = rat[v];
    let l = sub1 * p;
    let r = sub2 * q;
    let g = gcd(l, r);
    sub1 * (r / g) + sub2 * (l / g)
}

fn main() {
    input! {
        n: usize,
        conf: [(i64, i64, usize, usize); n],
    }
    let mut g = vec![(0, 0); n];
    let mut rat = vec![(0, 0); n];
    let mut indeg = vec![0; n];
    for i in 0..n {
        let (p, q, r, b) = conf[i];
        if r > 0 {
            indeg[r - 1] += 1;
        }
        if b > 0 {
            indeg[b - 1] += 1;
        }
        g[i] = (r, b);
        rat[i] = (p, q);
    }
    let root = indeg.iter().position(|&x| x == 0).unwrap();
    println!("{}", dfs(root, &g, &rat));
}
