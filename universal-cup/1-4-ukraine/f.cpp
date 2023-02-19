#include<algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<VI> g(2 * n);
  REP(_, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  REP(i, 0, 2 * n) {
    sort(g[i].begin(), g[i].end());
  }
  // http://lealgorithm.blogspot.com/2019/06/blog-post.html
  vector<vector<bool>> dp(2 * n, vector<bool>(2 * n));
  REP(i, 0, 2 * n) {
    REP(a, 0, g[i].size()) {
      int u = g[i][a];
      REP(b, 0, a) {
        int v = g[i][b];
        if (dp[u][v]) {
          cout << "2\n";
          return 0;
        }
        dp[u][v] = true;
      }
    }
  }
  cout << "3\n";
}
