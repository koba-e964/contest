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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        p: [usize1; n],
        q: [usize1; n],
    }
    let mut r = vec![0; n];
    for i in 0..n {
        r[p[i]] = q[i];
    }
    let mut vis = vec![false; n];
    const MOD: i64 = 998_244_353;
    let mut pre = vec![0; n + 1];
    pre[0] = 2;
    pre[1] = 1;
    for i in 2..n + 1 {
        pre[i] = (pre[i - 1] + pre[i - 2]) % MOD;
    }
    let mut ans = 1;
    for i in 0..n {
        if vis[i] { continue; }
        let mut v = i;
        let mut c = 0;
        while !vis[v] {
            vis[v] = true;
            v = r[v];
            c += 1;
        }
        ans = ans * pre[c] % MOD;
    }
    println!("{}", ans);
}
