fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let s = (0..n).map(|_| getline().trim().to_string()).collect::<Vec<_>>();
    let vals = getline().trim().split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let x = vals[0].parse::<usize>().unwrap() - 1;
    println!("{}", if s[x] == vals[1] {
        "Yes"
    } else {
        "No"
    });
}
