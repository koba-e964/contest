fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let a: Vec<i32> = getline().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = a.len();
    const W: usize = 400_000;
    let mut ans = 0i64;
    for k in 1..=W / 3 {
        let l = n.min(W / k + 1);
        let mut hm = std::collections::HashMap::new();
        for i in 0..l {
            let v = a[i] - k as i32 * (i as i32 + 1);
            if let Some(&c) = hm.get(&-v) {
                ans += c;
            }
            *hm.entry(v).or_insert(0i64) += 1;
        }
    }
    println!("{ans}");
}
