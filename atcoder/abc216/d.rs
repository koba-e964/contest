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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [[usize1]; m],
    }
    let mut occ = vec![vec![]; n];
    let mut f = vec![0; n];
    let mut avail = vec![];
    for i in 0..m {
        f[a[i][0]] += 1;
        if f[a[i][0]] == 2 {
            avail.push(a[i][0]);
        }
        for &w in &a[i] {
            occ[w].push(i);
        }
    }
    let mut pos = vec![0; m];
    let mut cnt = 0;
    while let Some(v) = avail.pop() {
        let x = occ[v][0];
        let y = occ[v][1];
        pos[x] += 1;
        pos[y] += 1;
        f[v] = 0;
        cnt += 1;
        for &w in &[x, y] {
            if pos[w] < a[w].len() {
                f[a[w][pos[w]]] += 1;
                if f[a[w][pos[w]]] == 2 {
                    avail.push(a[w][pos[w]]);
                }
            }
        }
    }
    println!("{}", if cnt == n { "Yes" } else { "No" });
}
