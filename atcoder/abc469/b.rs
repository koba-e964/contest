fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let mut s = getline().trim().chars().collect::<Vec<_>>();
    s.insert(0, 'x');
    s.push('x');
    let mut ans = 0;
    for x in s.windows(3) {
        if x.iter().all(|&x| x == 'x') {
            ans += 1;
        }
    }
    println!("{ans}");
}
