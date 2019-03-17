// returns (rank(a), dim ker(a))
fn gauss_elim_gf2(a: &mut [Vec<i32>], b: &mut [i32]) -> Option<(usize, usize)> {
    let n = a.len();
    let m = b.len();
    let mut row = 0;
    for col in 0..m {
        if row >= n { break; }
        let mut row2 = None;
        for i in row..n {
            if a[i][col] == 1 {
                row2 = Some(i);
                break;
            }
        }
        if row2.is_none() { continue; }
        let row2 = row2.unwrap();
        a.swap(row, row2);
        for k in row + 1..n {
            if a[k][col] == 1 {
                for j in 0..m {
                    a[k][j] ^= a[row][j];
                }
            }
        }
        if b[col] == 1 {
            for j in 0..m {
                b[j] ^= a[row][j];
            }
        }
        row += 1;
    }
    for i in 0..m {
        if b[i] != 0 { return None; }
    }
    Some((row, n - row))
}
