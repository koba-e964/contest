fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let s: i32 = a.iter().sum();
    for i in 0..n {
        if ints[1] == s - a[i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
