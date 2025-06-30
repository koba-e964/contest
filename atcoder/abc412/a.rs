fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut ans = 0;
    for _ in 0..n {
        let ints = getline()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if ints[0] < ints[1] {
            ans += 1;
        }
    }
    println!("{ans}");
}
