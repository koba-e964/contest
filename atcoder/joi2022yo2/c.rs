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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [[i64; w]; h],
    }
    let mut acc = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j] + a[i][j];
        }
    }
    let mut rows = vec![vec![]; h + 1];
    let mut cols = vec![vec![]; w + 1];
    'outer:
    for i in 1..h + 1 {
        if acc[h][w] % acc[i][w] != 0 { continue; }
        if acc[h][w] / acc[i][w] > h as i64 { continue; }
        let mut cur = i;
        let mut tmp = vec![i];
        for j in 2..acc[h][w] / acc[i][w] {
            while cur <= h && acc[cur][w] < j * acc[i][w] {
                cur += 1;
            }
            if acc[cur][w] != j * acc[i][w] {
                continue 'outer;
            }
            tmp.push(cur);
        }
        rows[i] = tmp;
    }
    'outer2:
    for i in 1..w + 1 {
        if acc[h][w] % acc[h][i] != 0 { continue; }
        if acc[h][w] / acc[h][i] > w as i64 { continue; }
        let mut cur = i;
        let mut tmp = vec![i];
        for j in 2..acc[h][w] / acc[h][i] {
            while cur <= w && acc[h][cur] < j * acc[h][i] {
                cur += 1;
            }
            if acc[h][cur] != j * acc[h][i] {
                continue 'outer2;
            }
            tmp.push(cur);
        }
        cols[i] = tmp;
    }
    let mut ans = 0;
    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if (i, j) == (h, w) { continue; }
            if acc[h][w] % acc[i][j] != 0 { continue; }
            if rows[i].is_empty() || cols[j].is_empty() {
                continue;
            }
            let mut ok = true;
            for x in 0..rows[i].len() {
                for y in 0..cols[j].len() {
                    if acc[rows[i][x]][cols[j][y]] != acc[i][j] * (x as i64 + 1) * (y as i64 + 1) {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
