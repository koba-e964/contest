fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut freq = vec![false; 101];
    for a in a {
        freq[a] ^= true;
    }
    let mut ans = 0;
    for i in 0..101 {
        if freq[i] {
            ans += i;
        }
    }
    println!("{ans}");
}
