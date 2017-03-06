#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int solve(const string &s) {
  string pat = "problemgoodproblem";
  reverse(pat.begin(), pat.end());
  int n = s.length();
  int inf = 1e7;
  vector<VI> dp(n + 1, VI(18, inf));
  dp[0][0] = 0;
  REP(i, 1, n + 1) {
    REP(j, 0, 18) {
      int mi = inf;
      if ((j == 0 || j == 7)) {
	mi = min(mi, dp[i - 1][j]);
      }
      if (j >= 1) {
	mi = min(mi, dp[i - 1][j - 1] + (s[i - 1] == pat[j - 1] ? 0 : 1));
      }
      if (j == 11) {
	REP(k, 11, 18) {
	  mi = min(mi, dp[i - 1][k] + (s[i - 1] == pat[k] ? 1 : 0));
	}
      }
      dp[i][j] = mi;
    }
  }
  int mi = inf;
  REP(j, 11, 18) {
    mi = min(mi, dp[n][j]);
  }
  return mi;
}

int main(void){
  int t;
  cin >> t;
  while (t--) {
    string s;
    cin >> s;
    reverse(s.begin(), s.end());
    cout << solve(s) << endl;
  }
}
