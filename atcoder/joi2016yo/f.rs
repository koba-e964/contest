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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// 実装に 52m かかった。
fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    const INF: i32 = 1 << 28;
    // .53
    // 4.1
    // 20
    let mut dp = vec![vec![[INF; 16]; w]; h];
    dp[0][0][0] = 0;
    let down = |x: usize, y: usize, bits: usize| {
        assert!(x + 1 < h);
        let mut me = 0;
        if (bits & 1) != 0 {
            me = (s[x + 1][y] as u8 - b'0') as i32;
        }
        // 2 -> 4, 1 -> 3
        let mut new = (bits << 2) & 24;
        if y + 1 < w && s[x + 1][y + 1] != '.' {
            new |= 2;
        }
        if x + 2 < h && s[x + 2][y] != '.' {
            new |= 1;
        }
        if x + 2 < h && y > 0 && s[x + 2][y - 1] != '.' {
            new |= 4;
        }
        (me, new)
    };
    let right = |x: usize, y: usize, bits: usize| {
        assert!(y + 1 < w);
        let mut me = 0;
        if (bits & 2) != 0 {
            me = (s[x][y + 1] as u8 - b'0') as i32;
        }
        // 0 -> 2, 3 -> 5
        let mut new = (bits << 2) & 36;
        if x + 1 < h && s[x + 1][y + 1] != '.' {
            new |= 1;
        }
        if y + 2 < w && s[x][y + 2] != '.' {
            new |= 2;
        }
        if y + 2 < w && x > 0 && s[x - 1][y + 2] != '.' {
            new |= 8;
        }
        (me, new)
    };
    let dxy = [(1i32, 0i32), (0, 1), (1, -1), (-1, 1), (0, -1), (-1, 0)];
    for i in 0..h {
        for j in 0..w {
            for bits in 0..16 {
                let val = dp[i][j][bits];
                if val >= INF { continue; }
                let mut trans = vec![];
                if i + 1 < h {
                    let (me, new) = down(i, j, bits);
                    trans.push((i + 1, j, me, new));
                }
                if j + 1 < w {
                    let (me, new) = right(i, j, bits);
                    trans.push((i, j + 1, me, new));
                }
                for (x, y, me, new) in trans {
                    if (new & 51) != 0 {
                        let mut sum = 0;
                        for k in 0..6 {
                            let (dx, dy) = dxy[k];
                            if (new & 51 & 1 << k) == 0 { continue; }
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            sum += (s[nx][ny] as u8 - b'0') as i32;
                        }
                        for k in 0..6 {
                            let (dx, dy) = dxy[k];
                            if (new & 51 & 1 << k) == 0 { continue; }
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            let tmp = val + me + sum - (s[nx][ny] as u8 - b'0') as i32;
                            dp[x][y][(new & 12) | (1 << k & 15)].chmin(tmp);
                        }
                    } else {
                        let tmp = val + me;
                        dp[x][y][new & 12].chmin(tmp);
                    }
                }
            }
        }
    }
    println!("{}", dp[h - 1][w - 1][0]);
}
