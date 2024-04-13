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

fn gcd(x: i64, y: i64) -> i64 {
    if x < y { gcd(y, x) } else if y == 0 { x } else { gcd(y, x % y) }
}

fn win(n: i64) -> bool {
    let mut x = 2;
    while x < n && n % x == 0 { x += 1; }
    gcd(n, x) == 1
}

// https://yukicoder.me/problems/no/2725 (3)
// 実験の結果、n = 2, 6, 18, 30, 42, 54, ... で後手必勝、それ以外で先手必勝であることがわかる。
// これは n = 2, 6 + 12i とみなすことができる。
// 6+12i が後手必勝なのは、先手は初手で奇数に行くしかなく、後手は 4 に行けば勝てるためである。
// 他の場合で先手必勝なのもほとんど明らか。(12|N なら初手 5、2|N and not(3|N) なら初手 3 など)
// -> WA。420, 840 が後手必勝であることを見落としていた。
// 最小の約数でない正の整数が重要で、420 => 8, 840 => 9 だが、
// これは元々の数と互いに素ではないので初手で行けない。
fn main() {
    let t: i32 = get();
    for _ in 0..t {
        let n: i64 = get();
        println!("{}", if win(n) {
            "K"
        } else {
            "P"
        });
    }
}
