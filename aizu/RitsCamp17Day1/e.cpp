#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;

const int N = 2438;
int dpma[N][N];
int dpmi[N][N];
const int inf = 1e8;
vector<int> solve(const vector<int> &a) {
  int n = a.size();
  vector<int> ret(n + 1);
  rep(i, N) rep(j, N) { dpma[i][j] = - inf; dpmi[i][j] = inf; }
  dpma[n][0] = 0;
  dpmi[n][0] = 0;

  for (int i = n - 1; i >= 0; --i) {
    rep(j, n - i + 1) {
      dpma[i][j] = max(dpma[i][j], dpma[i + 1][j] + a[i]);
      dpmi[i][j] = min(dpmi[i][j], dpmi[i + 1][j] + a[i]);
      if (j >= 1) {
	dpma[i][j] = max(dpma[i][j], -dpmi[i + 1][j - 1] + a[i]);
	dpmi[i][j] = min(dpmi[i][j], -dpma[i + 1][j - 1] + a[i]);
      }
    }
  }

  rep(i, n + 1) { ret[i] = max(abs(dpmi[0][i]), abs(dpma[0][i])); }
  return ret;
}

int main(void) {
  string s;
  int k;
  cin >> s >> k;
  vector<int> rl, ud;
  rep(i, s.length()) {
    if (s[i] == 'R' || s[i] == 'L') {
      rl.push_back(s[i] == 'R' ? 1 : -1);
    } else {
      ud.push_back(s[i] == 'U' ? 1 : -1);
    }
  }
  vector<int> rl_res = solve(rl);
  vector<int> ud_res = solve(ud);
  int ma = 0;
  rep(i, rl_res.size()) {
    rep(j, ud_res.size()) {
      if (i + j <= k) { ma = max(ma, rl_res[i] + ud_res[j]); }
    }
  }
  cout << ma << endl;
}
