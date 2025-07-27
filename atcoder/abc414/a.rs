fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let n = ints[0];
    let l = ints[1];
    let r = ints[2];
    let mut ans = 0;
    for _ in 0..n {
        let ints = getline()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let x = ints[0];
        let y = ints[1];
        if x <= l && y >= r {
            ans += 1;
        }
    }
    println!("{ans}");
}
