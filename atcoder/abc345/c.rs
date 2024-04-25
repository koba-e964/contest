fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let n = s.len() as i64;
    let mut f = [0; 26];
    for s in s {
        f[(s - b'a') as usize] += 1i64;
    }
    let mut ans = n * (n - 1) / 2;
    let mut same = 0;
    for i in 0..26 {
        ans -= f[i] * (f[i] - 1) / 2;
        if f[i] >= 2 {
            same = 1;
        }
    }
    println!("{}", ans + same);
}
