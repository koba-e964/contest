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
        s: [chars; h],
    }
    let mut s = s;
    let mut que = vec![];
    let mut nei = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }
            if i > 0 {
                nei[i - 1][j] += 1;
            }
            if i + 1 < h {
                nei[i + 1][j] += 1;
            }
            if j > 0 {
                nei[i][j - 1] += 1;
            }
            if j + 1 < w {
                nei[i][j + 1] += 1;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' && nei[i][j] == 1 {
                que.push((i, j));
            }
        }
    }
    while que.len() > 0 {
        let mut nxtque = vec![];
        let mut next = vec![];
        while let Some((x, y)) = que.pop() {
            if s[x][y] == '#' || nei[x][y] != 1 {
                continue;
            }
            s[x][y] = '#';
            if x > 0 {
                next.push((x - 1, y));
            }
            if x + 1 < h {
                next.push((x + 1, y));
            }
            if y > 0 {
                next.push((x, y - 1));
            }
            if y + 1 < w {
                next.push((x, y + 1));
            }
        }
        for (nx, ny) in next {
            nei[nx][ny] += 1;
            if s[nx][ny] == '.' && nei[nx][ny] == 1 {
                nxtque.push((nx, ny));
            }
        }
        que = nxtque;
    }
    let mut ans = 0;
    for i in 0..h {
        ans += s[i].iter().filter(|&&c| c == '#').count();
    }
    println!("{ans}");
}
