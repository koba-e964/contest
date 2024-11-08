fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut occ = vec![vec![]; 26];
    for (i, c) in getline().trim().bytes().enumerate() {
        occ[(c - b'A') as usize].push(i as i64);
    }
    let mut ans = 0;
    for occ in occ {
        let m = occ.len();
        for i in 0..m {
            ans += occ[i] * (2 * i as i64 - m as i64 + 1) - i as i64;
        }
    }
    println!("{ans}");
}
