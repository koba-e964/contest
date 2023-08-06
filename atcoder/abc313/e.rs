fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}


const MOD: i64 = 998_244_353;

fn main() {
    getline();
    let mut v = vec![];
    let mut c = 0;
    for ch in getline().trim().bytes() {
        if ch == b'1' {
            c += 1;
        } else {
            if c == 0 && !v.is_empty() {
                println!("-1");
                return;
            }
            let idx = (ch - b'0') as i64;
            v.push((idx, c));
            c = 0;
        }
    }
    let mut ans = c;
    for &(tip, len) in v.iter().rev() {
        if len == 0 {
            ans = (ans + 1) % MOD;
            break;
        }
        let len = (len + (tip - 1) * (ans + 1)) % MOD;
        ans = (ans + len + 1) % MOD;
    }
    println!("{}", (ans + MOD - 1) % MOD);
}
