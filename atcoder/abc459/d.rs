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
        let mut buc = vec!["".to_string(); ma.0];
        let mut tmp = vec![];
        for i in 0..26 {
            let x = (i + ma.1) % 26;
            let c = ('a' as u8 + x as u8) as char;
            for _ in 0..f[x] {
                tmp.push(c);
            }
        }
        for i in 0..tmp.len() {
            buc[i % ma.0].push(tmp[i]);
        }
        let mut ans = "".to_string();
        for b in buc {
            ans += &b;
        }
        println!("{ans}");
    }
}
