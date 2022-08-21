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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        g: [chars; h],
    }
    let mut p = vec![vec![(0, 0); w]; h];
    for i in 0..h {
        for j in 0..w {
            p[i][j] = (i, j);
            if g[i][j] == 'U' && i > 0 {
                p[i][j] = (i - 1, j);
            }
            if g[i][j] == 'D' && i < h - 1 {
                p[i][j] = (i + 1, j);
            }
            if g[i][j] == 'L' && j > 0 {
                p[i][j] = (i, j - 1);
            }
            if g[i][j] == 'R' && j + 1 < w {
                p[i][j] = (i, j + 1);
            }
        }
    }
    let mut vis = vec![vec![false; w]; h];
    let mut cur = (0, 0);
    while !vis[cur.0][cur.1] {
        vis[cur.0][cur.1] = true;
        cur = p[cur.0][cur.1];
    }
    if p[cur.0][cur.1] == cur {
        println!("{} {}", cur.0 + 1, cur.1 + 1);
    } else {
        println!("-1");
    }
}
