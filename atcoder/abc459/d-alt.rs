fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// Similar problems: https://atcoder.jp/contests/arc209/tasks/arc209_b
fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let s = getline().trim().chars().collect::<Vec<_>>();
        let n = s.len();
        let mut f = vec![0; 26];
        for &c in &s {
            f[(c as u8 - b'a') as usize] += 1;
        }
        let mut ma = (0, 0);
        for i in 0..26 {
            ma = ma.max((f[i], i));
        }
        if ma.0 * 2 - 1 > n {
            println!("No");
            continue;
        }
        println!("Yes");
        let mut ans = "".to_string();
        let mut last = 26;
        for _ in 0..n {
            let mut ma = (0, 0);
            for i in 0..26 {
                if i != last {
                    ma = ma.max((f[i], i));
                }
            }
            let c = ('a' as u8 + ma.1 as u8) as char;
            ans.push(c);
            f[ma.1] -= 1;
            last = ma.1;
        }
        println!("{ans}");
    }
}
