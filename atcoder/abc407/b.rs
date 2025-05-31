fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let xy = getline().trim().split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let x = xy[0];
    let y = xy[1];
    let mut ans = 0;
    for a in 1..=6 {
        for b in 1..=6 {
            if a + b >= x || (a - b).abs() >= y {
                ans += 1;
            }
        }
    }
    println!("{}", ans as f64 / 36.0);
}
