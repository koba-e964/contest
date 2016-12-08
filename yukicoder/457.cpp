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

const int N = 100000;
int dp[N][3];

int solve(const string &s) {
  int n = s.length();
  VI smiley(n);
  VI num_cl(n + 1);
  num_cl[n] = 0;
  for (int i = n - 1; i >= 0; --i) {
    num_cl[i] = num_cl[i + 1] + (s[i] == ')' ? 1 : 0);
  }
  string pat = "^^*";
  REP(i, 0, n + 1) {
    REP(j, 0, 3) {
      dp[i][j] = n;
    }
  }
  for (int j = 2; j >= 0; --j) {
    for (int i = n - 1; i >= 0; --i) {
      int r = dp[i + 1][j];
      if (s[i] == pat[j]) {
	r = min(r, j == 2 ? i + 1 : dp[i + 1][j + 1]);
      }
      dp[i][j] = r;
    }
  }
  REP(i, 0, n) {
    smiley[i] = dp[i][0];
  }
  ll tot = 0;
  REP(i, 0, n) {
    if (s[i] == '(') {
      tot += num_cl[smiley[i]];
    }
  }
  return tot;
}

int main(void){
  string s;
  cin >> s;
  cout << solve(s) << " ";
  reverse(s.begin(), s.end());
  REP(i, 0, s.length()) {
    if (s[i] == '(' || s[i] == ')') {
      s[i] = '(' + ')' - s[i];
    }
  }
  cout << solve(s) << endl;
}
