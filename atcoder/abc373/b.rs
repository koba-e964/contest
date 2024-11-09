fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().bytes().collect::<Vec<_>>();
    let mut p = [0; 26];
    for i in 0..26 {
        let idx = (s[i] - b'A') as usize;
        p[idx] = i as i32;
    }
    let mut ans = 0;
    for i in 0..25 {
        ans += (p[i] - p[i + 1]).abs();
    }
    println!("{ans}");
}
