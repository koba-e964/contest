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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: cumulative-sum, 3d-imos
fn main() {
    input! {
        n: usize, k: i32,
        fish: [[[i64; 3]; 2]; n],
    }
    let mut coord = vec![vec![]; 3];
    for i in 0..n {
        for j in 0..2 {
            for k in 0..3 {
                coord[k].push(fish[i][j][k]);
            }
        }
    }
    for k in 0..3 {
        coord[k].sort(); coord[k].dedup();
    }
    let a = coord[0].len();
    let b = coord[1].len();
    let c = coord[2].len();
    let mut dp = vec![vec![vec![0; c]; b]; a];
    for f in &fish {
        let mut idx = [[0; 3]; 2];
        for i in 0..2 {
            for k in 0..3 {
                idx[i][k] = coord[k].binary_search(&f[i][k]).unwrap();
            }
        }
        for i in 0..8 {
            let x = i >> 2;
            let y = (i >> 1) & 1;
            let z = i & 1;
            dp[idx[x][0]][idx[y][1]][idx[z][2]] +=
                if (x ^ y ^ z) == 0 { 1 } else { -1 };
        }
    }
    for i in 0..a - 1 {
        for j in 0..b {
            for k in 0..c {
                dp[i + 1][j][k] += dp[i][j][k];
            }
        }
    }
    for i in 0..a {
        for j in 0..b - 1 {
            for k in 0..c {
                dp[i][j + 1][k] += dp[i][j][k];
            }
        }
    }
    for i in 0..a {
        for j in 0..b {
            for k in 0..c - 1 {
                dp[i][j][k + 1] += dp[i][j][k];
            }
        }
    }
    let mut ans = 0;
    for i in 0..a - 1 {
        for j in 0..b - 1 {
            for l in 0..c - 1 {
                let vol = (coord[0][i + 1] - coord[0][i])
                    * (coord[1][j + 1] - coord[1][j])
                    * (coord[2][l + 1] - coord[2][l]);
                ans += if dp[i][j][l] >= k { vol } else { 0 };
            }
        }
    }
    println!("{}", ans);
}
