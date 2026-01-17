fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let s = getline();
    let t = getline();
    let q = getline().trim().parse::<i32>().unwrap();
    for _ in 0..q {
        let mut a = true;
        let mut b = true;
        for c in getline().trim().chars() {
            if !s.contains(c) {
                a = false;
            }
            if !t.contains(c) {
                b = false;
            }
        }
        println!("{}", match (a, b) {
            (true, false) => "Takahashi",
            (false, true) => "Aoki",
            _ => "Unknown",
        });
    }
}
