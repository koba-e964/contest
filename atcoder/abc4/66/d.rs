fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut row = vec![None; n];
    let mut col = vec![None; n];
    for _ in 0..m {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        let [a, b] = ints[..] else { panic!() };
        // remove
        if let Some((x, y)) = row[a].take() {
            if col[y] == Some((x, y)) {
                col[y] = None;
            }
        }
        if let Some((x, y)) = col[b].take() {
            if row[x] == Some((x, y)) {
                row[x] = None;
            }
        }

        // insert
        row[a] = Some((a, b));
        col[b] = Some((a, b));
    }
    let mut ans = 0;
    for i in 0..n {
        ans += row[i].is_some() as i32;
    }
    println!("{ans}");
}
