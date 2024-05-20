// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn place(board: Vec<usize>, h: usize, w: usize, i: usize, j: usize, a: usize, b: usize) -> Option<Vec<usize>> {
    let mut board = board;
    if i + a > h || j + b > w {
        return None;
    }
    let pat = (1 << (j + b)) - (1 << j);
    for y in i..i + a {
        if board[y] & pat != 0 {
            return None;
        }
        board[y] |= pat;
    }
    Some(board)
}

fn rec(n: usize, h: usize, w: usize, ab: &[(usize, usize)], mut board: Vec<usize>, used: usize) -> Result<(), ()> {
    if board.iter().all(|&x| x == (1 << w) - 1) {
        return Err(());
    }
    if used == (1 << n) - 1 {
        return Ok(());
    }
    let mut mi = (h, w);
    for i in 0..h {
        for j in 0..w {
            if board[i] & 1 << j != 0 {
                continue;
            }
            mi = mi.min((i, j));
        }
    }
    for i in 0..n {
        if (used & 1 << i) != 0 {
            continue;
        }
        let (a, b) = ab[i];
        if let Some(new_board) = place(board.clone(), h, w, mi.0, mi.1, a, b) {
            rec(n, h, w, ab, new_board, used | 1 << i)?
        }
        if let Some(new_board) = place(board.clone(), h, w, mi.0, mi.1, b, a) {
            rec(n, h, w, ab, new_board, used | 1 << i)?
        }
    }
    Ok(())
}

fn main() {
    input! {
        n: usize, h: usize, w: usize,
        ab: [(usize, usize); n],
    }
    println!("{}", if rec(n, h, w, &ab, vec![0; h], 0).is_err() { "Yes" } else { "No" });
}
