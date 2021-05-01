#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }
trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

const INF: i64 = 1 << 60;

fn mtch(s: &[char], t: &[char], alt: usize, pre: usize, not: bool) -> bool {
    let n = s.len();
    if pre > n {
        return false;
    }
    for i in 0..pre {
        let c = if not { (b'0' + b'1' - t[i] as u8) as char } else { t[i] };
        if s[n - pre + i] != c {
            return false;
        }
    }
    if s[0] == '0' {
        return pre == 1 && alt == 0;
    }
    let mut b = 0;
    let mut c = '0';
    for i in 0..n - pre {
        if c != s[i] {
            b += 1;
        }
        c = s[i];
    }
    b == alt
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for case_nr in 1..=t {
        let s: Vec<char> = get_word().chars().collect();
        let t: Vec<char> = get_word().chars().collect();
        let n = s.len();
        let m = t.len();
        let mut dist = HashMap::new();
        // #alt of 1+0+..., len of prefix to preserved, pre is not-ed?
        let init = (0, m, false);
        let mut que = VecDeque::new();
        que.push_back((0, init));
        while let Some((d, st)) = que.pop_front() {
            if *dist.get(&st).unwrap_or(&INF) <= d {
                continue;
            }
            dist.insert(st, d);
            let mut trans = vec![];
            let (alt, pre, not) = st;
            let prelast0 = if pre > 0 {
                not ^ (t[pre - 1] == '0')
            } else {
                alt > 0 && alt % 2 == 0
            };
            let prefirst0 = pre > 0 && (not ^ (t[0] == '0'));
            // *2
            if prelast0 {
                if pre > 0 && (pre > 1 || alt >= 1) {
                    trans.push((alt, pre - 1, not));
                } else if alt >= 2 {
                    trans.push((alt - 1, 0, false));
                }
            }
            // not
            if alt + pre < max(n, m) + 2 {
                trans.push((alt + 1, pre, !not));
            }
            if alt == 0 && pre == 1 && !prefirst0 {
                trans.push((0, 1, !not)); // "0" necessarily
            }
            if alt == 1 && pre == 0 {
                trans.push((0, 1, t[0] != '0'));
            }
            if alt == 0 && pre == 1 && prefirst0 {
                trans.push((1, 0, false)); // "1+" necessarily
            }
            for &t in &trans {
                que.push_back((d + 1, t));
            }
        }
        let mut ans = INF;
        for (&st, &d) in &dist {
            let (alt, pre, not) = st;
            if false {
                eprint!("{} ", d);
                for i in 0..alt {
                    eprint!("{}+", 1 - i % 2);
                }
                for i in 0..pre {
                    eprint!("{}", if not { (b'0' + b'1' - t[i] as u8) as char } else { t[i] });
                }
                eprintln!();
            }
            if mtch(&s, &t, alt, pre, not) {
                ans.chmin(d);
            }
        }
        // eprintln!("st = {}", dist.len());
        if ans >= INF {
            puts!("Case #{}: IMPOSSIBLE\n", case_nr);
        } else {
            puts!("Case #{}: {}\n", case_nr, ans);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
