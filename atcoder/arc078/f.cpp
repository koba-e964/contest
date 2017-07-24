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


const ll inf = 1e14;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<vector<PI> > edges(n);
  vector<vector<int> > mat(n, VI(n, 0));
  REP(i, 0, m) {
    int a, b, c;
    cin >> a >> b >> c;
    a--, b--;
    edges[a].push_back(PI(b, c));
    edges[b].push_back(PI(a, c));
    mat[a][b] = c;
    mat[b][a] = c;
  }
  VL conn(1 << n);
  REP(bits, 0, 1 << n) {
    ll sum = 0;
    REP(u, 0, n) {
      if ((bits & 1 << u) == 0) { continue; }
      REP(v, u + 1, n) {
	int c = mat[u][v];
	if (bits & 1 << v) {
	  sum += c;
	}
      }
    }
    conn[bits] = sum; // All edges are counted once
  }
  vector<VL> dp(1 << n, VL(n, -inf));
  REP(bits, 1, 1 << n) {
    REP(i, 0, n) {
      if ((bits & 1 << i) == 0) { continue; }
      ll ma = -inf;
      REP(u, 0, n) {
	if (u == i || (bits & 1 << u) == 0) { continue; }
	if (mat[u][i] == 0) { continue; }
	int iex = bits ^ 1 << i ^ 1 << u;
	for (int sub = iex; true; sub = (sub - 1) & iex) {
	  ll tmp = dp[sub ^ 1 << u][u] + conn[bits - sub - (1 << u)] + mat[u][i];
	  ma = max(ma, tmp);
	  if (sub == 0) {
	    break;
	  }
	}
      }
      if (i == 0) {
	ma = max(ma, conn[bits]);
      }
      // cerr << "dp[" << bits << "][" << i << "]=" << ma << "\n";
      dp[bits][i] = ma;
    }
  }
  int ma = dp[(1 << n) - 1][n - 1];
  cout << conn[(1 << n) - 1] - ma << "\n";
}
