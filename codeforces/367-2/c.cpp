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


const int N = 100100;
ll dp[N][2];
string t[N][2];
const ll inf = 1e16;
int main(void){
  int n;
  cin >> n;
  VL c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
    t[i + 1][0] = s[i];
    t[i + 1][1] = s[i];
    reverse(t[i + 1][1].begin(), t[i + 1][1].end());
  }
  t[0][0] = "";
  t[0][1] = "";
  REP(i, 0, n + 1) {
    dp[i][0] = dp[i][1] = inf;
  }
  dp[0][0] = 0;
  REP(i, 1, n + 1) {
    REP(j, 0, 2) {
      REP(k, 0, 2) {
	if (t[i - 1][j] <= t[i][k]) {
	  dp[i][k] = min(dp[i][k], dp[i - 1][j] + c[i - 1] * k);
	}
      }
    }
  }
  ll res = min(dp[n][0], dp[n][1]);
  cout << (res == inf ? -1 : res) << endl;
}
