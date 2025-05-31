fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().trim().bytes().map(|b| (b - b'0') as i32).collect::<Vec<i32>>();
    let n = s.len();
    s.push(0);
    let mut ans = n as i32;
    for i in 0..n {
        ans += (s[i] + 10 - s[i + 1]) % 10;
    }
    println!("{ans}");
}
