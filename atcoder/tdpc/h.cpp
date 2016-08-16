#include <cassert>
#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
int nn,ww,cc;
const int N=101;
const int W=10001;
const int C=50;
typedef long long ll;
typedef pair<int, int> PI;
ll dp[W][C + 1], dp2[W][C + 1];
vector<PI> seg[C];
#define REP(i, s, n) for (int i = int(s); i < int(n); ++i)
int main(void){
  cin >> nn >> ww >> cc;
  REP(i, 0, nn) {
    int w, v, c;
    cin >> w >> v >> c;
    c--;
    seg[c].push_back(PI(w, v));
  }
  REP(c, 0, C) {
    for (int d = C; d >= 1; --d) {
      REP(i, 0, W) {
	dp2[i][d] = dp[i][d - 1];
      }
      REP(i, 0, seg[c].size()) {
	PI wv = seg[c][i];
        for (int j = W - 1; j >= 0; --j) {
	  if (j + wv.first < W) {
	    dp2[j + wv.first][d] = max(dp2[j + wv.first][d], dp2[j][d] + wv.second);
	  }
	}
      }
    }
    REP(d, 0, C + 1) {
      REP(j, 0, W) {
	dp[j][d] = max(dp[j][d], dp2[j][d]);
      }
    }
  }
  ll ma = 0;
  REP(i, 0, cc + 1) {
    REP(j, 0, ww + 1) {
      ma = max(ma, dp[j][i]);
    }
  }
  cout << ma << endl;
}
