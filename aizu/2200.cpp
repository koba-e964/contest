#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
const int N = 210;
const int R = 1010;
ll ds[N][N], dl[N][N];

ll dp[R][N];

ll inf = 1e16;
const int DEBUG = 0;

int main(void){
  int n, m;
  while (cin >> n >> m && n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	ds[i][j] = dl[i][j] = i == j ? 0 : inf;
      }
    }
    REP(loop_cnt, 0, m) {
      int i, j;
      ll t;
      char sl;
      cin >> i >> j >> t >> sl;
      i--, j--;
      if (sl == 'S') {
	ds[i][j] = min(ds[i][j], t);
	ds[j][i] = min(ds[j][i], t);
      } else {
	dl[i][j] = min(dl[i][j], t);
	dl[j][i] = min(dl[j][i], t);
      }
    }
    REP(k, 0, n) {
      REP(i, 0, n) {
	REP(j, 0, n) {
	  ds[i][j] = min(ds[i][j], ds[i][k] + ds[k][j]);
	  dl[i][j] = min(dl[i][j], dl[i][k] + dl[k][j]);
	}
      }
    }
    int r;
    cin >> r;
    VI z(r);
    REP(i, 0, r) {
      cin >> z[i];
      z[i]--;
    }
    REP(i, 0, R) {
      REP(j, 0, N) {
	dp[i][j] = inf;
      }
    }
    dp[0][z[0]] = 0;
    REP(i, 1, r) {
      REP(j, 0, n) {
	REP(k, 0, n) {
	  ll sdist = dl[z[i - 1]][j] + ds[j][k] + dl[k][z[i]];
	  dp[i][k] = min(dp[i][k], dp[i - 1][j] + sdist);
	  if (j == k) {
	    ll ldist = dl[z[i - 1]][z[i]];
	    dp[i][j] = min(dp[i][j], dp[i - 1][j] + ldist);
	  }
	  if (DEBUG) {
	    cerr << "dp[" << i << "," << k << "]=" << dp[i][k] << endl;
	  }
	}
      }
    }
    ll mi = inf;
    REP(i, 0, n) {
      mi = min(mi, dp[r - 1][i]);
    }
    cout << mi << endl;
  }
}
