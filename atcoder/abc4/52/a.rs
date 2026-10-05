fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let s = vec![(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)];
    println!("{}", if s.contains(&(a[0], a[1])) {
        "Yes"
    } else {
        "No"
    });
}
