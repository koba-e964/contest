fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    a.sort(); a.reverse();
    for i in 0..3 {
        print!("{}", a[i]);
    }
    println!();
}
