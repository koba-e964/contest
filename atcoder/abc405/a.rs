fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let l = getline().trim().to_string();
    let l = l.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (lo, hi) = if l[1] == 1 {
        (1600, 3000)
    } else {
        (1200, 2400)
    };
    println!("{}", if lo <= l[0] && l[0] < hi {
        "Yes"
    } else {
        "No"
    });
}
