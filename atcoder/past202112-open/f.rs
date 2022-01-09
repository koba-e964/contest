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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        a: usize1, b: usize1,
        s: [chars; 3],
    }
    let mut vis = vec![vec![false; 9]; 9];
    let mut que = vec![(a, b)];
    let mut ans = 0;
    while let Some((x, y)) = que.pop() {
        if vis[x][y] { continue; }
        vis[x][y] = true;
        ans += 1;
        for i in 0..3 {
            for j in 0..3 {
                if s[i][j] != '#' { continue; }
                let nx = ((x + i) as i32 - 1) as usize;
                let ny = ((y + j) as i32 - 1) as usize;
                if nx < 9 && ny < 9 {
                    que.push((nx, ny));
                }
            }
        }
    }
    println!("{}", ans);
}
