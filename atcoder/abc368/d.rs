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

fn main() {
    input! {
        n: usize, k: usize,
        ab: [(usize1, usize1); n - 1],
        v: [usize1; k],
    }
    let mut deg = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        deg[a] += 1;
        deg[b] += 1;
        g[a].push(b);
        g[b].push(a);
    }
    let mut ret = vec![false; n];
    for v in v {
        ret[v] = true;
    }
    let mut que = vec![];
    for i in 0..n {
        if deg[i] == 1 && !ret[i] {
            que.push(i);
        }
    }
    let mut ans = n;
    while let Some(v) = que.pop() {
        ans -= 1;
        for &to in &g[v] {
            deg[to] -= 1;
            if deg[to] == 1 && !ret[to] {
                que.push(to);
            }
        }
    }
    println!("{}", ans);
}
