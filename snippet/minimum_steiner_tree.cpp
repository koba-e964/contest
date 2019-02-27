const int inf = 1e9;
// Dreyfus-Wagner. O(4^k)
int steiner_tree(const vector<vector<PI> > &g, const VI &term) {
  int n = g.size();
  int k = term.size();
  vector<vector<int> > dist(n, vector<int>(n, inf));
  REP(i, 0, n) {
    REP(j, 0, g[i].size()) {
      PI wc = g[i][j];
      int w = wc.first, c = wc.second;
      dist[i][w] = min(dist[i][w], c);
    }
  }
  REP(i, 0, n) dist[i][i] = 0;
  REP(l, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
        dist[i][j] = min(dist[i][j], dist[i][l] + dist[l][j]);
      }
    }
  }
  vector<vector<int> > dp(n, vector<int>(1 << k, inf));
  // base case: |bits| = 1
  REP(i, 0, k) {
    REP(j, 0, n) {
      dp[j][1 << i] = min(dp[j][1 << i], dist[term[i]][j]);
    }
  }
  REP(bits, 1, 1 << k) {
    // if popcount(bits) == 1 then skip
    if ((bits & -bits) == bits) continue;
    REP(i, 0, n) {
      REP(sub, 1, bits) {
        if ((sub & bits) != sub) continue;
        dp[i][bits] = min(dp[i][bits], dp[i][sub] + dp[i][bits - sub]);
      }
    }
    REP(i, 0, n) {
      REP(j, 0, n) {
        dp[i][bits] = min(dp[i][bits], dp[j][bits] + dist[i][j]);
      }
    }
  }
  int mi = inf;
  REP(i, 0, n) {
    mi = min(mi, dp[i][(1 << k) - 1]);
  }
  return mi;   
}
