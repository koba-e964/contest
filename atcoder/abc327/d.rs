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

fn dfs(v: usize, g: &[Vec<usize>], vis: &mut Vec<i32>, c: i32) -> Result<(), ()> {
    if vis[v] != 0 {
        if vis[v] != c {
            return Err(());
        }
        return Ok(());
    }
    vis[v] = c;
    for &to in &g[v] {
        dfs(to, g, vis, -c)?;
    }
    Ok(())
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize1; m],
        b: [usize1; m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }
    let mut vis = vec![0; n];
    for i in 0..n {
        if vis[i] == 0 {
            if dfs(i, &g, &mut vis, 1).is_err() {
                println!("No");
                return;
            };
        }
    }
    println!("Yes");
}
