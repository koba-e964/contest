#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef pair<int, int> PI;
typedef pair<PI, int> PPII;

const int N = 370;
ll dp[N][N];
ll ndp[N][N];
const ll inf = 1e15;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  while (cin >> n && n) {
    vector<PPII> pool;
    REP(i, 0, n) {
      int u, v, w;
      cin >> u >> v >> w;
      v++; // [u, v)
      pool.push_back(PPII(PI(u, v), w));
    }
    sort(pool.begin(), pool.end());
    REP(i, 0, N) {
      REP(j, 0, N) {
	dp[i][j] = -inf;
	ndp[i][j] = -inf;
      }
    }
    dp[0][0] = 0;
    REP(i, 0, n) {
      PPII cur = pool[i];
      int u = cur.first.first;
      int v = cur.first.second;
      ll w = cur.second;
      REP(j, 0, N) {
	ll &ma = ndp[v][j];
	ma = dp[v][j];
	REP(k, 0, u + 1) {
	  ma = max(ma, dp[k][j] + w); 
	}
      }
      REP(j, 0, N) {
	ll &ma = ndp[j][v];
	ma = dp[j][v];
	REP(k, 0, u + 1) {
	  ma = max(ma, dp[j][k] + w); 
	}
      }
      REP(j, 0, N) {
	REP(k, 0, N) {
	  dp[j][k] = max(dp[j][k], ndp[j][k]);
	}
      }
    }
    ll ma = 0;
    REP(i, 0, N) {
      REP(j, 0, N) {
	ma = max(ma, dp[i][j]);
      }
    }
    cout << ma << "\n";
  }
}
