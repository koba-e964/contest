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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut ret = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn matpow(a: &[Vec<f64>], mut e: i64) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut prod = vec![vec![0.0; n]; n];
    for i in 0..n {
        prod[i][i] = 1.0;
    }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = matmul(&prod, &cur);
        }
        cur = matmul(&cur, &cur);
        e /= 2;
    }
    prod
}

fn main() {
    input! {
        r: usize, c: usize, t: i64,
        s: (usize, usize),
        g: (usize, usize),
        b: [chars; r],
    }
    let mut mat = vec![vec![0.0; r * c]; r * c];
    let dxy = [(1i32, 0i32), (0i32, 1i32), (-1, 0), (0, -1)];
    for i in 0..r {
        for j in 0..c {
            if b[i][j] != '.' { continue; }
            let mut to = vec![];
            for &(dx, dy) in &dxy {
                let nx = (i as i32 + dx) as usize;
                let ny = (j as i32 + dy) as usize;
                if b[nx][ny] != '.' { continue; }
                to.push(nx * c + ny);
            }
            if !to.is_empty() {
                let p = 1.0 / to.len() as f64;
                for &v in &to {
                    mat[i * c + j][v] = p;
                }
            } else {
                mat[i * c + j][i * c + j] = 1.0;
            }
        }
    }
    let ans = matpow(&mat, t);
    println!("{}", ans[s.0 * c + s.1][g.0 * c + g.1]);
}
