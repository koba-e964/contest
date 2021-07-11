fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline();
    let n = s.trim().len() as i64;
    let mut last = '+';
    let mut ans = n * (n + 1) / 2;
    let mut l = 0;
    for c in s.trim().chars() {
        if last != c {
            last = c;
            l = 0;
        }
        l += 1;
        ans -= l;
    }
    println!("{}", ans);
}
