// p must be prime.
// Returns (B, H) where HB = A, rows(B) = rank(A)
// A: n * m, B: r * m, H: n * r
// Verified by: https://yukicoder.me/submissions/899530
fn find_basis_mod_p(a: &[Vec<i64>], p: i64) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let n = a.len();
    let m = a[0].len();
    let aold = a.clone();
    let mut a = a.to_vec();
    let mut pos = vec![];
    let mut r = 0;
    for i in 0..n {
        for j in 0..r {
            let idx = pos[j];
            assert_eq!(a[j][idx], 1);
            let val = a[i][idx];
            for k in 0..m {
                a[i][k] -= a[j][k] * val % p;
                if a[i][k] < 0 {
                    a[i][k] += p;
                }
            }
        }
        let mut c = 0;
        while c < m && a[i][c] == 0 {
            c += 1;
        }
        if c >= m {
            continue;
        }
        a.swap(r, i);
        pos.push(c);
        let aic = a[r][c];
        let aicinv = powmod(aic, p - 2, p);
        a[r][c] = 1.into();
        for j in c + 1..m {
            a[r][j] *= aicinv;
            a[r][j] %= p;
        }
        for j in r + 1..n {
            let ajc = a[j][c];
            a[j][c] = 0;
            for k in c + 1..m {
                let val = ajc * a[r][k] % p;
                a[j][k] -= val;
                if a[j][k] < 0 {
                    a[j][k] += p;
                }
            }
        }
        r += 1;
    }
    let mut tr = vec![vec![0; r]; n];
    for i in 0..n {
        let mut cur = aold[i].clone();
        for j in 0..r {
            let idx = pos[j];
            let val = cur[idx];
            tr[i][j] = val;
            for k in idx..m {
                cur[k] -= val * a[j][k] % p;
                if cur[k] < 0 {
                    cur[k] += p;
                }
            }
        }
        assert!(cur.iter().all(|&x| x == 0), "{:?} {:?} {:?}", cur, a, aold);
    }
    (a[..r].to_vec(), tr)
}
