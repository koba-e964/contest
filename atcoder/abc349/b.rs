fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut f = [0; 26];
    for c in getline().trim().bytes() {
        f[(c - b'a') as usize] += 1;
    }
    let mut g = vec![0; 101];
    for i in 0..26 {
        g[f[i] as usize] += 1;
    }
    println!("{}", if g[1..].iter().all(|&x| x == 0 || x == 2) { "Yes" } else { "No" });
}
