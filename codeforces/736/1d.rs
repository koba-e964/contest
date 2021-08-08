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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

// TODO: hard
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut buc = vec![vec![0i64; 4]; 4];
    for i in 0..n {
        let (x, y) = xy[i];
        buc[x as usize % 4][y as usize % 4] += 1;
    }
    let mut tot = 0;
    eprintln!("buc = {:?}", buc);
    for i in 0..4 {
        for j in 0..4 {
            let c = buc[i][j];
            for k in 0..4 {
                for l in 0..4 {
                    if (i + k) % 2 != 0 || (j + l) % 2 != 0 {
                        continue;
                    }
                    if (i, j) == (k, l) {
                        tot += c * (c - 1) * (c - 2) / 6;
                    } else {
                        tot += c * (c - 1) / 2 * buc[k][l];
                    }
                }
            }
        }
    }
    // odd
    eprintln!("tot = {}", tot);
    for i in 0..n {
        let (xi, yi) = xy[i];
        let mut g1 = vec![vec![0i64; 4]; 4];
        let mut g3 = vec![vec![0i64; 4]; 4];
        for j in 0..n {
            let (xj, yj) = xy[j];
            if i == j {
                continue;
            }
            let dx = ((xj - xi % 4 + 4) % 4) as usize;
            let dy = ((yj - yi % 4 + 4) % 4) as usize;
            let g = gcd((xi - xj).abs(), (yi - yj).abs()) % 4;
            if g == 1 {
                g1[dx][dy] += 1;
            }
            if g == 3 {
                g3[dx][dy] += 1;
            }
        }
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    if (a + c) % 2 != 0 {
                        continue;
                    }
                    for d in 0..4 {
                        if (b + d) % 2 != 0 {
                            continue;
                        }
                        let g = gcd((a as i32 - c as i32).abs(), (b as i32 - d as i32).abs());
                        let s2 = (a as i32 * d as i32 - b as i32 * c as i32 + 16) % 4;
                        if s2 % 2 != 0 {
                            continue;
                        }
                        if (s2 + g) % 4 == 0 {
                            tot += g1[a][b] * g3[c][d];
                        }
                        if (s2 + g) % 4 == 2 {
                            if (a, b) == (c, d) {
                                let k = g1[a][b];
                                tot += k * (k - 1) / 2;
                                let k = g3[a][b];
                                tot += k * (k - 1) / 2;
                            } else {
                                if (a, b) < (c, d) {
                                    tot += g1[a][b] * g1[c][d] + g3[a][b] * g3[c][d];
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", tot);
}
