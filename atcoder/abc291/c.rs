fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut v = vec![(0, 0)];
    let mut x = 0;
    let mut y = 0;
    for c in getline().trim().chars() {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => panic!(),
        }
        v.push((x, y));
    }
    let n = v.len();
    v.sort(); v.dedup();
    println!("{}", if v.len() != n { "Yes" } else { "No" });
}
