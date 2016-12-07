#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};


int main(void){
  int n, m, w;
  cin >> n >> m >> w;
  VI a(n);
  REP(i, 0, n) { cin >> a[i]; }
  VI b(n);
  REP(i, 0, n) { cin >> b[i]; }
  UnionFind uf(n);
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    uf.unite(x, y);
  }
  vector<VI> conn(n);
  REP(i, 0, n) {
    conn[uf.root(i)].push_back(i);
  }
  const int W = 1010;
  VI dp(W);
  int w_sofar = 0;
  REP(i, 0, n) {
    if (conn[i].size() == 0) {
      continue;
    }
    VI dp2(dp.begin(), dp.end());
    int tot_w = 0;
    int tot_b = 0;
    REP(j, 0, conn[i].size()) {
      int v = conn[i][j];
      tot_w += a[v];
      tot_b += b[v];
      REP(k, 0, W) {
	if (k >= a[v]) {
	  dp2[k] = max(dp2[k], dp[k - a[v]] + b[v]);
	}
      }
    }
    REP(k, 0, W) {
      if (k >= tot_w) {
	dp2[k] = max(dp2[k], dp[k - tot_w] + tot_b);
      }
    }
    dp = dp2;
    w_sofar += tot_w;
    if (0) {
      cerr << "dp[" << i + 1 << "]:";
      REP(j, 0, w_sofar) {
	cerr << " " << dp[j];
      }
      cerr << endl;
    }
  }
  int ma = 0;
  REP(i, 0, w + 1) {
    ma = max(ma, dp[i]);
  }
  cout << ma << endl;
}
