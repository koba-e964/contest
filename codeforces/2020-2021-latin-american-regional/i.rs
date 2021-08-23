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

const MOD: i64 = 1_000_000_007;

type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn inn((ax, ay): P, (bx, by): P) -> Coord {
    ax * bx + ay * by
}

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}

// Tags: convex-hull, argument-sort
fn main() {
    input! {
        xh: i64, yh: i64,
        xg: i64, yg: i64,
        n: usize,
        xy: [(i64, i64); n],
    }
    let g = (xg, yg);
    let h = sub((xh, yh), g);
    let mut xy = xy;
    for p in &mut xy {
        *p = sub(*p, g);
    }
    // arg-sort
    let quad = |p: P| {
        let i = inn(p, h);
        let o = det(h, p);
        if o == 0 {
            if i < 0 {
                0
            } else {
                2
            }
        } else if o < 0 {
            1
        } else {
            3
        }
    };
    xy.sort_by(|&a, &b| {
        let qa = quad(a);
        let qb = quad(b);
        qa.cmp(&qb).then_with(|| {
            0.cmp(&det(a, b))
        })
    });
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..i {
            // Is h inside P(j)P(i)?
            if det(sub(xy[i], xy[j]), sub(h, xy[j])) <= 0 {
                continue;
            }
            // init
            if det(xy[j], h) > 0 && det(xy[j], sub(xy[i], xy[j])) > 0 {
                dp[i][j] += 1;
            }
            // step
            for k in 0..j {
                if det(sub(xy[j], xy[k]), sub(xy[i], xy[j])) > 0 {
                    dp[i][j] = (dp[i][j] + dp[j][k]) % MOD;
                }
            }
        }
    }
    let mut tot = 0;
    for i in 0..n {
        for j in 0..i {
            if det(xy[i], h) < 0 && det(xy[i], sub(xy[j], xy[i])) < 0 {
                tot = (tot + dp[i][j]) % MOD;
            }
        }
    }
    println!("{}", tot);
}
