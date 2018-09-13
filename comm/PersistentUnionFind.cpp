/*
 * Persistent Union Find tree.
 * Reference: https://misteer.hatenablog.com/entry/persistentUF
 * Header requirement: algorithm, utility, vector
 * Verified by https://beta.atcoder.jp/contests/agc002/submissions/3185458
 */
class PersistentUnionFind {
public:
  static const int inf = 1e8;
  typedef std::pair<int, int> pii;
  std::vector<int> par, time, rank;
  std::vector<std::vector<pii> > num; // [(time, size of this component)]
  int now;
  PersistentUnionFind(int n):
    par(n), time(n, inf), rank(n, 0), num(n), now(0) {
    for (int i = 0; i < n; ++i) {
      par[i] = i;
      num[i].push_back(pii(-1, 1));
    }
  }
  int root(int x, int t) const {
    while (1) {
      if (time[x] > t) return x;
      x = par[x];
    }
  }
  // returns the current time
  int unite(int x, int y) {
    x = this->root(x, now);
    y = this->root(y, now);
    if (x == y) return now;
    ++now;
    if (rank[x] <= rank[y]) std::swap(x, y);
    par[y] = x;
    time[y] = now;
    rank[x] = std::max(rank[x], rank[y] + 1);
    num[x].push_back(pii(now, num[x].back().second + num[y].back().second));
    return now;
  }
  int size(int x, int t) const {
    x = root(x, t);
    return (upper_bound(num[x].begin(), num[x].end(), pii(t, inf)) - 1)->second;
  }
};
const int PersistentUnionFind::inf;
