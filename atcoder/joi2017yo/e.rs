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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(x: usize, y: usize, m: &[Vec<i32>], dp: &mut [Vec<i32>]) {
    if dp[x][y] >= 0 {
        return;
    }
    let h = m.len();
    let w = m[0].len();
    let mut sub = vec![];
    let dxy = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
    for &(dx, dy) in &dxy {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        if nx >= h || ny >= w {
            continue;
        }
        if m[nx][ny] < m[x][y] {
            dfs(nx, ny, m, dp);
            if dp[nx][ny] == 0 {
                sub.push(1);
                sub.push(2);
            } else {
                sub.push(dp[nx][ny]);
            }
        }
    }
    sub.sort(); sub.dedup();
    if sub.len() >= 2 {
        dp[x][y] = 0;
    } else if sub.len() == 1 {
        dp[x][y] = sub[0];
    } else {
        dp[x][y] = m[x][y];
    }
}

fn solve() {
    input! {
        h: usize, w: usize,
        m: [[i32; w]; h],
    }
    let mut dp = vec![vec![-1; w]; h];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            dfs(i, j, &m, &mut dp);
            if dp[i][j] == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
