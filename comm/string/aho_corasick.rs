// The Aho-Corasick automaton construction.
// Complexity: \sum |pat| * alpha
// Verified by: https://yukicoder.me/problems/no/1269
//              https://yukicoder.me/submissions/571533
fn aho_corasick(pat: &[Vec<usize>], alpha: usize)
                        -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut st = vec![vec![std::usize::MAX; alpha]];
    let mut fin = vec![false];
    let mut back = vec![0];
    for p in pat {
        let mut cur = 0;
        for i in 0..p.len() {
            let c = p[i];
            if st[cur][c] == std::usize::MAX {
                st.push(vec![std::usize::MAX; alpha]);
                fin.push(false);
                back.push(std::usize::MAX);
                st[cur][c] = st.len() - 1;
            }
            cur = st[cur][c];
        }
        fin[cur] = true;
    }
    // fill in back links
    // Note: states are *not necessarily* topologically sorted!
    // Therefore, we need to use a queue.
    let mut que = std::collections::VecDeque::new();
    que.push_back(0);
    while let Some(i) = que.pop_front() {
        assert_ne!(back[i], std::usize::MAX);
        if fin[back[i]] {
            fin[i] = true;
        }
        for j in 0..alpha {
            if st[i][j] != std::usize::MAX {
                let nxt = st[i][j];
                que.push_back(nxt);
                if i == 0 {
                    back[nxt] = 0;
                } else {
                    let mut cur = back[i];
                    while st[cur][j] == std::usize::MAX && cur > 0 {
                        assert_ne!(back[cur], std::usize::MAX);
                        cur = back[cur];
                    }
                    back[nxt] = [0, st[cur][j]][usize::from(st[cur][j] != std::usize::MAX)];
                }
            } 
        }
    }
    // fill in vacant transitions
    for i in 0..st.len() {
        for j in 0..alpha {
            if st[i][j] == std::usize::MAX {
                let mut cur = back[i];
                while st[cur][j] == std::usize::MAX && cur > 0 {
                    cur = back[cur];
                }
                st[i][j] = [0, st[cur][j]][usize::from(st[cur][j] != std::usize::MAX)];
            }
        }
    }
    (st, fin)
}
