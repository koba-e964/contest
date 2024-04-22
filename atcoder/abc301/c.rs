fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let t = getline().trim().bytes().collect::<Vec<_>>();
    let mut a1 = 0;
    let mut a2 = 0;
    let mut f = [0; 26];
    for c in s {
        if c == b'@' {
            a1 += 1;
            continue;
        }
        let idx = (c - b'a') as usize;
        f[idx] += 1;
    }
    for c in t {
        if c == b'@' {
            a2 += 1;
            continue;
        }
        let idx = (c - b'a') as usize;
        f[idx] -= 1;
    }
    let code = b"atcoder";
    let mut b1 = 0;
    let mut b2 = 0;
    for i in 0..26 {
        if !code.contains(&(b'a' + i as u8)) {
            if f[i] != 0 {
                println!("No");
                return;
            }
        }
        if f[i] > 0 {
            b2 += f[i];
        } else {
            b1 -= f[i];
        }
    }
    println!("{}", if b1 <= a1 && b2 <= a2 { "Yes" } else { "No" });
}
