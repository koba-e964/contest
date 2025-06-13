fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let a: Vec<i32> = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = a.len();
    let mut x = 0;
    for i in 0..n + 1 {
        let c = a.iter().filter(|&&v| v >= i as i32).count();
        if c >= i {
            x = i;
        } else {
            break;
        }
    }
    println!("{x}");
}
