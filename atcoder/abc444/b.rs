fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn digitsum(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        ans += x % 10;
        x /= 10;
    }
    ans
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ans = 0;
    for i in 1..=ints[0] {
        if digitsum(i) == ints[1] {
            ans += 1;
        }
    }
    println!("{ans}");
}
