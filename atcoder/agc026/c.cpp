#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string s;
  cin >> n >> s;
  ll tot = 0;
  string res = s.substr(n);
  reverse(res.begin(), res.end());
  REP(bits, 0, 1 << n) {
    string tap, ris;
    REP(i, 0, n) {
      if (bits & 1 << i) tap += s[i];
      else ris += s[i];
    }
    int a = tap.size();
    int b = ris.size();
    vector<VI> dp(a + 1, VI(b + 1, 0));
    dp[0][0] = 1;
    REP(i, 0, a + 1) {
      REP(j, 0, b + 1) {
	if (i == 0 && j == 0) continue;
	if (i > 0 && res[i + j - 1] == tap[i - 1]) {
	  dp[i][j] += dp[i - 1][j];
	}
	if (j > 0 && res[i + j - 1] == ris[j - 1]) {
	  dp[i][j] += dp[i][j - 1];
	}
      }
    }
    tot += dp[a][b];
  }
  cout << tot << endl;
}
