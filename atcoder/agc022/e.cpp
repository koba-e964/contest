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

const int N = 300100;
ll dp[3][2][3][N];

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  s += '1';
  dp[0][0][0][0] = 1;
  REP(i, 0, s.length()) {
    REP(a, 0, 3) {
      REP(b, 0, 2) {
	REP(c, 0, 3) {
	  if (s[i] != '0') { // 1
	    int nb = c % 2 == 1 ? b : (b + 1) % 2;
	    int na = a + (c == 0 && a == b ? 1 : 0);
	    add(dp[na][nb][0][i + 1], dp[a][b][c][i]);
	  }
	  if (s[i] != '1') {
	    int nc = c + 1;
	    if (nc >= 3) nc = 1;
	    add(dp[a][b][nc][i + 1], dp[a][b][c][i]);
	  }
	}
      }
    }
  }
  ll tot = 0;
  REP(b, 0, 2) {
    REP(c, 0, 3) {
      add(tot, dp[2][b][c][s.length()]);
    }
  }
  cout << tot << endl;
}
