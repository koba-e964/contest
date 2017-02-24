/**
 * 2-SAT solver.
 * n: the number of variables (v_1, ..., v_n)
 * cons: constraints, given in 2-cnf
 * i (1 <= i <= n) means v_i, -i (1 <= i <= n) means not v_i.
 * Returns: an empty vector if there's no assignment that satisfies cons.
 * Otherwise, it returns an assignment that safisfies cons. (1: true, 0: false)
 * Dependencies: SCC.cpp
 * Verified by: Codeforces #400 D
 *              (http://codeforces.com/contest/776/submission/24942283)
 */
std::vector<int> two_sat(int n, const vector<pair<int, int> > &cons) {
  SCC scc(2 * n);
  for (int i = 0; i < cons.size(); ++i) {
    std::pair<int, int> c = cons[i];
    int x, y;
    if (c.first > 0) {
      x = c.first - 1 + n;
    } else {
      x = -c.first - 1;
    }
    if (c.second > 0) {
      y = c.second - 1;
    } else {
      y = -c.second - 1 + n;
    }
    scc.add_edge(x, y);
    scc.add_edge((y + n) % (2 * n), (x + n) % (2 * n));
  }
  scc.scc();
  std::vector<int> result(n);
  std::vector<int> top_ord = scc.top_order();
  REP(i, 0, n) {
    if (top_ord[i] == top_ord[i + n]) {
      return std::vector<int>();
    }
    result[i] = top_ord[i] > top_ord[i + n] ? 1 : 0;
  }
  return result;
}
