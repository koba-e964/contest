use std::cmp::*;
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let k: usize = get();
    const W: usize = 2_000_001;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut nums = vec![];
    for i in (5..W / 2).rev() {
        if nums.len() >= 125 {
            break;
        }
        if pr[i] {
            nums.push(i - 3);
        }
    }
    let mut f = vec![0; 126];
    let mut asset = vec![W; 126];
    for i in 1..W / 4 {
        let v = 2 * i + 1;
        let c = (0..125).filter(|&j| pr[nums[j] + v]).count();
        f[c] += 1;
        asset[c] = v;
    }
    let mut rem = k;
    let mut ans = nums.clone();
    while rem > 0 {
        let mut r = min(125, rem);
        while asset[r] == W {
            r -= 1;
        }
        rem -= r;
        ans.push(asset[r]);
    }
    puts!("{}\n", ans.len());
    putvec!(ans);
}
