fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn f(s: Vec<u8>) -> Vec<u8> {
    let n = s.len();
    let mut ans = vec![0; n - 1];
    for i in 0..n - 1 {
        ans[i] = (s[i + 1] + 26 - s[i]) % 26;
    }
    ans
}

fn main() {
    let s: Vec<_> = getline().trim().bytes().collect();
    let t: Vec<_> = getline().trim().bytes().collect();
    println!("{}", if f(s) == f(t) { "Yes" } else { "No" });
}
