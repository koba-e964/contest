fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let t = getline().trim().bytes().collect::<Vec<_>>();
    let n = s.len();
    let m = t.len();
    let mut ans = 1_000_000;
    for i in 0..=n - m {
        let mut tmp = 0;
        for j in 0..m {
            let x = s[i + j] as u8 - b'0';
            let y = t[j] as u8 - b'0';
            tmp += (x + 10 - y) as i64 % 10;
        }
        ans = ans.min(tmp);
    }
    println!("{ans}");
}
