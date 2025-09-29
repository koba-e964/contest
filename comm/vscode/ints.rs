getline().trim().split_whitespace()
    .map(|x| x.parse::<$1>().unwrap())
    .collect::<Vec<_>>();
