use std::collections::*;
use std::io::{Write, BufWriter};
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

const INF: i32 = 1 << 28;

fn make_dist(s: &[Vec<char>], dist: &mut [Vec<(i32, u8)>]) {
    let h = s.len();
    let w = s[0].len();
    let dxy = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    let mut que = VecDeque::new();
    que.push_back((0, 0, 0));
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y].0 <= d { continue; }
        dist[x][y].0 = d;
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w { continue; }
            if s[nx][ny] == '#' { continue; }
            que.push_back((d + 1, nx, ny));
        }
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' { continue; }
            if i + 1 < h && i > 0 && s[i + 1][j] == '.' && s[i - 1][j] == '.' {
                dist[i][j].1 |= 1;
            }
            if j + 1 < w && j > 0 && s[i][j + 1] == '.' && s[i][j - 1] == '.' {
                dist[i][j].1 |= 2;
            }
        }
    }
}

fn recover_path(dist: &[Vec<(i32, u8)>], mut x: usize, mut y: usize, d: usize) -> Vec<char> {
    assert_eq!((dist[x][y].0 as usize + d) % 2, 0);
    assert!(dist[x][y].0 as usize <= d);
    let oldx = x;
    let oldy = y;
    let h = dist.len();
    let w = dist[0].len();
    let mut ans = vec![];
    let mut cur = dist[x][y].0;
    let dxy = [(0i32, 1i32, 'L'), (0, -1, 'R'), (1, 0, 'U'), (-1, 0, 'D')];
    while cur > 0 {
        let mut nxt = (h, w);
        for &(dx, dy, ch) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w { continue; }
            if dist[nx][ny].0 == cur - 1 {
                nxt = (nx, ny);
                ans.push(ch);
                break;
            }
        }
        cur -= 1;
        x = nxt.0;
        y = nxt.1;
    }
    ans.reverse();
    for i in 0..d - dist[oldx][oldy].0 as usize {
        if (dist[oldx][oldy].1 & 2) != 0 {
            ans.push(['R', 'L'][i % 2]);
        } else {
            ans.push(['D', 'U'][i % 2]);
        }
    }
    ans
}

// https://yukicoder.me/problems/no/2411 (3)
// L 番目から R 番目までは偶数長であり、なおかつ delta が 0 である必要がある。
// これが達成できるためには、L 番目の直前に縦 ３ マスあるいは横 3 マスの空きの中央に幾つ用があり、
// そのあと R 番目の直後から K 番目に (H, W) に到着できる必要がある。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize, k: usize, l: usize1, r: usize,
        s: [chars; h],
    }
    if (r - l) % 2 != 0 || (h + w + k) % 2 != 0 {
        puts!("No\n");
        return;
    }
    if h + w - 2 > k - (r - l) {
        puts!("No\n");
        return;
    }
    let mut dist1 = vec![vec![(INF, 0); w]; h];
    make_dist(&s, &mut dist1);
    let mut srev = s.clone();
    srev.reverse();
    for s in &mut srev {
        s.reverse();
    }
    let mut dist2 = vec![vec![(INF, 0); w]; h];
    make_dist(&srev, &mut dist2);
    let mut pivot = (h, w);
    'outer:
    for i in 0..h {
        for j in 0..w {
            if dist1[i][j].1 != 0 && (dist1[i][j].0 + l as i32) % 2 == 0
                && dist1[i][j].0 <= l as i32
                && dist2[h - 1 - i][w - 1 - j].0 <= (k - r) as i32 {
                assert_eq!((dist2[h - 1 - i][w - 1 - j].0 + (k - r) as i32) % 2, 0);
                pivot = (i, j);
                break 'outer;
            }
        }
    }
    if pivot == (h, w) {
        puts!("No\n");
        return;
    }
    let mut ans = recover_path(&dist1, pivot.0, pivot.1, l);
    for i in l..r {
        if (dist1[pivot.0][pivot.1].1 & 2) != 0 {
            ans.push(['R', 'L'][(i - l) % 2]);
        } else {
            ans.push(['D', 'U'][(i - l) % 2]);
        }
    }
    let mut tmp = recover_path(&dist2, h - 1 - pivot.0, w - 1 - pivot.1, k - r);
    tmp.reverse();
    ans.extend_from_slice(&tmp);
    puts!("Yes\n");
    puts!("{}\n", ans.into_iter().collect::<String>());
}
