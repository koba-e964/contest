use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Counter-clockwise.
// 1 -> (0, 0), 2 -> (1, 0), 3 -> (1, 1), ...
#[derive(Debug, Copy, Clone)]
struct Spiral(i64, i64);
impl Spiral {
    fn sqrt(a: i64) -> i64 {
        let mut pass = 0;
        let mut fail = std::cmp::min(a, 1 << 30) + 1;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            let tmp = mid * mid;
            if tmp <= a {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass
    }
    pub fn from(a: i64) -> Self {
        if a == 1 {
            return Spiral(0, 0);
        }
        let x = Self::sqrt(a - 1);
        let lay = (x + 1) / 2;
        let rem = a - 4 * lay * lay + 4 * lay - 2;
        if x % 2 == 1 {
            // [0, 2 * lay - 1]
            if rem < 2 * lay {
                return Spiral(lay, rem - lay + 1);
            }
            // [2 * lay - 1, 4 * lay - 1]
            return Spiral(lay - (rem - 2 * lay + 1), lay);
        }
        // [4 * lay - 1, 6 * lay - 1]
        if rem < 6 * lay {
            return Spiral(-lay, lay - rem + 4 * lay - 1);
        }
        // [6 * lay - 1, 8 * lay - 1]
        Spiral(-lay + rem - 6 * lay + 1, -lay)
    }
    #[allow(unused)]
    pub fn as_int(self) -> i64 {
        let Spiral(x, y) = self;
        let lay = std::cmp::max(x.abs(), y.abs());
        let rem = if y == -lay {
            // [6 * lay - 1, 8 * lay - 1]
            6 * lay - 1 + x + lay
        } else if x == -lay {
            // [4 * lay - 1, 6 * lay - 1]
            4 * lay - 1 + lay - y
        } else if y == lay {
            // [2 * lay - 1, 4 * lay - 1]
            2 * lay - 1 + lay - x
        } else {
            y + lay - 1
        };
        4 * lay * (lay - 1) + 2 + rem
    }
}

fn append(mut from: Spiral, to: Spiral, ans: &mut String) {
    assert_ne!((from.0 + from.1 + to.0 + to.1) % 2, 0);
    let mut even = true;
    while from.0 > to.0 {
        from.0 -= 1;
        ans.push('L');
        if even {
            ans.push('R');
            ans.push('L');
        }
        even = !even;
    }
    while from.0 < to.0 {
        from.0 += 1;
        ans.push('R');
        if even {
            ans.push('L');
            ans.push('R');
        }
        even = !even;
    }
    while from.1 > to.1 {
        from.1 -= 1;
        ans.push('D');
        if even {
            ans.push('U');
            ans.push('D');
        }
        even = !even;
    }
    while from.1 < to.1 {
        from.1 += 1;
        ans.push('U');
        if even {
            ans.push('D');
            ans.push('U');
        }
        even = !even;
    }
}

// https://yukicoder.me/problems/no/1315 (3.5)
// 1 から移動することを考える。距離が (10^5 以下の) 奇数であれば、ababcdcdefef... と移動することでペナルティを 0 にできる。
// 距離が偶数の場合、1 -> 2 -> 3 -> 4 (RUL) と進んで 4 から開始することにすれば、4 からの距離は奇数であるため、奇数の場合と同様にできる。
// 任意の 2 地点 S, T については、S -> 1 -> T と移動すれば良い。
// 詳しく書くと、1 から T までの距離が偶数であれば S -> 1 の後どこかから T までの道を二重にすれば良い。
// 1 から T までの距離が奇数であれば、 S -> 1 -> 2 -> 3 -> 4 -> 1 -> 4 -> (二重化された道) とすれば、
// 4 から T までの距離は偶数なので問題ない。
// Tags: spiral
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    let sn = Spiral::from(n);
    let sm = Spiral::from(m);
    let mut ans = "".to_string();
    if (sn.0 + sn.1 + sm.0 + sm.1) % 2 != 0 {
        append(sn, sm, &mut ans);
    } else if (sn.0 + sn.1) % 2 == 0 {
        append(sn, Spiral::from(2), &mut ans);
        ans += "LRUL";
        append(Spiral::from(4), sm, &mut ans);
    } else {
        append(sn, Spiral::from(1), &mut ans);
        ans += "RULDUD";
        append(Spiral::from(1), sm, &mut ans);
    }
    println!("{}\n{}\n{}", 0, ans.len(), &ans);
}
