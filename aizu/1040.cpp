#include <algorithm>
#include <cassert>
#include <iomanip>
#include <queue>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


const int inf = 1e9;
// Dreyfus-Wagner. O(4^k)
// https://kopricky.github.io/code/Academic/steiner_tree.html
int steiner_tree(const vector<vector<PI> > &g, const VI &term) {
  int n = g.size();
  int k = term.size();
  vector<vector<int> > dp(n, vector<int>(1 << k, inf));
  // base case: |bits| = 1
  REP(i, 0, k) {
    dp[term[i]][1 << i] = 0;
  }
  REP(bits, 1, 1 << k) {
    REP(i, 0, n) {
      REP(sub, 1, bits) {
        if ((sub & bits) != sub) continue;
        dp[i][bits] = min(dp[i][bits], dp[i][sub] + dp[i][bits - sub]);
      }
    }
    priority_queue<PI, vector<PI>, greater<PI> > que;
    REP(i, 0, n) {
      if (dp[i][bits] < inf) {
        que.push(PI(dp[i][bits], i));
        dp[i][bits] = inf;
      }
    }
    while (not que.empty()) {
      PI dv = que.top(); que.pop();
      int d = dv.first;
      int v = dv.second;
      if (dp[v][bits] <= d) continue;
      dp[v][bits] = d;
      REP(j, 0, g[v].size()) {
        PI wc = g[v][j];
        int w = wc.first;
        int c = wc.second;
        que.push(PI(d + c, w));
      }
    }
  }
  int mi = inf;
  REP(i, 0, n) {
    mi = min(mi, dp[i][(1 << k) - 1]);
  }
  return mi;   
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  while(cin >> h >> w && h && w) {
    vector<VI> s(h, VI(w));
    REP(i, 0, h) {
      REP(j, 0, w) {
        cin >> s[i][j];
      }
    }
    vector<vector<PI> > g(h * w);
    int dx[4] = {1, 0, -1, 0};
    int dy[4] = {0, 1, 0, -1};
    VI term;
    REP(i, 0, h) {
      REP(j, 0, w) {
        int v = i * w + j;
        if (s[i][j] == 1) term.push_back(v);
        REP(d, 0, 4) {
          int nx = i + dx[d];
          int ny = j + dy[d];
          if (nx < 0 || nx >= h || ny < 0 || ny >= w) continue;
          int u = nx * w + ny;
          g[v].push_back(PI(u, 1));
        }
      }
    }
    cout << h * w - 1 - steiner_tree(g, term) << endl;
  }
}
