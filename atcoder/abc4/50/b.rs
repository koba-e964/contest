fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        for j in i + 1..n {
            c[i][j] = ints[j - i - 1];
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if c[i][j] + c[j][k] < c[i][k] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
